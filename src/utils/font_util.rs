use anyhow::{anyhow, Result};
use colored::Colorize;
use nannou::text::Font;
use std::fs::read;

pub fn get(name: String) -> Result<Option<Font>> {
    let font_path = dirs::font_dir().expect("font dir not found");
    let ttf_path = font_path.join(format!("{}.ttf", name));
    if !ttf_path.exists() {
        let message = format!("{:?} not found in {:?}", ttf_path, font_path);
        return Err(anyhow!(message));
    }
    let bytes = read(&ttf_path)?;
    let font = Font::from_bytes(bytes)?;

    println!("[{}] Font {} loaded", "DEBUG".blue(), name);

    Ok(Some(font))
}
