//! Spike: a Cranelift JIT for function bodies whose values are all Ints.
//!
//! Compiled code takes its arguments as raw i64s and keeps every frame
//! slot in an i64 variable. A call of a global goes straight to the
//! callee's compiled code, with no VM frame. Only functions using nothing
//! but supported instructions compile. An overflow, a call that can't
//! finish natively, or the depth limit exits to the interpreter at that
//! instruction: the code
//! writes its slots to a buffer and `restore_frame` rebuilds the frame on
//! the VM stack, so the interpreter resumes with the state it would have
//! had.
use crate::common::chunk::Instr;
use crate::common::runtime_error::RuntimeError;
use crate::common::{ObjClosure, ObjFunction, Value};
use crate::vm::functions::OpResult;
use crate::vm::VirtualMachine;
use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{
    types, AbiParam, Block, InstBuilder, MemFlags, Signature, StackSlotData, StackSlotKind,
    UserFuncName,
};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{default_libcall_names, Module};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// `(vm, args, buf) -> status`. `RETURNED`: the result is in `buf[0]`.
/// `ERROR`: a helper stored the error in `jit_error`. Otherwise the code
/// exited at instruction `status & !PENDING`, with the slot count in
/// `buf[0]`, the slots in `buf[1..]` and a bitmask of slots that hold a
/// global's index (not yet loaded) in `buf[GLOBALS]`. With `PENDING` set,
/// `jit_pending` is one more slot on top.
type JitFn = unsafe extern "C" fn(*mut VirtualMachine, *const i64, *mut i64) -> i64;
const RETURNED: i64 = -1;
const ERROR: i64 = -2;
const NOT_COMPILED: i64 = -3;
const PENDING: i64 = 1 << 40;
const MAX_DEPTH: usize = 62;
const GLOBALS: usize = MAX_DEPTH + 1;
const BUF_LEN: usize = MAX_DEPTH + 2;
/// Rust stack budget for compiled frames: a native call costs 1, an
/// interpreter run nested inside compiled code costs `NESTED_RUN`.
const DEPTH_LIMIT: i64 = 1000;
const NESTED_RUN: i64 = 100;

pub(in crate::vm) const NOT_COMPILABLE: usize = 1;

struct Jit {
    module: JITModule,
}

thread_local! {
    static JIT: RefCell<Option<Jit>> = RefCell::new(Jit::new());
}

/// The compiled code for `function`, compiling it on first use.
fn compiled(function: &ObjFunction) -> Option<JitFn> {
    match function.jit.get() {
        0 => {
            let code = JIT.with(|jit| jit.borrow_mut().as_mut()?.compile(function));
            function
                .jit
                .set(code.map_or(NOT_COMPILABLE, |code| code as usize));
            code
        }
        NOT_COMPILABLE => None,
        // SAFETY: only `compiled` stores code addresses, all of this type.
        code => Some(unsafe { std::mem::transmute::<usize, JitFn>(code) }),
    }
}

impl Jit {
    fn new() -> Option<Self> {
        if std::env::var("NEON_JIT").as_deref() == Ok("0") {
            return None;
        }
        let mut flags = settings::builder();
        flags.set("opt_level", "speed").ok()?;
        let isa = cranelift_native::builder()
            .ok()?
            .finish(settings::Flags::new(flags))
            .ok()?;
        let module = JITModule::new(JITBuilder::with_isa(isa, default_libcall_names()));
        Some(Jit { module })
    }

    fn compile(&mut self, function: &ObjFunction) -> Option<JitFn> {
        let mut ctx = self.module.make_context();
        ctx.func.signature = jit_signature(&self.module);
        let id = self
            .module
            .declare_anonymous_function(&ctx.func.signature)
            .ok()?;
        ctx.func.name = UserFuncName::user(0, id.as_u32());

        let mut fctx = FunctionBuilderContext::new();
        let builder = FunctionBuilder::new(&mut ctx.func, &mut fctx);
        Emitter::new(builder, &self.module, function)?.emit(function)?;

        self.module.define_function(id, &mut ctx).ok()?;
        self.module.clear_context(&mut ctx);
        self.module.finalize_definitions().ok()?;
        let code = self.module.get_finalized_function(id);
        // SAFETY: the function was built with exactly this signature.
        Some(unsafe { std::mem::transmute::<*const u8, JitFn>(code) })
    }
}

fn jit_signature(module: &JITModule) -> Signature {
    let pointer = module.target_config().pointer_type();
    let mut sig = module.make_signature();
    sig.params.extend([AbiParam::new(pointer); 3]);
    sig.returns.push(AbiParam::new(types::I64));
    sig
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Int,
    /// A global's value, not loaded yet; the variable holds its index.
    Global,
}

type Val = cranelift_codegen::ir::Value;

struct Emitter<'a> {
    b: FunctionBuilder<'a>,
    slots: Vec<Variable>,
    kinds: Vec<Kind>,
    vm: Val,
    buf: Val,
    pointer: types::Type,
    jit_sig: Signature,
    lookup_sig: Signature,
    finish_sig: Signature,
    blocks: HashMap<usize, Block>,
    entries: HashMap<usize, Vec<Kind>>,
}

impl<'a> Emitter<'a> {
    fn new(mut b: FunctionBuilder<'a>, module: &JITModule, function: &ObjFunction) -> Option<Self> {
        let arity = function.arity as usize;
        if arity >= MAX_DEPTH {
            return None;
        }
        let pointer = module.target_config().pointer_type();
        let slots: Vec<Variable> = (0..MAX_DEPTH).map(|_| b.declare_var(types::I64)).collect();

        let entry = b.create_block();
        b.append_block_params_for_function_params(entry);
        b.switch_to_block(entry);
        let vm = b.block_params(entry)[0];
        let args = b.block_params(entry)[1];
        let buf = b.block_params(entry)[2];
        for (i, &slot) in slots.iter().enumerate().take(arity) {
            let arg = b
                .ins()
                .load(types::I64, MemFlags::trusted(), args, (8 * i) as i32);
            b.def_var(slot, arg);
        }

        let mut blocks = HashMap::new();
        for instr in &function.chunk.code {
            if let Some(target) = jump_target(instr) {
                blocks.entry(target).or_insert_with(|| b.create_block());
            }
        }

        let mut lookup_sig = module.make_signature();
        lookup_sig.params.extend([
            AbiParam::new(pointer),
            AbiParam::new(types::I64),
            AbiParam::new(types::I64),
        ]);
        lookup_sig.returns.push(AbiParam::new(pointer));
        let mut finish_sig = module.make_signature();
        finish_sig.params.extend([
            AbiParam::new(pointer),
            AbiParam::new(types::I64),
            AbiParam::new(types::I64),
            AbiParam::new(pointer),
            AbiParam::new(pointer),
            AbiParam::new(types::I64),
            AbiParam::new(types::I64),
        ]);
        finish_sig.returns.push(AbiParam::new(types::I64));

        Some(Emitter {
            b,
            slots,
            kinds: vec![Kind::Int; arity],
            vm,
            buf,
            pointer,
            jit_sig: jit_signature(module),
            lookup_sig,
            finish_sig,
            blocks,
            entries: HashMap::new(),
        })
    }

    fn emit(mut self, function: &ObjFunction) -> Option<()> {
        let chunk = &function.chunk;
        let int_constant = |index: u16| match chunk.constant(index as usize) {
            Value::Int(c) => Some(*c),
            _ => None,
        };
        let mut reachable = true;

        for (ip, instr) in chunk.code.iter().enumerate() {
            if let Some(&block) = self.blocks.get(&ip) {
                if reachable {
                    self.enter(ip)?;
                    self.b.ins().jump(block, &[]);
                }
                match self.entries.get(&ip) {
                    Some(kinds) => self.kinds = kinds.clone(),
                    None => {
                        // Only jumped to from code after this point that
                        // is itself unreachable.
                        reachable = false;
                        continue;
                    }
                }
                self.b.switch_to_block(block);
                reachable = true;
            }
            if !reachable {
                continue;
            }
            let depth = self.kinds.len();
            if depth + 2 >= MAX_DEPTH {
                return None;
            }

            match *instr {
                Instr::Constant(index) if int_constant(index).is_some() => {
                    let v = self.b.ins().iconst(types::I64, int_constant(index)?);
                    self.push(Kind::Int, v);
                }
                Instr::GetGlobal(index) => {
                    let v = self.b.ins().iconst(types::I64, index as i64);
                    self.push(Kind::Global, v);
                }
                Instr::GetLocal(slot) if self.is_int(slot as usize) => {
                    let v = self.get(slot as usize);
                    self.push(Kind::Int, v);
                }
                Instr::SetLocal(slot) if self.is_int(slot as usize) && self.is_int(depth - 1) => {
                    let v = self.get(depth - 1);
                    self.set(slot as usize, v);
                }
                Instr::StoreLocal(slot) if self.is_int(slot as usize) && self.is_int(depth - 1) => {
                    let v = self.get(depth - 1);
                    self.set(slot as usize, v);
                    self.kinds.pop();
                }
                Instr::Pop => {
                    self.kinds.pop();
                }
                Instr::Add | Instr::Subtract | Instr::Multiply
                    if self.is_int(depth - 2) && self.is_int(depth - 1) =>
                {
                    let a = self.get(depth - 2);
                    let c = self.get(depth - 1);
                    let r = self.checked(instr, a, c, ip);
                    self.set(depth - 2, r);
                    self.kinds.pop();
                }
                Instr::AddConstant(index)
                | Instr::SubtractConstant(index)
                | Instr::MultiplyConstant(index)
                    if self.is_int(depth - 1) && int_constant(index).is_some() =>
                {
                    let a = self.get(depth - 1);
                    let c = self.b.ins().iconst(types::I64, int_constant(index)?);
                    let r = self.checked(instr, a, c, ip);
                    self.set(depth - 1, r);
                }
                Instr::ModuloConstant(index)
                    if self.is_int(depth - 1)
                        && int_constant(index).is_some_and(|c| c != 0 && c != -1) =>
                {
                    let a = self.get(depth - 1);
                    let c = self.b.ins().iconst(types::I64, int_constant(index)?);
                    let r = self.b.ins().srem(a, c);
                    self.set(depth - 1, r);
                }
                Instr::Modulo if self.is_int(depth - 2) && self.is_int(depth - 1) => {
                    let a = self.get(depth - 2);
                    let c = self.get(depth - 1);
                    let zero = self.b.ins().icmp_imm(IntCC::Equal, c, 0);
                    let minus_one = self.b.ins().icmp_imm(IntCC::Equal, c, -1);
                    let bad = self.b.ins().bor(zero, minus_one);
                    self.guard(bad, ip);
                    let r = self.b.ins().srem(a, c);
                    self.set(depth - 2, r);
                    self.kinds.pop();
                }
                Instr::LessJumpIfFalse(target)
                | Instr::LessEqualJumpIfFalse(target)
                | Instr::GreaterJumpIfFalse(target)
                | Instr::GreaterEqualJumpIfFalse(target)
                    if self.is_int(depth - 2) && self.is_int(depth - 1) =>
                {
                    let a = self.get(depth - 2);
                    let c = self.get(depth - 1);
                    self.kinds.truncate(depth - 2);
                    self.branch(instr, a, c, target as usize)?;
                }
                Instr::LessConstantJumpIfFalse { constant, target }
                | Instr::LessEqualConstantJumpIfFalse { constant, target }
                | Instr::GreaterConstantJumpIfFalse { constant, target }
                | Instr::GreaterEqualConstantJumpIfFalse { constant, target }
                    if self.is_int(depth - 1) && int_constant(constant).is_some() =>
                {
                    let a = self.get(depth - 1);
                    let c = self.b.ins().iconst(types::I64, int_constant(constant)?);
                    self.kinds.pop();
                    self.branch(instr, a, c, target as usize)?;
                }
                Instr::Jump(target) | Instr::Loop(target) => {
                    let target = target as usize;
                    self.enter(target)?;
                    let block = self.blocks[&target];
                    self.b.ins().jump(block, &[]);
                    reachable = false;
                }
                Instr::Return if self.is_int(depth - 1) => {
                    let v = self.get(depth - 1);
                    self.b.ins().store(MemFlags::trusted(), v, self.buf, 0);
                    let status = self.b.ins().iconst(types::I64, RETURNED);
                    self.b.ins().return_(&[status]);
                    reachable = false;
                }
                Instr::Call(arg_count)
                    if (arg_count as usize) < depth
                        && self.kinds[depth - 1 - arg_count as usize] == Kind::Global
                        && (depth - arg_count as usize..depth).all(|s| self.is_int(s)) =>
                {
                    self.call(ip, arg_count as usize);
                }
                // Exiting on every call costs more than interpreting.
                _ => return None,
            }
        }
        if reachable {
            return None;
        }
        self.b.seal_all_blocks();
        self.b.finalize();
        Some(())
    }

    fn is_int(&self, slot: usize) -> bool {
        self.kinds.get(slot) == Some(&Kind::Int)
    }

    fn get(&mut self, slot: usize) -> Val {
        self.b.use_var(self.slots[slot])
    }

    fn set(&mut self, slot: usize, value: Val) {
        self.b.def_var(self.slots[slot], value);
    }

    fn push(&mut self, kind: Kind, value: Val) {
        let slot = self.kinds.len();
        self.set(slot, value);
        self.kinds.push(kind);
    }

    /// Records the slot kinds a jump into `target` arrives with; every
    /// edge into a block has to agree.
    fn enter(&mut self, target: usize) -> Option<()> {
        match self.entries.get(&target) {
            Some(kinds) if *kinds != self.kinds => None,
            Some(_) => Some(()),
            None => {
                self.entries.insert(target, self.kinds.clone());
                Some(())
            }
        }
    }

    /// Exits to the interpreter at `ip` (`flags` may add `PENDING`).
    /// Returns false: what follows is unreachable.
    fn deopt(&mut self, ip: usize, flags: i64) -> bool {
        let depth = self.kinds.len();
        let mut globals = 0i64;
        let d = self.b.ins().iconst(types::I64, depth as i64);
        self.b.ins().store(MemFlags::trusted(), d, self.buf, 0);
        for i in 0..depth {
            if self.kinds[i] == Kind::Global {
                globals |= 1 << i;
            }
            let v = self.get(i);
            self.b
                .ins()
                .store(MemFlags::trusted(), v, self.buf, (8 * (i + 1)) as i32);
        }
        let g = self.b.ins().iconst(types::I64, globals);
        self.b
            .ins()
            .store(MemFlags::trusted(), g, self.buf, (8 * GLOBALS) as i32);
        let status = self.b.ins().iconst(types::I64, ip as i64 | flags);
        self.b.ins().return_(&[status]);
        false
    }

    /// Exits to the interpreter at `ip` when `failed` is set.
    fn guard(&mut self, failed: Val, ip: usize) {
        let exit = self.b.create_block();
        let next = self.b.create_block();
        self.b.ins().brif(failed, exit, &[], next, &[]);
        self.b.switch_to_block(exit);
        self.deopt(ip, 0);
        self.b.switch_to_block(next);
    }

    fn checked(&mut self, instr: &Instr, a: Val, c: Val, ip: usize) -> Val {
        let (r, overflow) = match instr {
            Instr::Add | Instr::AddConstant(_) => self.b.ins().sadd_overflow(a, c),
            Instr::Subtract | Instr::SubtractConstant(_) => self.b.ins().ssub_overflow(a, c),
            _ => self.b.ins().smul_overflow(a, c),
        };
        self.guard(overflow, ip);
        r
    }

    fn branch(&mut self, instr: &Instr, a: Val, c: Val, target: usize) -> Option<()> {
        let cc = match instr {
            Instr::LessJumpIfFalse(_) | Instr::LessConstantJumpIfFalse { .. } => {
                IntCC::SignedLessThan
            }
            Instr::LessEqualJumpIfFalse(_) | Instr::LessEqualConstantJumpIfFalse { .. } => {
                IntCC::SignedLessThanOrEqual
            }
            Instr::GreaterJumpIfFalse(_) | Instr::GreaterConstantJumpIfFalse { .. } => {
                IntCC::SignedGreaterThan
            }
            _ => IntCC::SignedGreaterThanOrEqual,
        };
        let holds = self.b.ins().icmp(cc, a, c);
        self.enter(target)?;
        let next = self.b.create_block();
        let target_block = self.blocks[&target];
        self.b.ins().brif(holds, next, &[], target_block, &[]);
        self.b.switch_to_block(next);
        Some(())
    }

    fn helper(&mut self, address: usize, sig: Signature, args: &[Val]) -> Val {
        let sig = self.b.import_signature(sig);
        let callee = self.b.ins().iconst(self.pointer, address as i64);
        let call = self.b.ins().call_indirect(sig, callee, args);
        self.b.inst_results(call)[0]
    }

    /// `[.., global, args..] -> [.., result]`: calls the global's compiled
    /// code directly when it has some, and otherwise goes through
    /// `jit_finish`, which also resumes a callee that exited.
    fn call(&mut self, ip: usize, arg_count: usize) {
        let depth = self.kinds.len();
        let callee_slot = depth - 1 - arg_count;

        let jit_depth = std::mem::offset_of!(VirtualMachine, jit_depth) as i32;
        let current = self
            .b
            .ins()
            .load(types::I64, MemFlags::trusted(), self.vm, jit_depth);
        let too_deep = self
            .b
            .ins()
            .icmp_imm(IntCC::SignedGreaterThanOrEqual, current, DEPTH_LIMIT);
        self.guard(too_deep, ip);

        let args_slot = self.b.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            (8 * arg_count.max(1)) as u32,
            3,
        ));
        let buf_slot = self.b.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            (8 * BUF_LEN) as u32,
            3,
        ));
        for i in 0..arg_count {
            let v = self.get(callee_slot + 1 + i);
            self.b.ins().stack_store(v, args_slot, (8 * i) as i32);
        }
        let args = self.b.ins().stack_addr(self.pointer, args_slot, 0);
        let callee_buf = self.b.ins().stack_addr(self.pointer, buf_slot, 0);
        let global = self.get(callee_slot);
        let count = self.b.ins().iconst(types::I64, arg_count as i64);

        let code = self.helper(
            jit_lookup as *const () as usize,
            self.lookup_sig.clone(),
            &[self.vm, global, count],
        );
        let native = self.b.create_block();
        let finish = self.b.create_block();
        self.b.append_block_param(finish, types::I64);
        let not_compiled = self.b.ins().iconst(types::I64, NOT_COMPILED);
        self.b
            .ins()
            .brif(code, native, &[], finish, &[not_compiled.into()]);

        self.b.switch_to_block(native);
        let current = self
            .b
            .ins()
            .load(types::I64, MemFlags::trusted(), self.vm, jit_depth);
        let deeper = self.b.ins().iadd_imm(current, 1);
        self.b
            .ins()
            .store(MemFlags::trusted(), deeper, self.vm, jit_depth);
        let sig = self.b.import_signature(self.jit_sig.clone());
        let call = self
            .b
            .ins()
            .call_indirect(sig, code, &[self.vm, args, callee_buf]);
        let status = self.b.inst_results(call)[0];
        self.b
            .ins()
            .store(MemFlags::trusted(), current, self.vm, jit_depth);
        let returned = self.b.ins().icmp_imm(IntCC::Equal, status, RETURNED);
        let done = self.b.create_block();
        self.b
            .ins()
            .brif(returned, done, &[], finish, &[status.into()]);

        self.b.switch_to_block(finish);
        let status = self.b.block_params(finish)[0];
        let call_ip = self.b.ins().iconst(types::I64, ip as i64);
        let outcome = self.helper(
            jit_finish as *const () as usize,
            self.finish_sig.clone(),
            &[self.vm, global, count, args, callee_buf, status, call_ip],
        );
        let not_int = self.b.create_block();
        self.b.ins().brif(outcome, not_int, &[], done, &[]);

        // The callee's result isn't an Int (outcome 1) or it failed (2).
        self.b.switch_to_block(not_int);
        let failed = self.b.ins().icmp_imm(IntCC::Equal, outcome, 2);
        let error = self.b.create_block();
        let pending = self.b.create_block();
        self.b.ins().brif(failed, error, &[], pending, &[]);
        self.b.switch_to_block(error);
        let status = self.b.ins().iconst(types::I64, ERROR);
        self.b.ins().return_(&[status]);
        self.b.switch_to_block(pending);
        let below = self.kinds[..callee_slot].to_vec();
        let saved = std::mem::replace(&mut self.kinds, below);
        self.deopt(ip + 1, PENDING);
        self.kinds = saved;

        self.b.switch_to_block(done);
        let result = self
            .b
            .ins()
            .load(types::I64, MemFlags::trusted(), callee_buf, 0);
        self.kinds.truncate(callee_slot);
        self.push(Kind::Int, result);
    }
}

fn jump_target(instr: &Instr) -> Option<usize> {
    match *instr {
        Instr::Jump(t)
        | Instr::Loop(t)
        | Instr::JumpIfFalse(t)
        | Instr::PopJumpIfFalse(t)
        | Instr::LessJumpIfFalse(t)
        | Instr::LessEqualJumpIfFalse(t)
        | Instr::GreaterJumpIfFalse(t)
        | Instr::GreaterEqualJumpIfFalse(t)
        | Instr::JumpIfNotNil(t)
        | Instr::JumpIfNil(t)
        | Instr::LessConstantJumpIfFalse { target: t, .. }
        | Instr::LessEqualConstantJumpIfFalse { target: t, .. }
        | Instr::GreaterConstantJumpIfFalse { target: t, .. }
        | Instr::GreaterEqualConstantJumpIfFalse { target: t, .. } => Some(t as usize),
        _ => None,
    }
}

/// The compiled code of global `index` if it's a closure taking
/// `arg_count` arguments, else null.
extern "C" fn jit_lookup(vm: *mut VirtualMachine, index: i64, arg_count: i64) -> usize {
    // SAFETY: compiled code only runs inside `call_jit_candidate`, which
    // hands it a live, exclusively borrowed VM.
    let vm = unsafe { &mut *vm };
    match &vm.stack[index as usize] {
        Value::Closure(closure) if closure.function.arity as i64 == arg_count => {
            compiled(&closure.function).map_or(0, |code| code as usize)
        }
        _ => 0,
    }
}

/// Finishes a call compiled code couldn't complete natively: runs it in
/// the interpreter (`NOT_COMPILED`), resumes a callee that exited, or
/// passes on an `ERROR`. `ip` is the calling `Call`. Writes an Int result to `buf[0]` and returns 0;
/// returns 1 with a non-Int result in `jit_pending`, 2 with the error in
/// `jit_error`.
extern "C" fn jit_finish(
    vm: *mut VirtualMachine,
    index: i64,
    arg_count: i64,
    args: *const i64,
    buf: *mut i64,
    status: i64,
    ip: i64,
) -> i64 {
    // SAFETY: as in `jit_lookup`; `args` holds `arg_count` slots and `buf`
    // `BUF_LEN`, both on the calling compiled frame.
    let vm = unsafe { &mut *vm };
    let args = unsafe { std::slice::from_raw_parts(args, arg_count as usize) };
    let buf = unsafe { std::slice::from_raw_parts_mut(buf, BUF_LEN) };
    if status == ERROR {
        return 2;
    }
    // Where `op_call` would have left it, for the error location; only
    // right for the outermost compiled frame, which owns the VM frame.
    vm.ip = ip as usize + 1;
    vm.jit_depth += NESTED_RUN;
    let outcome = vm.finish_call(index as usize, args, buf, status);
    vm.jit_depth -= NESTED_RUN;
    match outcome {
        Ok(Value::Int(result)) => {
            buf[0] = result;
            0
        }
        Ok(value) => {
            vm.jit_pending = Some(value);
            1
        }
        Err(error) => {
            vm.jit_error = Some(error);
            2
        }
    }
}

impl VirtualMachine {
    /// The JIT path of `op_call`, kept out of line so it doesn't change
    /// how the dispatch loop compiles.
    #[inline(never)]
    pub(in crate::vm) fn call_jit_candidate(
        &mut self,
        arg_count: usize,
        closure: Rc<ObjClosure>,
    ) -> OpResult {
        let slot_start = self.stack.len() - arg_count - 1;
        let base = slot_start + 1;
        let mut args = [0i64; MAX_DEPTH];
        let all_ints = arg_count == closure.function.arity as usize
            && arg_count <= MAX_DEPTH
            && self.stack[base..]
                .iter()
                .zip(args.iter_mut())
                .all(|(value, arg)| match value {
                    Value::Int(i) => {
                        *arg = *i;
                        true
                    }
                    _ => false,
                });
        let code = match compiled(&closure.function) {
            Some(code) if all_ints && self.jit_depth < DEPTH_LIMIT => code,
            _ => return self.call_closure(arg_count, closure),
        };

        self.push_frame(closure, slot_start as isize);
        let mut buf = [0i64; BUF_LEN];
        self.jit_depth += 1;
        // SAFETY: `args` and `buf` are as large as the code may address.
        let status = unsafe { code(self, args.as_ptr(), buf.as_mut_ptr()) };
        self.jit_depth -= 1;
        match status {
            RETURNED => {
                self.pop_frame();
                self.stack.truncate(slot_start);
                self.push(Value::Int(buf[0]));
            }
            ERROR => {
                return Err(self
                    .jit_error
                    .take()
                    .unwrap_or_else(|| self.runtime_error("JIT error went missing")));
            }
            _ => self.restore_frame(base, &buf, status)?,
        }
        Ok(())
    }

    /// Rebuilds the frame starting at `base` from an exit's buffer and
    /// points the interpreter at the exit's instruction.
    fn restore_frame(&mut self, base: usize, buf: &[i64], status: i64) -> OpResult {
        self.stack.truncate(base);
        let depth = buf[0] as usize;
        let globals = buf[GLOBALS];
        for (i, &slot) in buf[1..=depth].iter().enumerate() {
            let value = if globals & (1 << i) != 0 {
                let value = self.stack[slot as usize].clone();
                if let Some(message) = Self::uninitialized_error(&value) {
                    return Err(self.runtime_error(message));
                }
                value
            } else {
                Value::Int(slot)
            };
            self.push(value);
        }
        if status & PENDING != 0 {
            if let Some(value) = self.jit_pending.take() {
                self.push(value);
            }
        }
        self.ip = (status & !PENDING) as usize;
        Ok(())
    }

    fn finish_call(
        &mut self,
        index: usize,
        args: &[i64],
        buf: &[i64],
        status: i64,
    ) -> Result<Value, RuntimeError> {
        let callee = self.stack[index].clone();
        if let Some(message) = Self::uninitialized_error(&callee) {
            return Err(self.runtime_error(message));
        }
        let depth = self.call_frames.len();
        if status == NOT_COMPILED {
            self.push(callee);
            for &arg in args {
                self.push(Value::Int(arg));
            }
            self.dispatch_call(args.len())?;
        } else {
            let Value::Closure(closure) = callee.clone() else {
                return Err(self.runtime_error("JIT resumed a non-closure"));
            };
            let slot_start = self.stack.len();
            self.push(callee);
            self.push_frame(closure, slot_start as isize);
            self.restore_frame(slot_start + 1, buf, status)?;
        }
        if self.call_frames.len() > depth {
            self.run_until(depth)?;
        }
        Ok(self.pop())
    }
}
