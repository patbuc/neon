use serde::{Deserialize, Serialize};

/// Magic number for Neon binary format: ASCII "NEON"
pub const MAGIC_NUMBER: [u8; 4] = [0x4E, 0x45, 0x4F, 0x4E];

/// Current binary format version
pub const FORMAT_VERSION: u16 = 1;

/// Header for the Neon binary format
///
/// The header contains identification and version information to ensure
/// proper parsing and compatibility checking.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BinaryHeader {
    /// Magic number identifying this as a Neon binary file
    pub magic: [u8; 4],

    /// Format version number for compatibility checking
    pub version: u16,

    /// Reserved bytes for future use (alignment and extensibility)
    pub reserved: [u8; 10],
}

impl BinaryHeader {
    /// Creates a new binary header with the current format version
    pub fn new() -> Self {
        Self {
            magic: MAGIC_NUMBER,
            version: FORMAT_VERSION,
            reserved: [0; 10],
        }
    }

    /// Validates that this header has the correct magic number and a supported version
    pub fn validate(&self) -> Result<(), String> {
        if self.magic != MAGIC_NUMBER {
            return Err(format!(
                "Invalid magic number: expected {:?}, found {:?}",
                MAGIC_NUMBER, self.magic
            ));
        }

        if self.version > FORMAT_VERSION {
            return Err(format!(
                "Unsupported format version: {} (current version is {})",
                self.version, FORMAT_VERSION
            ));
        }

        Ok(())
    }
}

impl Default for BinaryHeader {
    fn default() -> Self {
        Self::new()
    }
}

/// The complete Neon binary format structure
///
/// This will contain the header and the compiled bytecode chunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryFormat {
    /// Format header with version information
    pub header: BinaryHeader,

    // TODO: Add chunk field in next task
    // pub chunk: Chunk,
}

impl BinaryFormat {
    /// Creates a new binary format with the given components
    pub fn new() -> Self {
        Self {
            header: BinaryHeader::new(),
        }
    }
}

impl Default for BinaryFormat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_number() {
        assert_eq!(MAGIC_NUMBER, [0x4E, 0x45, 0x4F, 0x4E]);
        assert_eq!(&MAGIC_NUMBER, b"NEON");
    }

    #[test]
    fn test_binary_header_creation() {
        let header = BinaryHeader::new();
        assert_eq!(header.magic, MAGIC_NUMBER);
        assert_eq!(header.version, FORMAT_VERSION);
        assert_eq!(header.reserved, [0; 10]);
    }

    #[test]
    fn test_binary_header_validation_success() {
        let header = BinaryHeader::new();
        assert!(header.validate().is_ok());
    }

    #[test]
    fn test_binary_header_validation_invalid_magic() {
        let mut header = BinaryHeader::new();
        header.magic = [0x00, 0x00, 0x00, 0x00];
        assert!(header.validate().is_err());
    }

    #[test]
    fn test_binary_header_validation_unsupported_version() {
        let mut header = BinaryHeader::new();
        header.version = FORMAT_VERSION + 1;
        assert!(header.validate().is_err());
    }

    #[test]
    fn test_binary_format_creation() {
        let format = BinaryFormat::new();
        assert_eq!(format.header.magic, MAGIC_NUMBER);
        assert_eq!(format.header.version, FORMAT_VERSION);
    }
}
