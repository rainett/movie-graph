use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // File errors
    #[error("Project not found: {0}")]
    ProjectNotFound(String),

    #[error("Invalid project structure: {0}")]
    InvalidProject(String),

    #[error("File read error: {0}")]
    FileRead(String),

    #[error("File write error: {0}")]
    FileWrite(String),

    #[error("Project corrupted: {0}")]
    CorruptedProject(String),

    // TMDB errors
    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("TMDB rate limit exceeded")]
    RateLimited,

    #[error("TMDB API error: {0}")]
    ApiError(u16),

    #[error("Network error: {0}")]
    NetworkError(String),

    // Image errors
    #[error("Invalid image format: {0}")]
    InvalidImage(String),

    #[error("Image too large: {0} bytes")]
    ImageTooLarge(u64),

    #[error("Image download failed: {0}")]
    ImageDownload(String),

    // General
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Tauri commands must return serializable errors
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
