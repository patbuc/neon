//! Spike only: hand-written stand-ins for what a JIT would emit for
//! `fib` and `work`. NEON_JIT_SPIKE=1 is a baseline JIT (the same handler
//! calls, no dispatch); =2 is a type-specialized one (native i64); =3 is
//! a baseline JIT with inline Int fast paths over tagged stack slots.
use crate::common::runtime_error::RuntimeError;
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
        "fib" if mode() == 3 => Some(fib_guarded),
        "work" if mode() == 3 => Some(work_guarded),
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

fn int_local(vm: &VirtualMachine, base: usize, slot: usize) -> Result<i64, RuntimeError> {
    match vm.stack[base + slot] {
        Value::Int(i) => Ok(i),
        _ => Err(vm.runtime_error("deopt")),
    }
}

fn overflow(vm: &VirtualMachine) -> RuntimeError {
    vm.runtime_error("overflow")
}

fn return_int(vm: &mut VirtualMachine, result: i64) -> OpResult {
    let slot_start = vm.current_frame().slot_start;
    vm.pop_frame();
    vm.stack.truncate(slot_start as usize);
    vm.push(Value::Int(result));
    Ok(())
}

fn call_global_int(vm: &mut VirtualMachine, global: usize, arg: i64) -> Result<i64, RuntimeError> {
    let callee = vm.stack[global].clone();
    vm.push(callee);
    vm.push(Value::Int(arg));
    vm.jit_call(1)?;
    match vm.pop() {
        Value::Int(i) => Ok(i),
        _ => Err(vm.runtime_error("deopt")),
    }
}

fn fib_guarded(vm: &mut VirtualMachine) -> OpResult {
    let base = (vm.current_frame().slot_start + 1) as usize;
    let n = int_local(vm, base, 0)?;
    if n <= 1 {
        return return_int(vm, n);
    }
    let a = call_global_int(vm, 1, n.checked_sub(1).ok_or_else(|| overflow(vm))?)?;
    let b = call_global_int(vm, 1, n.checked_sub(2).ok_or_else(|| overflow(vm))?)?;
    let result = a.checked_add(b).ok_or_else(|| overflow(vm))?;
    return_int(vm, result)
}

fn work_guarded(vm: &mut VirtualMachine) -> OpResult {
    let base = (vm.current_frame().slot_start + 1) as usize;
    vm.push(Value::Int(0));
    vm.push(Value::Int(0));
    loop {
        let i = int_local(vm, base, 2)?;
        if i >= int_local(vm, base, 0)? {
            break;
        }
        let total = int_local(vm, base, 1)?;
        let t = i.checked_mul(3).ok_or_else(|| overflow(vm))?;
        let t = total.checked_add(t).ok_or_else(|| overflow(vm))?;
        let t = t.checked_sub(i % 7).ok_or_else(|| overflow(vm))?;
        vm.stack[base + 1] = Value::Int(t % 1000000007);
        let i = int_local(vm, base, 2)?;
        vm.stack[base + 2] = Value::Int(i.checked_add(1).ok_or_else(|| overflow(vm))?);
    }
    let total = int_local(vm, base, 1)?;
    return_int(vm, total)
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

    pub(in crate::vm) fn jit_call(&mut self, arg_count: u8) -> OpResult {
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
