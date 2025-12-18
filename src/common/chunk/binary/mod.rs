pub mod error;
pub mod format;
pub mod io;

pub use error::BinaryError;
pub use format::{
    deserialize_chunk, serialize_chunk, BinaryFormat, BinaryHeader, FORMAT_VERSION, MAGIC_NUMBER,
};
pub use io::{read_binary_file, write_binary_file};
