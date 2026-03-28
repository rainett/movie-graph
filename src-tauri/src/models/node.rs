use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieNode {
    pub id: String,
    pub tmdb_id: u32,
    pub title: String,
    pub year: u16,
    pub status: Status,
    pub my_rating: Option<u8>,
    pub poster: String,
    pub poster_options: Vec<String>,
    pub notes: String,
    pub position: Position,
    pub added_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorNode {
    pub id: String,
    pub tmdb_id: u32,
    pub name: String,
    pub photo: String,
    pub photo_options: Vec<String>,
    pub notes: String,
    pub position: Position,
    pub added_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Watched,
    Watching,
    WantToWatch,
    Dropped,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
