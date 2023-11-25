use crate::compiler::Compiler;
use crate::vm::opcodes::OpCode;
use crate::vm::{Block, Result, Value, VirtualMachine};

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: String) -> Result {
        let mut compiler = Compiler::new();
        let block = compiler.compile(source);
        return if let Some(block) = block {
            self.run(block)
        } else {
            Result::CompileError
        };
    }

    #[inline(always)]
    fn run(&mut self, mut block: Block) -> Result {
        loop {
            #[cfg(feature = "disassemble")]
            block.disassemble_instruction(self.ip);
            match OpCode::from_u8(block.read_u8(self.ip)) {
                OpCode::Return => {
                    let value = self.pop();
                    VirtualMachine::print(value);
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

    fn print(value: Value) {
        print!("{}", value);
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_create_vm() {
        let vm = super::VirtualMachine::new();
        assert_eq!(0, vm.ip);
        // assert_eq!(0, vm.block.instructions.len());
        // assert_eq!(0, vm.block.constants.len());
        assert_eq!(0, vm.stack.len());
    }

    #[test]
    fn can_execute_simple_arithmetics() {
        let mut block = super::Block::new("ZeBlock");

        block.write_constant(1.0, 0);
        block.write_constant(2.0, 0);
        block.write_op_code(super::OpCode::Add, 0);
        block.write_constant(3.0, 0);
        block.write_op_code(super::OpCode::Multiply, 0);
        block.write_constant(2.0, 0);
        block.write_op_code(super::OpCode::Subtract, 0);
        block.write_constant(2.0, 0);
        block.write_op_code(super::OpCode::Divide, 0);

        // Pushing throw away value to the stack.
        // This is needed because the Return OpCode will pop a value from the stack and print it.
        block.write_constant(0.0, 0);
        block.write_op_code(super::OpCode::Return, 0);

        let mut vm = super::VirtualMachine {
            ip: 0,
            stack: Vec::new(),
        };

        let result = vm.run(block);
        assert_eq!(super::Result::Ok, result);
        assert_eq!(3.5, vm.pop());
    }
}
