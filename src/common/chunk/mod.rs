mod constants;
mod decode;
mod r#impl;

#[cfg(any(test, feature = "disassemble"))]
mod disassembler;
#[cfg(test)]
mod tests;

pub(crate) use decode::Instr;
