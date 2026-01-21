pub mod web;
mod server;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum InstallCommands {
    Web(web::WebInstallArgs),
    Server(server::ServerInstallArgs),
}

pub async fn execute(command: InstallCommands) -> Result<()> {
    match command {
        InstallCommands::Web(args) => web::execute(args).await,
        InstallCommands::Server(args) => server::execute(args).await,
    }
}
