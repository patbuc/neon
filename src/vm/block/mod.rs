pub(self) mod block;
mod constants;
mod lines;
mod opcodes;

#[cfg(feature = "disassemble")]
pub(super) mod disassembler;

pub(crate) use crate::vm::block::opcodes::OpCode;
use crate::vm::vm::Value;

pub struct Block {
    name: String,
    constants: Constants,
    instructions: Vec<u8>,
    lines: Vec<Line>,
}

struct Constants {
    values: Vec<Value>,
}

struct Line {
    pub line: usize,
    pub offset: usize,
}

#[cfg(test)]
mod tests;
