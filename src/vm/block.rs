use crate::vm::constants::Constants;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum OpCode {
    Constant,
    Return,
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Constant),
            1 => Ok(OpCode::Return),
            _ => Err(()),
        }
    }
}

#[allow(dead_code)]
pub struct Block {
    name: String,
    instructions: Vec<u8>,
    constants: Constants,
}

impl Block {
    pub fn new(name: &str) -> Self {
        Block {
            name: String::from(name),
            instructions: Vec::new(),
            constants: Constants::new(),
        }
    }

    pub fn push_op_code(&mut self, op_code: OpCode) {
        self.instructions.push(op_code as u8)
    }

    pub fn push_constant(&mut self, value: f64) -> i8 {
        self.constants.push_value(value)
    }

    pub(crate) fn write_byte(&mut self, byte: i8) {
        self.instructions.push(byte as u8)
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
            offset += self.disassemble_instruction(offset);
        }

        println!("=== </{}> ===", self.name);
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04x} ", offset);

        let instruction = OpCode::try_from(self.instructions[offset]).unwrap();
        return match instruction {
            OpCode::Constant => self.constant_instruction(OpCode::Constant, offset),
            OpCode::Return => self.simple_instruction(OpCode::Return, offset),
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
        block.push_op_code(OpCode::Return);

        assert_eq!(1, block.instructions.len());
        assert_eq!(OpCode::Return as u8, block.instructions[0]);
    }
}
