use crate::{
    compositor::window::render,
    context::runtime::run,
    models::compositor_config::Screen,
    utils::{get_compositor_config::compositor_config, get_lua::from_screen},
};
use anyhow::{anyhow, Result};

fn find_screen<'a>(screens: &'a [Screen], name: &str) -> Result<&'a Screen> {
    screens
        .iter()
        .find(|screen| screen.name == name)
        .ok_or_else(|| anyhow!("screen not found"))
}

pub fn widget(screen_name: &str, can_close: bool) -> Result<()> {
    let config = compositor_config()?;
    let screen = find_screen(&config.screens, screen_name)?;
    let lua_code = from_screen(screen)?;

    run(lua_code)?;

    render(screen, can_close).expect("Error: cannot render the windows");

    Ok(())
}
