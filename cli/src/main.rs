mod cli;
mod commands;
mod config;
mod download;
mod error;
mod media;
mod state;
mod update;

use clap::Parser;
use cli::Cli;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn setup_logging(verbose: bool) {
    let filter = if verbose {
        EnvFilter::new("info")
    } else {
        EnvFilter::new("warn")
    };

    tracing_subscriber::registry()
        .with(fmt::layer().compact())
        .with(filter)
        .init();
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    setup_logging(cli.verbose);

    // Setup miette for minimalist output
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .context_lines(2)
                .build(),
        )
    }))
    .expect("Failed to set miette hook");

    commands::handle(cli.command, config::Config::load(), cli.yes)
        .await
        .map_err(|e| miette::miette!("{}", e))?;

    if let Some(latest) = update::check_for_update().await {
        use colored::*;
        eprintln!(
            "\n{}",
            format!(
                "✨ A new version of audiobook-downloader is available: v{} (Current: v{})",
                latest,
                env!("CARGO_PKG_VERSION")
            )
            .dimmed()
        );
    }

    Ok(())
}
