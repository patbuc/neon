use crate::common::{CallFrame, Chunk, ObjClosure, Upvalue, Value};
use std::cell::RefCell;
use std::collections::HashMap;
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
    #[cfg(test)]
    pub(crate) call_frames: Vec<CallFrame>,
    #[cfg(not(test))]
    call_frames: Vec<CallFrame>,
    #[cfg(test)]
    pub(crate) stack: Vec<Value>,
    #[cfg(not(test))]
    stack: Vec<Value>,
    chunk: Option<Chunk>,
    /// Global built-in values (like Math) stored separately from the call stack
    builtin: indexmap::IndexMap<String, Value>,
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
    pub(crate) fn run_chunk(&mut self, chunk: Chunk) -> Result {
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
            iterator_depth: self.iterator_stack.len(),
        };
        self.call_frames.push(frame);

        self.run_until(0)
    }
}
