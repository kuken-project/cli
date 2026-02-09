use crate::commands;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None, name = "kuken")]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: commands::Commands,
}
