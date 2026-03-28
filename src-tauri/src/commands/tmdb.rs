use crate::error::Error;
use crate::models::tmdb::*;
use crate::services::tmdb_client::TmdbClient;

#[tauri::command]
pub async fn search_movies(
    query: String,
    page: Option<u32>,
    api_key: Option<String>,
) -> Result<SearchResults<MovieResult>, Error> {
    let client = TmdbClient::new(api_key)?;
    client.search_movies(&query, page.unwrap_or(1)).await
}

#[tauri::command]
pub async fn search_people(
    query: String,
    page: Option<u32>,
    api_key: Option<String>,
) -> Result<SearchResults<PersonResult>, Error> {
    let client = TmdbClient::new(api_key)?;
    client.search_people(&query, page.unwrap_or(1)).await
}

#[tauri::command]
pub async fn get_movie_details(
    tmdb_id: u32,
    api_key: Option<String>,
) -> Result<MovieDetails, Error> {
    let client = TmdbClient::new(api_key)?;
    client.get_movie(tmdb_id).await
}

#[tauri::command]
pub async fn get_person_details(
    tmdb_id: u32,
    api_key: Option<String>,
) -> Result<PersonDetails, Error> {
    let client = TmdbClient::new(api_key)?;
    client.get_person(tmdb_id).await
}

#[tauri::command]
pub async fn test_api_key(api_key: String) -> Result<bool, Error> {
    let client = TmdbClient::new(Some(api_key))?;
    client.test_token().await
}
