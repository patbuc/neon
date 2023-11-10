use crate::vm::block::{Block, OpCode};
use num_traits::FromPrimitive;

pub struct VirtualMachine {
    block: Block,
    ip: usize,
}

pub enum Result {
    Ok,
    // CompileError,
    // RuntimeError,
}

impl VirtualMachine {
    pub fn new(block: Block) -> Self {
        VirtualMachine { block, ip: 0 }
    }
    pub fn interpret(&mut self) -> Result {
        return self.run();
    }

    #[inline(always)]
    fn run(&mut self) -> Result {
        loop {
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
