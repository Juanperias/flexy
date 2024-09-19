use crate::{
    compositor::window::render_window,
    context::runtime::run,
    models::compositor_config::Screen,
    utils::{get_compositor_config::compositor_config, get_lua::get_lua_from_screen},
};
use anyhow::{anyhow, Result};

fn find_screen<'a>(screens: &'a [Screen], name: &str) -> Result<&'a Screen> {
    screens
        .iter()
        .find(|screen| screen.name == name)
        .ok_or_else(|| anyhow!("Error: screen not found"))
}

pub fn start_widget(screen_name: String) -> Result<()> {
    let config = compositor_config()?;
    let screen = find_screen(&config.screens, &screen_name)?;
    let lua_code = get_lua_from_screen(screen)?;

    run(lua_code)?;

    render_window(screen)?;

    Ok(())
}
