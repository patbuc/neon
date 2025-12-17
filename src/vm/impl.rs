use crate::common::opcodes::OpCode;
use crate::common::{BitsSize, Bloq, CallFrame, ObjFunction, Value};
use crate::compiler::Compiler;
use crate::vm::{Result, VirtualMachine};
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
            bloq: None,
            builtin: common::builtin::create_builtin_objects(args),
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
        let bloq = compiler.compile(&source);

        #[cfg(not(target_arch = "wasm32"))]
        info!("Compile time: {}ms", start.elapsed().as_millis());

        #[cfg(not(target_arch = "wasm32"))]
        let start = std::time::Instant::now();
        if bloq.is_none() {
            self.compilation_errors = compiler.get_compilation_errors();
            self.structured_errors = compiler.get_structured_errors();
            return Result::CompileError;
        }

        let bloq = bloq.unwrap();

        let script_function = Rc::new(ObjFunction {
            name: "<script>".to_string(),
            arity: 0,
            bloq: Rc::new(bloq),
        });

        // Use -1 for slot_start since the script has no function object on the stack
        let frame = CallFrame {
            function: script_function,
            ip: 0,
            slot_start: -1,
        };
        self.call_frames.push(frame);

        let result = self.run(&Bloq::new("dummy"));
        self.bloq = None;

        #[cfg(not(target_arch = "wasm32"))]
        info!("Run time: {}ms", start.elapsed().as_millis());

        result
    }

    #[inline(always)]
    pub(in crate::vm) fn run(&mut self, _bloq: &Bloq) -> Result {
        #[cfg(feature = "disassemble")]
        {
            let frame = self.call_frames.last().unwrap();
            frame.function.bloq.disassemble_bloq();
        }
        loop {
            let op_code = {
                let frame = self.current_frame();
                let ip = frame.ip;
                OpCode::from_u8(frame.function.bloq.read_u8(ip))
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
                OpCode::Subtract => self.fn_subtract(),
                OpCode::Multiply => self.fn_multiply(),
                OpCode::Divide => self.fn_divide(),
                OpCode::FloorDivide => self.fn_floor_divide(),
                OpCode::Modulo => self.fn_modulo(),
                OpCode::Nil => self.push(nil!()),
                OpCode::True => self.push(boolean!(true)),
                OpCode::False => self.push(boolean!(false)),
                OpCode::Equal => self.fn_equal(),
                OpCode::Greater => self.fn_greater(),
                OpCode::Less => self.fn_less(),
                OpCode::Not => self.fn_not(),
                OpCode::String => self.fn_string(),
                OpCode::String2 => self.fn_string2(),
                OpCode::String4 => self.fn_string4(),
                OpCode::Print => self.fn_print(),
                OpCode::Pop => _ = self.pop(),
                OpCode::GetLocal => self.fn_get_local(BitsSize::Eight),
                OpCode::GetLocal2 => self.fn_get_local(BitsSize::Sixteen),
                OpCode::GetLocal4 => self.fn_get_local(BitsSize::ThirtyTwo),
                OpCode::SetLocal => self.fn_set_local(BitsSize::Eight),
                OpCode::SetLocal2 => self.fn_set_local(BitsSize::Sixteen),
                OpCode::SetLocal4 => self.fn_set_local(BitsSize::ThirtyTwo),
                OpCode::GetBuiltin => self.fn_get_builtin(BitsSize::Eight),
                OpCode::GetBuiltin2 => self.fn_get_builtin(BitsSize::Sixteen),
                OpCode::GetBuiltin4 => self.fn_get_builtin(BitsSize::ThirtyTwo),
                OpCode::GetGlobal => self.fn_get_global(BitsSize::Eight),
                OpCode::GetGlobal2 => self.fn_get_global(BitsSize::Sixteen),
                OpCode::GetGlobal4 => self.fn_get_global(BitsSize::ThirtyTwo),
                OpCode::SetGlobal => self.fn_set_global(BitsSize::Eight),
                OpCode::SetGlobal2 => self.fn_set_global(BitsSize::Sixteen),
                OpCode::SetGlobal4 => self.fn_set_global(BitsSize::ThirtyTwo),
                OpCode::JumpIfFalse => self.fn_jump_if_false(),
                OpCode::Jump => self.fn_jump(),
                OpCode::Loop => self.fn_loop(),
                OpCode::Call => {
                    if let Some(result) = self.fn_call() {
                        return result;
                    }
                    continue;
                }
                OpCode::GetField => self.fn_get_field(BitsSize::Eight),
                OpCode::GetField2 => self.fn_get_field(BitsSize::Sixteen),
                OpCode::GetField4 => self.fn_get_field(BitsSize::ThirtyTwo),
                OpCode::SetField => self.fn_set_field(BitsSize::Eight),
                OpCode::SetField2 => self.fn_set_field(BitsSize::Sixteen),
                OpCode::SetField4 => self.fn_set_field(BitsSize::ThirtyTwo),
                OpCode::CallMethod => {
                    if let Some(result) = self.fn_call_method(BitsSize::Eight) {
                        return result;
                    }
                    continue;
                }
                OpCode::CallMethod2 => {
                    if let Some(result) = self.fn_call_method(BitsSize::Sixteen) {
                        return result;
                    }
                    continue;
                }
                OpCode::CallMethod4 => {
                    if let Some(result) = self.fn_call_method(BitsSize::ThirtyTwo) {
                        return result;
                    }
                    continue;
                }
                OpCode::CallStaticMethod => {
                    if let Some(result) = self.fn_call_static_method(BitsSize::Eight) {
                        return result;
                    }
                    continue;
                }
                OpCode::CallStaticMethod2 => {
                    if let Some(result) = self.fn_call_static_method(BitsSize::Sixteen) {
                        return result;
                    }
                    continue;
                }
                OpCode::CallStaticMethod4 => {
                    if let Some(result) = self.fn_call_static_method(BitsSize::ThirtyTwo) {
                        return result;
                    }
                    continue;
                }
                OpCode::CallConstructor => {
                    if let Some(result) = self.fn_call_constructor(BitsSize::Eight) {
                        return result;
                    }
                    continue;
                }
                OpCode::CallConstructor2 => {
                    if let Some(result) = self.fn_call_constructor(BitsSize::Sixteen) {
                        return result;
                    }
                    continue;
                }
                OpCode::CallConstructor4 => {
                    if let Some(result) = self.fn_call_constructor(BitsSize::ThirtyTwo) {
                        return result;
                    }
                    continue;
                }
                OpCode::CreateMap => self.fn_create_map(),
                OpCode::CreateArray => self.fn_create_array(),
                OpCode::CreateSet => self.fn_create_set(),
                OpCode::GetIndex => self.fn_get_index(),
                OpCode::SetIndex => self.fn_set_index(),
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
                OpCode::IteratorDone => self.fn_iterator_done(),
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
            }
            self.current_frame_mut().ip += 1;
        }
    }

    #[inline(always)]
    pub(crate) fn current_frame(&self) -> &CallFrame {
        // Single point of access with debug assertion
        debug_assert!(!self.call_frames.is_empty());
        unsafe { self.call_frames.get_unchecked(self.call_frames.len() - 1) }
    }

    #[inline(always)]
    pub(crate) fn current_frame_mut(&mut self) -> &mut CallFrame {
        let len = self.call_frames.len();
        debug_assert!(len > 0);
        unsafe { self.call_frames.get_unchecked_mut(len - 1) }
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

    #[cfg(all(target_arch = "wasm32", not(test)))]
    pub fn get_output(&self) -> String {
        self.string_buffer.clone()
    }

    #[cfg(any(test, all(debug_assertions, not(target_arch = "wasm32"))))]
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
            if let Some(location) = frame.function.bloq.get_source_location(frame.ip) {
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
        self.bloq = None;
        self.runtime_errors.clear();
    }

    pub(in crate::vm) fn get_native_method(
        type_name: &str,
        method_name: &str,
    ) -> Option<&'static crate::common::method_registry::NativeCallable> {
        crate::common::method_registry::get_native_method(type_name, method_name)
    }
}
