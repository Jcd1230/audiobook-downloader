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

    /// Download and decrypt an audiobook
    Download {
        /// The ID of the audiobook to download
        id: Option<String>,

        /// Download all missing books
        #[arg(long)]
        all: bool,
    },

    /// View or modify CLI settings
    Config,
}
