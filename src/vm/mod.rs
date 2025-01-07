use crate::vm::utils::output_handler::OutputHandler;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

#[macro_use]
mod value;

mod block;
pub(crate) mod opcodes;

mod utils;
mod virtual_machine;

#[repr(u8)]
pub enum BitsSize {
    Eight,
    Sixteen,
    ThirtyTwo,
}

impl BitsSize {
    pub fn as_bytes(&self) -> usize {
        match self {
            BitsSize::Eight => 1,
            BitsSize::Sixteen => 2,
            BitsSize::ThirtyTwo => 4,
        }
    }
}

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
    block: Option<Rc<Block>>,
    globals: HashMap<String, Value>,
    output_handler: Box<dyn OutputHandler>,
}

impl VirtualMachine {
    pub(crate) fn get_output(&self) -> String {
        self.output_handler.get_output()
    }
}

#[derive(Debug)]
pub(crate) struct Block {
    name: String,
    constants: Constants,
    globals: Vec<String>,
    strings: Constants,
    instructions: Vec<u8>,
    lines: Vec<Line>,
}

#[derive(Debug)]
struct Constants {
    values: Vec<Value>,
}

#[derive(Debug, Clone)]
struct Line {
    pub offset: usize,
    pub line: u32,
}
