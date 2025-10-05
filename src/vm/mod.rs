use crate::common::{Brick, CallFrame, Value};
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
    brick: Option<Brick>,
    // values: HashMap<String, Value>,
    // variables: HashMap<String, Value>,
    #[cfg(test)]
    pub(crate) string_buffer: String,
    compilation_errors: String,
}

// Test-only methods
#[cfg(test)]
impl VirtualMachine {
    pub(crate) fn run_brick(&mut self, brick: Brick) -> Result {
        use crate::common::ObjFunction;
        use std::rc::Rc;

        // Create a synthetic function for the test brick
        let test_function = Rc::new(ObjFunction {
            name: "<test>".to_string(),
            arity: 0,
            brick: Rc::new(brick),
        });

        // Create the initial call frame
        let frame = CallFrame {
            function: test_function,
            ip: 0,
            slot_start: -1,  // Like script frame, no function object on stack
        };
        self.call_frames.push(frame);

        self.run(&Brick::new("dummy"))
    }
}
