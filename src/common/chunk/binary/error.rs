use std::fmt;
use std::io;

/// Error type for binary serialization and deserialization operations
#[derive(Debug)]
pub enum BinaryError {
    /// IO error during read/write operations
    IoError(io::Error),

    /// Error during serialization
    SerializationError(String),

    /// Error during deserialization
    DeserializationError(String),

    /// Invalid binary format detected
    InvalidFormat(String),

    /// Unsupported binary format version
    UnsupportedVersion { found: u16, current: u16 },
}

impl fmt::Display for BinaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryError::IoError(e) => write!(f, "IO error: {}", e),
            BinaryError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            BinaryError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            BinaryError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            BinaryError::UnsupportedVersion { found, current } => write!(
                f,
                "Unsupported format version: {} (current version is {})",
                found, current
            ),
        }
    }
}

impl std::error::Error for BinaryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BinaryError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for BinaryError {
    fn from(error: io::Error) -> Self {
        BinaryError::IoError(error)
    }
}

impl From<Box<bincode::ErrorKind>> for BinaryError {
    fn from(error: Box<bincode::ErrorKind>) -> Self {
        BinaryError::SerializationError(error.to_string())
    }
}
