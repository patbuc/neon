use crate::vm::opcodes::OpCode;
use crate::vm::Block;

#[cfg(feature = "disassemble")]
impl Block {
    #[allow(dead_code)]
    pub(crate) fn disassemble_block(&self) {
        println!();
        println!("=== <{}>  ===", self.name);

        let mut offset: usize = 0;
        while offset < self.instructions.len() {
            offset = self.disassemble_instruction(offset);
        }

        println!("=== </{}> ===", self.name);
    }

    pub(in crate::vm) fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04x} ", offset);

        let line = self.get_line(offset);
        if offset > 0 && line == self.get_line(offset - 1) {
            print!("     | ");
        } else {
            print!("{:6} ", line.unwrap());
        }

        let instruction = OpCode::from_u8(self.instructions[offset]);
        return match instruction {
            OpCode::Return => self.simple_instruction(OpCode::Return, offset),
            OpCode::Constant => self.constant_instruction(instruction, offset),
            OpCode::Constant2 => self.constant_instruction(instruction, offset),
            OpCode::Constant4 => self.constant_instruction(instruction, offset),
            OpCode::Negate => self.simple_instruction(OpCode::Negate, offset),
            OpCode::Add => self.simple_instruction(OpCode::Add, offset),
            OpCode::Subtract => self.simple_instruction(OpCode::Subtract, offset),
            OpCode::Multiply => self.simple_instruction(OpCode::Multiply, offset),
            OpCode::Divide => self.simple_instruction(OpCode::Divide, offset),
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
