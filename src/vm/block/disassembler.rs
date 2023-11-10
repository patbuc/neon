use crate::vm::block::{Block, OpCode};
use num_traits::FromPrimitive;

pub trait Disassembler {
    fn disassemble_block(&self);
    fn disassemble_instruction(&self, offset: usize, line: usize) -> usize;
    fn simple_instruction(&self, op_code: OpCode, offset: usize) -> usize;
    fn constant_instruction(&self, op_code: OpCode, offset: usize) -> usize;
}

impl Disassembler for Block {
    fn disassemble_block(&self) {
        println!();
        println!("=== <{}>  ===", self.name);

        let mut offset: usize = 0;
        let mut line: usize = 0;
        while offset < self.instructions.len() {
            offset = self.disassemble_instruction(offset, line);
            line += 1;
        }

        println!("=== </{}> ===", self.name);
    }

    fn disassemble_instruction(&self, offset: usize, line: usize) -> usize {
        print!("{:04x} ", offset);

        if line > 0 && self.lines[line] == self.lines[line - 1] {
            print!("     | ");
        } else {
            print!("{:6} ", self.lines[line]);
        }

        let instruction = OpCode::from_u8(self.instructions[offset]).unwrap();
        return match instruction {
            OpCode::Return => self.simple_instruction(OpCode::Return, offset),
            OpCode::Constant => self.constant_instruction(instruction, offset),
            OpCode::Constant2 => self.constant_instruction(instruction, offset),
            OpCode::Constant4 => self.constant_instruction(instruction, offset),
        };
    }

    fn simple_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        println!("{:?}", op_code);
        offset + 1
    }

    fn constant_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        fn get_constant_index(block: &Block, op_code: &OpCode, offset: usize) -> (usize, usize) {
            match op_code {
                OpCode::Constant => (block.read_u8(offset) as usize, 1),
                OpCode::Constant2 => (block.read_u16(offset) as usize, 2),
                OpCode::Constant4 => (block.read_u32(offset) as usize, 4),
                _ => panic!("Invalid OpCode"),
            }
        }

        let (index, offset_shift) = get_constant_index(self, &op_code, offset + 1);
        let constant = self.constants.read_value(index);
        println!("{:?} {:02} '{}'", op_code, index, constant);
        offset + 1 + offset_shift
    }
}
