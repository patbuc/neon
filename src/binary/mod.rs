/// Binary format module for Neon compiled bytecode
///
/// This module provides functionality for serializing and deserializing
/// Neon bytecode to/from a binary format that can be saved to disk and
/// executed later.

pub mod format;

pub use format::{BinaryFormat, BinaryHeader, FORMAT_VERSION, MAGIC_NUMBER};
