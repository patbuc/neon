pub mod binary;
pub mod common;
pub mod compiler;
pub mod macros;
pub mod vm;

#[cfg(target_arch = "wasm32")]
pub mod wasm;
