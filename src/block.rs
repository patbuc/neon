use std::convert::TryFrom;

#[derive(Debug)]
pub enum OpCode {
    Return,
}
impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Return),
            _ => Err(()),
        }
    }
}

#[allow(dead_code)]
pub struct Block {
    name: String,
    op_codes: Vec<u8>,
}

impl Block {
    pub fn new(name: &str) -> Self {
        Block {
            name: String::from(name),
            op_codes: Vec::new(),
        }
    }

    pub fn push_op_code(&mut self, op_code: OpCode) {
        self.op_codes.push(op_code as u8)
    }
}

#[cfg(feature = "disassemble")]
pub trait BlockDbg {
    fn disassemble_block(&self);
    fn disassemble_instruction(&self, offset: usize) -> usize;
    fn simple_instruction(&self, op_code: OpCode, offset: usize) -> usize;
}

#[cfg(feature = "disassemble")]
impl BlockDbg for Block {
    fn disassemble_block(&self) {
        println!();
        println!("== <{}> ==", self.name);

        let mut offset: usize = 0;
        while offset < self.op_codes.len() {
            offset += self.disassemble_instruction(offset);
        }

        println!("== </{}> ==", self.name);
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:02x} ", offset);

        let instruction = OpCode::try_from(self.op_codes[offset]).unwrap();
        return match instruction {
            OpCode::Return => self.simple_instruction(OpCode::Return, offset),
        };
    }

    fn simple_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        println!("{:?}", op_code);
        offset + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_block_is_empty() {
        let block = Block::new("origin");

        assert_eq!("origin", block.name);
        assert_eq!(0, block.op_codes.len());
    }

    #[test]
    fn op_code_can_be_pushed_to_an_block() {
        let mut block = Block::new("jenny");
        block.push_op_code(OpCode::Return);

        assert_eq!(1, block.op_codes.len());
        assert_eq!(OpCode::Return as u8, block.op_codes[0]);
    }
}
