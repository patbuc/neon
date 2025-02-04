mod functions;

use crate::compiler::Compiler;
use crate::vm::opcodes::OpCode;
use crate::vm::virtual_machine::functions::*;
use crate::vm::{BitsSize, Block, Result, Value, VirtualMachine};
use log::info;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            ip: 0,
            stack: Vec::new(),
            block: None,
            globals: HashMap::new(),
            #[cfg(test)]
            string_buffer: String::new(),
        }
    }

    pub fn interpret(&mut self, source: String) -> Result {
        self.reset();

        let start = std::time::Instant::now();
        let compiler = Compiler::new();
        let block = Compiler::compile(Rc::new(RefCell::new(compiler)), source);

        info!("Compile time: {}ms", start.elapsed().as_millis());

        let start = std::time::Instant::now();
        if let None = block {
            return Result::CompileError;
        }

        let block = Rc::new(block.unwrap());
        self.block = Option::from(block.clone());
        let result = self.run(block.as_ref());
        self.block = None;

        info!("Run time: {}ms", start.elapsed().as_millis());
        result
    }

    #[inline(always)]
    fn run(&mut self, block: &Block) -> Result {
        #[cfg(feature = "disassemble")]
        block.disassemble_block();
        loop {
            let op_code = OpCode::from_u8(block.read_u8(self.ip));
            match op_code {
                OpCode::Return => return Result::Ok,
                OpCode::Constant => fn_constant(self, block),
                OpCode::Constant2 => fn_constant2(self, block),
                OpCode::Constant4 => fn_constant4(self, block),
                OpCode::Negate => {
                    if let Some(value) = fn_negate(self) {
                        return value;
                    }
                }
                OpCode::Add => {
                    if let Some(value) = fn_add(self) {
                        return value;
                    }
                }
                OpCode::Subtract => fn_subtract(self),
                OpCode::Multiply => fn_multiply(self),
                OpCode::Divide => fn_divide(self),
                OpCode::Nil => self.push(nil!()),
                OpCode::True => self.push(boolean!(true)),
                OpCode::False => self.push(boolean!(false)),
                OpCode::Equal => fn_equal(self),
                OpCode::Greater => fn_greater(self),
                OpCode::Less => fn_less(self),
                OpCode::Not => fn_not(self),
                OpCode::String => fn_string(self, block),
                OpCode::String2 => fn_string2(self, block),
                OpCode::String4 => fn_string4(self, block),
                OpCode::Print => fn_print(self),
                OpCode::Pop => _ = self.pop(),
                OpCode::DefineGlobal => fn_define_global(self, block, BitsSize::Eight),
                OpCode::DefineGlobal2 => fn_define_global(self, block, BitsSize::Sixteen),
                OpCode::DefineGlobal4 => fn_define_global(self, block, BitsSize::ThirtyTwo),
                OpCode::GetGlobal => fn_get_global(self, block, BitsSize::Eight),
                OpCode::GetGlobal2 => fn_get_global(self, block, BitsSize::Sixteen),
                OpCode::GetGlobal4 => fn_get_global(self, block, BitsSize::ThirtyTwo),
                OpCode::JumpIfFalse => fn_jump_if_false(self, block),
                OpCode::Jump => fn_jump(self, block),
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
        let line = self.get_current_execution_line();
        let offset = self.get_current_execution_offset();
        eprintln!("[line {} char {}] {}", line, offset, error);
    }

    fn get_current_execution_line(&self) -> u32 {
        self.block.as_ref().unwrap().get_line(self.ip).unwrap().line + 1
    }

    fn get_current_execution_offset(&self) -> usize {
        self.block
            .as_ref()
            .unwrap()
            .get_line(self.ip)
            .unwrap()
            .offset
    }

    #[cfg(test)]
    fn get_output(&self) -> String {
        self.string_buffer.trim().to_string()
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.stack.clear();
        self.block = None;
    }
}

#[cfg(test)]
mod tests;
