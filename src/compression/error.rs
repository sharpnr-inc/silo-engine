use std::fmt;

#[derive(Debug)]
pub enum CompressionError {
    CompressionFailed(String),
    DecompressionFailed(String),
    Io(std::io::Error),
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CompressionFailed(message) => {
                write!(f, "compression failed: {message}")
            }

            Self::DecompressionFailed(message) => {
                write!(f, "decompression failed: {message}")
            }

            Self::Io(error) => {
                write!(f, "compression IO error: {error}")
            }
        }
    }
}

impl std::error::Error for CompressionError {}

impl From<std::io::Error> for CompressionError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
