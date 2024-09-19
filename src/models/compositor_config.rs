use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositorConfig {
    pub screens: Vec<Screen>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Screen {
    pub name: String,
    pub widgets: Vec<String>,
    pub blur: bool,
    pub size_x: Option<u32>,
    pub size_y: Option<u32>,
}
