use crate::common::opcodes::OpCode;
use crate::common::{BitsSize, Bloq, CallFrame, ObjFunction, Value};
use crate::compiler::Compiler;
use crate::vm::{Result, VirtualMachine};
use crate::{boolean, nil};
use log::info;
use std::rc::Rc;

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            call_frames: Vec::new(),
            stack: Vec::new(),
            bloq: None,
            #[cfg(any(test, debug_assertions))]
            string_buffer: String::new(),
            compilation_errors: String::new(),
        }
    }

    pub fn interpret(&mut self, source: String) -> Result {
        self.reset();

        let start = std::time::Instant::now();
        let mut compiler = Compiler::new();
        let bloq = compiler.compile(source);

        info!("Compile time: {}ms", start.elapsed().as_millis());

        let start = std::time::Instant::now();
        if bloq.is_none() {
            self.compilation_errors = compiler.get_compilation_errors();
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
                    // Don't increment IP after call since we pushed a new frame
                    should_increment_ip = false;
                }
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
        eprintln!("[{}] {}", source_location, error);
    }

    #[cfg(any(test, debug_assertions))]
    pub fn get_output(&self) -> String {
        self.string_buffer.trim().to_string()
    }

    #[cfg(test)]
    pub(in crate::vm) fn get_compiler_error(&self) -> String {
        self.compilation_errors.clone()
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
    }
}
