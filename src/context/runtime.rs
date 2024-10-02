use crate::{
    context::{
        lua_out::{lua_debug, lua_error},
        widgets::clear,
    },
    models::job::Job,
};
use anyhow::Result;
use colored::Colorize;
use mlua::{Lua, Table};

use super::widgets::text;

use std::cell::RefCell;
use std::rc::Rc;

pub struct LuaRuntime {
    lua: Lua,
    jobs: Rc<RefCell<Vec<Job>>>, // Replacement of the global variable JOBS
}

impl LuaRuntime {
    pub fn new(widget: String) -> Result<Self> {
        let jobs = Rc::new(RefCell::new(Vec::new()));

        let lua = Lua::new();

        {
            let jobs_clone = Rc::clone(&jobs);
            let ui_text =
                lua.create_function(move |_, (param, table): (String, Option<Table>)| {
                    let mut jobs = jobs_clone.borrow_mut();
                    text(param, table, &mut jobs);
                    Ok(())
                })?;

            let jobs_clone = Rc::clone(&jobs);
            let ui_clear = lua.create_function(move |_, ()| {
                let mut jobs = jobs_clone.borrow_mut();
                clear(&mut jobs);

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

        Ok(Self { lua, jobs })
    }

    pub fn get_lua(&self) -> &Lua {
        &self.lua
    }

    pub fn get_jobs(&self) -> Vec<Job> {
        self.jobs.borrow().to_vec()
    }
}

pub fn run(widget: String) -> Result<LuaRuntime> {
    LuaRuntime::new(widget)
}
