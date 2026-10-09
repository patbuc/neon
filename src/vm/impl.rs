use crate::common::chunk::Instr;
use crate::common::method_registry::native_method_table;
use crate::common::runtime_error::{RuntimeError, TraceFrame, TRACE_EDGE_FRAMES};
use crate::common::{CallFrame, Chunk, ObjClosure, ObjError, ObjFunction, Value};
use crate::compiler::compiler_impl::Compiled;
use crate::compiler::global_env::GlobalEnv;
use crate::compiler::module_graph::EntryLocation;
use crate::compiler::Compiler;
use crate::vm::functions::{Comparison, OpResult};
use crate::vm::{Handler, InterpretResult, VirtualMachine};
use crate::{boolean, common, is_false_like, nil};
#[cfg(not(target_arch = "wasm32"))]
use log::info;
use std::cell::OnceCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualMachine {
    pub fn with_args(args: Vec<String>) -> Self {
        VirtualMachine {
            call_frames: Vec::new(),
            ip: 0,
            frame_base: 0,
            chunk: Rc::new(Chunk::new("")),
            stack: Vec::new(),
            builtin: common::stdlib::create_builtin_objects(args),
            #[cfg(any(test, debug_assertions, target_arch = "wasm32"))]
            string_buffer: String::new(),
            structured_errors: Vec::new(),
            runtime_error: None,
            source: String::new(),
            module_sources: HashMap::new(),
            open_upvalues: Vec::new(),
            handlers: Vec::new(),
            native_call_depth: 0,
            builtin_methods: std::array::from_fn(|_| Vec::new()),
            method_journal: None,
            native_methods: Vec::new(),
            repl_env: GlobalEnv::default(),
            #[cfg(feature = "opcode-stats")]
            opcode_counts: HashMap::new(),
            #[cfg(feature = "opcode-stats")]
            opcode_pair_counts: HashMap::new(),
            #[cfg(feature = "opcode-stats")]
            last_opcode: None,
        }
    }

    pub fn new() -> Self {
        Self::with_args(vec![])
    }

    fn compile(&mut self, source: String, entry: EntryLocation) -> Option<Compiled> {
        self.reset();

        self.source = source.clone();

        #[cfg(not(target_arch = "wasm32"))]
        let start = std::time::Instant::now();

        let mut compiler = Compiler::new();
        let compiled = compiler.compile_entry(&source, entry, &GlobalEnv::default());

        #[cfg(not(target_arch = "wasm32"))]
        info!("Compile time: {}ms", start.elapsed().as_millis());

        if compiled.is_none() {
            self.structured_errors = compiler.get_structured_errors();
            self.module_sources = compiler.module_sources().clone();
        }
        compiled
    }

    pub fn check_file(&mut self, path: &Path, source: String) -> InterpretResult {
        match self.compile(source, EntryLocation::File(path.to_path_buf())) {
            Some(_) => InterpretResult::Ok,
            None => InterpretResult::CompileError,
        }
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        self.interpret_entry(source, EntryLocation::None)
    }

    pub fn interpret_file(&mut self, path: &Path, source: String) -> InterpretResult {
        self.interpret_entry(source, EntryLocation::File(path.to_path_buf()))
    }

    fn interpret_entry(&mut self, source: String, entry: EntryLocation) -> InterpretResult {
        let compiled = self.compile(source, entry);

        #[cfg(not(target_arch = "wasm32"))]
        let start = std::time::Instant::now();
        let Some(compiled) = compiled else {
            return InterpretResult::CompileError;
        };

        let result = self.run_program(
            compiled.modules,
            compiled.module_slot_counts,
            compiled.entry,
        );

        #[cfg(not(target_arch = "wasm32"))]
        info!("Run time: {}ms", start.elapsed().as_millis());

        result
    }

    /// Runs each module chunk, then `entry`, as the script frame over the
    /// same globals. The entry's symbols are a superset of every module's,
    /// so one native method table serves them all.
    fn run_program(
        &mut self,
        modules: Vec<Chunk>,
        module_slot_counts: Vec<u32>,
        entry: Chunk,
    ) -> InterpretResult {
        self.native_methods = native_method_table(&entry.symbols);
        for (module, slot_count) in modules.into_iter().zip(module_slot_counts) {
            let result = self.run_script_chunk(module);
            if result != InterpretResult::Ok {
                return result;
            }
            self.stack.truncate(slot_count as usize);
        }
        self.run_script_chunk(entry)
    }

    /// Makes `chunk` the running script frame and runs it to completion.
    fn run_script_chunk(&mut self, mut chunk: Chunk) -> InterpretResult {
        chunk.decode();
        let script_function = Rc::new(ObjFunction {
            name: "<script>".to_string(),
            arity: 0,
            chunk: Rc::new(chunk),
        });
        let script_closure = Rc::new(ObjClosure {
            function: script_function,
            upvalues: Vec::new(),
        });

        // Use -1 for slot_start since the script has no function object on the stack
        self.push_frame(script_closure, -1);

        self.run_script(0)
    }

    /// Interprets one REPL line, keeping globals and methods earlier lines
    /// defined. A compile error leaves the VM untouched; a runtime error
    /// rolls back everything the line declared.
    pub fn interpret_line(&mut self, source: String) -> InterpretResult {
        self.interpret_line_at(source, EntryLocation::None)
    }

    /// Like `interpret_line`, resolving the line's imports relative to `dir`.
    pub fn interpret_line_in(&mut self, source: String, dir: &Path) -> InterpretResult {
        self.interpret_line_at(source, EntryLocation::Directory(dir.to_path_buf()))
    }

    fn interpret_line_at(&mut self, source: String, entry: EntryLocation) -> InterpretResult {
        self.source = source.clone();

        let previous_env = self.repl_env.clone();
        let previous_builtin_methods = self.builtin_methods.clone();
        let previous_slot_count = previous_env.slot_count as usize;

        let mut compiler = Compiler::new();
        let Some(compiled) = compiler.compile_entry(&source, entry, &previous_env) else {
            self.structured_errors = compiler.get_structured_errors();
            self.module_sources = compiler.module_sources().clone();
            return InterpretResult::CompileError;
        };
        let new_env = compiled.env;

        self.call_frames.clear();
        self.open_upvalues.clear();
        self.handlers.clear();
        self.native_call_depth = 0;
        self.runtime_error = None;
        self.method_journal = Some(Vec::new());

        let result = self.run_program(
            compiled.modules,
            compiled.module_slot_counts,
            compiled.entry,
        );
        let method_journal = self.method_journal.take().unwrap_or_default();

        match result {
            InterpretResult::Ok => {
                self.stack.truncate(new_env.slot_count as usize);
                self.repl_env = new_env;
            }
            InterpretResult::RuntimeError => {
                let new_slot_count = new_env.slot_count as usize;
                self.close_upvalues_above(new_slot_count);
                self.stack.truncate(new_slot_count);
                self.stack.resize_with(new_slot_count, || {
                    Value::Uninitialized(Rc::new(String::new()))
                });
                for (name, symbol) in &new_env.globals {
                    let Some(&slot) = new_env.decl_slots.get(&symbol.decl_id) else {
                        continue;
                    };
                    if slot as usize >= previous_slot_count {
                        self.stack[slot as usize] = Value::Uninitialized(Rc::new(name.clone()));
                    }
                }
                self.call_frames.clear();
                self.native_call_depth = 0;
                for r#struct in method_journal.into_iter().rev() {
                    r#struct.methods.borrow_mut().pop();
                }
                self.builtin_methods = previous_builtin_methods;
                self.repl_env = previous_env.after_runtime_error(&new_env);
            }
            InterpretResult::CompileError => unreachable!("compile already handled"),
        }

        result
    }

    /// Runs until `target_depth`, converting a runtime error into the VM's
    /// stored error and the public `InterpretResult` enum.
    pub(in crate::vm) fn run_script(&mut self, target_depth: usize) -> InterpretResult {
        match self.run_until(target_depth) {
            Ok(()) => InterpretResult::Ok,
            Err(mut e) => {
                if let Some(thrown) = e.thrown.take() {
                    match &thrown {
                        Value::Error(obj) => {
                            if let Some(origin) = obj.thrown_at.get() {
                                e = origin.clone();
                            }
                            e.message = obj.message.clone();
                        }
                        _ => e.message = format!("Uncaught: {}", thrown),
                    }
                }
                self.runtime_error = Some(e);
                InterpretResult::RuntimeError
            }
        }
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn run_until(&mut self, target_depth: usize) -> OpResult {
        #[cfg(feature = "disassemble")]
        if target_depth == 0 {
            let frame = self
                .call_frames
                .last()
                .expect("run_until(0) runs the script frame, which is always on the stack");
            frame.closure.function.chunk.disassemble_chunk();
        }
        loop {
            match self.run_loop(target_depth) {
                Err(error) => self.catch_error(error, target_depth)?,
                Ok(()) => return Ok(()),
            }
        }
    }

    /// Unwinds to the innermost handler that belongs to this loop (one set
    /// up in a frame deeper than `target_depth`) and resumes at its catch
    /// target with the caught value on the stack. Without one, the error
    /// goes on to the caller.
    fn catch_error(&mut self, error: RuntimeError, target_depth: usize) -> OpResult {
        let Some(handler) = self
            .handlers
            .last()
            .copied()
            .filter(|handler| handler.frame_depth > target_depth)
        else {
            return Err(error);
        };
        self.handlers.pop();
        self.close_upvalues_above(handler.stack_height);
        while self.call_frames.len() > handler.frame_depth {
            self.pop_frame();
        }
        self.stack.truncate(handler.stack_height);
        let caught = match error.thrown {
            Some(thrown) => thrown,
            None => Value::Error(Rc::new(ObjError {
                message: error.message.clone(),
                thrown_at: OnceCell::from(error),
            })),
        };
        self.push(caught);
        self.ip = handler.catch_ip;
        Ok(())
    }

    #[inline(always)]
    fn run_loop(&mut self, target_depth: usize) -> OpResult {
        loop {
            let instr = self.chunk.code[self.ip];

            #[cfg(feature = "opcode-stats")]
            if let Some(name) = instr.name() {
                *self.opcode_counts.entry(name).or_insert(0) += 1;
                if let Some(prev) = self.last_opcode {
                    *self.opcode_pair_counts.entry((prev, name)).or_insert(0) += 1;
                }
                self.last_opcode = Some(name);
            }

            match instr {
                Instr::Invalid(byte) => {
                    return Err(self.runtime_error(format!("Unknown opcode {:#04x}", byte)));
                }
                Instr::Return => {
                    self.op_return();
                    if self.call_frames.len() == target_depth {
                        return Ok(());
                    }
                    continue;
                }
                Instr::Constant(index) => self.op_constant(index),
                Instr::Negate => self.op_negate()?,
                Instr::Add => self.op_add()?,
                Instr::Subtract => self.op_subtract()?,
                Instr::Multiply => self.op_multiply()?,
                Instr::Divide => self.op_divide()?,
                Instr::Modulo => self.op_modulo()?,
                Instr::Exponent => self.op_exponent()?,
                Instr::Nil => self.push(nil!()),
                Instr::True => self.push(boolean!(true)),
                Instr::False => self.push(boolean!(false)),
                Instr::Equal => self.op_equal(),
                Instr::Greater => self.op_compare(Comparison::Greater)?,
                Instr::GreaterEqual => self.op_compare(Comparison::GreaterEqual)?,
                Instr::Less => self.op_compare(Comparison::Less)?,
                Instr::LessEqual => self.op_compare(Comparison::LessEqual)?,
                Instr::Not => self.op_not(),
                Instr::Pop => self.pop().discard(),
                Instr::GetLocal(slot) => self.op_get_local(slot)?,
                Instr::SetLocal(slot) => self.op_set_local(slot)?,
                Instr::GetBuiltin(index) => self.op_get_builtin(index)?,
                Instr::GetGlobal(index) => self.op_get_global(index)?,
                Instr::SetGlobal(index) => self.op_set_global(index)?,
                Instr::JumpIfFalse(target) => {
                    if is_false_like!(self.peek(0)) {
                        self.ip = target as usize;
                        continue;
                    }
                }
                Instr::PopJumpIfFalse(target) => {
                    let condition = self.pop();
                    let is_false = is_false_like!(condition);
                    condition.discard();
                    if is_false {
                        self.ip = target as usize;
                        continue;
                    }
                }
                Instr::GreaterJumpIfFalse(target) => {
                    if !self.op_compare_and_pop(Comparison::Greater)? {
                        self.ip = target as usize;
                        continue;
                    }
                }
                Instr::GreaterEqualJumpIfFalse(target) => {
                    if !self.op_compare_and_pop(Comparison::GreaterEqual)? {
                        self.ip = target as usize;
                        continue;
                    }
                }
                Instr::LessJumpIfFalse(target) => {
                    if !self.op_compare_and_pop(Comparison::Less)? {
                        self.ip = target as usize;
                        continue;
                    }
                }
                Instr::LessEqualJumpIfFalse(target) => {
                    if !self.op_compare_and_pop(Comparison::LessEqual)? {
                        self.ip = target as usize;
                        continue;
                    }
                }
                Instr::GreaterConstantJumpIfFalse { constant, target } => {
                    if !self.op_compare_constant_and_pop(constant, Comparison::Greater)? {
                        self.ip = target as usize;
                        continue;
                    }
                }
                Instr::GreaterEqualConstantJumpIfFalse { constant, target } => {
                    if !self.op_compare_constant_and_pop(constant, Comparison::GreaterEqual)? {
                        self.ip = target as usize;
                        continue;
                    }
                }
                Instr::LessConstantJumpIfFalse { constant, target } => {
                    if !self.op_compare_constant_and_pop(constant, Comparison::Less)? {
                        self.ip = target as usize;
                        continue;
                    }
                }
                Instr::LessEqualConstantJumpIfFalse { constant, target } => {
                    if !self.op_compare_constant_and_pop(constant, Comparison::LessEqual)? {
                        self.ip = target as usize;
                        continue;
                    }
                }
                Instr::JumpIfNotNil(target) => {
                    if !matches!(self.peek(0), Value::Nil) {
                        self.ip = target as usize;
                        continue;
                    }
                }
                Instr::JumpIfNil(target) => {
                    if matches!(self.peek(0), Value::Nil) {
                        self.ip = target as usize;
                        continue;
                    }
                }
                Instr::Jump(target) | Instr::Loop(target) => {
                    self.ip = target as usize;
                    continue;
                }
                Instr::NoMatchArm => self.op_no_match_arm()?,
                Instr::IsArrayOfLen { length, at_least } => {
                    self.op_is_array_of_len(length, at_least)
                }
                Instr::IsNumber => self.op_is_number(),
                Instr::IsVariant(index) => self.op_is_variant(index)?,
                Instr::EnumConstruct(index) => self.op_enum_construct(index)?,
                Instr::Call(arg_count) => {
                    self.op_call(arg_count)?;
                    continue;
                }
                Instr::Invoke {
                    method_symbol,
                    arg_count,
                } => {
                    self.op_invoke(method_symbol, arg_count)?;
                    continue;
                }
                Instr::TailCall(arg_count) => {
                    if !self.op_tail_call(arg_count)? {
                        self.op_return();
                        if self.call_frames.len() == target_depth {
                            return Ok(());
                        }
                    }
                    continue;
                }
                Instr::TailInvoke {
                    method_symbol,
                    arg_count,
                } => {
                    if !self.op_tail_invoke(method_symbol, arg_count)? {
                        self.op_return();
                        if self.call_frames.len() == target_depth {
                            return Ok(());
                        }
                    }
                    continue;
                }
                Instr::GetField(symbol) => self.op_get_field(symbol)?,
                Instr::SetField(symbol) => self.op_set_field(symbol)?,
                Instr::GetLocalField { slot, symbol } => self.op_get_local_field(slot, symbol)?,

                Instr::CreateMap(count) => self.op_create_map(count)?,
                Instr::CreateArray(count) => self.op_create_array(count),
                Instr::CreateSet(count) => self.op_create_set(count)?,
                Instr::GetIndex => self.op_get_index()?,
                Instr::SetIndex => self.op_set_index()?,
                Instr::GetIterator { pairs } => self.op_get_iterator(pairs)?,
                Instr::IteratorNext(slot) => self.op_iterator_next(slot)?,
                Instr::IteratorDone(slot) => self.op_iterator_done(slot)?,
                Instr::CreateRange { inclusive } => self.op_create_range(inclusive)?,
                Instr::ToString => self.op_to_string(),
                Instr::BitwiseAnd => self.op_bitwise_and()?,
                Instr::BitwiseOr => self.op_bitwise_or()?,
                Instr::BitwiseXor => self.op_bitwise_xor()?,
                Instr::BitwiseNot => self.op_bitwise_not()?,
                Instr::LeftShift => self.op_left_shift()?,
                Instr::RightShift => self.op_right_shift()?,
                Instr::Closure {
                    const_index,
                    upvalue_count,
                    upvalues,
                } => self.op_closure(const_index, upvalue_count, upvalues)?,
                Instr::GetUpvalue(index) => self.op_get_upvalue(index)?,
                Instr::SetUpvalue(index) => self.op_set_upvalue(index)?,
                Instr::CloseUpvalue => self.op_close_upvalue(),
                Instr::DefineMethod {
                    method_symbol,
                    takes_self,
                    ..
                } => self.op_define_method(method_symbol, takes_self),
                Instr::DefineBuiltinMethod {
                    type_symbol,
                    method_symbol,
                    takes_self,
                } => self.op_define_builtin_method(type_symbol, method_symbol, takes_self),
                Instr::CheckInitialized => self.op_check_initialized()?,
                Instr::CheckTuple(n) => self.op_check_tuple(n)?,
                Instr::StoreLocal(slot) => self.op_store_local(slot)?,
                Instr::StoreField(symbol) => self.op_store_field(symbol)?,
                Instr::StoreLocalField { slot, symbol } => {
                    self.op_store_local_field(slot, symbol)?
                }
                Instr::AddConstant(index) => self.op_add_constant(index)?,
                Instr::SubtractConstant(index) => self.op_subtract_constant(index)?,
                Instr::ModuloConstant(index) => self.op_modulo_constant(index)?,
                Instr::MultiplyConstant(index) => self.op_multiply_constant(index)?,
                Instr::GetLocalAddConstant { slot, constant } => {
                    self.op_get_local_add_constant(slot, constant)?
                }
                Instr::GetLocalSubtractConstant { slot, constant } => {
                    self.op_get_local_subtract_constant(slot, constant)?
                }
                Instr::GetLocalMultiplyConstant { slot, constant } => {
                    self.op_get_local_multiply_constant(slot, constant)?
                }
                Instr::GetLocalModuloConstant { slot, constant } => {
                    self.op_get_local_modulo_constant(slot, constant)?
                }
                Instr::IncrementLocal { slot, constant } => {
                    self.op_increment_local(slot, constant)?
                }
                Instr::AddLocal(slot) => self.op_add_local(slot)?,
                Instr::SubtractLocal(slot) => self.op_subtract_local(slot)?,
                Instr::MultiplyLocal(slot) => self.op_multiply_local(slot)?,
                Instr::DivideLocal(slot) => self.op_divide_local(slot)?,
                Instr::AddLocalField { slot, symbol } => self.op_add_local_field(slot, symbol)?,
                Instr::SubtractLocalField { slot, symbol } => {
                    self.op_subtract_local_field(slot, symbol)?
                }
                Instr::MultiplyLocalField { slot, symbol } => {
                    self.op_multiply_local_field(slot, symbol)?
                }
                Instr::DivideLocalField { slot, symbol } => {
                    self.op_divide_local_field(slot, symbol)?
                }
                Instr::GreaterConstant(index) => {
                    self.op_compare_constant(index, Comparison::Greater)?
                }
                Instr::GreaterEqualConstant(index) => {
                    self.op_compare_constant(index, Comparison::GreaterEqual)?
                }
                Instr::LessConstant(index) => self.op_compare_constant(index, Comparison::Less)?,
                Instr::LessEqualConstant(index) => {
                    self.op_compare_constant(index, Comparison::LessEqual)?
                }
                Instr::BeginTry(catch_ip) => self.handlers.push(Handler {
                    catch_ip: catch_ip as usize,
                    frame_depth: self.call_frames.len(),
                    stack_height: self.stack.len(),
                }),
                Instr::EndTry => {
                    self.handlers.pop();
                }
                Instr::Throw => {
                    let thrown = self.pop();
                    let mut error = self.runtime_error(String::new());
                    if let Value::Error(obj) = &thrown {
                        let _ = obj.thrown_at.set(error.clone());
                    }
                    error.thrown = Some(thrown);
                    return Err(error);
                }
                Instr::Dup => self.push(self.peek(0).copy_or_clone()),
                Instr::Dup2 => {
                    let second = self.peek(1).copy_or_clone();
                    let top = self.peek(0).copy_or_clone();
                    self.push(second);
                    self.push(top);
                }
            }
            self.ip += 1;
        }
    }

    /// Makes `closure` the running frame. The caller's `ip` is saved on its
    /// frame, since `self.ip` and `self.chunk` only track the top frame.
    #[inline(always)]
    pub(in crate::vm) fn push_frame(&mut self, closure: Rc<ObjClosure>, slot_start: isize) {
        if let Some(caller) = self.call_frames.last_mut() {
            caller.ip = self.ip;
        }
        self.ip = 0;
        self.frame_base = (slot_start + 1) as usize;
        self.chunk = Rc::clone(&closure.function.chunk);
        self.call_frames.push(CallFrame {
            closure,
            ip: 0,
            slot_start,
        });
    }

    /// Drops the running frame and resumes its caller, if any.
    pub(in crate::vm) fn pop_frame(&mut self) {
        self.call_frames.pop();
        if let Some(caller) = self.call_frames.last() {
            self.ip = caller.ip;
            self.frame_base = (caller.slot_start + 1) as usize;
            self.chunk = Rc::clone(&caller.closure.function.chunk);
        }
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(crate) fn current_frame(&self) -> &CallFrame {
        self.call_frames
            .last()
            .expect("current_frame is only called while a call frame is running")
    }

    #[inline(always)]
    pub(in crate::vm) fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    #[inline(always)]
    #[allow(clippy::expect_used)]
    pub(in crate::vm) fn pop(&mut self) -> Value {
        self.stack
            .pop()
            .expect("pop is only called when the compiler has proven a value is on the stack")
    }

    #[inline(always)]
    pub(in crate::vm) fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.stack.len() - 1 - distance]
    }

    pub(in crate::vm) fn runtime_error(&self, message: impl Into<String>) -> RuntimeError {
        self.build_runtime_error(message, 0)
    }

    /// Moves `error`, raised by the field read of a fused `*LocalField`
    /// arithmetic instruction, to that field read's location.
    pub(in crate::vm) fn at_fused_field(&self, mut error: RuntimeError) -> RuntimeError {
        let info = self.chunk.fused_field_line_info(self.ip);
        error.location = info.map(|i| (i.line, i.column));
        if let Some(frame) = error.frames.first_mut() {
            frame.line = info.map(|i| i.line);
        }
        error
    }

    /// Like `runtime_error`, but for a failure raised while dispatching a
    /// call, whose `ip` has already moved past the CALL the same way a
    /// caller frame's has.
    pub(in crate::vm) fn call_error(&self, message: impl Into<String>) -> RuntimeError {
        self.build_runtime_error(message, 1)
    }

    fn build_runtime_error(
        &self,
        message: impl Into<String>,
        innermost_offset: usize,
    ) -> RuntimeError {
        let total = self.call_frames.len();
        let omitted_frames = if total > TRACE_EDGE_FRAMES * 2 + 1 {
            total - TRACE_EDGE_FRAMES * 2
        } else {
            0
        };

        let innermost_kept = if omitted_frames == 0 {
            total
        } else {
            TRACE_EDGE_FRAMES
        };
        let walk = self.call_frames.iter().rev().enumerate();
        let kept = walk
            .clone()
            .take(innermost_kept)
            .chain(walk.skip(innermost_kept + omitted_frames));

        let mut frames = Vec::with_capacity(total - omitted_frames);
        let mut location = None;
        let mut file = None;

        for (depth, frame) in kept {
            let ip = if depth == 0 {
                self.ip.saturating_sub(innermost_offset)
            } else {
                frame.ip.saturating_sub(1)
            };
            let chunk = &frame.closure.function.chunk;
            let info = chunk.instr_line_info(ip);
            if depth == 0 {
                location = info.as_ref().map(|i| (i.line, i.column));
                file = chunk.file.clone();
            }
            frames.push(TraceFrame {
                function: frame.closure.function.name.clone(),
                line: info.map(|i| i.line),
                file: chunk.file.clone(),
            });
        }

        RuntimeError {
            message: message.into(),
            location,
            file,
            frames,
            omitted_frames,
            thrown: None,
        }
    }

    #[cfg(any(test, debug_assertions, target_arch = "wasm32"))]
    pub fn get_output(&self) -> String {
        self.string_buffer.trim().to_string()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn clear_output(&mut self) {
        self.string_buffer.clear();
    }

    #[cfg(test)]
    pub(in crate::vm) fn get_compiler_error(&self) -> String {
        self.structured_errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn get_formatted_errors(&self, filename: &str) -> String {
        use crate::common::error_renderer::ErrorRenderer;

        let renderer = ErrorRenderer::default().with_module_sources(self.module_sources.clone());
        renderer.render_errors(&self.structured_errors, &self.source, filename)
    }

    pub fn get_compile_errors(&self) -> &[crate::common::errors::CompilationError] {
        &self.structured_errors
    }

    pub fn get_runtime_error(&self) -> Option<&RuntimeError> {
        self.runtime_error.as_ref()
    }

    pub fn get_runtime_errors(&self) -> String {
        self.runtime_error
            .as_ref()
            .map(|e| e.to_string())
            .unwrap_or_default()
    }

    #[cfg(feature = "opcode-stats")]
    fn pad_and_join(entries: &[(String, u64)]) -> String {
        let name_width = entries
            .iter()
            .map(|(name, _)| name.len())
            .max()
            .unwrap_or(0);
        entries
            .iter()
            .map(|(name, count)| format!("{:<width$} {}", name, count, width = name_width))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Executed-instruction histogram, one `<name> <count>` line per
    /// instruction that ran (named by its `Instr` variant, so a fused
    /// instruction appears under its fused name), followed (after a blank
    /// line, when any pair ran) by one `Prev->Next <count>` line per
    /// executed instruction pair, named the same way.
    #[cfg(feature = "opcode-stats")]
    pub fn opcode_stats_report(&self) -> String {
        let mut counts: Vec<(String, u64)> = self
            .opcode_counts
            .iter()
            .map(|(name, &count)| (name.to_string(), count))
            .collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let mut report = Self::pad_and_join(&counts);

        let mut pairs: Vec<(String, u64)> = self
            .opcode_pair_counts
            .iter()
            .map(|((prev, next), &count)| (format!("{}->{}", prev, next), count))
            .collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

        if !pairs.is_empty() {
            report.push_str("\n\n");
            report.push_str(&Self::pad_and_join(&pairs));
        }

        report
    }

    fn reset(&mut self) {
        self.call_frames.clear();
        self.stack.clear();
        self.runtime_error = None;
        self.open_upvalues.clear();
        self.handlers.clear();
        self.native_call_depth = 0;
        self.builtin_methods.iter_mut().for_each(Vec::clear);
        self.method_journal = None;
        self.repl_env = GlobalEnv::default();
        #[cfg(feature = "opcode-stats")]
        {
            self.opcode_counts.clear();
            self.opcode_pair_counts.clear();
            self.last_opcode = None;
        }
    }
}
