use crate::{
    compositor::windows::render_window, context::runtime::run, models::compositor_config::Screen,
    utils::get_compositor_config::compositor_config,
};
use anyhow::{anyhow, Result};

fn find_screen(screens: Vec<Screen>, name: String) -> Result<Screen> {
    let el = screens.iter().find(|screen| screen.name == name);

    if let Some(screen) = el {
        Ok((*screen).to_owned())
    } else {
        return Err(anyhow!("Error: screen not found"));
    }
}

pub fn start_widget(screen_name: String) -> Result<()> {
    let config = compositor_config()?;

    run()?;
    let screen = find_screen(config.screens, screen_name)?;

    render_window(screen)?;

    Ok(())
}
