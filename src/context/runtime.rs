use crate::context::globals::{init_globals, JOBS};
use anyhow::Result;
use colored::Colorize;
use mlua::Lua;

use super::widgets::text;

pub fn run() -> Result<()> {
    init_globals();

    let lua = Lua::new();

    let ui_text = lua.create_function(|_, param: String| {
        text(param);
        Ok(())
    })?;

    let globals = lua.globals();
    globals.set("text", ui_text)?;

    lua.load(
        r#"
        function widget()
            text("hello 2")
        end

        widget()
    "#,
    )
    .exec()?;

    println!("[{}] Lua loaded correctly", "OUTPUT".yellow());

    let jobs = JOBS.get().expect("Error: cannot open jobs").lock().unwrap();

    Ok(())
}
