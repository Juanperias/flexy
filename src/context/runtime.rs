use crate::context::{
    globals::init,
    lua_out::{lua_debug, lua_error},
};
use anyhow::Result;
use colored::Colorize;
use mlua::{Lua, Table};

use super::widgets::text;

pub fn run(widgets: Vec<String>) -> Result<()> {
    init();

    let lua = Lua::new();

    let ui_text = lua.create_function(|_, (param, table): (String, Option<Table>)| {
        text(param, table);
        Ok(())
    })?;

    let out_debug = lua.create_function(|_, message: String| {
        lua_debug(message);
        Ok(())
    })?;

    let out_error = lua.create_function(|_, message: String| {
        lua_error(message);
        Ok(())
    })?;

    let globals = lua.globals();
    globals.set("text", ui_text)?;
    globals.set("debug", out_debug)?;
    globals.set("error", out_error)?;

    widgets.iter().try_for_each(|widget| {
        lua.load(widget).exec()?;

        let widget: mlua::Function = lua.globals().get("widget")?;

        widget.call(())
    })?;

    println!("[{}] Lua loaded correctly", "OUTPUT".yellow());

    Ok(())
}
