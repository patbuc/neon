mod constants;
mod r#impl;

#[cfg(feature = "disassemble")]
mod disassembler;

pub mod binary;
#[cfg(test)]
mod tests;
