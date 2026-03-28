use crate::config::app_config::AppConfig;
use crate::error::Error;
use crate::models::tmdb::*;
use crate::services::tmdb_client::TmdbClient;

/// Token priority: explicit param > AppConfig.tmdb_read_access_token > TMDB_READ_TOKEN env var
async fn resolve_token(explicit: Option<String>) -> Option<String> {
    if explicit.is_some() {
        return explicit;
    }
    AppConfig::load().await.ok().and_then(|c| c.tmdb_read_access_token)
}

#[tauri::command]
pub async fn search_movies(
    query: String,
    page: Option<u32>,
    api_key: Option<String>,
) -> Result<SearchResults<MovieResult>, Error> {
    let client = TmdbClient::new(resolve_token(api_key).await)?;
    client.search_movies(&query, page.unwrap_or(1)).await
}

#[tauri::command]
pub async fn search_people(
    query: String,
    page: Option<u32>,
    api_key: Option<String>,
) -> Result<SearchResults<PersonResult>, Error> {
    let client = TmdbClient::new(resolve_token(api_key).await)?;
    client.search_people(&query, page.unwrap_or(1)).await
}

#[tauri::command]
pub async fn get_movie_details(
    tmdb_id: u32,
    api_key: Option<String>,
) -> Result<MovieDetails, Error> {
    let client = TmdbClient::new(resolve_token(api_key).await)?;
    client.get_movie(tmdb_id).await
}

#[tauri::command]
pub async fn get_person_details(
    tmdb_id: u32,
    api_key: Option<String>,
) -> Result<PersonDetails, Error> {
    let client = TmdbClient::new(resolve_token(api_key).await)?;
    client.get_person(tmdb_id).await
}

#[tauri::command]
pub async fn test_api_key(api_key: String) -> Result<bool, Error> {
    let client = TmdbClient::new(Some(api_key))?;
    client.test_token().await
}
