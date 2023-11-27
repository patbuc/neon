use crate::compiler::Compiler;
use crate::vm::opcodes::OpCode;
use crate::vm::{Block, Result, Value, ValueType, VirtualMachine};
use tracing::error;

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: String) -> Result {
        self.reset();

        let mut compiler = Compiler::new();
        let block = compiler.compile(source);
        return if let Some(block) = block {
            self.run(block)
        } else {
            Result::CompileError
        };
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.stack.clear();
    }

    #[inline(always)]
    fn run(&mut self, mut block: Block) -> Result {
        loop {
            // #[cfg(feature = "disassemble")]
            // block.disassemble_instruction(self.ip);
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
                    if self.peek(0).value_type != ValueType::Number {
                        self.runtime_error("Operand must be a number", block);
                        return Result::RuntimeError;
                    }
                    let value = self.pop();
                    unsafe {
                        self.push(Value::from_number(-value.value.number));
                    }
                }
                OpCode::Add => self.addition(),
                OpCode::Subtract => self.subtraction(),
                OpCode::Multiply => self.multiplication(),
                OpCode::Divide => self.division(),
                OpCode::Nil => {
                    self.push(Value::nil());
                }
                OpCode::True => {
                    self.push(Value::from_bool(true));
                }
                OpCode::False => {
                    self.push(Value::from_bool(false));
                }
            }
            self.ip += 1;
        }
    }

    fn addition(&mut self) {
        let b = self.pop();
        let a = self.pop();
        unsafe {
            self.push(Value::from_number(a.value.number + b.value.number));
        }
    }

    fn subtraction(&mut self) {
        let b = self.pop();
        let a = self.pop();
        unsafe {
            self.push(Value::from_number(a.value.number - b.value.number));
        }
    }

    fn multiplication(&mut self) {
        let b = self.pop();
        let a = self.pop();
        unsafe {
            self.push(Value::from_number(a.value.number * b.value.number));
        }
    }

    fn division(&mut self) {
        let b = self.pop();
        let a = self.pop();
        unsafe {
            self.push(Value::from_number(a.value.number / b.value.number));
        }
    }

    fn print(value: Value) {
        pub(crate) fn print_nil() {
            print!("nil")
        }
        pub(crate) fn print_string(value: Value) {
            print!("{}", unsafe { &*value.value.string })
        }
        pub(crate) fn print_bool(value: Value) {
            print!("{}", unsafe { value.value.boolean });
        }
        pub(crate) fn print_number(value: Value) {
            print!("{}", unsafe { value.value.number });
        }

        match value.value_type {
            ValueType::Number => print_number(value),
            ValueType::Bool => print_bool(value),
            ValueType::String => print_string(value),
            ValueType::Nil => print_nil(),
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn peek(&mut self, distance: usize) -> Value {
        self.stack[self.stack.len() - 1 - distance]
    }
    fn runtime_error(&mut self, error: &str, block: Block) {
        eprint!("{} ", error);
        let line = block.get_line(self.ip).unwrap();
        eprintln!("[line {}] in script", line);
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

        block.write_constant(Value::from_number(1.0), 0);
        block.write_constant(Value::from_number(2.0), 0);
        block.write_op_code(super::OpCode::Add, 0);
        block.write_constant(Value::from_number(3.0), 0);
        block.write_op_code(super::OpCode::Multiply, 0);
        block.write_constant(Value::from_number(2.0), 0);
        block.write_op_code(super::OpCode::Subtract, 0);
        block.write_constant(Value::from_number(2.0), 0);
        block.write_op_code(super::OpCode::Divide, 0);

        // Pushing throw away value to the stack.
        // This is needed because the Return OpCode will pop a value from the stack and print it.
        block.write_constant(Value::from_number(0.0), 0);
        block.write_op_code(super::OpCode::Return, 0);

        let mut vm = super::VirtualMachine {
            ip: 0,
            stack: Vec::new(),
        };

        let result = vm.run(block);
        assert_eq!(super::Result::Ok, result);
        unsafe {
            assert_eq!(3.5, vm.pop().value.number);
        }
    }
}
