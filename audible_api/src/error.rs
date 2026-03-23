use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Amazon registration error ({status}): {body}")]
    RegisterError { status: String, body: String },
}

pub type Result<T> = std::result::Result<T, Error>;
