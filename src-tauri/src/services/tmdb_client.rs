use crate::error::Error;
use crate::models::tmdb::*;
use crate::services::cache::CacheService;
use reqwest::Client;
use std::env;

const BASE_URL: &str = "https://api.themoviedb.org/3";
pub const IMAGE_BASE_URL: &str = "https://image.tmdb.org/t/p";

/// Environment variable name for TMDB read access token
const TMDB_TOKEN_ENV: &str = "TMDB_READ_TOKEN";

pub struct TmdbClient {
    client: Client,
    cache: CacheService,
    token: String,
}

impl TmdbClient {
    /// Creates a new TmdbClient.
    /// Token priority: explicit param > TMDB_READ_TOKEN env var
    /// Returns Error::InvalidApiKey if no token is available.
    pub fn new(token: Option<String>) -> Result<Self, Error> {
        let resolved_token = token
            .or_else(|| env::var(TMDB_TOKEN_ENV).ok())
            .ok_or(Error::InvalidApiKey)?;

        Ok(Self {
            client: Client::new(),
            cache: CacheService::new(),
            token: resolved_token,
        })
    }

    pub async fn search_movies(
        &self,
        query: &str,
        page: u32,
    ) -> Result<SearchResults<MovieResult>, Error> {
        let cache_key = CacheService::make_key("search_movie", &[("q", query), ("p", &page.to_string())]);

        if let Some(cached) = self.cache.get::<SearchResults<MovieResult>>(&cache_key).await {
            return Ok(cached);
        }

        let url = format!(
            "{BASE_URL}/search/movie?query={}&page={}",
            urlencoding::encode(query),
            page
        );

        let data: SearchResults<MovieResult> = self.fetch(&url).await?;
        self.cache.set(&cache_key, &data).await?;
        Ok(data)
    }

    pub async fn search_people(
        &self,
        query: &str,
        page: u32,
    ) -> Result<SearchResults<PersonResult>, Error> {
        let cache_key = CacheService::make_key("search_person", &[("q", query), ("p", &page.to_string())]);

        if let Some(cached) = self.cache.get::<SearchResults<PersonResult>>(&cache_key).await {
            return Ok(cached);
        }

        let url = format!(
            "{BASE_URL}/search/person?query={}&page={}",
            urlencoding::encode(query),
            page
        );

        let data: SearchResults<PersonResult> = self.fetch(&url).await?;
        self.cache.set(&cache_key, &data).await?;
        Ok(data)
    }

    pub async fn get_movie(&self, id: u32) -> Result<MovieDetails, Error> {
        let cache_key = CacheService::make_key("movie_details", &[("id", &id.to_string())]);

        if let Some(cached) = self.cache.get::<MovieDetails>(&cache_key).await {
            return Ok(cached);
        }

        let url = format!("{BASE_URL}/movie/{id}?append_to_response=credits");

        let data: MovieDetails = self.fetch(&url).await?;
        self.cache.set(&cache_key, &data).await?;
        Ok(data)
    }

    pub async fn get_person(&self, id: u32) -> Result<PersonDetails, Error> {
        let cache_key = CacheService::make_key("person_details", &[("id", &id.to_string())]);

        if let Some(cached) = self.cache.get::<PersonDetails>(&cache_key).await {
            return Ok(cached);
        }

        let url = format!("{BASE_URL}/person/{id}?append_to_response=movie_credits");

        let data: PersonDetails = self.fetch(&url).await?;
        self.cache.set(&cache_key, &data).await?;
        Ok(data)
    }

    pub async fn test_token(&self) -> Result<bool, Error> {
        let url = format!("{BASE_URL}/configuration");
        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|e| Error::NetworkError(e.to_string()))?;

        match response.status().as_u16() {
            200 => Ok(true),
            401 => Ok(false),
            429 => Err(Error::RateLimited),
            code => Err(Error::ApiError(code)),
        }
    }

    pub fn image_url(path: &str, size: &str) -> String {
        format!("{IMAGE_BASE_URL}/{size}{path}")
    }

    async fn fetch<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|e| Error::NetworkError(e.to_string()))?;

        match response.status().as_u16() {
            200 => response.json::<T>().await.map_err(|e| Error::NetworkError(e.to_string())),
            401 => Err(Error::InvalidApiKey),
            429 => Err(Error::RateLimited),
            code => Err(Error::ApiError(code)),
        }
    }
}

// Percent-encode query string values
mod urlencoding {
    pub fn encode(s: &str) -> String {
        s.chars()
            .flat_map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
                    vec![c]
                } else if c == ' ' {
                    vec!['+']
                } else {
                    let bytes = c.to_string().into_bytes();
                    bytes.iter().flat_map(|b| {
                        format!("%{b:02X}").chars().collect::<Vec<_>>()
                    }).collect()
                }
            })
            .collect()
    }
}
