use crate::common::{Bloq, CallFrame, Value};
use std::fmt::Debug;

mod functions;
mod r#impl;
pub(crate) mod boolean_functions;
pub(crate) mod number_functions;
pub(crate) mod string_functions;
pub(crate) mod array_functions;
pub(crate) mod map_functions;
pub(crate) mod set_functions;
mod math_functions;
pub(crate) mod file_functions;
#[cfg(test)]
pub mod tests;

// Native functions for testing and built-in operations
pub mod native_functions {
    use crate::common::Value;
    use crate::vm::VirtualMachine;

    /// Test native function that adds two numbers
    /// This demonstrates how native functions work and can be used for testing
    pub fn native_add(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
        if args.len() != 2 {
            return Err(format!("native_add expects 2 arguments, got {}", args.len()));
        }

        match (&args[0], &args[1]) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            _ => Err("native_add requires two number arguments".to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Result {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VirtualMachine {
    #[cfg(test)]
    pub(crate) call_frames: Vec<CallFrame>,
    #[cfg(not(test))]
    call_frames: Vec<CallFrame>,
    #[cfg(test)]
    pub(crate) stack: Vec<Value>,
    #[cfg(not(test))]
    stack: Vec<Value>,
    bloq: Option<Bloq>,
    /// Global built-in values (like Math) stored separately from the call stack
    globals: std::collections::HashMap<String, Value>,
    #[cfg(any(test, debug_assertions, target_arch = "wasm32"))]
    string_buffer: String,
    compilation_errors: String,
    structured_errors: Vec<crate::common::errors::CompilationError>,
    runtime_errors: String,
    source: String,
    /// Iterator stack: Vec of (current_index, collection_value)
    /// Used for for-in loops to track iteration progress
    /// Supports nested for-in loops by maintaining a stack of iterators
    iterator_stack: Vec<(usize, Value)>,
}

