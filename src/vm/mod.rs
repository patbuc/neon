use crate::common::{Brick, Value};
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[macro_use]
mod value;

mod virtual_machine;

#[derive(Debug, PartialEq)]
pub enum Result {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VirtualMachine {
    ip: usize,
    stack: Vec<Value>,
    brick: Option<Rc<Brick>>,
    // values: HashMap<String, Value>,
    // variables: HashMap<String, Value>,
    #[cfg(test)]
    string_buffer: String,
    compilation_errors: String,
}
