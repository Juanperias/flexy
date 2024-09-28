use crate::context::{
    globals::init,
    lua_out::{lua_debug, lua_error},
    widgets::clear,
};
use anyhow::Result;
use colored::Colorize;
use mlua::{Lua, Table};

use super::widgets::text;

pub fn run(widget: String) -> Result<()> {
    init();

    let lua = Lua::new();

    let ui_text = lua.create_function(|_, (param, table): (String, Option<Table>)| {
        text(param, table);
        Ok(())
    })?;

    let ui_clear = lua.create_function(|_, ()| {
        clear();
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
    globals.set("clear", ui_clear)?;

    lua.load(widget).exec()?;

    let widget: mlua::Function = lua.globals().get("widget")?;

    widget.call::<(), ()>(())?;

    println!("[{}] Lua loaded correctly", "OUTPUT".yellow());

    Ok(())
}
