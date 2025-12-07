use crate::common::constants::VARIADIC_ARITY;
use crate::common::opcodes::OpCode;
use crate::common::{BitsSize, Bloq, CallFrame, ObjFunction, ObjInstance, ObjStruct, Value, Object, ObjString};
use crate::compiler::Compiler;
use crate::vm::math_functions::*;
use crate::vm::file_functions::*;
use crate::vm::http_functions::*;
use crate::vm::{Result, VirtualMachine};
use crate::{boolean, nil};
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(not(target_arch = "wasm32"))]
use log::info;

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualMachine {
    /// Creates a new VirtualMachine with command-line arguments
    pub fn with_args(args: Vec<String>) -> Self {
        let mut globals = HashMap::new();

        // Initialize built-in global objects
        globals.insert("Math".to_string(), Self::create_math_object());

        // Initialize built-in global functions
        globals.insert("File".to_string(), Value::new_native_function("File".to_string(), 1, native_file_constructor));
        globals.insert(
            "HttpServer".to_string(),
            Value::new_native_function("HttpServer".to_string(), 1, native_http_server_constructor)
        );

        // Initialize args array with command-line arguments
        let args_array = Self::create_args_array(args);
        globals.insert("args".to_string(), args_array);

        VirtualMachine {
            call_frames: Vec::new(),
            stack: Vec::new(),
            bloq: None,
            globals,
            #[cfg(any(test, debug_assertions, target_arch = "wasm32"))]
            string_buffer: String::new(),
            compilation_errors: String::new(),
            structured_errors: Vec::new(),
            runtime_errors: String::new(),
            source: String::new(),
            iterator_stack: Vec::new(),
        }
    }

    /// Creates a new VirtualMachine with no command-line arguments
    pub fn new() -> Self {
        Self::with_args(vec![])
    }

    /// Creates an array containing command-line arguments as strings
    fn create_args_array(args: Vec<String>) -> Value {
        let elements: Vec<Value> = args
            .into_iter()
            .map(|arg| {
                Value::Object(Rc::new(Object::String(ObjString {
                    value: Rc::from(arg.as_str()),
                })))
            })
            .collect();
        Value::new_array(elements)
    }

    /// Creates the Math built-in object with all math functions as fields
    /// The Math object is a struct-like object with function fields
    fn create_math_object() -> Value {
        // Create the Math struct definition with field names
        let math_struct = Rc::new(ObjStruct {
            name: "Math".to_string(),
            fields: vec![
                "abs".to_string(),
                "floor".to_string(),
                "ceil".to_string(),
                "sqrt".to_string(),
                "min".to_string(),
                "max".to_string(),
            ],
        });

        // Create an instance of the Math struct with native function values
        let mut fields = HashMap::new();
        fields.insert(
            "abs".to_string(),
            Value::new_native_function("abs".to_string(), 1, native_math_abs),
        );
        fields.insert(
            "floor".to_string(),
            Value::new_native_function("floor".to_string(), 1, native_math_floor),
        );
        fields.insert(
            "ceil".to_string(),
            Value::new_native_function("ceil".to_string(), 1, native_math_ceil),
        );
        fields.insert(
            "sqrt".to_string(),
            Value::new_native_function("sqrt".to_string(), 1, native_math_sqrt),
        );
        fields.insert(
            "min".to_string(),
            Value::new_native_function("min".to_string(), VARIADIC_ARITY, native_math_min),
        );
        fields.insert(
            "max".to_string(),
            Value::new_native_function("max".to_string(), VARIADIC_ARITY, native_math_max),
        );

        let math_instance = ObjInstance {
            r#struct: math_struct,
            fields,
        };

        Value::new_object(math_instance)
    }

    pub fn interpret(&mut self, source: String) -> Result {
        self.reset();

        // Store source for error reporting
        self.source = source.clone();

        #[cfg(not(target_arch = "wasm32"))]
        let start = std::time::Instant::now();

        let mut compiler = Compiler::new();
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

        // Create a synthetic function for the script
        let script_function = Rc::new(ObjFunction {
            name: "<script>".to_string(),
            arity: 0,
            bloq: Rc::new(bloq),
        });

        // Create the initial call frame
        // Use -1 for slot_start since the script has no function object on the stack
        let frame = CallFrame {
            function: script_function,
            ip: 0,
            slot_start: -1,
        };
        self.call_frames.push(frame);

        let result = self.run(&Bloq::new("dummy")); // bloq param is not used anymore
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
            let frame = self.call_frames.last_mut().unwrap();
            let ip = frame.ip;
            let op_code = OpCode::from_u8(frame.function.bloq.read_u8(ip));

            // Track whether we should increment IP at the end
            let mut should_increment_ip = true;

            match op_code {
                OpCode::Return => {
                    if let Some(result) = self.fn_return() {
                        return result;
                    }
                    // Don't increment IP after return since we switched frames
                    should_increment_ip = false;
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
                    // Don't increment IP after call since we pushed a new frame or instantiated
                    should_increment_ip = false;
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
                    should_increment_ip = false;
                }
                OpCode::CallMethod2 => {
                    if let Some(result) = self.fn_call_method(BitsSize::Sixteen) {
                        return result;
                    }
                    should_increment_ip = false;
                }
                OpCode::CallMethod4 => {
                    if let Some(result) = self.fn_call_method(BitsSize::ThirtyTwo) {
                        return result;
                    }
                    should_increment_ip = false;
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
                    // Pop the current iterator from the iterator stack
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

            // Increment IP for the current frame
            if should_increment_ip {
                let frame = self.call_frames.last_mut().unwrap();
                frame.ip += 1;
            }
        }
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

        // Always print to stderr for native/debug builds
        eprintln!("{}", error_message);

        // Also capture in buffer for WASM and testing
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

    /// Call a Neon function from native code with arguments
    pub(crate) fn call_function_with_args(
        &mut self,
        function: &Value,
        args: Vec<Value>
    ) -> std::result::Result<Value, String> {
        match function {
            Value::Object(obj) => match obj.as_ref() {
                Object::Function(func) => {
                    // Push function and arguments onto stack
                    self.push(function.clone());
                    for arg in &args {
                        self.push(arg.clone());
                    }

                    // Create call frame
                    let arg_count = args.len();
                    let slot_start = self.stack.len() as isize - arg_count as isize - 1;
                    let frame = CallFrame {
                        function: Rc::clone(func),
                        ip: 0,
                        slot_start,
                    };
                    self.call_frames.push(frame);

                    // Execute
                    match self.run(&func.bloq) {
                        Result::Ok => Ok(self.pop()),
                        Result::RuntimeError => Err(self.runtime_errors.clone()),
                        Result::CompileError => Err("Unexpected compile error".to_string()),
                    }
                }
                Object::NativeFunction(native_fn) => {
                    (native_fn.function)(self, &args)
                }
                _ => Err("Not a function".to_string()),
            },
            _ => Err("Not a function".to_string()),
        }
    }

    /// Look up a native method for a given type and method name.
    ///
    /// This function provides runtime dispatch for native methods. The method registry
    /// (MethodRegistry) is used at compile-time to validate method calls.
    ///
    /// The dispatch table is automatically generated by the define_native_methods! macro
    /// in src/common/method_registry.rs to ensure consistency with compile-time validation.
    ///
    /// Note: In debug builds, we verify consistency with the MethodRegistry to catch
    /// any discrepancies between compile-time validation and runtime dispatch.
    pub(in crate::vm) fn get_native_method(
        type_name: &str,
        method_name: &str,
    ) -> Option<crate::common::NativeFn> {
        crate::common::method_registry::get_native_method(type_name, method_name)
    }
}
