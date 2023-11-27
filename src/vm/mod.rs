use std::fmt::{Debug, Formatter};

mod block;
pub(crate) mod opcodes;
mod value;
mod virtual_machine;

// pub type Value = f64;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValueType {
    Number,
    Bool,
    String,
    Nil,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ValueUnion {
    number: f64,
    boolean: bool,
    string: *const String,
}

impl Debug for ValueUnion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValueUnion")
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Value {
    pub value_type: ValueType,
    pub value: ValueUnion,
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
