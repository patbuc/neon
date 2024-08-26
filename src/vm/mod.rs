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
    output_handler: Box<dyn OutputHandler>,
    values: HashMap<String, Value>,
    variables: HashMap<String, Value>,
}

#[derive(Debug)]
pub(crate) struct Block {
    name: String,
    constants: Constants,
    variables: Variables,
    strings: Constants,
    instructions: Vec<u8>,
    lines: Vec<Line>,
}

#[derive(Debug)]
struct Constants {
    values: Vec<Value>,
}

#[derive(Debug)]
struct Variables {
    values: Vec<String>,
    variables: Vec<String>,
}

#[derive(Debug, Clone)]
struct Line {
    pub offset: usize,
    pub line: u32,
}
