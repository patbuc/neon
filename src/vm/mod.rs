mod block;
mod opcodes;
mod virtual_machine;

pub type Value = f64;

#[derive(Debug, PartialEq)]
pub enum Result {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VirtualMachine {
    ip: usize,
    stack: Vec<Value>,
}

pub(crate) struct Block {
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
