use std::io;

type AnyError = Box<dyn core::error::Error + Send + Sync + 'static>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Cli(#[from] lexopt::Error),
    #[error(transparent)]
    Other(#[from] AnyError),
}
