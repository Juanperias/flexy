use crate::context::globals::init_globals;
use anyhow::Result;
use colored::Colorize;
use mlua::Lua;

use super::widgets::text;

pub fn run(widgets: Vec<String>) -> Result<()> {
    init_globals();

    let lua = Lua::new();

    let ui_text = lua.create_function(|_, param: String| {
        text(param);
        Ok(())
    })?;

    let globals = lua.globals();
    globals.set("text", ui_text)?;

    widgets.iter().try_for_each(|widget| {
        lua.load(widget).exec()?;

        let widget: mlua::Function = lua.globals().get("widget")?;

        widget.call(())
    })?;

    println!("[{}] Lua loaded correctly", "OUTPUT".yellow());

    Ok(())
}
