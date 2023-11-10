pub(self) mod block;
mod constants;
mod opcodes;

#[cfg(feature = "disassemble")]
mod disassembler;

pub(crate) use crate::vm::block::opcodes::OpCode;

#[allow(dead_code)]
pub struct Block {
    name: String,
    constants: Constants,
    instructions: Vec<u8>,
    lines: Vec<usize>,
}

type Value = f64;
pub struct Constants {
    values: Vec<Value>,
}

#[cfg(test)]
mod tests;
