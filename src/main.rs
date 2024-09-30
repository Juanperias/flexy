mod cli;
mod commands;
mod compositor;
mod config;
mod context;
mod models;
mod utils;
mod widgets;

use anyhow::Result;
use cli::get;

fn main() -> Result<()> {
    let cli = get();
    cli.commands.run()
}
