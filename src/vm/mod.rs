use crate::common::{CallFrame, Chunk, ModuleState, Value};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;
use std::rc::Rc;

mod functions;
mod r#impl;
#[cfg(test)]
mod tests;

/// Execution context for the VM - tracks whether we're running a script or module
#[derive(Debug, Clone)]
pub(crate) enum ExecutionContext {
    /// Normal script execution
    Script,
    /// Module initialization - captures globals after execution
    Module {
        source_path: PathBuf,
        stack_base: usize,  // Stack position where module globals start
    },
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
    /// Module cache: Maps canonical module paths to compiled module states
    /// Ensures modules are loaded and initialized only once
    module_cache: HashMap<PathBuf, Rc<ModuleState>>,
    /// Current execution context - determines how the VM handles execution
    execution_context: ExecutionContext,
}

// Test-only methods
#[cfg(test)]
impl VirtualMachine {
    pub(crate) fn run_chunk(&mut self, chunk: Chunk) -> Result {
        use crate::common::ObjFunction;
        use std::rc::Rc;

        // Create a synthetic function for the test chunk
        let test_function = Rc::new(ObjFunction {
            name: "<test>".to_string(),
            arity: 0,
            chunk: Rc::new(chunk),
            metadata: None, // Tests don't need module metadata
        });

        // Create the initial call frame
        let frame = CallFrame {
            function: test_function,
            ip: 0,
            slot_start: -1, // Like script frame, no function object on stack
        };
        self.call_frames.push(frame);

        self.run()
    }
}
