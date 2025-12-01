use crate::common::{Bloq, CallFrame, Value};
use std::fmt::Debug;

mod functions;
mod r#impl;
mod boolean_functions;
mod number_functions;
mod string_functions;
mod map_functions;
mod math_functions;
#[cfg(test)]
mod tests;

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
    // values: HashMap<String, Value>,
    // variables: HashMap<String, Value>,
    #[cfg(any(test, debug_assertions, target_arch = "wasm32"))]
    string_buffer: String,
    compilation_errors: String,
    structured_errors: Vec<crate::common::errors::CompilationError>,
    source: String,
}

// Test-only methods
#[cfg(test)]
impl VirtualMachine {
    pub(crate) fn run_bloq(&mut self, bloq: Bloq) -> Result {
        use crate::common::ObjFunction;
        use std::rc::Rc;

        // Create a synthetic function for the test bloq
        let test_function = Rc::new(ObjFunction {
            name: "<test>".to_string(),
            arity: 0,
            bloq: Rc::new(bloq),
        });

        // Create the initial call frame
        let frame = CallFrame {
            function: test_function,
            ip: 0,
            slot_start: -1, // Like script frame, no function object on stack
        };
        self.call_frames.push(frame);

        self.run(&Bloq::new("dummy"))
    }
}
