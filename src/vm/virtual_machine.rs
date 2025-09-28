mod functions;

use crate::common::opcodes::OpCode;
use crate::common::{BitsSize, Brick, SourceLocation, Value};
use crate::compiler::Compiler;
use crate::vm::{Result, VirtualMachine};
use crate::{boolean, nil};
use log::info;

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            ip: 0,
            stack: Vec::new(),
            brick: None,
            #[cfg(test)]
            string_buffer: String::new(),
            compilation_errors: String::new(),
        }
    }

    pub fn interpret(&mut self, source: String) -> Result {
        self.reset();

        let start = std::time::Instant::now();
        let mut compiler = Compiler::new();
        let brick = compiler.compile(source);

        info!("Compile time: {}ms", start.elapsed().as_millis());

        let start = std::time::Instant::now();
        if brick.is_none() {
            self.compilation_errors = compiler.get_compilation_errors();
            return Result::CompileError;
        }

        let brick = brick.unwrap();
        let result = self.run(&brick);
        self.brick = None;

        info!("Run time: {}ms", start.elapsed().as_millis());
        result
    }

    #[inline(always)]
    fn run(&mut self, brick: &Brick) -> Result {
        #[cfg(feature = "disassemble")]
        brick.disassemble_brick();
        loop {
            let op_code = OpCode::from_u8(brick.read_u8(self.ip));
            match op_code {
                OpCode::Return => return Result::Ok,
                OpCode::Constant => self.fn_constant(brick),
                OpCode::Constant2 => self.fn_constant2(brick),
                OpCode::Constant4 => self.fn_constant4(brick),
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
                OpCode::Nil => self.push(nil!()),
                OpCode::True => self.push(boolean!(true)),
                OpCode::False => self.push(boolean!(false)),
                OpCode::Equal => self.fn_equal(),
                OpCode::Greater => self.fn_greater(),
                OpCode::Less => self.fn_less(),
                OpCode::Not => self.fn_not(),
                OpCode::String => self.fn_string(brick),
                OpCode::String2 => self.fn_string2(brick),
                OpCode::String4 => self.fn_string4(brick),
                OpCode::Print => self.fn_print(),
                OpCode::Pop => _ = self.pop(),
                OpCode::GetValue => self.fn_get_value(brick, BitsSize::Eight),
                OpCode::GetValue2 => self.fn_get_value(brick, BitsSize::Sixteen),
                OpCode::GetValue4 => self.fn_get_value(brick, BitsSize::ThirtyTwo),
                OpCode::SetValue => self.fn_set_value(brick, BitsSize::Eight),
                OpCode::SetValue2 => self.fn_set_value(brick, BitsSize::Sixteen),
                OpCode::SetValue4 => self.fn_set_value(brick, BitsSize::ThirtyTwo),
                OpCode::GetVariable => self.fn_get_variable(brick, BitsSize::Eight),
                OpCode::GetVariable2 => self.fn_get_variable(brick, BitsSize::Sixteen),
                OpCode::GetVariable4 => self.fn_get_variable(brick, BitsSize::ThirtyTwo),
                OpCode::SetVariable => self.fn_set_variable(brick, BitsSize::Eight),
                OpCode::SetVariable2 => self.fn_set_variable(brick, BitsSize::Sixteen),
                OpCode::SetVariable4 => self.fn_set_variable(brick, BitsSize::ThirtyTwo),
                OpCode::JumpIfFalse => self.fn_jump_if_false(brick),
                OpCode::Jump => self.fn_jump(brick),
            }
            self.ip += 1;
        }
    }

    #[inline(always)]
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    #[inline(always)]
    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    #[inline(always)]
    fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack.len() - 1 - distance].clone()
    }

    fn runtime_error(&mut self, error: &str) {
        let source_location = self.get_current_source_location();
        eprintln!("[{}] {}", source_location, error);
    }

    fn get_current_source_location(&self) -> SourceLocation {
        self.brick
            .as_ref()
            .unwrap()
            .get_source_location(self.ip)
            .unwrap()
    }

    #[cfg(test)]
    fn get_output(&self) -> String {
        self.string_buffer.trim().to_string()
    }

    #[cfg(test)]
    fn get_compiler_error(&self) -> String {
        self.compilation_errors.clone()
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.stack.clear();
        self.brick = None;
    }
}

#[cfg(test)]
mod tests;
