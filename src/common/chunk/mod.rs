mod constants;
mod decode;
mod r#impl;

#[cfg(any(test, feature = "disassemble"))]
mod disassembler;
#[cfg(test)]
mod tests;

#[allow(unused_imports)]
pub(crate) use decode::Instr;
