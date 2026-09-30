//! Fiber switching.
//!
//! A fiber owns a value stack, call frames, and open upvalues. Exactly one
//! fiber runs at a time: its three vectors are the VM's own `stack`,
//! `call_frames`, and `open_upvalues`, and every other fiber's are parked
//! in its `ObjFiber`. Switching swaps the vectors, so the interpreter loop
//! is oblivious to fibers. The main script is a fiber too, created lazily
//! the first time the script resumes another one.
//!
//! Switching is only allowed while no native callback (`map`, `filter`,
//! `reduce`) is on the Rust stack: `call_value` runs the loop re-entrantly
//! and counts frames to know when its callee returned, which a different
//! fiber's frames would confuse.

use crate::common::fiber::{FiberKind, FiberState, ObjFiber};
use crate::common::method_registry::ControlOp;
use crate::common::Value;
use crate::vm::functions::OpResult;
use crate::vm::{RuntimeError, VirtualMachine};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// The fibers the VM knows about once the script has switched away from
/// the main fiber at least once.
pub(in crate::vm) struct ActiveFibers {
    /// The script itself. Its saved stack is where globals live while
    /// another fiber runs.
    pub main: Rc<RefCell<ObjFiber>>,
    /// The fiber whose state is live in the VM right now.
    pub current: Rc<RefCell<ObjFiber>>,
}

impl VirtualMachine {
    /// Whether a fiber other than the main script is running.
    #[inline(always)]
    pub(in crate::vm) fn in_child_fiber(&self) -> bool {
        self.fibers
            .as_ref()
            .is_some_and(|active| !Rc::ptr_eq(&active.current, &active.main))
    }

    /// Executes a control operation. The stack holds `[base, args...]`
    /// where `base` is the receiver (`fiber.call`) or the
    /// static callable (`Fiber.yield`); all of it is removed before the
    /// switch, and the operation's result is pushed by whichever fiber
    /// switches back.
    pub(in crate::vm) fn control_op(
        &mut self,
        op: ControlOp,
        base_index: usize,
        arg_count: usize,
    ) -> OpResult {
        let name = match op {
            ControlOp::FiberCall => "call",
            ControlOp::FiberYield => "yield",
        };
        if arg_count > 1 {
            return Err(self.call_error(format!(
                "{}() takes at most one argument, got {}",
                name, arg_count
            )));
        }
        if self.native_call_depth > 0 {
            return Err(self.call_error(format!(
                "Cannot {} a fiber from inside a native callback",
                if op == ControlOp::FiberYield {
                    "yield"
                } else {
                    "resume"
                }
            )));
        }
        let arg = (arg_count == 1).then(|| self.peek(0).clone());

        match op {
            ControlOp::FiberYield => self.yield_fiber(base_index, arg.unwrap_or(Value::Nil)),
            ControlOp::FiberCall => {
                let Value::Fiber(fiber) = self.stack[base_index].clone() else {
                    unreachable!("control op dispatched on a non-fiber receiver")
                };
                self.resume_fiber(base_index, &fiber, arg)
            }
        }
    }

    /// `fiber.call(arg)`: park the running fiber as the
    /// callee's caller and make the callee current. A fresh fiber gets its
    /// body called with `arg` (if the body takes a parameter); a suspended
    /// one gets `arg` as the value of the `Fiber.yield` it is parked on.
    fn resume_fiber(
        &mut self,
        base_index: usize,
        fiber: &Rc<RefCell<ObjFiber>>,
        arg: Option<Value>,
    ) -> OpResult {
        let (state, body) = {
            let f = fiber.borrow();
            (f.state, Rc::clone(&f.body))
        };
        match state {
            FiberState::Done => {
                return Err(self.call_error("Cannot resume a finished fiber"));
            }
            FiberState::Running | FiberState::Waiting => {
                return Err(self.call_error("The fiber is already running"));
            }
            FiberState::Fresh | FiberState::Suspended => {}
        }
        let arity = body.function.arity as usize;
        if state == FiberState::Fresh && arity == 0 && arg.is_some() {
            return Err(self.call_error(format!(
                "Fiber body '{}' takes no parameter, but one argument was passed",
                body.function.name
            )));
        }
        self.stack.truncate(base_index);

        let current = self.ensure_active();
        current.borrow_mut().state = FiberState::Waiting;
        {
            let mut target = fiber.borrow_mut();
            target.caller = Some(current);
            target.depth = self.fiber_depth + self.call_frames.len();
        }
        self.switch_to(fiber);
        fiber.borrow_mut().state = FiberState::Running;

        match state {
            FiberState::Fresh => {
                // Stack layout of a fiber's root frame: `[body, arg?]`, the
                // same `[callee, args...]` shape as any other call.
                self.push(Value::Closure(Rc::clone(&body)));
                if arity == 1 {
                    self.push(arg.unwrap_or(Value::Nil));
                }
                self.call_closure(arity, &body)
            }
            _ => {
                self.push(arg.unwrap_or(Value::Nil));
                Ok(())
            }
        }
    }

    /// `Fiber.yield(value)`: suspend the running fiber and make its caller
    /// current again, with `value` as the result of the caller's `call`.
    fn yield_fiber(&mut self, base_index: usize, value: Value) -> OpResult {
        let current = match &self.fibers {
            Some(active) => Rc::clone(&active.current),
            None => return Err(self.call_error("Cannot yield from the main script")),
        };
        if current.borrow().kind == FiberKind::Main {
            return Err(self.call_error("Cannot yield from the main script"));
        }

        self.stack.truncate(base_index);

        let caller = {
            let mut f = current.borrow_mut();
            f.state = FiberState::Suspended;
            f.caller
                .take()
                .expect("a running fiber always has a caller")
        };
        self.switch_to(&caller);
        caller.borrow_mut().state = FiberState::Running;
        self.push(value);
        Ok(())
    }

    /// A fiber's root frame returned with `value`: the fiber is done, and
    /// its caller resumes with the value as the result of its `call`.
    pub(in crate::vm) fn finish_fiber(&mut self, value: Value) -> OpResult {
        let current = Rc::clone(
            &self
                .fibers
                .as_ref()
                .expect("finish_fiber is only called in a child fiber")
                .current,
        );

        self.call_frames.pop();
        self.close_upvalues_above(0);
        self.stack.clear();

        let caller = {
            let mut f = current.borrow_mut();
            f.state = FiberState::Done;
            f.caller
                .take()
                .expect("a running fiber always has a caller")
        };
        self.switch_to(&caller);
        caller.borrow_mut().state = FiberState::Running;
        self.push(value);
        Ok(())
    }

    /// Creates the main fiber on first use and returns the running fiber.
    fn ensure_active(&mut self) -> Rc<RefCell<ObjFiber>> {
        if self.fibers.is_none() {
            let script = Rc::clone(&self.call_frames[0].closure);
            let mut main = ObjFiber::new(FiberKind::Main, script);
            main.state = FiberState::Running;
            let main = Rc::new(RefCell::new(main));
            self.fibers = Some(ActiveFibers {
                main: Rc::clone(&main),
                current: main,
            });
        }
        Rc::clone(&self.fibers.as_ref().expect("just created").current)
    }

    /// Parks the running fiber's state in its object and makes `target`'s
    /// state live.
    /// `ip` and `chunk` only track the running frame, so the outgoing top
    /// frame saves `ip` and the target's top frame (if it has one yet)
    /// restores both.
    fn switch_to(&mut self, target: &Rc<RefCell<ObjFiber>>) {
        if let Some(frame) = self.call_frames.last_mut() {
            frame.ip = self.ip;
        }
        let active = self
            .fibers
            .as_mut()
            .expect("switch_to requires an active main fiber");
        {
            let mut current = active.current.borrow_mut();
            std::mem::swap(&mut current.frames, &mut self.call_frames);
            std::mem::swap(&mut current.stack, &mut self.stack);
            std::mem::swap(&mut current.open_upvalues, &mut self.open_upvalues);
        }
        {
            let mut next = target.borrow_mut();
            std::mem::swap(&mut next.frames, &mut self.call_frames);
            std::mem::swap(&mut next.stack, &mut self.stack);
            std::mem::swap(&mut next.open_upvalues, &mut self.open_upvalues);
            self.fiber_depth = next.depth;
        }
        active.current = Rc::clone(target);
        if let Some(frame) = self.call_frames.last() {
            self.ip = frame.ip;
            self.chunk = Rc::clone(&frame.closure.function.chunk);
        }
    }

    /// The running fiber as an open upvalue's owner: `None` for the main
    /// script (see `Upvalue::Open`).
    pub(in crate::vm) fn running_child_fiber(&self) -> Option<Weak<RefCell<ObjFiber>>> {
        self.fibers
            .as_ref()
            .filter(|active| !Rc::ptr_eq(&active.current, &active.main))
            .map(|active| Rc::downgrade(&active.current))
    }

    /// Whether `owner`, an open upvalue's owner, is the running fiber, so
    /// its slot is in the VM's own stack.
    #[inline(always)]
    pub(in crate::vm) fn is_running(&self, owner: &Option<Weak<RefCell<ObjFiber>>>) -> bool {
        match owner {
            None => !self.in_child_fiber(),
            Some(owner) => self
                .fibers
                .as_ref()
                .is_some_and(|active| owner.as_ptr() == Rc::as_ptr(&active.current)),
        }
    }

    /// Reads slot `index` of a parked fiber's stack, for an open upvalue
    /// owned by a fiber other than the running one.
    pub(in crate::vm) fn parked_stack_get(
        &self,
        owner: &Option<Weak<RefCell<ObjFiber>>>,
        index: usize,
    ) -> Result<Value, RuntimeError> {
        let Some(owner) = owner else {
            return self.main_stack_get(index);
        };
        let fiber = self.upgrade_owner(owner)?;
        let value = fiber.borrow().stack.get(index).cloned();
        value.ok_or_else(|| self.runtime_error(format!("Invalid captured stack slot {}", index)))
    }

    /// Writes slot `index` of a parked fiber's stack; see `parked_stack_get`.
    pub(in crate::vm) fn parked_stack_set(
        &mut self,
        owner: &Option<Weak<RefCell<ObjFiber>>>,
        index: usize,
        value: Value,
    ) -> OpResult {
        let Some(owner) = owner else {
            return self.main_stack_set(index, value);
        };
        let fiber = self.upgrade_owner(owner)?;
        let stored = match fiber.borrow_mut().stack.get_mut(index) {
            Some(slot) => {
                *slot = value;
                true
            }
            None => false,
        };
        if stored {
            Ok(())
        } else {
            Err(self.runtime_error(format!("Invalid captured stack slot {}", index)))
        }
    }

    fn upgrade_owner(
        &self,
        owner: &Weak<RefCell<ObjFiber>>,
    ) -> Result<Rc<RefCell<ObjFiber>>, RuntimeError> {
        owner.upgrade().ok_or_else(|| {
            self.runtime_error("Captured variable belongs to a fiber that no longer exists")
        })
    }

    /// The main fiber's stack slot `index` while a child fiber runs: where
    /// globals live for `GetGlobal`/`SetGlobal`.
    pub(in crate::vm) fn main_stack_get(&self, index: usize) -> Result<Value, RuntimeError> {
        let active = self.fibers.as_ref().expect("no child fiber without a main");
        let main = active.main.borrow();
        main.stack.get(index).cloned().ok_or_else(|| {
            self.runtime_error(format!(
                "Global variable index {} out of bounds (stack size: {})",
                index,
                main.stack.len()
            ))
        })
    }

    pub(in crate::vm) fn main_stack_set(&mut self, index: usize, value: Value) -> OpResult {
        let active = self.fibers.as_ref().expect("no child fiber without a main");
        let mut main = active.main.borrow_mut();
        match main.stack.get_mut(index) {
            Some(slot) => {
                *slot = value;
                Ok(())
            }
            None => {
                let len = main.stack.len();
                drop(main);
                Err(self.runtime_error(format!(
                    "Global variable index {} out of bounds (stack size: {})",
                    index, len
                )))
            }
        }
    }

    /// The call frames of every fiber waiting on the running one, innermost
    /// caller first, for a runtime error's trace.
    pub(in crate::vm) fn caller_chain(&self) -> Vec<Rc<RefCell<ObjFiber>>> {
        let mut chain = Vec::new();
        let mut next = self
            .fibers
            .as_ref()
            .and_then(|active| active.current.borrow().caller.clone());
        while let Some(fiber) = next {
            next = fiber.borrow().caller.clone();
            chain.push(fiber);
        }
        chain
    }
}
