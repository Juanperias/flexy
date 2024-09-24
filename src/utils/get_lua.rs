use crate::models::compositor_config::Screen;
use anyhow::Result;
use std::fs::read_to_string;

pub fn from_screen(screen: &Screen) -> Result<Vec<String>> {
    let mut codes: Vec<String> = Vec::new();

    let mut dir = dirs::config_dir().expect("Fatal: config dir not found");

    for file in &screen.widgets {
        let path = &mut dir;
        path.push("flexy");
        path.push(format!("{file}.lua"));

        let code = read_to_string(path)?;

        codes.push(code);
    }

    Ok(codes)
}

