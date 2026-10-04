use crate::common::method_registry::NativeMethodTable;
use crate::common::runtime_error::RuntimeError;
use crate::common::{CallFrame, Chunk, ObjClosure, Upvalue, Value};
use crate::compiler::global_env::GlobalEnv;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

mod functions;
mod r#impl;
#[cfg(test)]
mod tests;

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
    /// The running (top) frame's instruction pointer and chunk; its
    /// `CallFrame.ip` is only kept current for the frames below it.
    ip: usize,
    chunk: Rc<Chunk>,
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
    /// User-defined methods from `impl` blocks, indexed by type symbol.
    /// Each entry is a Vec of (method symbol, closure, takes `self`).
    methods: Vec<Vec<(u16, Rc<ObjClosure>, bool)>>,
    /// Native methods of the builtin types, built from the running
    /// compile's symbol table.
    native_methods: NativeMethodTable,
    /// Globals, symbols and slot layout the REPL has accumulated so far.
    repl_env: GlobalEnv,
    /// Execution count per opcode byte, for the `opcode-stats` histogram.
    #[cfg(feature = "opcode-stats")]
    opcode_counts: [u64; 256],
    /// Execution count per consecutive opcode pair, indexed `prev * 256 +
    /// next`.
    #[cfg(feature = "opcode-stats")]
    opcode_pair_counts: Vec<u64>,
    /// The previously executed opcode byte, for pairing with the next one.
    #[cfg(feature = "opcode-stats")]
    last_opcode: Option<u8>,
}

// Test-only methods
#[cfg(test)]
impl VirtualMachine {
    pub(crate) fn run_chunk(&mut self, chunk: Chunk) -> InterpretResult {
        use crate::common::method_registry::native_method_table;
        use crate::common::{ObjClosure, ObjFunction};
        use std::rc::Rc;

        self.native_methods = native_method_table(&chunk.symbols);

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

        // Like the script frame: no function object on the stack
        self.push_frame(test_closure, -1);

        self.run_script(0)
    }
}
