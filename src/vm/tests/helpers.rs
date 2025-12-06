use crate::common::{Bloq, CallFrame, ObjFunction};
use crate::vm::{Result, VirtualMachine};
use std::rc::Rc;

/// Test helper trait that provides run_bloq() for testing VM execution
pub trait VmTestHelpers {
    fn run_bloq(&mut self, bloq: Bloq) -> Result;
}

impl VmTestHelpers for VirtualMachine {
    fn run_bloq(&mut self, bloq: Bloq) -> Result {
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
