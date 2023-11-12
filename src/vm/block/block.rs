use crate::vm::block::opcodes::OpCode;
use crate::vm::block::{Block, Constants};

impl Block {
    pub fn new(name: &str) -> Self {
        Block {
            name: String::from(name),
            constants: Constants::new(),
            instructions: Vec::new(),
            lines: Vec::new(),
        }
    }
}

impl Block {
    pub fn write_op_code(&mut self, op_code: OpCode, line: usize) {
        self.add_line(self.instructions.len(), line);
        self.instructions.push(op_code as u8)
    }

    pub fn write_constant(&mut self, value: f64, line: usize) {
        let constant_index = self.constants.write_value(value);

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

    pub(super) fn write_u8(&mut self, value: u8) {
        self.instructions.push(value)
    }
    pub(super) fn write_u16(&mut self, value: u16) {
        self.instructions.push((value) as u8);
        self.instructions.push((value >> 8) as u8);
    }
    pub(super) fn write_u32(&mut self, value: u32) {
        self.instructions.push((value) as u8);
        self.instructions.push((value >> 8) as u8);
        self.instructions.push((value >> 16) as u8);
        self.instructions.push((value >> 24) as u8);
    }

    pub fn read_constant(&mut self, index: usize) -> f64 {
        self.constants.read_value(index)
    }

    pub fn read_u8(&self, offset: usize) -> u8 {
        self.instructions[offset]
    }

    pub fn read_u16(&self, offset: usize) -> u16 {
        let byte1 = self.instructions[offset] as u16;
        let byte2 = self.instructions[offset + 1] as u16;
        (byte2 << 8) | byte1
    }

    pub fn read_u32(&self, offset: usize) -> u32 {
        let byte1 = self.instructions[offset] as u32;
        let byte2 = self.instructions[offset + 1] as u32;
        let byte3 = self.instructions[offset + 2] as u32;
        let byte4 = self.instructions[offset + 3] as u32;
        (byte4 << 24) | (byte3 << 16) | (byte2 << 8) | byte1
    }
}
