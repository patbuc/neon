use crate::common::opcodes::OpCode;
use crate::common::{Chunk, Constants, LineInfo, Value};

impl Chunk {
    pub(crate) fn new(name: &str) -> Self {
        Chunk {
            name: String::from(name),
            constants: Constants::new(),
            strings: Constants::new(),
            instructions: Vec::new(),
            line_infos: Vec::new(),
        }
    }
}

impl Chunk {
    pub(crate) fn write_op_code(&mut self, op_code: OpCode, line: u32, column: u32) {
        self.line_infos.push(LineInfo {
            ip: self.instructions.len(),
            line,
            column,
        });
        self.instructions.push(op_code as u8)
    }

    pub(crate) fn add_constant(&mut self, value: Value) -> u32 {
        self.constants.write_value(value)
    }

    pub(crate) fn add_string(&mut self, value: Value) -> u32 {
        self.strings.write_value(value)
    }

    #[cfg(test)]
    pub(crate) fn write_indexed(&mut self, op_code: OpCode, index: u32, line: u32, column: u32) {
        self.write_op_code(op_code, line, column);
        self.write_u16(u16::try_from(index).expect("index fits in u16"));
    }

    #[cfg(test)]
    pub(crate) fn write_constant(&mut self, value: Value, line: u32, column: u32) -> u32 {
        let constant_index = self.add_constant(value);
        self.write_indexed(OpCode::Constant, constant_index, line, column);
        constant_index
    }

    pub(crate) fn write_u8(&mut self, value: u8) {
        self.instructions.push(value)
    }
    pub(crate) fn write_u16(&mut self, value: u16) {
        self.instructions.push((value) as u8);
        self.instructions.push((value >> 8) as u8);
    }
    pub(crate) fn write_u32(&mut self, value: u32) {
        self.instructions.push((value) as u8);
        self.instructions.push((value >> 8) as u8);
        self.instructions.push((value >> 16) as u8);
        self.instructions.push((value >> 24) as u8);
    }

    #[inline(always)]
    pub(crate) fn read_constant(&self, index: usize) -> Value {
        self.constants.read_value(index)
    }

    #[inline(always)]
    pub(crate) fn read_string(&self, index: usize) -> Value {
        self.strings.read_value(index)
    }

    #[inline(always)]
    pub(crate) fn read_u8(&self, offset: usize) -> u8 {
        self.instructions[offset]
    }

    #[inline(always)]
    pub(crate) fn read_u16(&self, offset: usize) -> u16 {
        let byte1 = self.instructions[offset] as u16;
        let byte2 = self.instructions[offset + 1] as u16;
        (byte2 << 8) | byte1
    }

    #[inline(always)]
    pub(crate) fn read_u32(&self, offset: usize) -> u32 {
        let byte1 = self.instructions[offset] as u32;
        let byte2 = self.instructions[offset + 1] as u32;
        let byte3 = self.instructions[offset + 2] as u32;
        let byte4 = self.instructions[offset + 3] as u32;
        (byte4 << 24) | (byte3 << 16) | (byte2 << 8) | byte1
    }

    pub(crate) fn emit_jump(&mut self, op_code: OpCode, line: u32, column: u32) -> u32 {
        self.write_op_code(op_code, line, column);
        self.write_u32(0xFFFF_FFFF);
        self.instructions.len() as u32 - 4
    }

    pub(crate) fn patch_jump(&mut self, offset: u32) {
        let jump = self.instructions.len() as u32 - offset - 4;
        let offset = offset as usize;
        self.instructions[offset] = jump as u8;
        self.instructions[offset + 1] = (jump >> 8) as u8;
        self.instructions[offset + 2] = (jump >> 16) as u8;
        self.instructions[offset + 3] = (jump >> 24) as u8;
    }

    pub(crate) fn emit_loop(&mut self, loop_start: u32, line: u32, column: u32) {
        self.write_op_code(OpCode::Loop, line, column);
        let offset = self.instructions.len() as u32 - loop_start + 4;
        self.write_u32(offset);
    }

    pub(crate) fn instruction_count(&self) -> usize {
        self.instructions.len()
    }
}

impl Chunk {
    pub(crate) fn get_line_info(&self, ip: usize) -> Option<LineInfo> {
        if ip >= self.instructions.len() {
            return None;
        }

        // partition_point requires entries sorted by ip; they are pushed in increasing order.
        let index = self.line_infos.partition_point(|info| info.ip <= ip);
        index
            .checked_sub(1)
            .and_then(|i| self.line_infos.get(i))
            .cloned()
    }
}
