# Movie Graph - Backend / Tauri Specification

## Overview

The backend is built with Tauri 2.0, providing native desktop capabilities through a Rust core. The frontend (Svelte) communicates with the backend via Tauri's IPC (Inter-Process Communication) system.

**Runtime:** Tauri 2.0 (Rust)
**Role:** File system access, image processing, HTTP requests, system dialogs, app lifecycle

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Svelte Frontend                       │
│    (UI rendering, state management, user interaction)    │
└───────────────────────────┬─────────────────────────────┘
                            │ IPC (invoke/events)
┌───────────────────────────┴─────────────────────────────┐
│                     Tauri Core                           │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────┐    │
│  │   Commands  │ │   Events    │ │  Plugin System  │    │
│  └──────┬──────┘ └──────┬──────┘ └────────┬────────┘    │
│         │               │                  │             │
│  ┌──────┴───────────────┴──────────────────┴──────┐     │
│  │              Rust Backend Modules               │     │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────┐    │     │
│  │  │  FileIO  │ │  Images  │ │  TMDB Client │    │     │
│  │  └──────────┘ └──────────┘ └──────────────┘    │     │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────┐    │     │
│  │  │  Config  │ │  Cache   │ │   Backup     │    │     │
│  │  └──────────┘ └──────────┘ └──────────────┘    │     │
│  └─────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────┘
                            │
              ┌─────────────┴─────────────┐
              │      Operating System      │
              │  (File System, Network)    │
              └───────────────────────────┘
```

---

## Tauri Plugins Used

| Plugin | Purpose |
|--------|---------|
| `@tauri-apps/plugin-fs` | File system read/write |
| `@tauri-apps/plugin-dialog` | Native file/folder pickers |
| `@tauri-apps/plugin-http` | HTTP requests (TMDB API) |
| `@tauri-apps/plugin-os` | OS information |
| `@tauri-apps/plugin-process` | App lifecycle |
| `@tauri-apps/plugin-shell` | Open URLs in browser |

---

## Module Structure

```
src-tauri/
├── Cargo.toml
├── tauri.conf.json
├── capabilities/
│   └── default.json          # Permission definitions
├── src/
│   ├── main.rs               # Entry point
│   ├── lib.rs                # Module exports
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── project.rs        # Project CRUD operations
│   │   ├── nodes.rs          # Node operations
│   │   ├── edges.rs          # Edge operations
│   │   ├── images.rs         # Image operations
│   │   └── tmdb.rs           # TMDB API wrapper
│   ├── services/
│   │   ├── mod.rs
│   │   ├── file_io.rs        # Low-level file operations
│   │   ├── cache.rs          # API response caching
│   │   ├── backup.rs         # Auto-backup system
│   │   └── image_processor.rs # Resize, compress
│   ├── models/
│   │   ├── mod.rs
│   │   ├── project.rs        # Project structs
│   │   ├── node.rs           # Movie/Actor nodes
│   │   ├── edge.rs           # Relationships
│   │   └── tmdb.rs           # TMDB response types
│   ├── config/
│   │   ├── mod.rs
│   │   └── app_config.rs     # App-level settings
│   └── error.rs              # Custom error types
└── icons/                    # App icons
```

---

## Commands (IPC Interface)

Commands are invoked from frontend via `invoke()`. All commands are async.

### Project Commands

```rust
// src/commands/project.rs

/// Create new project in specified directory
#[tauri::command]
async fn create_project(path: PathBuf) -> Result<Project, Error>;

/// Open existing project from directory
#[tauri::command]
async fn open_project(path: PathBuf) -> Result<Project, Error>;

/// Save project state (manifest + data files)
#[tauri::command]
async fn save_project(project: Project) -> Result<(), Error>;

/// Get list of recent projects
#[tauri::command]
async fn get_recent_projects() -> Result<Vec<RecentProject>, Error>;

/// Validate project directory structure
#[tauri::command]
async fn validate_project(path: PathBuf) -> Result<ValidationResult, Error>;

/// Attempt recovery of corrupted project
#[tauri::command]
async fn recover_project(path: PathBuf) -> Result<Project, Error>;
```

### Node Commands

```rust
// src/commands/nodes.rs

/// Save all movie nodes (batch write)
#[tauri::command]
async fn save_movies(project_path: PathBuf, movies: Vec<MovieNode>) -> Result<(), Error>;

/// Save all actor nodes (batch write)
#[tauri::command]
async fn save_actors(project_path: PathBuf, actors: Vec<ActorNode>) -> Result<(), Error>;

/// Load all movie nodes
#[tauri::command]
async fn load_movies(project_path: PathBuf) -> Result<Vec<MovieNode>, Error>;

/// Load all actor nodes
#[tauri::command]
async fn load_actors(project_path: PathBuf) -> Result<Vec<ActorNode>, Error>;
```

### Edge Commands

```rust
// src/commands/edges.rs

/// Save all edges
#[tauri::command]
async fn save_edges(project_path: PathBuf, edges: Vec<Edge>) -> Result<(), Error>;

/// Load all edges
#[tauri::command]
async fn load_edges(project_path: PathBuf) -> Result<Vec<Edge>, Error>;
```

### Image Commands

```rust
// src/commands/images.rs

/// Download and cache poster from URL
#[tauri::command]
async fn cache_poster(
    project_path: PathBuf,
    url: String,
    filename: String
) -> Result<String, Error>;  // Returns local path

/// Process and save custom uploaded image
#[tauri::command]
async fn process_custom_image(
    project_path: PathBuf,
    source_path: PathBuf,
    target_filename: String
) -> Result<String, Error>;

/// Get available poster alternatives (checks cache first)
#[tauri::command]
async fn get_poster_options(
    project_path: PathBuf,
    tmdb_id: u32,
    media_type: MediaType  // Movie or Person
) -> Result<Vec<PosterOption>, Error>;

/// Delete unused cached images (cleanup)
#[tauri::command]
async fn cleanup_unused_images(
    project_path: PathBuf,
    used_images: Vec<String>
) -> Result<CleanupResult, Error>;
```

### TMDB Commands

```rust
// src/commands/tmdb.rs

/// Search movies by title
#[tauri::command]
async fn search_movies(
    query: String,
    page: Option<u32>,
    api_key: Option<String>
) -> Result<SearchResults<MovieResult>, Error>;

/// Search people by name
#[tauri::command]
async fn search_people(
    query: String,
    page: Option<u32>,
    api_key: Option<String>
) -> Result<SearchResults<PersonResult>, Error>;

/// Get movie details with credits
#[tauri::command]
async fn get_movie_details(
    tmdb_id: u32,
    api_key: Option<String>
) -> Result<MovieDetails, Error>;

/// Get person details with filmography
#[tauri::command]
async fn get_person_details(
    tmdb_id: u32,
    api_key: Option<String>
) -> Result<PersonDetails, Error>;

/// Get all poster images for movie/person
#[tauri::command]
async fn get_images(
    tmdb_id: u32,
    media_type: MediaType,
    api_key: Option<String>
) -> Result<ImageResults, Error>;

/// Test if API key is valid
#[tauri::command]
async fn test_api_key(api_key: String) -> Result<bool, Error>;
```

### Config Commands

```rust
// src/commands/config.rs

/// Get app configuration
#[tauri::command]
async fn get_config() -> Result<AppConfig, Error>;

/// Update app configuration
#[tauri::command]
async fn set_config(config: AppConfig) -> Result<(), Error>;

/// Get specific config value
#[tauri::command]
async fn get_config_value(key: String) -> Result<Option<String>, Error>;

/// Set specific config value
#[tauri::command]
async fn set_config_value(key: String, value: String) -> Result<(), Error>;
```

### Backup Commands

```rust
// src/commands/backup.rs

/// Create manual backup
#[tauri::command]
async fn create_backup(project_path: PathBuf) -> Result<BackupInfo, Error>;

/// List available backups
#[tauri::command]
async fn list_backups(project_path: PathBuf) -> Result<Vec<BackupInfo>, Error>;

/// Restore from backup
#[tauri::command]
async fn restore_backup(
    project_path: PathBuf,
    backup_id: String
) -> Result<Project, Error>;
```

---

## Data Models (Rust)

### Project

```rust
// src/models/project.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub path: PathBuf,
    pub manifest: Manifest,
    pub movies: Vec<MovieNode>,
    pub actors: Vec<ActorNode>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub version: String,           // Schema version for migrations
    pub name: String,              // Project display name
    pub created_at: String,        // ISO timestamp
    pub modified_at: String,       // ISO timestamp
    pub view_state: ViewState,     // Camera position, zoom, etc.
    pub device_layout: DeviceLayout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewState {
    pub camera_x: f64,
    pub camera_y: f64,
    pub zoom: f64,
    pub selected_node: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceLayout {
    pub preset: String,            // "default", "compact", "custom"
    pub devices: Vec<DevicePosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevicePosition {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentProject {
    pub path: PathBuf,
    pub name: String,
    pub last_opened: String,
}
```

### Nodes

```rust
// src/models/node.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Node {
    #[serde(rename = "movie")]
    Movie(MovieNode),
    #[serde(rename = "actor")]
    Actor(ActorNode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieNode {
    pub id: String,                // "m:{tmdb_id}"
    pub tmdb_id: u32,
    pub title: String,
    pub year: u16,
    pub status: Status,
    pub my_rating: Option<u8>,     // 1-10
    pub poster: String,            // Relative path
    pub poster_options: Vec<String>,
    pub notes: String,
    pub position: Position,
    pub added_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorNode {
    pub id: String,                // "a:{tmdb_id}"
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
```

### Edges

```rust
// src/models/edge.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub from: String,              // Node ID
    pub to: String,                // Node ID
    pub relationship: Relationship,
    pub note: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Relationship {
    ActedIn,
    LikedActor,
    Recommended,
}
```

### TMDB Types

```rust
// src/models/tmdb.rs

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResults {
    pub posters: Vec<ImageInfo>,   // For movies
    pub profiles: Vec<ImageInfo>,  // For people
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub file_path: String,
    pub width: u32,
    pub height: u32,
    pub vote_average: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MediaType {
    Movie,
    Person,
}
```

---

## Services

### File I/O Service

```rust
// src/services/file_io.rs

use std::path::PathBuf;
use tokio::fs;

pub struct FileService;

impl FileService {
    /// Create project directory structure
    pub async fn create_project_structure(path: &PathBuf) -> Result<(), Error> {
        fs::create_dir_all(path.join("data")).await?;
        fs::create_dir_all(path.join("images/posters")).await?;
        fs::create_dir_all(path.join("images/custom")).await?;
        fs::create_dir_all(path.join(".backups")).await?;
        Ok(())
    }

    /// Read JSON file with type inference
    pub async fn read_json<T: DeserializeOwned>(path: &PathBuf) -> Result<T, Error>;

    /// Write JSON file with pretty formatting
    pub async fn write_json<T: Serialize>(path: &PathBuf, data: &T) -> Result<(), Error>;

    /// Atomic write (write to temp, then rename)
    pub async fn write_json_atomic<T: Serialize>(path: &PathBuf, data: &T) -> Result<(), Error>;

    /// Check if path exists
    pub async fn exists(path: &PathBuf) -> bool;

    /// Copy file
    pub async fn copy_file(from: &PathBuf, to: &PathBuf) -> Result<(), Error>;

    /// Delete file
    pub async fn delete_file(path: &PathBuf) -> Result<(), Error>;

    /// List files in directory with pattern
    pub async fn list_files(dir: &PathBuf, pattern: &str) -> Result<Vec<PathBuf>, Error>;
}
```

### Cache Service

```rust
// src/services/cache.rs

use std::path::PathBuf;
use std::time::Duration;

const CACHE_DURATION: Duration = Duration::from_secs(7 * 24 * 60 * 60); // 7 days

pub struct CacheService {
    cache_dir: PathBuf,
}

impl CacheService {
    pub fn new() -> Self {
        // Uses app data directory: ~/.config/movie-graph/cache/
        let cache_dir = dirs::config_dir()
            .unwrap()
            .join("movie-graph")
            .join("cache");
        Self { cache_dir }
    }

    /// Get cached API response if valid
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T>;

    /// Store API response in cache
    pub async fn set<T: Serialize>(&self, key: &str, data: &T) -> Result<(), Error>;

    /// Check if cache entry exists and is valid
    pub async fn is_valid(&self, key: &str) -> bool;

    /// Clear expired cache entries
    pub async fn clear_expired(&self) -> Result<u32, Error>;

    /// Clear all cache
    pub async fn clear_all(&self) -> Result<(), Error>;

    /// Generate cache key from request parameters
    pub fn make_key(endpoint: &str, params: &[(&str, &str)]) -> String {
        // e.g., "search_movie_inception_1" or "movie_details_550"
    }
}
```

### Backup Service

```rust
// src/services/backup.rs

use std::path::PathBuf;

const MAX_BACKUPS: usize = 5;

pub struct BackupService;

impl BackupService {
    /// Create timestamped backup of project data
    pub async fn create_backup(project_path: &PathBuf) -> Result<BackupInfo, Error> {
        let backup_dir = project_path.join(".backups");
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_path = backup_dir.join(format!("backup_{}", timestamp));

        // Copy data files to backup
        fs::create_dir_all(&backup_path).await?;
        fs::copy(
            project_path.join("manifest.json"),
            backup_path.join("manifest.json")
        ).await?;
        // ... copy data/*.json files

        // Prune old backups
        Self::prune_old_backups(&backup_dir).await?;

        Ok(BackupInfo { ... })
    }

    /// Keep only MAX_BACKUPS most recent
    async fn prune_old_backups(backup_dir: &PathBuf) -> Result<(), Error>;

    /// List available backups sorted by date
    pub async fn list_backups(project_path: &PathBuf) -> Result<Vec<BackupInfo>, Error>;

    /// Restore project from backup
    pub async fn restore(
        project_path: &PathBuf,
        backup_id: &str
    ) -> Result<(), Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: String,
    pub created_at: String,
    pub size_bytes: u64,
}
```

### Image Processor Service

```rust
// src/services/image_processor.rs

use image::{DynamicImage, ImageFormat};
use std::path::PathBuf;

const THUMBNAIL_WIDTH: u32 = 200;
const FULL_WIDTH: u32 = 500;
const JPEG_QUALITY: u8 = 80;

pub struct ImageProcessor;

impl ImageProcessor {
    /// Download image from URL
    pub async fn download(url: &str) -> Result<Vec<u8>, Error>;

    /// Process and save image with standard sizes
    pub async fn process_and_save(
        image_data: &[u8],
        output_path: &PathBuf
    ) -> Result<ProcessedImage, Error> {
        let img = image::load_from_memory(image_data)?;

        // Resize to standard width, maintain aspect ratio
        let resized = img.resize(
            FULL_WIDTH,
            u32::MAX,
            image::imageops::FilterType::Lanczos3
        );

        // Save as JPEG
        let mut output = std::fs::File::create(output_path)?;
        resized.write_to(&mut output, ImageFormat::Jpeg)?;

        Ok(ProcessedImage {
            path: output_path.clone(),
            width: resized.width(),
            height: resized.height(),
        })
    }

    /// Generate thumbnail from full image
    pub async fn create_thumbnail(
        source: &PathBuf,
        output: &PathBuf
    ) -> Result<(), Error>;

    /// Validate image file (format, size limits)
    pub fn validate(path: &PathBuf) -> Result<ImageValidation, Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedImage {
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageValidation {
    pub valid: bool,
    pub format: Option<String>,
    pub width: u32,
    pub height: u32,
    pub size_bytes: u64,
    pub error: Option<String>,
}
```

---

## TMDB Client

```rust
// src/services/tmdb_client.rs

use reqwest::Client;

const BASE_URL: &str = "https://api.themoviedb.org/3";
const IMAGE_BASE_URL: &str = "https://image.tmdb.org/t/p";
const DEFAULT_API_KEY: &str = "YOUR_DEFAULT_KEY"; // Embedded key

pub struct TmdbClient {
    client: Client,
    cache: CacheService,
    api_key: String,
}

impl TmdbClient {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            cache: CacheService::new(),
            api_key: api_key.unwrap_or(DEFAULT_API_KEY.to_string()),
        }
    }

    /// Set custom API key
    pub fn set_api_key(&mut self, key: String) {
        self.api_key = key;
    }

    /// Search movies
    pub async fn search_movies(
        &self,
        query: &str,
        page: u32
    ) -> Result<SearchResults<MovieResult>, Error> {
        let cache_key = CacheService::make_key("search_movie", &[
            ("q", query),
            ("p", &page.to_string())
        ]);

        // Check cache first
        if let Some(cached) = self.cache.get(&cache_key).await {
            return Ok(cached);
        }

        // Fetch from API
        let url = format!(
            "{}/search/movie?api_key={}&query={}&page={}",
            BASE_URL, self.api_key, query, page
        );

        let response = self.client.get(&url).send().await?;
        self.handle_response(response, &cache_key).await
    }

    /// Get movie details with credits
    pub async fn get_movie(&self, id: u32) -> Result<MovieDetails, Error>;

    /// Get person details with filmography
    pub async fn get_person(&self, id: u32) -> Result<PersonDetails, Error>;

    /// Get poster URLs for movie/person
    pub async fn get_images(
        &self,
        id: u32,
        media_type: MediaType
    ) -> Result<ImageResults, Error>;

    /// Build full image URL from path
    pub fn image_url(path: &str, size: ImageSize) -> String {
        let size_str = match size {
            ImageSize::Thumbnail => "w200",
            ImageSize::Medium => "w500",
            ImageSize::Original => "original",
        };
        format!("{}/{}{}", IMAGE_BASE_URL, size_str, path)
    }

    /// Handle API response with error checking
    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
        cache_key: &str
    ) -> Result<T, Error> {
        match response.status() {
            status if status.is_success() => {
                let data: T = response.json().await?;
                self.cache.set(cache_key, &data).await?;
                Ok(data)
            }
            status if status == 401 => Err(Error::InvalidApiKey),
            status if status == 429 => Err(Error::RateLimited),
            status => Err(Error::ApiError(status.as_u16())),
        }
    }
}

pub enum ImageSize {
    Thumbnail,  // w200
    Medium,     // w500
    Original,
}
```

---

## Error Handling

```rust
// src/error.rs

use thiserror::Error;

#[derive(Error, Debug, Serialize)]
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
    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Implement conversion for Tauri IPC
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
```

---

## App Configuration

```rust
// src/config/app_config.rs

use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub tmdb_api_key: Option<String>,
    pub recent_projects: Vec<RecentProject>,
    pub max_recent_projects: usize,
    pub auto_save: bool,
    pub auto_save_interval_ms: u64,
    pub auto_backup: bool,
    pub max_backups: usize,
    pub sound_enabled: bool,
    pub reduced_motion: bool,
    pub theme: String,
    pub device_layout_preset: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            tmdb_api_key: None,
            recent_projects: vec![],
            max_recent_projects: 10,
            auto_save: true,
            auto_save_interval_ms: 2000,
            auto_backup: true,
            max_backups: 5,
            sound_enabled: true,
            reduced_motion: false,
            theme: "default".to_string(),
            device_layout_preset: "default".to_string(),
        }
    }
}

impl AppConfig {
    /// Load config from app data directory
    pub async fn load() -> Result<Self, Error> {
        let path = Self::config_path();
        if path.exists() {
            FileService::read_json(&path).await
        } else {
            Ok(Self::default())
        }
    }

    /// Save config to app data directory
    pub async fn save(&self) -> Result<(), Error> {
        let path = Self::config_path();
        FileService::write_json(&path, self).await
    }

    /// Get config file path
    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap()
            .join("movie-graph")
            .join("config.json")
    }

    /// Add project to recent list
    pub fn add_recent_project(&mut self, path: PathBuf, name: String) {
        // Remove if already exists
        self.recent_projects.retain(|p| p.path != path);

        // Add to front
        self.recent_projects.insert(0, RecentProject {
            path,
            name,
            last_opened: chrono::Utc::now().to_rfc3339(),
        });

        // Trim to max
        self.recent_projects.truncate(self.max_recent_projects);
    }
}
```

---

## Events (Backend to Frontend)

Tauri events for async notifications:

```rust
// Event names
const EVENT_SAVE_COMPLETE: &str = "save:complete";
const EVENT_SAVE_ERROR: &str = "save:error";
const EVENT_BACKUP_CREATED: &str = "backup:created";
const EVENT_CACHE_CLEARED: &str = "cache:cleared";
const EVENT_API_ERROR: &str = "api:error";
const EVENT_IMAGE_CACHED: &str = "image:cached";

// Emit from backend
app.emit_all(EVENT_SAVE_COMPLETE, SaveCompletePayload {
    project_path: path.to_string_lossy().to_string(),
    timestamp: chrono::Utc::now().to_rfc3339(),
}).ok();

// Listen in frontend (Svelte)
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen('save:complete', (event) => {
    console.log('Project saved:', event.payload);
});
```

---

## Security

### Tauri Capabilities

```json
// src-tauri/capabilities/default.json
{
  "identifier": "default",
  "description": "Default app capabilities",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "fs:default",
    "fs:allow-read",
    "fs:allow-write",
    "dialog:default",
    "dialog:allow-open",
    "dialog:allow-save",
    "http:default",
    "shell:allow-open"
  ]
}
```

### File System Scope

```json
// tauri.conf.json
{
  "security": {
    "csp": "default-src 'self'; img-src 'self' https://image.tmdb.org data:;",
    "dangerousDisableAssetCspModification": false
  },
  "plugins": {
    "fs": {
      "scope": {
        "allow": ["$DOCUMENT/**", "$CONFIG/**", "$DATA/**"],
        "deny": ["$HOME/.ssh/**", "$HOME/.gnupg/**"]
      }
    },
    "http": {
      "scope": {
        "allow": ["https://api.themoviedb.org/*", "https://image.tmdb.org/*"]
      }
    }
  }
}
```

### API Key Storage

- User API key stored in app config (not project files)
- Config file in user's app data directory (not world-readable)
- Never log or expose API keys in errors

---

## Performance Considerations

### Async All The Things

All I/O operations are async to prevent blocking the main thread:

```rust
#[tauri::command]
async fn heavy_operation() -> Result<(), Error> {
    // Runs on async runtime, doesn't block UI
    tokio::spawn(async move {
        // Heavy work here
    }).await?;
    Ok(())
}
```

### Batch Operations

Node saves are batched to reduce file writes:

```rust
// Instead of saving each node individually:
// BAD: for node in nodes { save_node(node); }

// Batch save entire collection:
// GOOD: save_movies(project_path, all_movies);
```

### Debounced Auto-Save

Frontend debounces save requests; backend handles atomically:

```rust
// Atomic write prevents corruption on crash
pub async fn write_json_atomic<T: Serialize>(path: &PathBuf, data: &T) -> Result<(), Error> {
    let temp_path = path.with_extension("json.tmp");

    // Write to temp file
    let content = serde_json::to_string_pretty(data)?;
    fs::write(&temp_path, content).await?;

    // Atomic rename
    fs::rename(&temp_path, path).await?;

    Ok(())
}
```

### Image Lazy Loading

Images are fetched and cached on demand:

1. Node created with TMDB poster path
2. When node becomes visible, frontend requests image
3. Backend checks local cache first
4. If not cached, downloads and processes
5. Returns local file path

### Memory Management

- Large images processed in streams, not loaded fully into memory
- Cache has size limits and expiration
- Old backups automatically pruned

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_project_structure() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().to_path_buf();

        FileService::create_project_structure(&path).await.unwrap();

        assert!(path.join("data").exists());
        assert!(path.join("images/posters").exists());
        assert!(path.join("images/custom").exists());
    }

    #[tokio::test]
    async fn test_json_roundtrip() {
        let node = MovieNode { ... };
        let path = tempfile::NamedTempFile::new().unwrap().path().to_path_buf();

        FileService::write_json(&path, &node).await.unwrap();
        let loaded: MovieNode = FileService::read_json(&path).await.unwrap();

        assert_eq!(node.id, loaded.id);
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    #[tokio::test]
    async fn test_full_project_lifecycle() {
        // Create project
        // Add nodes
        // Save
        // Reload
        // Verify data integrity
    }

    #[tokio::test]
    async fn test_backup_and_restore() {
        // Create project with data
        // Create backup
        // Modify data
        // Restore from backup
        // Verify original data restored
    }
}
```

### Mock TMDB for Tests

```rust
// Use mockito or wiremock for API tests
#[tokio::test]
async fn test_search_movies() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/3/search/movie"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(mock_search_results()))
        .mount(&mock_server)
        .await;

    let client = TmdbClient::new_with_base_url(mock_server.uri());
    let results = client.search_movies("test", 1).await.unwrap();

    assert_eq!(results.results.len(), 2);
}
```

---

## Dependencies (Cargo.toml)

```toml
[package]
name = "movie-graph"
version = "0.1.0"
edition = "2021"

[dependencies]
tauri = { version = "2.0", features = ["protocol-asset"] }
tauri-plugin-fs = "2.0"
tauri-plugin-dialog = "2.0"
tauri-plugin-http = "2.0"
tauri-plugin-shell = "2.0"
tauri-plugin-os = "2.0"
tauri-plugin-process = "2.0"

serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
image = "0.24"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
dirs = "5.0"
uuid = { version = "1.0", features = ["v4"] }

[dev-dependencies]
tempfile = "3.0"
wiremock = "0.5"
```

---

## Development Phases (Backend)

### Phase 1: Scaffold
- Tauri project setup
- Module structure
- Basic command registration
- Error types

### Phase 2: File I/O
- FileService implementation
- Project create/open/save
- JSON serialization

### Phase 3: Data Models
- Node/Edge types
- Manifest structure
- Validation logic

### Phase 4: TMDB Client
- API client with caching
- Search endpoints
- Detail endpoints
- Rate limit handling

### Phase 5: Image Processing
- Download and cache
- Resize/compress
- Custom upload handling

### Phase 6: Backup System
- Auto-backup on save
- Manual backup
- Restore functionality

### Phase 7: Configuration
- App config management
- Recent projects
- User preferences

### Phase 8: Polish
- Error handling refinement
- Performance optimization
- Security audit
