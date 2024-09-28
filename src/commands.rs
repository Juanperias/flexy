use anyhow::Result;
use clap::Subcommand;

mod channel;
mod start;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Channel {},
    Start {
        screen: String,
        #[clap(short, long)]
        can_close: bool,
    },
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Channel {} => {
                channel::get();
            }
            Self::Start { screen, can_close } => {
                start::widget(screen.to_owned(), can_close.to_owned())?;
            }
        }

        Ok(())
    }
}
