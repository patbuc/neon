mod functions;

use crate::compiler::Compiler;
use crate::vm::opcodes::OpCode;
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
                OpCode::Constant => self.fn_constant(block),
                OpCode::Constant2 => self.fn_constant2(block),
                OpCode::Constant4 => self.fn_constant4(block),
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
                OpCode::String => self.fn_string(block),
                OpCode::String2 => self.fn_string2(block),
                OpCode::String4 => self.fn_string4(block),
                OpCode::Print => self.fn_print(),
                OpCode::Pop => _ = self.pop(),
                OpCode::DefineGlobal => self.fn_define_global(block, BitsSize::Eight),
                OpCode::DefineGlobal2 => self.fn_define_global(block, BitsSize::Sixteen),
                OpCode::DefineGlobal4 => self.fn_define_global(block, BitsSize::ThirtyTwo),
                OpCode::GetGlobal => self.fn_get_global(block, BitsSize::Eight),
                OpCode::GetGlobal2 => self.fn_get_global(block, BitsSize::Sixteen),
                OpCode::GetGlobal4 => self.fn_get_global(block, BitsSize::ThirtyTwo),
                OpCode::JumpIfFalse => self.fn_jump_if_false(block),
                OpCode::Jump => self.fn_jump(block),
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
        eprintln!("[line {} char {}] {}", line.0, line.1, error);
    }

    fn get_current_execution_line(&self) -> (u32, u32) {
        let line = self.block.as_ref().unwrap().get_line(self.ip).unwrap();
        (line.line, line.char)
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
