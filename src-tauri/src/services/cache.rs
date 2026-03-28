use crate::error::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

const CACHE_TTL_SECS: i64 = 7 * 24 * 60 * 60; // 7 days

#[derive(Serialize, Deserialize)]
struct CachedEntry<T> {
    data: T,
    cached_at: i64, // Unix timestamp
}

pub struct CacheService {
    cache_dir: PathBuf,
}

impl CacheService {
    pub fn new() -> Self {
        let cache_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("movie-graph")
            .join("cache");
        Self { cache_dir }
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let path = self.entry_path(key);
        let content = fs::read_to_string(&path).await.ok()?;
        let entry: CachedEntry<T> = serde_json::from_str(&content).ok()?;

        let now = chrono::Utc::now().timestamp();
        if now - entry.cached_at > CACHE_TTL_SECS {
            let _ = fs::remove_file(&path).await;
            return None;
        }

        Some(entry.data)
    }

    pub async fn set<T: Serialize>(&self, key: &str, data: &T) -> Result<(), Error> {
        fs::create_dir_all(&self.cache_dir).await?;
        let entry = CachedEntry {
            data,
            cached_at: chrono::Utc::now().timestamp(),
        };
        let content = serde_json::to_string(&entry)?;
        fs::write(self.entry_path(key), content).await?;
        Ok(())
    }

    pub fn make_key(endpoint: &str, params: &[(&str, &str)]) -> String {
        let parts: Vec<String> = std::iter::once(endpoint.to_string())
            .chain(params.iter().map(|(k, v)| format!("{k}_{v}")))
            .collect();
        let raw = parts.join("_");
        // Keep only alphanumeric, dash, underscore — safe as filename
        raw.chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
            .collect()
    }

    fn entry_path(&self, key: &str) -> PathBuf {
        self.cache_dir.join(format!("{key}.json"))
    }
}
