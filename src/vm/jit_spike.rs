//! Spike only: hand-written stand-ins for what a JIT would emit for
//! `fib` and `work`. NEON_JIT_SPIKE=1 is a baseline JIT (the same handler
//! calls, no dispatch); =2 is a type-specialized one (native i64).
use crate::common::{ObjClosure, Value};
use crate::vm::functions::{Comparison, OpResult};
use crate::vm::VirtualMachine;
use std::rc::Rc;
use std::sync::OnceLock;

type Compiled = fn(&mut VirtualMachine) -> OpResult;

static MODE: OnceLock<u8> = OnceLock::new();

#[inline(always)]
pub(in crate::vm) fn mode() -> u8 {
    *MODE.get_or_init(|| {
        std::env::var("NEON_JIT_SPIKE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
    })
}

#[inline(always)]
pub(in crate::vm) fn lookup(closure: &ObjClosure) -> Option<Compiled> {
    match closure.function.name.as_str() {
        "fib" => Some(fib),
        "work" => Some(work),
        _ => None,
    }
}

fn fib(vm: &mut VirtualMachine) -> OpResult {
    vm.op_get_local(0)?;
    if vm.op_compare_constant_and_pop(0, Comparison::LessEqual)? {
        vm.op_get_local(0)?;
        vm.op_return();
        return Ok(());
    }
    vm.op_get_global(1)?;
    vm.op_get_local(0)?;
    vm.op_subtract_constant(0)?;
    vm.jit_call(1)?;
    vm.op_get_global(1)?;
    vm.op_get_local(0)?;
    vm.op_subtract_constant(1)?;
    vm.jit_call(1)?;
    vm.op_add()?;
    vm.op_return();
    Ok(())
}

fn work(vm: &mut VirtualMachine) -> OpResult {
    vm.op_constant(0);
    vm.op_constant(0);
    loop {
        vm.op_get_local(2)?;
        vm.op_get_local(0)?;
        if !vm.op_compare_and_pop(Comparison::Less)? {
            break;
        }
        vm.op_get_local(1)?;
        vm.op_get_local(2)?;
        vm.op_multiply_constant(1)?;
        vm.op_add()?;
        vm.op_get_local(2)?;
        vm.op_modulo_constant(2)?;
        vm.op_subtract()?;
        vm.op_modulo_constant(3)?;
        vm.op_store_local(1)?;
        vm.op_get_local(2)?;
        vm.op_add_constant(4)?;
        vm.op_store_local(2)?;
    }
    vm.op_get_local(1)?;
    vm.op_return();
    Ok(())
}

fn fib_i64(n: i64) -> Option<i64> {
    if n <= 1 {
        return Some(n);
    }
    fib_i64(n.checked_sub(1)?)?.checked_add(fib_i64(n.checked_sub(2)?)?)
}

fn work_i64(n: i64) -> Option<i64> {
    let mut total: i64 = 0;
    let mut i: i64 = 0;
    while i < n {
        total = total
            .checked_add(i.checked_mul(3)?)?
            .checked_sub(i % 7)?
            % 1000000007;
        i = i.checked_add(1)?;
    }
    Some(total)
}

impl VirtualMachine {
    pub(in crate::vm) fn run_compiled(
        &mut self,
        arg_count: usize,
        closure: Rc<ObjClosure>,
        compiled: Compiled,
    ) -> OpResult {
        if mode() == 2 && arg_count == 1 {
            if let Some(&Value::Int(n)) = self.stack.last() {
                let native = match closure.function.name.as_str() {
                    "fib" => fib_i64,
                    _ => work_i64,
                };
                if let Some(result) = native(n) {
                    let len = self.stack.len();
                    self.stack.truncate(len - 2);
                    self.push(Value::Int(result));
                    return Ok(());
                }
            }
        }
        if arg_count != closure.function.arity as usize {
            return self.call_closure(arg_count, closure);
        }
        let slot_start = self.stack.len() as isize - arg_count as isize - 1;
        self.push_frame(closure, slot_start);
        compiled(self)
    }

    fn jit_call(&mut self, arg_count: u8) -> OpResult {
        let arg_count = arg_count as usize;
        self.check_frame_limit()?;
        let callable_index = self.stack.len() - 1 - arg_count;
        let callable_value = std::mem::replace(&mut self.stack[callable_index], Value::Nil);
        match callable_value {
            Value::Closure(closure) => match lookup(&closure) {
                Some(compiled) => self.run_compiled(arg_count, closure, compiled),
                None => {
                    let depth = self.call_frames.len();
                    self.call_closure(arg_count, closure)?;
                    self.run_until(depth)
                }
            },
            other => self.dispatch_call_value(other, arg_count),
        }
    }
}
