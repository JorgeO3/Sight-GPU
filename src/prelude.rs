#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    StaticStr(&'static str),
    #[error("Generic {0}")]
    Generic(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    StdError(#[from] Box<dyn std::error::Error>),
    #[error(transparent)]
    StdSenderError(#[from] std::sync::mpsc::RecvError),
}

/// A specialized [`Result`] type for this crate's operations.
pub type Result<T> = core::result::Result<T, Error>;

/// Shorhand for [`std::format`].
pub use std::format as f;
