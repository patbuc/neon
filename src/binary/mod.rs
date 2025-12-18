/// Binary format module for Neon compiled bytecode
///
/// This module provides functionality for serializing and deserializing
/// Neon bytecode to/from a binary format that can be saved to disk and
/// executed later.

pub mod error;
pub mod format;

pub use error::BinaryError;
pub use format::{
    deserialize_chunk, serialize_chunk, BinaryFormat, BinaryHeader, FORMAT_VERSION, MAGIC_NUMBER,
};
