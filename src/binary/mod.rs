/// Binary format module for Neon compiled bytecode
///
/// This module provides functionality for serializing and deserializing
/// Neon bytecode to/from a binary format that can be saved to disk and
/// executed later.
///
/// # File I/O
///
/// The `io` module provides high-level functions for reading and writing
/// compiled Neon bytecode to disk:
///
/// * [`io::write_binary_file`] - Write a chunk to a .nbc file
/// * [`io::read_binary_file`] - Read a chunk from a .nbc file
///
/// # Format
///
/// The `format` module provides low-level serialization/deserialization:
///
/// * [`format::serialize_chunk`] - Serialize a chunk to bytes
/// * [`format::deserialize_chunk`] - Deserialize a chunk from bytes
///
/// # Example
///
/// ```ignore
/// use neon::binary::io::{write_binary_file, read_binary_file};
/// use neon::common::Chunk;
/// use std::path::Path;
///
/// // Compile and save a chunk
/// let chunk = Chunk::new("my_program");
/// write_binary_file(Path::new("program.nbc"), &chunk)?;
///
/// // Later, load and execute the chunk
/// let loaded_chunk = read_binary_file(Path::new("program.nbc"))?;
/// // Execute with VM...
/// ```
pub mod error;
pub mod format;
pub mod io;

pub use error::BinaryError;
pub use format::{
    deserialize_chunk, serialize_chunk, BinaryFormat, BinaryHeader, FORMAT_VERSION, MAGIC_NUMBER,
};
pub use io::{read_binary_file, write_binary_file};
