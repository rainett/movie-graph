use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::error::Error;
use crate::models::project::RecentProject;
use crate::services::file_io::FileService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub tmdb_api_key: Option<String>,
    pub tmdb_read_access_token: Option<String>, // Bearer token; overrides embedded default
    pub recent_projects: Vec<RecentProject>,
    pub max_recent_projects: usize,
    pub auto_save: bool,
    pub auto_save_interval_ms: u64,
    pub auto_backup: bool,
    pub max_backups: usize,
    pub sound_enabled: bool,
    pub reduced_motion: bool,
    pub theme: String,
    pub device_layout_preset: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            tmdb_api_key: None,
            tmdb_read_access_token: None,
            recent_projects: vec![],
            max_recent_projects: 10,
            auto_save: true,
            auto_save_interval_ms: 2000,
            auto_backup: true,
            max_backups: 5,
            sound_enabled: true,
            reduced_motion: false,
            theme: "default".to_string(),
            device_layout_preset: "default".to_string(),
        }
    }
}

impl AppConfig {
    pub async fn load() -> Result<Self, Error> {
        let path = Self::config_path()?;
        if FileService::exists(&path).await {
            FileService::read_json(&path).await
        } else {
            Ok(Self::default())
        }
    }

    pub async fn save(&self) -> Result<(), Error> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        FileService::write_json(&path, self).await
    }

    fn config_path() -> Result<PathBuf, Error> {
        dirs::config_dir()
            .map(|d| d.join("movie-graph").join("config.json"))
            .ok_or_else(|| Error::Unknown("Could not determine config directory".to_string()))
    }

    pub fn add_recent_project(&mut self, path: PathBuf, name: String) {
        self.recent_projects.retain(|p| p.path != path);
        self.recent_projects.insert(
            0,
            RecentProject {
                path,
                name,
                last_opened: chrono::Utc::now().to_rfc3339(),
            },
        );
        self.recent_projects.truncate(self.max_recent_projects);
    }
}
