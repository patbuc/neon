use crate::common::{CallFrame, ObjClosure, Upvalue, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// `ObjFiber::stack_id` before the fiber has run for the first time. The
/// VM assigns a real id when it first switches to the fiber, so an open
/// upvalue can name which fiber's stack holds the slot it points at.
pub const UNASSIGNED_STACK_ID: u32 = u32::MAX;

/// The stack id of the main script, which always runs first.
pub const MAIN_STACK_ID: u32 = 0;

/// What kind of coroutine an `ObjFiber` is. The main script is one too, so
/// the VM can save and restore its state like any other fiber.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FiberKind {
    /// The script itself; the fiber everything else is eventually resumed from.
    Main,
    /// A cooperative coroutine sharing the heap with its caller
    /// (`Fiber(...)`, `call`, `Fiber.yield`).
    Fiber,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FiberState {
    /// Created but never resumed; the body has not been called yet.
    Fresh,
    /// Parked in `Fiber.yield`, waiting for the next `call`.
    Suspended,
    /// The fiber whose stack and frames are currently live in the VM.
    Running,
    /// Resumed another fiber and is waiting for it to yield or finish.
    Waiting,
    /// Its body returned; it can never run again.
    Done,
}

/// A coroutine: its own call frames, value stack, and open upvalues, saved
/// here while it is not the running fiber. Only one fiber runs at a time;
/// the VM swaps a fiber's three vectors in and out of its own fields when
/// switching, so the interpreter loop never knows which fiber it runs.
pub struct ObjFiber {
    pub kind: FiberKind,
    pub state: FiberState,
    /// The closure the fiber runs; its arity (0 or 1) decides whether the
    /// first `call` argument is passed to it.
    pub body: Rc<ObjClosure>,
    pub frames: Vec<CallFrame>,
    pub stack: Vec<Value>,
    pub open_upvalues: Vec<Rc<RefCell<Upvalue>>>,
    /// The fiber that resumed this one, while this one is running or
    /// waiting on a fiber it resumed itself. `Fiber.yield` and a root-frame
    /// return switch back to it.
    pub caller: Option<Rc<RefCell<ObjFiber>>>,
    /// Which stack `Upvalue::Open` entries pointing into this fiber name.
    pub stack_id: u32,
}

impl ObjFiber {
    pub fn new(kind: FiberKind, body: Rc<ObjClosure>) -> Self {
        ObjFiber {
            kind,
            state: FiberState::Fresh,
            body,
            frames: Vec::new(),
            stack: Vec::new(),
            open_upvalues: Vec::new(),
            caller: None,
            stack_id: UNASSIGNED_STACK_ID,
        }
    }
}

impl Drop for ObjFiber {
    /// A closure that captured one of this fiber's locals may outlive the
    /// fiber. Its upvalue still points into this stack, so copy the final
    /// value out before the stack goes away, exactly as a returning frame
    /// does for its own locals.
    fn drop(&mut self) {
        for upvalue in &self.open_upvalues {
            let index = match *upvalue.borrow() {
                Upvalue::Open { index, .. } => index,
                Upvalue::Closed(_) => continue,
            };
            let value = self.stack.get(index).cloned().unwrap_or(Value::Nil);
            *upvalue.borrow_mut() = Upvalue::Closed(value);
        }
    }
}

impl std::fmt::Debug for ObjFiber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjFiber")
            .field("kind", &self.kind)
            .field("state", &self.state)
            .field("body", &self.body.function.name)
            .finish_non_exhaustive()
    }
}
