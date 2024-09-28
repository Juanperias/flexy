use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub fn get(name: &str) -> Result<PathBuf> {
    let font_path = dirs::font_dir().expect("font dir not found");
    let ttf_path = font_path.join(format!("{name}.ttf"));
    if !ttf_path.exists() {
        let message = format!("{ttf_path:?} not found in {font_path:?}");
        return Err(anyhow!(message));
    }

    Ok(ttf_path)
}
