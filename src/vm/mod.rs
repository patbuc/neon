use crate::common::{Brick, Value};
use std::fmt::Debug;

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
    stack: Vec<Value>,
    brick: Option<Brick>,
    // values: HashMap<String, Value>,
    // variables: HashMap<String, Value>,
    #[cfg(test)]
    string_buffer: String,
    compilation_errors: String,
}
