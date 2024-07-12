use crate::compiler::Compiler;
use crate::vm::opcodes::OpCode;
use crate::vm::{Block, Result, Value, VirtualMachine};
use std::rc::Rc;

use log::info;

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            ip: 0,
            stack: Vec::new(),
            block: None,
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
            match OpCode::from_u8(block.read_u8(self.ip)) {
                OpCode::Return => {
                    let value = self.pop();
                    println!("{}", value);
                    return Result::Ok;
                }
                OpCode::Constant => {
                    let constant_index = block.read_u8(self.ip + 1) as usize;
                    let constant = block.read_constant(constant_index);
                    self.push(constant);
                    self.ip += 1;
                }
                OpCode::Constant2 => {
                    let constant_index = block.read_u16(self.ip + 1) as usize;
                    let constant = block.read_constant(constant_index);
                    self.push(constant);
                    self.ip += 2;
                }
                OpCode::Constant4 => {
                    let constant_index = block.read_u32(self.ip + 1) as usize;
                    let constant = block.read_constant(constant_index);
                    self.push(constant);
                    self.ip += 4;
                }
                OpCode::Negate => {
                    if let Value::Number(..) = self.peek(0) {
                        self.runtime_error("Operand must be a number");
                        return Result::RuntimeError;
                    }
                    let value = self.pop();
                    self.push(number!(-as_number!(value)));
                }
                OpCode::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => self.push(Value::Number(a + b)),
                        (Value::String(a), Value::String(b)) => {
                            self.push(Value::String(format!("{a}{b}")))
                        }
                        _ => {
                            self.runtime_error("Operands must be two numbers or two strings");
                            return Result::RuntimeError;
                        }
                    }
                }
                OpCode::Subtract => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Number(as_number!(a) - as_number!(b)));
                }
                OpCode::Multiply => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Number(as_number!(a) * as_number!(b)));
                }
                OpCode::Divide => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::Number(as_number!(a) / as_number!(b)));
                }
                OpCode::Nil => {
                    self.push(nil!());
                }
                OpCode::True => {
                    self.push(boolean!(true));
                }
                OpCode::False => {
                    self.push(boolean!(false));
                }
                OpCode::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(boolean!(a == b));
                }
                OpCode::Greater => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(boolean!(as_number!(a) > as_number!(b)));
                }
                OpCode::Less => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(boolean!(as_number!(a) < as_number!(b)));
                }
                OpCode::Not => {
                    let value = self.pop();
                    self.push(boolean!(is_falsey!(value)));
                }
                OpCode::String => {
                    let string_index = block.read_u8(self.ip + 1) as usize;
                    let string = block.read_string(string_index);
                    self.push(string);
                    self.ip += 1;
                }
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
        eprintln!("[line {}] {}", line, error);
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

        // Pushing throw away value to the stack.
        // This is needed because the Return OpCode will pop a value from the stack and print it.
        block.write_constant(number!(0.0), 0);
        block.write_op_code(super::OpCode::Return, 0);

        let mut vm = super::VirtualMachine {
            ip: 0,
            stack: Vec::new(),
            block: None,
        };

        let result = vm.run(&block);
        assert_eq!(super::Result::Ok, result);
        assert_eq!(3.5, as_number!(vm.pop()));
    }
}
