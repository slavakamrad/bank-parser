use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("CSV parsing error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Binary format error: {0}")]
    BinaryFormat(String),

    #[error("Text format error: {0}")]
    TextFormat(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
}

pub type Result<T> = std::result::Result<T, ParserError>;
