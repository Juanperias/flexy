use anyhow::Result;
use clap::Subcommand;

mod channel;
mod start;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Channel {},
    Start { screen: String },
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Channel {} => {
                channel::get_channel();
            }
            Self::Start { screen } => {
                start::start_widget()?;
            }
        }

        Ok(())
    }
}
