mod functions;

use crate::compiler::Compiler;
use crate::vm::opcodes::OpCode;
use crate::vm::utils::output_handler::ConsoleOutputHandler;
use crate::vm::{Block, Result, Value, VirtualMachine};
use crate::vm::{BitsSize, Block, Result, Value, VirtualMachine};
use std::collections::HashMap;
use std::rc::Rc;

use crate::vm::virtual_machine::functions::*;
use log::info;

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            ip: 0,
            stack: Vec::new(),
            block: None,
            globals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, source: String) -> Result {
        self.reset();

        let start = std::time::Instant::now();
        let mut compiler = Compiler::new();
        let block = compiler.compile(source);

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

    fn reset(&mut self) {
        self.ip = 0;
        self.stack.clear();
        self.block = None;
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::Value;

    #[test]
    fn can_create_vm() {
        let vm = super::VirtualMachine::new();
        assert_eq!(0, vm.ip);
        assert_eq!(0, vm.stack.len());
    }

    #[test]
    fn can_execute_simple_arithmetics() {
        let mut block = super::Block::new("ZeBlock");

        block.write_constant(number!(1.0), 0);
        block.write_constant(number!(2.0), 0);
        block.write_op_code(super::OpCode::Add, 0);
        block.write_constant(number!(3.0), 0);
        block.write_op_code(super::OpCode::Multiply, 0);
        block.write_constant(number!(2.0), 0);
        block.write_op_code(super::OpCode::Subtract, 0);
        block.write_constant(number!(2.0), 0);
        block.write_op_code(super::OpCode::Divide, 0);
        block.write_op_code(super::OpCode::Return, 0);

        let mut vm = super::VirtualMachine::new();

        let result = vm.run(&block);
        assert_eq!(super::Result::Ok, result);
        assert_eq!(3.5, as_number!(vm.pop()));
    }

    #[test]
    fn can_print_hello_world() {
        let program = r#"
        print "Hello World 🌍"
        "#;

        let mut vm = super::VirtualMachine::new();
        let result = vm.interpret(program.to_string());

        assert_eq!(super::Result::Ok, result);
    }

    #[test]
    fn can_print_the_answer_to_everything_times_pi() {
        let program = r#"
        print 42 * 3.14
        "#;

        let mut vm = super::VirtualMachine::new();
        let result = vm.interpret(program.to_string());

        assert_eq!(super::Result::Ok, result);
    }

    #[test]
    fn can_run_multi_line_statements() {
        let program = r#"
        print "Hello World 🌎"
        print 13
        "#;

        let mut vm = super::VirtualMachine::new();
        let result = vm.interpret(program.to_string());

        assert_eq!(super::Result::Ok, result);
    }
}
