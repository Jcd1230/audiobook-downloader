pub mod auth;
pub mod config;
pub mod download;
pub mod info;
pub mod list;
pub mod search;
pub mod sync;
pub mod utils;

use crate::cli::Commands;
use crate::error::Result;

pub async fn handle(command: Commands) -> Result<()> {
    match command {
        Commands::Auth => auth::auth().await,
        Commands::Sync => sync::sync().await,
        Commands::List { json } => list::list(json).await,
        Commands::Search { query } => search::search(&query).await,
        Commands::Info { id } => info::info(&id).await,
        Commands::Download {
            query,
            all,
            no_cue,
            no_folder,
            filename,
        } => download::download(query.as_deref(), all, no_cue, no_folder, filename).await,
        Commands::Config => config::config().await,
    }
}
