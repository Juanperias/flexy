use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompositorConfig {
    pub screens: Vec<Screen>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Screen {
    pub widgets: Vec<String>,
    pub size_x: i32,
    pub size_y: i32,
    pub pos_x: i32,
    pub pos_y: i32,
}
