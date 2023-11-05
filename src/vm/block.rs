use crate::vm::constants::Constants;

use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

#[repr(u8)]
#[derive(Debug, PartialEq, Primitive)]
pub enum OpCode {
    Return = 0x00,
    Constant = 0x01,
    Constant2 = 0x02,
    Constant4 = 0x03,
}

#[allow(dead_code)]
pub struct Block {
    name: String,
    constants: Constants,
    instructions: Vec<u8>,
    lines: Vec<usize>,
}

impl Block {
    pub fn new(name: &str) -> Self {
        Block {
            name: String::from(name),
            constants: Constants::new(),
            instructions: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_op_code(&mut self, op_code: OpCode, line: usize) {
        self.lines.push(line);
        self.instructions.push(op_code as u8)
    }

    pub fn write_constant(&mut self, value: f64, line: usize) {
        let constant_index = self.constants.push_value(value);

        if constant_index <= 0xFF {
            self.write_op_code(OpCode::Constant, line);
            self.write_u8(constant_index as u8)
        } else if constant_index <= 0xFFFF {
            self.write_op_code(OpCode::Constant2, line);
            self.write_u16(constant_index as u16)
        } else {
            self.write_op_code(OpCode::Constant4, line);
            self.write_u32(constant_index)
        }
    }

    fn write_u8(&mut self, value: u8) {
        self.instructions.push(value)
    }
    fn write_u16(&mut self, value: u16) {
        self.instructions.push((value) as u8);
        self.instructions.push((value >> 8) as u8);
    }
    fn write_u32(&mut self, value: u32) {
        self.instructions.push((value) as u8);
        self.instructions.push((value >> 8) as u8);
        self.instructions.push((value >> 16) as u8);
        self.instructions.push((value >> 24) as u8);
    }
}

#[cfg(feature = "disassemble")]
pub trait BlockDbg {
    fn disassemble_block(&self);
    fn disassemble_instruction(&self, offset: usize) -> usize;
    fn simple_instruction(&self, op_code: OpCode, offset: usize) -> usize;
    fn constant_instruction(&self, op_code: OpCode, offset: usize) -> usize;
}

#[cfg(feature = "disassemble")]
impl BlockDbg for Block {
    fn disassemble_block(&self) {
        println!();
        println!("=== <{}>  ===", self.name);

        let mut offset: usize = 0;
        while offset < self.instructions.len() {
            offset = self.disassemble_instruction(offset);
        }

        println!("=== </{}> ===", self.name);
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04x} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("     | ");
        } else {
            print!("{:6} ", self.lines[offset]);
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
        let constant_index = self.instructions[offset + 1];
        let constant = self.constants.values[constant_index as usize];
        println!("{:?} {:02} '{}'", op_code, constant_index, constant);
        offset + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_block_is_empty() {
        let block = Block::new("origin");

        assert_eq!("origin", block.name);
        assert_eq!(0, block.instructions.len());
        assert_eq!(0, block.constants.values.len());
    }

    #[test]
    fn op_code_can_be_pushed_to_an_block() {
        let mut block = Block::new("jenny");
        block.write_op_code(OpCode::Return, 123);

        assert_eq!(1, block.instructions.len());
        assert_eq!(OpCode::Return as u8, block.instructions[0]);
    }

    #[test]
    fn can_write_more_then_256_constants() {
        let mut block = Block::new("maggie");
        for i in 0..258 {
            block.write_constant(i as f64, i);
        }

        assert_eq!(2 * 256 + 6, block.instructions.len());
        assert_eq!(
            OpCode::Constant2,
            OpCode::from_u8(block.instructions[2 * 256]).unwrap()
        );

        let byte1 = block.instructions[2 * 256 + 1] as u16;
        let byte2 = block.instructions[2 * 256 + 2] as u16;
        let constant_index: u16 = (byte2 << 8) | byte1;
        assert_eq!(256, constant_index);

        assert_eq!(
            OpCode::Constant2,
            OpCode::from_u8(block.instructions[2 * 256 + 3]).unwrap()
        );
        let byte1 = block.instructions[2 * 256 + 4] as u16;
        let byte2 = block.instructions[2 * 256 + 5] as u16;
        let constant_index: u16 = (byte2 << 8) | byte1;
        assert_eq!(257, constant_index);

        assert_eq!(257f64, block.constants.get_value(constant_index as u32));
    }
}
