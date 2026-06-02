use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown error: {0}")]
    Unexpected(String),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("cli error: {0}")]
    Cli(#[from] lexopt::Error),
}
