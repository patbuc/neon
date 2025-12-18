use crate::binary::BinaryError;
use crate::common::Chunk;
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

/// Serializes a Chunk into binary format
///
/// The binary format consists of:
/// 1. Magic number (4 bytes): ASCII "NEON"
/// 2. Format version (2 bytes)
/// 3. Reserved bytes (10 bytes)
/// 4. Bincode-encoded chunk data
///
/// # Arguments
/// * `chunk` - The chunk to serialize
///
/// # Returns
/// * `Ok(Vec<u8>)` - The serialized binary data
/// * `Err(BinaryError)` - If serialization fails
pub fn serialize_chunk(chunk: &Chunk) -> Result<Vec<u8>, BinaryError> {
    // Create header
    let header = BinaryHeader::new();

    // Serialize header
    let header_bytes = bincode::serialize(&header)?;

    // Serialize chunk
    let chunk_bytes = bincode::serialize(chunk)?;

    // Combine header and chunk
    let mut result = Vec::with_capacity(header_bytes.len() + chunk_bytes.len());
    result.extend_from_slice(&header_bytes);
    result.extend_from_slice(&chunk_bytes);

    Ok(result)
}

/// Deserializes a Chunk from binary format
///
/// Validates the magic number and version before deserializing the chunk.
///
/// # Arguments
/// * `bytes` - The binary data to deserialize
///
/// # Returns
/// * `Ok(Chunk)` - The deserialized chunk
/// * `Err(BinaryError)` - If deserialization fails or format is invalid
pub fn deserialize_chunk(bytes: &[u8]) -> Result<Chunk, BinaryError> {
    // First, deserialize just the header to validate it
    let header: BinaryHeader = bincode::deserialize(bytes).map_err(|e| {
        BinaryError::DeserializationError(format!("Failed to deserialize header: {}", e))
    })?;

    // Validate magic number
    if header.magic != MAGIC_NUMBER {
        return Err(BinaryError::InvalidFormat(format!(
            "Invalid magic number: expected {:?}, found {:?}",
            MAGIC_NUMBER, header.magic
        )));
    }

    // Validate version
    if header.version > FORMAT_VERSION {
        return Err(BinaryError::UnsupportedVersion {
            found: header.version,
            current: FORMAT_VERSION,
        });
    }

    // Calculate header size to skip it when deserializing chunk
    let header_size = bincode::serialized_size(&header).map_err(|e| {
        BinaryError::DeserializationError(format!("Failed to calculate header size: {}", e))
    })? as usize;

    // Ensure we have enough bytes
    if bytes.len() < header_size {
        return Err(BinaryError::InvalidFormat(
            "Binary data too short".to_string(),
        ));
    }

    // Deserialize chunk from remaining bytes
    let chunk: Chunk = bincode::deserialize(&bytes[header_size..]).map_err(|e| {
        BinaryError::DeserializationError(format!("Failed to deserialize chunk: {}", e))
    })?;

    Ok(chunk)
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

    #[test]
    fn test_serialize_deserialize_round_trip() {
        use crate::common::Chunk;

        // Create a simple chunk for testing
        let chunk = Chunk::new("test_chunk");

        // Serialize the chunk
        let serialized = serialize_chunk(&chunk).expect("Serialization should succeed");

        // Verify the magic number is at the start
        assert!(serialized.len() >= 4);
        assert_eq!(&serialized[0..4], &MAGIC_NUMBER);

        // Deserialize the chunk
        let deserialized = deserialize_chunk(&serialized).expect("Deserialization should succeed");

        // Verify the chunk name matches
        // Note: We can't directly compare chunks since they don't implement Eq,
        // but we can verify they serialize to the same binary representation
        let reserialized =
            serialize_chunk(&deserialized).expect("Re-serialization should succeed");
        assert_eq!(serialized, reserialized);
    }

    #[test]
    fn test_deserialize_invalid_magic() {
        let mut bytes = vec![0xFF, 0xFF, 0xFF, 0xFF]; // Invalid magic
        bytes.extend_from_slice(&[0; 20]); // Add some padding

        let result = deserialize_chunk(&bytes);
        assert!(result.is_err());
        match result {
            Err(BinaryError::InvalidFormat(msg)) => {
                assert!(msg.contains("Invalid magic number"));
            }
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_deserialize_unsupported_version() {
        use crate::common::Chunk;

        // Create a chunk and serialize it
        let chunk = Chunk::new("test");
        let mut serialized = serialize_chunk(&chunk).expect("Serialization should succeed");

        // Modify the version in the header to be higher than supported
        // The version is stored after the magic number (4 bytes) in the bincode format
        // We need to deserialize, modify, and re-serialize the header
        let mut header = BinaryHeader::new();
        header.version = FORMAT_VERSION + 1;

        // Serialize the modified header
        let modified_header_bytes =
            bincode::serialize(&header).expect("Header serialization should succeed");

        // Calculate original header size
        let original_header = BinaryHeader::new();
        let original_header_size = bincode::serialized_size(&original_header)
            .expect("Should calculate size") as usize;

        // Replace the header in the serialized data
        serialized.splice(0..original_header_size, modified_header_bytes);

        // Try to deserialize
        let result = deserialize_chunk(&serialized);
        assert!(result.is_err());
        match result {
            Err(BinaryError::UnsupportedVersion { found, current }) => {
                assert_eq!(found, FORMAT_VERSION + 1);
                assert_eq!(current, FORMAT_VERSION);
            }
            _ => panic!("Expected UnsupportedVersion error"),
        }
    }

    #[test]
    fn test_deserialize_truncated_data() {
        let bytes = vec![0x4E, 0x45, 0x4F]; // Only 3 bytes, not enough for header

        let result = deserialize_chunk(&bytes);
        assert!(result.is_err());
        // Should get a deserialization error due to insufficient data
    }
}
