use crate::common::{Bloq, CallFrame, Value};
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
    #[cfg(any(test, debug_assertions))]
    pub(crate) string_buffer: String,
    compilation_errors: String,
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
            slot_start: -1,  // Like script frame, no function object on stack
        };
        self.call_frames.push(frame);

        self.run(&Bloq::new("dummy"))
    }
}
