use std::path::PathBuf;

use crate::config::app_config::AppConfig;
use crate::error::Error;
use crate::models::project::{Manifest, Project, RecentProject, ValidationResult};
use crate::services::file_io::FileService;

#[tauri::command]
pub async fn pick_folder(app: tauri::AppHandle) -> Result<Option<String>, Error> {
    use tauri_plugin_dialog::DialogExt;
    let folder = tokio::task::spawn_blocking(move || {
        app.dialog()
            .file()
            .set_title("Select Project Folder")
            .blocking_pick_folder()
    })
    .await
    .map_err(|e| Error::Unknown(e.to_string()))?;

    Ok(folder.map(|f| f.to_string()))
}

#[tauri::command]
pub async fn create_project(path: PathBuf, name: String) -> Result<Project, Error> {
    if FileService::exists(&path.join("manifest.json")).await {
        return Err(Error::InvalidProject(
            "Directory already contains a project".to_string(),
        ));
    }

    FileService::create_project_structure(&path).await?;

    let manifest = Manifest::new(name.clone());
    FileService::write_json(&path.join("manifest.json"), &manifest).await?;

    // Initialize empty data files
    let empty_movies: Vec<serde_json::Value> = vec![];
    let empty_actors: Vec<serde_json::Value> = vec![];
    let empty_edges: Vec<serde_json::Value> = vec![];
    FileService::write_json(&path.join("data/movies.json"), &empty_movies).await?;
    FileService::write_json(&path.join("data/actors.json"), &empty_actors).await?;
    FileService::write_json(&path.join("data/edges.json"), &empty_edges).await?;

    // Add to recent projects
    let mut config = AppConfig::load().await.unwrap_or_default();
    config.add_recent_project(path.clone(), name);
    let _ = config.save().await;

    Ok(Project {
        path,
        manifest,
        movies: vec![],
        actors: vec![],
        edges: vec![],
    })
}

#[tauri::command]
pub async fn open_project(path: PathBuf) -> Result<Project, Error> {
    let manifest_path = path.join("manifest.json");

    if !FileService::exists(&manifest_path).await {
        return Err(Error::ProjectNotFound(path.to_string_lossy().to_string()));
    }

    let manifest: Manifest = FileService::read_json(&manifest_path).await?;

    let movies = FileService::read_json(&path.join("data/movies.json"))
        .await
        .unwrap_or_default();
    let actors = FileService::read_json(&path.join("data/actors.json"))
        .await
        .unwrap_or_default();
    let edges = FileService::read_json(&path.join("data/edges.json"))
        .await
        .unwrap_or_default();

    // Update recent projects
    let mut config = AppConfig::load().await.unwrap_or_default();
    config.add_recent_project(path.clone(), manifest.name.clone());
    let _ = config.save().await;

    Ok(Project {
        path,
        manifest,
        movies,
        actors,
        edges,
    })
}

#[tauri::command]
pub async fn save_project(project: Project) -> Result<(), Error> {
    let path = &project.path;

    let mut manifest = project.manifest.clone();
    manifest.modified_at = chrono::Utc::now().to_rfc3339();

    FileService::write_json_atomic(&path.join("manifest.json"), &manifest).await?;
    FileService::write_json_atomic(&path.join("data/movies.json"), &project.movies).await?;
    FileService::write_json_atomic(&path.join("data/actors.json"), &project.actors).await?;
    FileService::write_json_atomic(&path.join("data/edges.json"), &project.edges).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_recent_projects() -> Result<Vec<RecentProject>, Error> {
    let config = AppConfig::load().await.unwrap_or_default();
    // Filter out projects that no longer exist
    let recent: Vec<RecentProject> = config
        .recent_projects
        .into_iter()
        .filter(|p| p.path.exists())
        .collect();
    Ok(recent)
}

#[tauri::command]
pub async fn backup_project(project_path: String) -> Result<String, Error> {
    let source = PathBuf::from(&project_path);
    if !FileService::exists(&source).await {
        return Err(Error::ProjectNotFound(project_path));
    }

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let parent = source
        .parent()
        .ok_or_else(|| Error::Unknown("Cannot determine parent directory".to_string()))?;
    let folder_name = source
        .file_name()
        .ok_or_else(|| Error::Unknown("Cannot determine folder name".to_string()))?
        .to_string_lossy();

    let backup_name = format!("{}_backup_{}", folder_name, timestamp);
    let backup_path = parent.join(&backup_name);

    copy_dir_all(&source, &backup_path).await?;

    Ok(backup_path.to_string_lossy().to_string())
}

async fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> Result<(), Error> {
    tokio::fs::create_dir_all(dst).await?;
    let mut entries = tokio::fs::read_dir(src).await?;
    while let Some(entry) = entries.next_entry().await? {
        let ty = entry.file_type().await?;
        if ty.is_dir() {
            Box::pin(copy_dir_all(&entry.path(), &dst.join(entry.file_name()))).await?;
        } else {
            tokio::fs::copy(entry.path(), dst.join(entry.file_name())).await?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn validate_project(path: PathBuf) -> Result<ValidationResult, Error> {
    let mut errors = Vec::new();

    if !FileService::exists(&path.join("manifest.json")).await {
        errors.push("Missing manifest.json".to_string());
    }
    if !FileService::exists(&path.join("data")).await {
        errors.push("Missing data directory".to_string());
    }
    if !FileService::exists(&path.join("images")).await {
        errors.push("Missing images directory".to_string());
    }

    Ok(ValidationResult {
        valid: errors.is_empty(),
        errors,
    })
}
