use std::fmt::Debug;

#[macro_use]
mod value;

mod block;
pub(crate) mod opcodes;

mod virtual_machine;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
}

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
    pub offset: usize,
    pub line: u32,
}
