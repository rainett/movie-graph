mod commands;
mod config;
mod error;
mod models;
mod services;

use commands::project::{
    create_project, get_recent_projects, open_project, save_project, validate_project,
};
use commands::tmdb::{
    get_movie_details, get_person_details, search_movies, search_people, test_api_key,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            create_project,
            open_project,
            save_project,
            get_recent_projects,
            validate_project,
            search_movies,
            search_people,
            get_movie_details,
            get_person_details,
            test_api_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
