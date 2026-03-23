use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum CLIError {
    #[error("Audible authentication token has expired")]
    #[diagnostic(help("Run 'audiobook-downloader auth' to refresh your credentials."))]
    AuthExpired,

    #[error("No books in local library")]
    #[diagnostic(help("Run 'audiobook-downloader sync' to update your local library."))]
    EmptyLibrary,

    #[error(transparent)]
    Library(#[from] audible_api::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, CLIError>;
