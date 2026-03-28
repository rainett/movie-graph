use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults<T> {
    pub page: u32,
    pub total_pages: u32,
    pub total_results: u32,
    pub results: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieResult {
    pub id: u32,
    pub title: String,
    pub release_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub vote_average: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonResult {
    pub id: u32,
    pub name: String,
    pub profile_path: Option<String>,
    pub known_for_department: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieDetails {
    pub id: u32,
    pub title: String,
    pub release_date: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub runtime: Option<u32>,
    pub vote_average: f32,
    pub genres: Vec<Genre>,
    pub credits: Credits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonDetails {
    pub id: u32,
    pub name: String,
    pub biography: Option<String>,
    pub profile_path: Option<String>,
    pub birthday: Option<String>,
    pub place_of_birth: Option<String>,
    pub movie_credits: MovieCredits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credits {
    pub cast: Vec<CastMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastMember {
    pub id: u32,
    pub name: String,
    pub character: Option<String>,
    pub profile_path: Option<String>,
    pub order: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieCredits {
    pub cast: Vec<MovieCredit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieCredit {
    pub id: u32,
    pub title: String,
    pub release_date: Option<String>,
    pub character: Option<String>,
    pub poster_path: Option<String>,
    #[serde(default)]
    pub popularity: Option<f32>,
    #[serde(default)]
    pub vote_average: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    Movie,
    Person,
}
