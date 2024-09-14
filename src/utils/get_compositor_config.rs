use anyhow::{anyhow, Result};
use dirs::config_dir;
use std::fs::read_to_string;

use crate::models::compositor_config::CompositorConfig;

pub fn compositor_config() -> Result<CompositorConfig> {
    let base = config_dir().expect("Fatal: config dir not found");
    let mut config_path = base;

    config_path.push("flexy");
    config_path.push("compositor.toml");

    if let Ok(config) = read_to_string(&config_path) {
        let parsed: CompositorConfig = toml::from_str(&config)?;

        Ok(parsed)
    } else {
        return Err(anyhow!(format!(
            "A configuration file was not found in {:?}",
            config_path
        )));
    }
}
