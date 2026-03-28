use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::edge::Edge;
use super::node::{ActorNode, MovieNode};

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
    pub version: String,
    pub name: String,
    pub created_at: String,
    pub modified_at: String,
    pub view_state: ViewState,
    pub device_layout: DeviceLayout,
}

impl Manifest {
    pub fn new(name: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            version: "1.0".to_string(),
            name,
            created_at: now.clone(),
            modified_at: now,
            view_state: ViewState::default(),
            device_layout: DeviceLayout::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewState {
    pub camera_x: f64,
    pub camera_y: f64,
    pub zoom: f64,
    pub selected_node: Option<String>,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            camera_x: 0.0,
            camera_y: 0.0,
            zoom: 1.0,
            selected_node: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceLayout {
    pub preset: String,
    pub devices: Vec<DevicePosition>,
}

impl Default for DeviceLayout {
    fn default() -> Self {
        Self {
            preset: "default".to_string(),
            devices: vec![],
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
}
