use crate::vm::block::{Block, OpCode};
use crate::vm::compiler::Compiler;
use num_traits::FromPrimitive;

pub(in crate::vm) type Value = f64;

pub struct VirtualMachine {
    ip: usize,
    block: Block,
    stack: Vec<Value>,
}

pub enum Result {
    Ok,
    CompileError,
    RuntimeError,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            ip: 0,
            block: Block::new("ZeBlock"),
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: String) -> Result {
        let compiler = Compiler {};
        compiler.compile(source);
        return Result::Ok;
    }

    #[inline(always)]
    fn run(&mut self) -> Result {
        loop {
            #[cfg(feature = "disassemble")]
            self.block.disassemble_instruction(self.ip);
            match OpCode::from_u8(self.block.read_u8(self.ip)).unwrap() {
                OpCode::Return => {
                    let value = self.pop();
                    VirtualMachine::print_value(value);
                    return Result::Ok;
                }
                OpCode::Constant => {
                    let constant_index = self.block.read_u8(self.ip + 1) as usize;
                    let constant = self.block.read_constant(constant_index);
                    self.push(constant);
                    self.ip += 1;
                }
                OpCode::Constant2 => {
                    let constant_index = self.block.read_u16(self.ip + 1) as usize;
                    let constant = self.block.read_constant(constant_index);
                    self.push(constant);
                    self.ip += 2;
                }
                OpCode::Constant4 => {
                    let constant_index = self.block.read_u32(self.ip + 1) as usize;
                    let constant = self.block.read_constant(constant_index);
                    self.push(constant);
                    self.ip += 4;
                }
                OpCode::Negate => {
                    let value = self.pop();
                    self.push(-value);
                }
                OpCode::Add => self.addition(),
                OpCode::Subtract => self.subtraction(),
                OpCode::Multiply => self.multiplication(),
                OpCode::Divide => self.division(),
            }
            self.ip += 1;
        }
    }

    fn addition(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(a + b);
    }

    fn subtraction(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(a - b);
    }

    fn multiplication(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(a * b);
    }

    fn division(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(a / b);
    }

    fn print_value(value: Value) {
        print!("{}", value);
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}
