mod block;
pub(crate) mod opcodes;
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

#[derive(Debug)]
pub(crate) struct Block {
    name: String,
    constants: Constants,
    instructions: Vec<u8>,
    lines: Vec<Line>,
}

#[derive(Debug)]
struct Constants {
    values: Vec<Value>,
}

#[derive(Debug)]
struct Line {
    pub line: usize,
    pub offset: usize,
}
