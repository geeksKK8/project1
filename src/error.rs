use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {
    #[error("Key not found")]
    KeyNotFound,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Unexpected command type")]
    UnexpectedCommandType,
    #[error("{0}")]
    StringError(String),
    #[error("Sled error: {0}")]
    Sled(#[from] sled::Error),
    #[error("FromUtf8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, KvsError>;
