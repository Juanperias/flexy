use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub fn get(name: String) -> Result<PathBuf> {
    let font_path = dirs::font_dir().expect("font dir not found");
    let ttf_path = font_path.join(format!("{}.ttf", name));
    if !ttf_path.exists() {
        let message = format!("{:?} not found in {:?}", ttf_path, font_path);
        return Err(anyhow!(message));
    }

    Ok(ttf_path)
}
