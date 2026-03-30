mod commands;
mod config;
mod error;
mod models;
mod services;

use commands::config::{get_config, save_config};
use commands::images::cache_image;
use commands::project::{
    backup_project, create_project, get_recent_projects, open_project, pick_folder, save_project,
    validate_project,
};
use commands::tmdb::{
    get_movie_details, get_person_details, search_movies, search_people, test_api_key,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load .env file if present (dev convenience; no-op if missing)
    let _ = dotenvy::dotenv();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            create_project,
            open_project,
            save_project,
            get_recent_projects,
            validate_project,
            pick_folder,
            backup_project,
            search_movies,
            search_people,
            get_movie_details,
            get_person_details,
            test_api_key,
            cache_image,
            get_config,
            save_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
