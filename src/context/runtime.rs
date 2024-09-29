use crate::context::{
    globals::init,
    lua_out::{lua_debug, lua_error},
    widgets::clear,
};
use anyhow::Result;
use colored::Colorize;
use mlua::{Lua, Table};

use super::widgets::text;

pub struct LuaRuntime {
    lua: Lua,
}

impl LuaRuntime {
    pub fn new(widget: String) -> Result<Self> {
        init();

        let lua = Lua::new();

        {
            let ui_text = lua.create_function(|_, (param, table): (String, Option<Table>)| {
                text(param, table);
                Ok(())
            })?;

            let ui_clear = lua.create_function(|_, ()| {
                clear();
                Ok(())
            })?;

            let out_debug = lua.create_function(|_, message: String| {
                lua_debug(&message);
                Ok(())
            })?;

            let out_error = lua.create_function(|_, message: String| {
                lua_error(&message);
                Ok(())
            })?;

            let globals = lua.globals();
            globals.set("text", ui_text)?;
            globals.set("debug", out_debug)?;
            globals.set("error", out_error)?;
            globals.set("clear", ui_clear)?;

            lua.load(&widget).exec()?;

            let widget_func: mlua::Function = globals.get("Widget")?;
            widget_func.call::<(), ()>(())?;
        }

        println!("[{}] Lua loaded correctly", "OUTPUT".yellow());

        Ok(Self { lua })
    }

    pub fn get_lua(&self) -> &Lua {
        &self.lua
    }
}

pub fn run(widget: String) -> Result<LuaRuntime> {
    LuaRuntime::new(widget)
}
