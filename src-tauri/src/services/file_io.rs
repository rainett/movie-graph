use serde::{de::DeserializeOwned, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::error::Error;

pub struct FileService;

impl FileService {
    /// Create the standard project directory structure
    pub async fn create_project_structure(path: &Path) -> Result<(), Error> {
        fs::create_dir_all(path.join("data")).await?;
        fs::create_dir_all(path.join("images/posters")).await?;
        fs::create_dir_all(path.join("images/photos")).await?;
        fs::create_dir_all(path.join("images/custom")).await?;
        fs::create_dir_all(path.join(".backups")).await?;
        Ok(())
    }

    /// Read and deserialize a JSON file
    pub async fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, Error> {
        let content = fs::read_to_string(path)
            .await
            .map_err(|e| Error::FileRead(e.to_string()))?;
        serde_json::from_str(&content).map_err(|e| Error::CorruptedProject(e.to_string()))
    }

    /// Write data as pretty-printed JSON
    pub async fn write_json<T: Serialize>(path: &Path, data: &T) -> Result<(), Error> {
        let content = serde_json::to_string_pretty(data)
            .map_err(|e| Error::FileWrite(e.to_string()))?;
        fs::write(path, content)
            .await
            .map_err(|e| Error::FileWrite(e.to_string()))
    }

    /// Write JSON atomically: write to .tmp, then rename
    pub async fn write_json_atomic<T: Serialize>(path: &Path, data: &T) -> Result<(), Error> {
        let temp_path = path.with_extension("json.tmp");
        let content = serde_json::to_string_pretty(data)
            .map_err(|e| Error::FileWrite(e.to_string()))?;
        fs::write(&temp_path, content)
            .await
            .map_err(|e| Error::FileWrite(e.to_string()))?;
        fs::rename(&temp_path, path)
            .await
            .map_err(|e| Error::FileWrite(e.to_string()))
    }

    /// Check if a path exists
    pub async fn exists(path: &Path) -> bool {
        fs::metadata(path).await.is_ok()
    }

    /// Copy a file
    pub async fn copy_file(from: &Path, to: &Path) -> Result<(), Error> {
        fs::copy(from, to)
            .await
            .map(|_| ())
            .map_err(|e| Error::FileWrite(e.to_string()))
    }

    /// Delete a file
    pub async fn delete_file(path: &Path) -> Result<(), Error> {
        fs::remove_file(path)
            .await
            .map_err(|e| Error::FileWrite(e.to_string()))
    }

    /// List files in a directory (non-recursive)
    pub async fn list_files(dir: &Path) -> Result<Vec<PathBuf>, Error> {
        let mut entries = fs::read_dir(dir)
            .await
            .map_err(|e| Error::FileRead(e.to_string()))?;
        let mut files = Vec::new();
        while let Some(entry) = entries.next_entry().await.map_err(|e| Error::FileRead(e.to_string()))? {
            if entry.file_type().await.map(|t| t.is_file()).unwrap_or(false) {
                files.push(entry.path());
            }
        }
        Ok(files)
    }
}
