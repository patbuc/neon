use crate::vm::block::{Block, OpCode};
use num_traits::FromPrimitive;

pub(in crate::vm) type Value = f64;

pub struct VirtualMachine {
    ip: usize,
    block: Block,
    stack: Vec<Value>,
}

pub enum Result {
    Ok,
    // CompileError,
    // RuntimeError,
}

impl VirtualMachine {
    pub fn new(block: Block) -> Self {
        VirtualMachine {
            ip: 0,
            block,
            stack: Vec::new(),
        }
    }
    pub fn interpret(&mut self) -> Result {
        return self.run();
    }

    #[inline(always)]
    fn run(&mut self) -> Result {
        loop {
            #[cfg(feature = "disassemble")]
            self.block.disassemble_instruction(self.ip);
            match OpCode::from_u8(self.block.read_u8(self.ip)).unwrap() {
                OpCode::Return => return Result::Ok,
                OpCode::Constant => {
                    let constant_index = self.block.read_u8(self.ip + 1) as usize;
                    let constant = self.block.read_constant(constant_index);
                    println!("{}", constant);
                    self.ip += 1;
                }
                OpCode::Constant2 => {
                    let constant_index = self.block.read_u16(self.ip + 1) as usize;
                    let constant = self.block.read_constant(constant_index);
                    println!("{}", constant);
                    self.ip += 2;
                }
                OpCode::Constant4 => {
                    let constant_index = self.block.read_u32(self.ip + 1) as usize;
                    let constant = self.block.read_constant(constant_index);
                    println!("{}", constant);
                    self.ip += 4;
                }
            }
            self.ip += 1;
        }
    }
}
