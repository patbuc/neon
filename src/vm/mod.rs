use crate::common::{Brick, Value};
use std::fmt::Debug;
use std::rc::Rc;

mod functions;
mod r#impl;
#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum Result {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VirtualMachine {
    ip: usize,
    // frames: Vec<CallFrame>,
    // frame_count: usize,
    stack: Vec<Value>,
    brick: Option<Rc<Brick>>,
    // values: HashMap<String, Value>,
    // variables: HashMap<String, Value>,
    #[cfg(test)]
    string_buffer: String,
    compilation_errors: String,
}
