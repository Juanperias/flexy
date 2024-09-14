use crate::context::runtime::run;
use anyhow::Result;

pub fn start_widget() -> Result<()> {
    run()?;

    Ok(())
}
