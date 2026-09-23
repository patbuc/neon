use crate::common::opcodes::OpCode;
use crate::common::{BitsSize, CallFrame, Chunk, ObjFunction, Value};
use crate::compiler::Compiler;
use crate::vm::{Result, VirtualMachine};
use crate::{boolean, common, nil};
#[cfg(not(target_arch = "wasm32"))]
use log::info;
use std::cmp::Ordering;
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
            chunk: None,
            builtin: common::stdlib::create_builtin_objects(args),
            #[cfg(any(test, debug_assertions, target_arch = "wasm32"))]
            string_buffer: String::new(),
            compilation_errors: String::new(),
            structured_errors: Vec::new(),
            runtime_errors: String::new(),
            source: String::new(),
            iterator_stack: Vec::new(),
        }
    }

    pub fn new() -> Self {
        Self::with_args(vec![])
    }

    pub fn interpret(&mut self, source: String) -> Result {
        self.reset();

        self.source = source.clone();

        #[cfg(not(target_arch = "wasm32"))]
        let start = std::time::Instant::now();

        let mut compiler = Compiler::new(self.builtin.clone());
        let chunk = compiler.compile(&source);

        #[cfg(not(target_arch = "wasm32"))]
        info!("Compile time: {}ms", start.elapsed().as_millis());

        #[cfg(not(target_arch = "wasm32"))]
        let start = std::time::Instant::now();
        if chunk.is_none() {
            self.compilation_errors = compiler.get_compilation_errors();
            self.structured_errors = compiler.get_structured_errors();
            return Result::CompileError;
        }

        let chunk = chunk.unwrap();

        let script_function = Rc::new(ObjFunction {
            name: "<script>".to_string(),
            arity: 0,
            chunk: Rc::new(chunk),
        });

        // Use -1 for slot_start since the script has no function object on the stack
        let frame = CallFrame {
            function: script_function,
            ip: 0,
            slot_start: -1,
        };
        self.call_frames.push(frame);

        let result = self.run(&Chunk::new("dummy"));
        self.chunk = None;

        #[cfg(not(target_arch = "wasm32"))]
        info!("Run time: {}ms", start.elapsed().as_millis());

        result
    }

    #[inline(always)]
    pub(in crate::vm) fn run(&mut self, _chunk: &Chunk) -> Result {
        #[cfg(feature = "disassemble")]
        {
            let frame = self.call_frames.last().unwrap();
            frame.function.chunk.disassemble_chunk();
        }
        loop {
            let byte = {
                let frame = self.current_frame();
                frame.function.chunk.read_u8(frame.ip)
            };
            let op_code = match OpCode::from_u8(byte) {
                Some(op_code) => op_code,
                None => {
                    self.runtime_error(&format!("Unknown opcode {:#04x}", byte));
                    return Result::RuntimeError;
                }
            };

            match op_code {
                OpCode::Return => {
                    if let Some(result) = self.fn_return() {
                        return result;
                    }
                    continue;
                }
                OpCode::Constant => self.fn_constant(),
                OpCode::Constant2 => self.fn_constant2(),
                OpCode::Constant4 => self.fn_constant4(),
                OpCode::Negate => {
                    if let Some(value) = self.fn_negate() {
                        return value;
                    }
                }
                OpCode::Add => {
                    if let Some(value) = self.fn_add() {
                        return value;
                    }
                }
                OpCode::Subtract => {
                    if let Some(result) = self.fn_subtract() {
                        return result;
                    }
                }
                OpCode::Multiply => {
                    if let Some(result) = self.fn_multiply() {
                        return result;
                    }
                }
                OpCode::Divide => {
                    if let Some(result) = self.fn_divide() {
                        return result;
                    }
                }
                OpCode::Modulo => {
                    if let Some(result) = self.fn_modulo() {
                        return result;
                    }
                }
                OpCode::Exponent => {
                    if let Some(result) = self.fn_exponent() {
                        return result;
                    }
                }
                OpCode::Nil => self.push(nil!()),
                OpCode::True => self.push(boolean!(true)),
                OpCode::False => self.push(boolean!(false)),
                OpCode::Equal => self.fn_equal(),
                OpCode::Greater => {
                    if let Some(result) = self.fn_compare(Ordering::Greater) {
                        return result;
                    }
                }
                OpCode::Less => {
                    if let Some(result) = self.fn_compare(Ordering::Less) {
                        return result;
                    }
                }
                OpCode::Not => self.fn_not(),
                OpCode::String => self.fn_string(),
                OpCode::String2 => self.fn_string2(),
                OpCode::String4 => self.fn_string4(),
                OpCode::Pop => _ = self.pop(),
                OpCode::GetLocal => {
                    if let Some(result) = self.fn_get_local(BitsSize::Eight) {
                        return result;
                    }
                }
                OpCode::GetLocal2 => {
                    if let Some(result) = self.fn_get_local(BitsSize::Sixteen) {
                        return result;
                    }
                }
                OpCode::GetLocal4 => {
                    if let Some(result) = self.fn_get_local(BitsSize::ThirtyTwo) {
                        return result;
                    }
                }
                OpCode::SetLocal => {
                    if let Some(result) = self.fn_set_local(BitsSize::Eight) {
                        return result;
                    }
                }
                OpCode::SetLocal2 => {
                    if let Some(result) = self.fn_set_local(BitsSize::Sixteen) {
                        return result;
                    }
                }
                OpCode::SetLocal4 => {
                    if let Some(result) = self.fn_set_local(BitsSize::ThirtyTwo) {
                        return result;
                    }
                }
                OpCode::GetBuiltin => {
                    if let Some(result) = self.fn_get_builtin(BitsSize::Eight) {
                        return result;
                    }
                }
                OpCode::GetBuiltin2 => {
                    if let Some(result) = self.fn_get_builtin(BitsSize::Sixteen) {
                        return result;
                    }
                }
                OpCode::GetBuiltin4 => {
                    if let Some(result) = self.fn_get_builtin(BitsSize::ThirtyTwo) {
                        return result;
                    }
                }
                OpCode::GetGlobal => {
                    if let Some(result) = self.fn_get_global(BitsSize::Eight) {
                        return result;
                    }
                }
                OpCode::GetGlobal2 => {
                    if let Some(result) = self.fn_get_global(BitsSize::Sixteen) {
                        return result;
                    }
                }
                OpCode::GetGlobal4 => {
                    if let Some(result) = self.fn_get_global(BitsSize::ThirtyTwo) {
                        return result;
                    }
                }
                OpCode::SetGlobal => {
                    if let Some(result) = self.fn_set_global(BitsSize::Eight) {
                        return result;
                    }
                }
                OpCode::SetGlobal2 => {
                    if let Some(result) = self.fn_set_global(BitsSize::Sixteen) {
                        return result;
                    }
                }
                OpCode::SetGlobal4 => {
                    if let Some(result) = self.fn_set_global(BitsSize::ThirtyTwo) {
                        return result;
                    }
                }
                OpCode::JumpIfFalse => self.fn_jump_if_false(),
                OpCode::Jump => self.fn_jump(),
                OpCode::Loop => self.fn_loop(),
                OpCode::Call => {
                    if let Some(result) = self.fn_call() {
                        return result;
                    }
                    continue;
                }
                OpCode::GetField => {
                    if let Some(result) = self.fn_get_field(BitsSize::Eight) {
                        return result;
                    }
                }
                OpCode::GetField2 => {
                    if let Some(result) = self.fn_get_field(BitsSize::Sixteen) {
                        return result;
                    }
                }
                OpCode::GetField4 => {
                    if let Some(result) = self.fn_get_field(BitsSize::ThirtyTwo) {
                        return result;
                    }
                }
                OpCode::SetField => {
                    if let Some(result) = self.fn_set_field(BitsSize::Eight) {
                        return result;
                    }
                }
                OpCode::SetField2 => {
                    if let Some(result) = self.fn_set_field(BitsSize::Sixteen) {
                        return result;
                    }
                }
                OpCode::SetField4 => {
                    if let Some(result) = self.fn_set_field(BitsSize::ThirtyTwo) {
                        return result;
                    }
                }

                OpCode::CreateMap => {
                    if let Some(result) = self.fn_create_map() {
                        return result;
                    }
                }
                OpCode::CreateArray => self.fn_create_array(),
                OpCode::CreateSet => {
                    if let Some(result) = self.fn_create_set() {
                        return result;
                    }
                }
                OpCode::GetIndex => {
                    if let Some(result) = self.fn_get_index() {
                        return result;
                    }
                }
                OpCode::SetIndex => {
                    if let Some(result) = self.fn_set_index() {
                        return result;
                    }
                }
                OpCode::GetIterator => {
                    if let Some(result) = self.fn_get_iterator() {
                        return result;
                    }
                }
                OpCode::IteratorNext => {
                    if let Some(result) = self.fn_iterator_next() {
                        return result;
                    }
                }
                OpCode::IteratorDone => {
                    if let Some(result) = self.fn_iterator_done() {
                        return result;
                    }
                }
                OpCode::PopIterator => {
                    if self.iterator_stack.is_empty() {
                        self.runtime_error("No iterator to pop");
                        return Result::RuntimeError;
                    }
                    self.iterator_stack.pop();
                }
                OpCode::CreateRange => {
                    if let Some(result) = self.fn_create_range() {
                        return result;
                    }
                }
                OpCode::ToString => self.fn_to_string(),
                OpCode::BitwiseAnd => {
                    if let Some(result) = self.fn_bitwise_and() {
                        return result;
                    }
                }
                OpCode::BitwiseOr => {
                    if let Some(result) = self.fn_bitwise_or() {
                        return result;
                    }
                }
                OpCode::BitwiseXor => {
                    if let Some(result) = self.fn_bitwise_xor() {
                        return result;
                    }
                }
                OpCode::BitwiseNot => {
                    if let Some(value) = self.fn_bitwise_not() {
                        return value;
                    }
                }
                OpCode::LeftShift => {
                    if let Some(result) = self.fn_left_shift() {
                        return result;
                    }
                }
                OpCode::RightShift => {
                    if let Some(result) = self.fn_right_shift() {
                        return result;
                    }
                }
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

    pub(in crate::vm) fn runtime_error(&mut self, error: &str) {
        let source_location = self.get_current_source_location();
        let error_message = format!("[{}] {}", source_location, error);

        eprintln!("{}", error_message);

        if !self.runtime_errors.is_empty() {
            self.runtime_errors.push('\n');
        }
        self.runtime_errors.push_str(&error_message);
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
        self.compilation_errors.clone()
    }

    pub fn get_formatted_errors(&self, filename: &str) -> String {
        use crate::common::error_renderer::ErrorRenderer;

        let renderer = ErrorRenderer::default();
        renderer.render_errors(&self.structured_errors, &self.source, filename)
    }

    pub fn get_runtime_errors(&self) -> String {
        self.runtime_errors.clone()
    }

    pub fn clear_runtime_errors(&mut self) {
        self.runtime_errors.clear();
    }

    fn get_current_source_location(&self) -> String {
        if let Some(frame) = self.call_frames.last() {
            if let Some(location) = frame.function.chunk.get_source_location(frame.ip) {
                format!("{}:{}", location.line, location.column)
            } else {
                "unknown".to_string()
            }
        } else {
            "unknown".to_string()
        }
    }

    fn reset(&mut self) {
        self.call_frames.clear();
        self.stack.clear();
        self.chunk = None;
        self.runtime_errors.clear();
    }
}
