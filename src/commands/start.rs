use crate::{compositor::windows::render_window, context::runtime::run};
use anyhow::Result;

pub fn start_widget() -> Result<()> {
    run()?;

    render_window()?;

    Ok(())
}
