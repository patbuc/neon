#[cfg(test)]
use crate::common::Chunk;
use crate::common::{CallFrame, ObjClosure, Upvalue, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

mod functions;
mod r#impl;
mod runtime_error;
#[cfg(test)]
mod tests;

pub use runtime_error::{RuntimeError, TraceFrame};

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
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
    /// Runtime builtin values (e.g. `args`), stored separately from the
    /// call stack. Math and File are namespaces, not values here.
    builtin: Vec<Value>,
    #[cfg(any(test, debug_assertions, target_arch = "wasm32"))]
    string_buffer: String,
    structured_errors: Vec<crate::common::errors::CompilationError>,
    runtime_error: Option<RuntimeError>,
    source: String,
    /// Upvalues still pointing at a live stack slot, so closures created
    /// from the same slot share one cell instead of each getting their own.
    open_upvalues: Vec<Rc<RefCell<Upvalue>>>,
    /// How many `call_value` calls are currently nested on the Rust stack.
    native_call_depth: usize,
    /// User-defined methods from `impl` blocks, keyed by type name then
    /// method name, alongside whether the method takes `self`.
    methods: HashMap<String, HashMap<String, (Rc<ObjClosure>, bool)>>,
}

// Test-only methods
#[cfg(test)]
impl VirtualMachine {
    pub(crate) fn run_chunk(&mut self, chunk: Chunk) -> InterpretResult {
        use crate::common::{ObjClosure, ObjFunction};
        use std::rc::Rc;

        // Create a synthetic function for the test chunk
        let test_function = Rc::new(ObjFunction {
            name: "<test>".to_string(),
            arity: 0,
            chunk: Rc::new(chunk),
        });
        let test_closure = Rc::new(ObjClosure {
            function: test_function,
            upvalues: Vec::new(),
        });

        // Create the initial call frame
        let frame = CallFrame {
            closure: test_closure,
            ip: 0,
            slot_start: -1, // Like script frame, no function object on stack
        };
        self.call_frames.push(frame);

        self.run_script(0)
    }
}
