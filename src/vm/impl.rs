use crate::common::opcodes::OpCode;
use crate::common::{CallFrame, ObjClosure, ObjFunction, Value};
use crate::compiler::Compiler;
use crate::vm::functions::{Comparison, OpResult};
use crate::vm::{InterpretResult, RuntimeError, TraceFrame, VirtualMachine};
use crate::{boolean, common, nil};
#[cfg(not(target_arch = "wasm32"))]
use log::info;
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
            stack: Vec::new(),
            builtin: common::stdlib::create_builtin_objects(args),
            #[cfg(any(test, debug_assertions, target_arch = "wasm32"))]
            string_buffer: String::new(),
            structured_errors: Vec::new(),
            runtime_error: None,
            source: String::new(),
            open_upvalues: Vec::new(),
            native_call_depth: 0,
            methods: std::collections::HashMap::new(),
        }
    }

    pub fn new() -> Self {
        Self::with_args(vec![])
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        self.reset();

        self.source = source.clone();

        #[cfg(not(target_arch = "wasm32"))]
        let start = std::time::Instant::now();

        let mut compiler = Compiler::new();
        let chunk = compiler.compile(&source);

        #[cfg(not(target_arch = "wasm32"))]
        info!("Compile time: {}ms", start.elapsed().as_millis());

        #[cfg(not(target_arch = "wasm32"))]
        let start = std::time::Instant::now();
        if chunk.is_none() {
            self.structured_errors = compiler.get_structured_errors();
            return InterpretResult::CompileError;
        }

        let chunk = chunk.unwrap();

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
        let frame = CallFrame {
            closure: script_closure,
            ip: 0,
            slot_start: -1,
        };
        self.call_frames.push(frame);

        let result = self.run_script(0);

        #[cfg(not(target_arch = "wasm32"))]
        info!("Run time: {}ms", start.elapsed().as_millis());

        result
    }

    /// Runs until `target_depth`, converting a runtime error into the VM's
    /// stored error and the public `InterpretResult` enum.
    pub(in crate::vm) fn run_script(&mut self, target_depth: usize) -> InterpretResult {
        match self.run_until(target_depth) {
            Ok(()) => InterpretResult::Ok,
            Err(e) => {
                self.runtime_error = Some(e);
                InterpretResult::RuntimeError
            }
        }
    }

    #[inline(always)]
    pub(in crate::vm) fn run_until(&mut self, target_depth: usize) -> OpResult {
        #[cfg(feature = "disassemble")]
        if target_depth == 0 {
            let frame = self.call_frames.last().unwrap();
            frame.closure.function.chunk.disassemble_chunk();
        }
        loop {
            let byte = {
                let frame = self.current_frame();
                frame.closure.function.chunk.read_u8(frame.ip)
            };
            let op_code = match OpCode::from_u8(byte) {
                Some(op_code) => op_code,
                None => {
                    return Err(self.runtime_error(format!("Unknown opcode {:#04x}", byte)));
                }
            };

            match op_code {
                OpCode::Return => {
                    self.op_return();
                    if self.call_frames.len() == target_depth {
                        return Ok(());
                    }
                    continue;
                }
                OpCode::Constant => self.op_constant(),
                OpCode::Negate => self.op_negate()?,
                OpCode::Add => self.op_add()?,
                OpCode::Subtract => self.op_subtract()?,
                OpCode::Multiply => self.op_multiply()?,
                OpCode::Divide => self.op_divide()?,
                OpCode::Modulo => self.op_modulo()?,
                OpCode::Exponent => self.op_exponent()?,
                OpCode::Nil => self.push(nil!()),
                OpCode::True => self.push(boolean!(true)),
                OpCode::False => self.push(boolean!(false)),
                OpCode::Equal => self.op_equal(),
                OpCode::Greater => self.op_compare(Comparison::Greater)?,
                OpCode::GreaterEqual => self.op_compare(Comparison::GreaterEqual)?,
                OpCode::Less => self.op_compare(Comparison::Less)?,
                OpCode::LessEqual => self.op_compare(Comparison::LessEqual)?,
                OpCode::Not => self.op_not(),
                OpCode::Pop => _ = self.pop(),
                OpCode::GetLocal => self.op_get_local()?,
                OpCode::SetLocal => self.op_set_local()?,
                OpCode::GetBuiltin => self.op_get_builtin()?,
                OpCode::GetGlobal => self.op_get_global()?,
                OpCode::SetGlobal => self.op_set_global()?,
                OpCode::JumpIfFalse => self.op_jump_if_false(),
                OpCode::Jump => self.op_jump(),
                OpCode::Loop => self.op_loop(),
                OpCode::Call => {
                    self.op_call()?;
                    continue;
                }
                OpCode::Invoke => {
                    self.op_invoke()?;
                    continue;
                }
                OpCode::GetField => self.op_get_field()?,
                OpCode::SetField => self.op_set_field()?,

                OpCode::CreateMap => self.op_create_map()?,
                OpCode::CreateArray => self.op_create_array(),
                OpCode::CreateSet => self.op_create_set()?,
                OpCode::GetIndex => self.op_get_index()?,
                OpCode::SetIndex => self.op_set_index()?,
                OpCode::GetIterator => self.op_get_iterator()?,
                OpCode::IteratorNext => self.op_iterator_next()?,
                OpCode::IteratorDone => self.op_iterator_done()?,
                OpCode::CreateRange => self.op_create_range()?,
                OpCode::ToString => self.op_to_string(),
                OpCode::BitwiseAnd => self.op_bitwise_and()?,
                OpCode::BitwiseOr => self.op_bitwise_or()?,
                OpCode::BitwiseXor => self.op_bitwise_xor()?,
                OpCode::BitwiseNot => self.op_bitwise_not()?,
                OpCode::LeftShift => self.op_left_shift()?,
                OpCode::RightShift => self.op_right_shift()?,
                OpCode::Closure => self.op_closure()?,
                OpCode::GetUpvalue => self.op_get_upvalue()?,
                OpCode::SetUpvalue => self.op_set_upvalue()?,
                OpCode::CloseUpvalue => self.op_close_upvalue(),
                OpCode::CloseUpvalueInPlace => self.op_close_upvalue_in_place(),
                OpCode::DefineMethod => self.op_define_method(),
                OpCode::CheckInitialized => self.op_check_initialized()?,
            }
            self.current_frame_mut().ip += 1;
        }
    }

    #[inline(always)]
    pub(crate) fn current_frame(&self) -> &CallFrame {
        self.call_frames.last().expect("call frame stack is empty")
    }

    #[inline(always)]
    pub(crate) fn current_frame_mut(&mut self) -> &mut CallFrame {
        self.call_frames
            .last_mut()
            .expect("call frame stack is empty")
    }

    #[inline(always)]
    pub(in crate::vm) fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    #[inline(always)]
    pub(in crate::vm) fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    #[inline(always)]
    pub(in crate::vm) fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack.len() - 1 - distance].clone()
    }

    pub(in crate::vm) fn runtime_error(&self, message: impl Into<String>) -> RuntimeError {
        self.build_runtime_error(message, 0)
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
        let mut frames = Vec::with_capacity(self.call_frames.len());
        let mut location = None;

        for (depth, frame) in self.call_frames.iter().rev().enumerate() {
            let ip = if depth == 0 {
                frame.ip.saturating_sub(innermost_offset)
            } else {
                frame.ip.saturating_sub(1)
            };
            let info = frame.closure.function.chunk.get_line_info(ip);
            if depth == 0 {
                location = info.as_ref().map(|i| (i.line, i.column));
            }
            frames.push(TraceFrame {
                function: frame.closure.function.name.clone(),
                line: info.map(|i| i.line),
            });
        }

        RuntimeError {
            message: message.into(),
            location,
            frames,
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

        let renderer = ErrorRenderer::default();
        renderer.render_errors(&self.structured_errors, &self.source, filename)
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

    fn reset(&mut self) {
        self.call_frames.clear();
        self.stack.clear();
        self.runtime_error = None;
        self.open_upvalues.clear();
        self.native_call_depth = 0;
        self.methods.clear();
    }
}
