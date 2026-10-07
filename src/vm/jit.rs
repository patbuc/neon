//! Spike: a Cranelift JIT for function bodies whose values are all Ints.
//!
//! Every frame slot (locals and operand stack) becomes an i64 variable.
//! Arguments are tag-checked on entry. An unsupported instruction, a
//! failed check, or an overflow exits to the interpreter at that
//! instruction: the compiled code writes its slots to a buffer, and
//! `run_jitted` turns them back into `Value::Int`s on the VM stack, so the
//! interpreter resumes with exactly the state it would have had.
use crate::common::chunk::Instr;
use crate::common::{ObjClosure, ObjFunction, Value};
use crate::vm::functions::OpResult;
use crate::vm::VirtualMachine;
use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{types, AbiParam, Block, InstBuilder, MemFlags, UserFuncName};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{default_libcall_names, Module};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// `(args, buf) -> status`. `RETURNED`: the result is in `buf[0]`.
/// `ENTRY_DEOPT`: an argument isn't an Int; nothing ran. Otherwise the
/// status is the instruction to resume at, `buf[0]` the slot count and
/// `buf[1..]` the slots.
type JitFn = unsafe extern "C" fn(*const Value, *mut i64) -> i64;
const RETURNED: i64 = -1;
const ENTRY_DEOPT: i64 = -2;
const MAX_DEPTH: usize = 64;

#[derive(Clone, Copy)]
struct Layout {
    tag_offset: i32,
    int_tag: u8,
    payload_offset: i32,
}

fn value_bytes(value: &Value) -> [u8; 16] {
    // SAFETY: spike only; `Value` is 16 bytes (checked by the caller).
    unsafe { std::mem::transmute_copy(value) }
}

/// Finds where `Value::Int` keeps its tag and payload, since `Value` has
/// no fixed `repr`.
fn probe_layout() -> Option<Layout> {
    if std::mem::size_of::<Value>() != 16 {
        return None;
    }
    let a = value_bytes(&Value::Int(0x0102_0304_0506_0708));
    let b = value_bytes(&Value::Int(-0x1122_3344_5566_7788));
    let payload_offset = [0usize, 8].into_iter().find(|&o| {
        a[o..o + 8] == 0x0102_0304_0506_0708i64.to_ne_bytes()
            && b[o..o + 8] == (-0x1122_3344_5566_7788i64).to_ne_bytes()
    })?;
    let others = [
        value_bytes(&Value::Number(f64::from_bits(0x0102_0304_0506_0708))),
        value_bytes(&Value::Boolean(true)),
        value_bytes(&Value::Nil),
    ];
    let tag_offset = (0..16)
        .filter(|o| !(payload_offset..payload_offset + 8).contains(o))
        .find(|&o| a[o] == b[o] && others.iter().all(|other| other[o] != a[o]))?;
    Some(Layout {
        tag_offset: tag_offset as i32,
        int_tag: a[tag_offset],
        payload_offset: payload_offset as i32,
    })
}

struct Jit {
    module: JITModule,
    layout: Layout,
    /// Keyed by function address; the `Rc` keeps that address from being
    /// reused by another function.
    cache: HashMap<*const ObjFunction, (Rc<ObjFunction>, Option<JitFn>)>,
}

thread_local! {
    static JIT: RefCell<Option<Jit>> = RefCell::new(Jit::new());
}

/// The compiled code for `function`, compiling it on first use.
pub(in crate::vm) fn compiled(function: &Rc<ObjFunction>) -> Option<JitFn> {
    JIT.with(|jit| {
        let mut jit = jit.borrow_mut();
        let jit = jit.as_mut()?;
        let key = Rc::as_ptr(function);
        if let Some((_, compiled)) = jit.cache.get(&key) {
            return *compiled;
        }
        let compiled = jit.compile(function);
        jit.cache.insert(key, (Rc::clone(function), compiled));
        compiled
    })
}

impl Jit {
    fn new() -> Option<Self> {
        if std::env::var("NEON_JIT").as_deref() == Ok("0") {
            return None;
        }
        let layout = probe_layout()?;
        let mut flags = settings::builder();
        flags.set("opt_level", "speed").ok()?;
        let isa = cranelift_native::builder()
            .ok()?
            .finish(settings::Flags::new(flags))
            .ok()?;
        let module = JITModule::new(JITBuilder::with_isa(isa, default_libcall_names()));
        Some(Jit {
            module,
            layout,
            cache: HashMap::new(),
        })
    }

    fn compile(&mut self, function: &ObjFunction) -> Option<JitFn> {
        let mut ctx = self.module.make_context();
        let pointer = self.module.target_config().pointer_type();
        ctx.func.signature.params.push(AbiParam::new(pointer));
        ctx.func.signature.params.push(AbiParam::new(pointer));
        ctx.func.signature.returns.push(AbiParam::new(types::I64));
        let id = self
            .module
            .declare_anonymous_function(&ctx.func.signature)
            .ok()?;
        ctx.func.name = UserFuncName::user(0, id.as_u32());

        let mut fctx = FunctionBuilderContext::new();
        let builder = FunctionBuilder::new(&mut ctx.func, &mut fctx);
        let emitter = Emitter::new(builder, self.layout, function)?;
        emitter.emit(function)?;

        self.module.define_function(id, &mut ctx).ok()?;
        self.module.clear_context(&mut ctx);
        self.module.finalize_definitions().ok()?;
        let code = self.module.get_finalized_function(id);
        // SAFETY: the function was built with exactly this signature.
        Some(unsafe { std::mem::transmute::<*const u8, JitFn>(code) })
    }
}

struct Emitter<'a> {
    b: FunctionBuilder<'a>,
    slots: Vec<Variable>,
    buf: cranelift_codegen::ir::Value,
    blocks: HashMap<usize, Block>,
    depths: HashMap<usize, usize>,
}

impl<'a> Emitter<'a> {
    fn new(mut b: FunctionBuilder<'a>, layout: Layout, function: &ObjFunction) -> Option<Self> {
        let arity = function.arity as usize;
        if arity >= MAX_DEPTH {
            return None;
        }
        let slots: Vec<Variable> = (0..MAX_DEPTH).map(|_| b.declare_var(types::I64)).collect();

        let entry = b.create_block();
        b.append_block_params_for_function_params(entry);
        b.switch_to_block(entry);
        let args = b.block_params(entry)[0];
        let buf = b.block_params(entry)[1];

        let entry_deopt = b.create_block();
        for (i, &slot) in slots.iter().enumerate().take(arity) {
            let base = (i * 16) as i32;
            let tag = b
                .ins()
                .uload8(types::I32, MemFlags::trusted(), args, base + layout.tag_offset);
            let is_int = b.ins().icmp_imm(IntCC::Equal, tag, layout.int_tag as i64);
            let next = b.create_block();
            b.ins().brif(is_int, next, &[], entry_deopt, &[]);
            b.switch_to_block(next);
            let payload = b.ins().load(
                types::I64,
                MemFlags::trusted(),
                args,
                base + layout.payload_offset,
            );
            b.def_var(slot, payload);
        }
        let body = b.create_block();
        b.ins().jump(body, &[]);
        b.switch_to_block(entry_deopt);
        let status = b.ins().iconst(types::I64, ENTRY_DEOPT);
        b.ins().return_(&[status]);
        b.switch_to_block(body);

        let mut blocks = HashMap::new();
        for instr in &function.chunk.code {
            if let Some(target) = jump_target(instr) {
                blocks.entry(target).or_insert_with(|| b.create_block());
            }
        }
        Some(Emitter {
            b,
            slots,
            buf,
            blocks,
            depths: HashMap::new(),
        })
    }

    fn emit(mut self, function: &ObjFunction) -> Option<()> {
        let chunk = &function.chunk;
        let int_constant = |index: u16| match chunk.constant(index as usize) {
            Value::Int(c) => Some(*c),
            _ => None,
        };
        let mut depth = function.arity as usize;
        let mut reachable = true;

        for (ip, instr) in chunk.code.iter().enumerate() {
            if let Some(&block) = self.blocks.get(&ip) {
                if reachable {
                    self.b.ins().jump(block, &[]);
                    self.depths.entry(ip).or_insert(depth);
                }
                match self.depths.get(&ip) {
                    Some(&d) => depth = d,
                    None => {
                        // Only reachable by a later backward jump from
                        // code that itself is unreachable.
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
            if depth + 2 >= MAX_DEPTH {
                return None;
            }

            match *instr {
                Instr::Constant(index) => match int_constant(index) {
                    Some(c) => {
                        let v = self.b.ins().iconst(types::I64, c);
                        self.set(depth, v);
                        depth += 1;
                    }
                    None => reachable = self.deopt(ip, depth),
                },
                Instr::GetLocal(slot) if (slot as usize) < depth => {
                    let v = self.get(slot as usize);
                    self.set(depth, v);
                    depth += 1;
                }
                Instr::SetLocal(slot) if (slot as usize) < depth => {
                    let v = self.get(depth - 1);
                    self.set(slot as usize, v);
                }
                Instr::StoreLocal(slot) if (slot as usize) < depth => {
                    let v = self.get(depth - 1);
                    self.set(slot as usize, v);
                    depth -= 1;
                }
                Instr::Pop => depth -= 1,
                Instr::Add | Instr::Subtract | Instr::Multiply => {
                    let a = self.get(depth - 2);
                    let c = self.get(depth - 1);
                    let r = self.checked(instr, a, c, ip, depth);
                    self.set(depth - 2, r);
                    depth -= 1;
                }
                Instr::AddConstant(index)
                | Instr::SubtractConstant(index)
                | Instr::MultiplyConstant(index) => match int_constant(index) {
                    Some(c) => {
                        let a = self.get(depth - 1);
                        let c = self.b.ins().iconst(types::I64, c);
                        let r = self.checked(instr, a, c, ip, depth);
                        self.set(depth - 1, r);
                    }
                    None => reachable = self.deopt(ip, depth),
                },
                Instr::ModuloConstant(index) => match int_constant(index) {
                    Some(c) if c != 0 && c != -1 => {
                        let a = self.get(depth - 1);
                        let c = self.b.ins().iconst(types::I64, c);
                        let r = self.b.ins().srem(a, c);
                        self.set(depth - 1, r);
                    }
                    _ => reachable = self.deopt(ip, depth),
                },
                Instr::Modulo => {
                    let a = self.get(depth - 2);
                    let c = self.get(depth - 1);
                    let zero = self.b.ins().icmp_imm(IntCC::Equal, c, 0);
                    let minus_one = self.b.ins().icmp_imm(IntCC::Equal, c, -1);
                    let bad = self.b.ins().bor(zero, minus_one);
                    self.guard(bad, ip, depth);
                    let r = self.b.ins().srem(a, c);
                    self.set(depth - 2, r);
                    depth -= 1;
                }
                Instr::LessJumpIfFalse(target)
                | Instr::LessEqualJumpIfFalse(target)
                | Instr::GreaterJumpIfFalse(target)
                | Instr::GreaterEqualJumpIfFalse(target) => {
                    let a = self.get(depth - 2);
                    let c = self.get(depth - 1);
                    depth -= 2;
                    self.branch(instr, a, c, target as usize, depth);
                }
                Instr::LessConstantJumpIfFalse { constant, target }
                | Instr::LessEqualConstantJumpIfFalse { constant, target }
                | Instr::GreaterConstantJumpIfFalse { constant, target }
                | Instr::GreaterEqualConstantJumpIfFalse { constant, target } => {
                    match int_constant(constant) {
                        Some(c) => {
                            let a = self.get(depth - 1);
                            let c = self.b.ins().iconst(types::I64, c);
                            depth -= 1;
                            self.branch(instr, a, c, target as usize, depth);
                        }
                        None => reachable = self.deopt(ip, depth),
                    }
                }
                Instr::Jump(target) | Instr::Loop(target) => {
                    let target = target as usize;
                    self.depths.entry(target).or_insert(depth);
                    let block = self.blocks[&target];
                    self.b.ins().jump(block, &[]);
                    reachable = false;
                }
                Instr::Return => {
                    let v = self.get(depth - 1);
                    self.b.ins().store(MemFlags::trusted(), v, self.buf, 0);
                    let status = self.b.ins().iconst(types::I64, RETURNED);
                    self.b.ins().return_(&[status]);
                    reachable = false;
                }
                _ => reachable = self.deopt(ip, depth),
            }
        }
        if reachable {
            return None;
        }
        self.b.seal_all_blocks();
        self.b.finalize();
        Some(())
    }

    fn get(&mut self, slot: usize) -> cranelift_codegen::ir::Value {
        self.b.use_var(self.slots[slot])
    }

    fn set(&mut self, slot: usize, value: cranelift_codegen::ir::Value) {
        self.b.def_var(self.slots[slot], value);
    }

    /// Writes the frame's `depth` slots to the buffer and returns `ip` to
    /// resume at. Returns false: what follows is unreachable.
    fn deopt(&mut self, ip: usize, depth: usize) -> bool {
        let d = self.b.ins().iconst(types::I64, depth as i64);
        self.b.ins().store(MemFlags::trusted(), d, self.buf, 0);
        for i in 0..depth {
            let v = self.get(i);
            self.b
                .ins()
                .store(MemFlags::trusted(), v, self.buf, (8 * (i + 1)) as i32);
        }
        let status = self.b.ins().iconst(types::I64, ip as i64);
        self.b.ins().return_(&[status]);
        false
    }

    /// Exits to the interpreter at `ip` when `failed` is set.
    fn guard(&mut self, failed: cranelift_codegen::ir::Value, ip: usize, depth: usize) {
        let exit = self.b.create_block();
        let next = self.b.create_block();
        self.b.ins().brif(failed, exit, &[], next, &[]);
        self.b.switch_to_block(exit);
        self.deopt(ip, depth);
        self.b.switch_to_block(next);
    }

    fn checked(
        &mut self,
        instr: &Instr,
        a: cranelift_codegen::ir::Value,
        c: cranelift_codegen::ir::Value,
        ip: usize,
        depth: usize,
    ) -> cranelift_codegen::ir::Value {
        let (r, overflow) = match instr {
            Instr::Add | Instr::AddConstant(_) => self.b.ins().sadd_overflow(a, c),
            Instr::Subtract | Instr::SubtractConstant(_) => self.b.ins().ssub_overflow(a, c),
            _ => self.b.ins().smul_overflow(a, c),
        };
        self.guard(overflow, ip, depth);
        r
    }

    fn branch(
        &mut self,
        instr: &Instr,
        a: cranelift_codegen::ir::Value,
        c: cranelift_codegen::ir::Value,
        target: usize,
        depth: usize,
    ) {
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
        self.depths.entry(target).or_insert(depth);
        let next = self.b.create_block();
        let target_block = self.blocks[&target];
        self.b.ins().brif(holds, next, &[], target_block, &[]);
        self.b.switch_to_block(next);
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

impl VirtualMachine {
    /// Calls `closure` through its compiled code. On a deopt the frame is
    /// left running at the resume point for the interpreter loop.
    pub(in crate::vm) fn run_jitted(
        &mut self,
        arg_count: usize,
        closure: Rc<ObjClosure>,
        jitted: JitFn,
    ) -> OpResult {
        if arg_count != closure.function.arity as usize {
            return self.call_closure(arg_count, closure);
        }
        let slot_start = self.stack.len() - arg_count - 1;
        let base = slot_start + 1;
        self.push_frame(closure, slot_start as isize);
        let mut buf = [0i64; MAX_DEPTH + 1];
        // SAFETY: `base..base + arg_count` are the arguments on the stack,
        // and `buf` holds MAX_DEPTH slots plus the count.
        let status = unsafe { jitted(self.stack.as_ptr().add(base), buf.as_mut_ptr()) };
        match status {
            RETURNED => {
                self.pop_frame();
                self.stack.truncate(slot_start);
                self.push(Value::Int(buf[0]));
            }
            ENTRY_DEOPT => {}
            ip => {
                self.stack.truncate(base);
                for &slot in &buf[1..=buf[0] as usize] {
                    self.push(Value::Int(slot));
                }
                self.ip = ip as usize;
            }
        }
        Ok(())
    }
}
