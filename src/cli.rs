use clap::Parser;
use clap_verbosity_flag::Verbosity;

use crate::commands::Commands;

#[derive(Parser)]
#[command(name = "flexy")]
#[command(about = "An extremely fast widget composer written in rust with skia", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,

    #[clap(flatten)]
    verbose: Verbosity,
}

pub fn get() -> Cli {
    let cli = Cli::parse();
    let log_filter = cli.verbose.log_level_filter();

    env_logger::Builder::new()
        .default_format()
        .parse_default_env()
        .filter_level(log_filter)
        .init();

    cli
}
