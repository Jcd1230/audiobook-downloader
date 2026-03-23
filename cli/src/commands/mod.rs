pub mod auth;
pub mod completions;
pub mod config;
pub mod download;
pub mod import;
pub mod info;
pub mod list;
pub mod search;
pub mod sync;
pub mod utils;

use crate::cli::Commands;
use crate::config::Config;
use crate::error::Result;

pub async fn handle(command: Commands, config: Config, yes: bool) -> Result<()> {
    match command {
        Commands::Auth => auth::auth().await,
        Commands::Sync => sync::sync().await,
        Commands::Import { path } => import::import(config, path).await,
        Commands::List { json } => list::list(json).await,
        Commands::Search { query } => search::search(&query, yes).await,
        Commands::Info { id } => info::info(&id).await,
        Commands::Download {
            query,
            all,
            no_cue,
            no_folder,
            filename,
        } => {
            download::download(
                query.as_deref(),
                all,
                no_cue,
                no_folder,
                filename,
                config,
                yes,
            )
            .await
        }
        Commands::Config { subcommand } => config::config(subcommand, config).await,
        Commands::Completions { shell } => completions::completions(shell).await,
    }
}
