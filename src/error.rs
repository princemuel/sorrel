use core::error::Error;
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum GlobalError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Cli(#[from] lexopt::Error),
    #[error(transparent)]
    Other(#[from] Box<dyn Error + Send + Sync + 'static>),
}
