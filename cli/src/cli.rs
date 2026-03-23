use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "audiobook-downloader")]
#[command(about = "A fast, modular CLI to manage and download audiobooks.", long_about = None)]
pub struct Cli {
    /// Enable verbose logging
    #[arg(long, short, global = true)]
    pub verbose: bool,

    /// Non-interactive mode (auto-confirm prompts)
    #[arg(long, short, global = true)]
    pub yes: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Authenticate your account
    Auth,

    /// Fetch metadata from the server and update local state
    Sync,

    /// Scan the library directory for existing books and update local state
    Import,

    /// List available books in the local state
    List {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

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

        /// Do not generate a .cue file
        #[arg(long)]
        no_cue: bool,

        /// Do not create a dedicated folder for the book
        #[arg(long)]
        no_folder: bool,

        /// Template for the filename (e.g. "{author} - {title}")
        #[arg(long)]
        filename: Option<String>,
    },

    /// View or modify CLI settings
    Config {
        #[command(subcommand)]
        subcommand: ConfigSubcommand,
    },
}

#[derive(Subcommand)]
pub enum ConfigSubcommand {
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        /// The setting key (e.g. library_path, filename_template)
        key: String,
        /// The value to set
        value: String,
    },
}
