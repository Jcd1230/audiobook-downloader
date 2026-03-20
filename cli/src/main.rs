mod cli;
mod commands;
mod api;
mod state;
mod download;
mod media;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    commands::handle(cli.command).await
}
