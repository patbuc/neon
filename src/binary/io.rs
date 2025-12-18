use crate::binary::error::BinaryError;
use crate::binary::format::{deserialize_chunk, serialize_chunk};
use crate::common::Chunk;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Writes a compiled chunk to a binary file (.nbc)
///
/// This function serializes the chunk using the Neon binary format and writes
/// it to the specified file path. The file will be created if it doesn't exist,
/// or overwritten if it does.
///
/// # Arguments
/// * `path` - The file path where the binary should be written
/// * `chunk` - The compiled chunk to serialize
///
/// # Returns
/// * `Ok(())` - If the write operation succeeds
/// * `Err(BinaryError)` - If serialization or I/O fails
///
/// # Errors
/// * `BinaryError::SerializationError` - If chunk serialization fails
/// * `BinaryError::IoError` - If file creation or writing fails (e.g., permission denied)
///
/// # Example
/// ```ignore
/// use neon::binary::io::write_binary_file;
/// use neon::common::Chunk;
/// use std::path::Path;
///
/// let chunk = Chunk::new("my_program");
/// write_binary_file(Path::new("output.nbc"), &chunk)?;
/// ```
pub fn write_binary_file(path: &Path, chunk: &Chunk) -> Result<(), BinaryError> {
    // Serialize the chunk to binary format
    let binary_data = serialize_chunk(chunk)?;

    // Create the file and write the binary data
    let mut file = File::create(path).map_err(|e| {
        BinaryError::IoError(std::io::Error::new(
            e.kind(),
            format!("Failed to create file '{}': {}", path.display(), e),
        ))
    })?;

    file.write_all(&binary_data).map_err(|e| {
        BinaryError::IoError(std::io::Error::new(
            e.kind(),
            format!("Failed to write to file '{}': {}", path.display(), e),
        ))
    })?;

    Ok(())
}

/// Reads a compiled chunk from a binary file (.nbc)
///
/// This function reads a Neon binary file and deserializes it back into a Chunk
/// that can be executed by the VM.
///
/// # Arguments
/// * `path` - The file path to read from
///
/// # Returns
/// * `Ok(Chunk)` - The deserialized chunk
/// * `Err(BinaryError)` - If I/O or deserialization fails
///
/// # Errors
/// * `BinaryError::IoError` - If file reading fails (e.g., file not found, permission denied)
/// * `BinaryError::DeserializationError` - If the file content cannot be deserialized
/// * `BinaryError::InvalidFormat` - If the file is not a valid Neon binary
/// * `BinaryError::UnsupportedVersion` - If the binary format version is not supported
///
/// # Example
/// ```ignore
/// use neon::binary::io::read_binary_file;
/// use std::path::Path;
///
/// let chunk = read_binary_file(Path::new("program.nbc"))?;
/// // Execute the chunk with the VM
/// ```
pub fn read_binary_file(path: &Path) -> Result<Chunk, BinaryError> {
    // Open and read the file
    let mut file = File::open(path).map_err(|e| {
        BinaryError::IoError(std::io::Error::new(
            e.kind(),
            format!("Failed to open file '{}': {}", path.display(), e),
        ))
    })?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| {
        BinaryError::IoError(std::io::Error::new(
            e.kind(),
            format!("Failed to read from file '{}': {}", path.display(), e),
        ))
    })?;

    // Deserialize the binary data into a chunk
    deserialize_chunk(&buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_write_read_round_trip() {
        // Create a temporary directory for test files
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("test.nbc");

        // Create a simple chunk
        let original_chunk = Chunk::new("test_program");

        // Write the chunk to file
        write_binary_file(&file_path, &original_chunk).expect("Write should succeed");

        // Verify file exists
        assert!(file_path.exists());

        // Read the chunk back from file
        let read_chunk = read_binary_file(&file_path).expect("Read should succeed");

        // Verify the chunks match by comparing their serialized forms
        let original_serialized = serialize_chunk(&original_chunk).expect("Serialization should succeed");
        let read_serialized = serialize_chunk(&read_chunk).expect("Serialization should succeed");
        assert_eq!(original_serialized, read_serialized);
    }

    #[test]
    fn test_read_nonexistent_file() {
        let path = Path::new("/nonexistent/path/to/file.nbc");
        let result = read_binary_file(path);

        assert!(result.is_err());
        match result {
            Err(BinaryError::IoError(e)) => {
                // Should contain information about the file path
                let error_msg = e.to_string();
                assert!(error_msg.contains("file.nbc"));
            }
            _ => panic!("Expected IoError for nonexistent file"),
        }
    }

    #[test]
    fn test_read_invalid_binary_format() {
        // Create a temporary directory for test files
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("invalid.nbc");

        // Write invalid data to file
        let invalid_data = b"This is not a valid Neon binary file";
        fs::write(&file_path, invalid_data).expect("Write should succeed");

        // Try to read it
        let result = read_binary_file(&file_path);

        assert!(result.is_err());
        // Should get either InvalidFormat or DeserializationError
        assert!(matches!(
            result,
            Err(BinaryError::InvalidFormat(_)) | Err(BinaryError::DeserializationError(_))
        ));
    }

    #[test]
    fn test_write_to_readonly_directory() {
        // Try to write to a path that should fail (e.g., root directory on Unix-like systems)
        // This test might not work on all systems, so we'll make it conditional
        #[cfg(unix)]
        {
            let path = Path::new("/root/test.nbc");
            let chunk = Chunk::new("test");
            let result = write_binary_file(path, &chunk);

            // This should fail with permission denied or similar
            // Note: If running as root, this test might not fail, so we check if it's an error
            if result.is_err() {
                match result {
                    Err(BinaryError::IoError(_)) => {
                        // Expected error type
                    }
                    _ => panic!("Expected IoError for permission denied"),
                }
            }
        }
    }

    #[test]
    fn test_overwrite_existing_file() {
        // Create a temporary directory for test files
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("overwrite.nbc");

        // Write first chunk
        let chunk1 = Chunk::new("first");
        write_binary_file(&file_path, &chunk1).expect("First write should succeed");

        // Write second chunk to same path
        let chunk2 = Chunk::new("second");
        write_binary_file(&file_path, &chunk2).expect("Second write should succeed");

        // Read back and verify it's the second chunk
        let read_chunk = read_binary_file(&file_path).expect("Read should succeed");
        let read_serialized = serialize_chunk(&read_chunk).expect("Serialization should succeed");
        let chunk2_serialized = serialize_chunk(&chunk2).expect("Serialization should succeed");
        assert_eq!(read_serialized, chunk2_serialized);
    }
}
