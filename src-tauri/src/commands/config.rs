use crate::config::app_config::AppConfig;
use crate::error::Error;

#[tauri::command]
pub async fn get_config() -> Result<AppConfig, Error> {
    AppConfig::load().await
}

#[tauri::command]
pub async fn save_config(config: AppConfig) -> Result<(), Error> {
    config.save().await
}
