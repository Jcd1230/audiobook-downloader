use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "audiobook-downloader")]
#[command(about = "A fast, modular CLI to manage and download audiobooks.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Authenticate your account
    Auth,

    /// Fetch metadata from the server and update local state
    Sync,

    /// List available books in the local state
    List,

    /// View details and metadata for a specific book
    Info {
        /// The ID of the audiobook
        id: String,
    },

    /// Search for an audiobook in the local library
    Search {
        /// The title or ID to search for
        query: String,
    },

    /// Download and decrypt an audiobook
    Download {
        /// The title or ID of the audiobook to download
        query: Option<String>,

        /// Download all missing books or all books matching the query
        #[arg(long, short)]
        all: bool,
    },

    /// View or modify CLI settings
    Config,
}
