use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositorConfig {
    pub screens: Vec<Screen>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Screen {
    pub name: String,
    pub size_x: Option<u32>,
    pub size_y: Option<u32>,
    pub bg_color: Option<String>,
    pub can_close: Option<bool>,
}
