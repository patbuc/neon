use crate::vm::opcodes::OpCode;
use crate::vm::Block;
use crate::vm::Value;

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

        let line = self.get_line(offset).unwrap();
        if offset > 0 && line.line == self.get_line(offset - 1).unwrap().line {
            print!("     | ");
        } else {
            print!("{:6} ", line.line);
        }

        let instruction = OpCode::from_u8(self.instructions[offset]);
        match instruction {
            OpCode::Return => self.simple_instruction(OpCode::Return, offset),
            OpCode::Constant => self.constant_instruction(instruction, offset),
            OpCode::Constant2 => self.constant_instruction(instruction, offset),
            OpCode::Constant4 => self.constant_instruction(instruction, offset),
            OpCode::Negate => self.simple_instruction(OpCode::Negate, offset),
            OpCode::Add => self.simple_instruction(OpCode::Add, offset),
            OpCode::Subtract => self.simple_instruction(OpCode::Subtract, offset),
            OpCode::Multiply => self.simple_instruction(OpCode::Multiply, offset),
            OpCode::Divide => self.simple_instruction(OpCode::Divide, offset),
            OpCode::Nil => self.simple_instruction(OpCode::Nil, offset),
            OpCode::True => self.simple_instruction(OpCode::True, offset),
            OpCode::False => self.simple_instruction(OpCode::False, offset),
            OpCode::Equal => self.simple_instruction(OpCode::Equal, offset),
            OpCode::Greater => self.simple_instruction(OpCode::Greater, offset),
            OpCode::Less => self.simple_instruction(OpCode::Less, offset),
            OpCode::Not => self.simple_instruction(OpCode::Not, offset),
            OpCode::String => self.string_instruction(instruction, offset),
            OpCode::String2 => self.string_instruction(instruction, offset),
            OpCode::String4 => self.string_instruction(instruction, offset),
            OpCode::Print => self.simple_instruction(OpCode::Print, offset),
            OpCode::Pop => self.simple_instruction(OpCode::Pop, offset),
            OpCode::DefineGlobal => self.variable_instruction(OpCode::DefineGlobal, offset),
            OpCode::DefineGlobal2 => self.variable_instruction(OpCode::DefineGlobal2, offset),
            OpCode::DefineGlobal4 => self.variable_instruction(OpCode::DefineGlobal4, offset),
            OpCode::GetGlobal => self.variable_instruction(OpCode::GetGlobal, offset),
            OpCode::GetGlobal2 => self.variable_instruction(OpCode::GetGlobal2, offset),
            OpCode::GetGlobal4 => self.variable_instruction(OpCode::GetGlobal4, offset),
            OpCode::JumpIfFalse => self.jump_instruction(instruction, offset),
            OpCode::Jump => self.jump_instruction(instruction, offset),
        }
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
        let constant = self.read_constant(index);
        println!("{:?} {:02} '{}'", op_code, index, constant.to_string());
        offset + 1 + offset_shift
    }

    fn variable_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        fn get_variable_index(block: &Block, op_code: &OpCode, offset: usize) -> (usize, usize) {
            match op_code {
                OpCode::DefineGlobal => (block.read_u8(offset) as usize, 1),
                OpCode::DefineGlobal2 => (block.read_u16(offset) as usize, 2),
                OpCode::DefineGlobal4 => (block.read_u32(offset) as usize, 4),
                OpCode::GetGlobal => (block.read_u8(offset) as usize, 1),
                OpCode::GetGlobal2 => (block.read_u16(offset) as usize, 2),
                OpCode::GetGlobal4 => (block.read_u32(offset) as usize, 4),
                _ => panic!("Invalid OpCode"),
            }
        }

        let (index, offset_shift) = get_variable_index(self, &op_code, offset + 1);
        let constant = self.read_constant(index);
        println!("{:?} {:02} '{}'", op_code, index, constant.to_string());
        offset + 1 + offset_shift
    }

    fn jump_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        let jump = self.read_u32(offset + 1);
        println!(
            "{:?} {:04x} -> {:04x}",
            op_code,
            offset,
            offset + 5 + jump as usize
        );
        offset + 5
    }

    fn string_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        let index = self.read_u8(offset + 1) as usize;
        let string = self.read_string(index);
        println!("{:?} {:02} '{}'", op_code, index, string);
        offset + 2
    }
}
