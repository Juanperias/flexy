use crate::models::compositor_config::Screen;
use anyhow::Result;
use std::fs::read_to_string;

pub fn from_screen(screen: &Screen) -> Result<String> {
    let mut dir = dirs::config_dir().expect("Fatal: config dir not found");
    let path = &mut dir;
    path.push("flexy");
    path.push(format!("{}.lua", screen.name));

    match read_to_string(&path) {
        Ok(code) => Ok(code),
        Err(_) => Err(anyhow::anyhow!(format!("Lua file {:?} not exist", path))),
    }
}
