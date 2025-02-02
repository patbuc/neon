use crate::vm::opcodes::OpCode;
use crate::vm::{Block, Constants, Line, Value};

impl Block {
    pub(crate) fn new(name: &str) -> Self {
        Block {
            name: String::from(name),
            constants: Constants::new(),
            globals: Vec::new(),
            strings: Constants::new(),
            instructions: Vec::new(),
            lines: Vec::new(),
        }
    }
}

impl Block {
    pub(crate) fn write_op_code(&mut self, op_code: OpCode, line: u32) {
        self.add_line(self.instructions.len(), line);
        self.instructions.push(op_code as u8)
    }

    pub(crate) fn add_constant(&mut self, value: Value) -> u32 {
        self.constants.write_value(value)
    }

    pub(crate) fn write_constant(&mut self, value: Value, line: u32) -> u32 {
        let constant_index = self.add_constant(value);
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
        constant_index
    }

    pub(crate) fn define_global(&mut self, name: String, line: u32) {
        self.globals.push(name);
        let index = (self.globals.len() - 1) as u32;
        if index <= 0xFF {
            self.write_op_code(OpCode::DefineGlobal, line);
            self.write_u8(index as u8)
        } else if index <= 0xFFFF {
            self.write_op_code(OpCode::DefineGlobal2, line);
            self.write_u16(index as u16)
        } else {
            self.write_op_code(OpCode::DefineGlobal4, line);
            self.write_u32(index)
        }
    }

    pub(crate) fn write_string(&mut self, value: Value, line: u32) -> u32 {
        let string_index = self.strings.write_value(value);
        if string_index <= 0xFF {
            self.write_op_code(OpCode::String, line);
            self.write_u8(string_index as u8)
        } else if string_index <= 0xFFFF {
            self.write_op_code(OpCode::String2, line);
            self.write_u16(string_index as u16)
        } else {
            self.write_op_code(OpCode::String4, line);
            self.write_u32(string_index)
        }
        string_index
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
    pub(in crate::vm) fn read_constant(&self, index: usize) -> Value {
        self.constants.read_value(index)
    }

    #[inline(always)]
    pub(in crate::vm) fn read_string(&self, index: usize) -> Value {
        self.strings.read_value(index)
    }

    #[inline(always)]
    pub(in crate::vm) fn read_global(&self, index: usize) -> String {
        self.globals[index].clone()
    }

    #[inline(always)]
    pub(in crate::vm) fn read_u8(&self, offset: usize) -> u8 {
        self.instructions[offset]
    }

    #[inline(always)]
    pub(in crate::vm) fn read_u16(&self, offset: usize) -> u16 {
        let byte1 = self.instructions[offset] as u16;
        let byte2 = self.instructions[offset + 1] as u16;
        (byte2 << 8) | byte1
    }

    #[inline(always)]
    pub(in crate::vm) fn read_u32(&self, offset: usize) -> u32 {
        let byte1 = self.instructions[offset] as u32;
        let byte2 = self.instructions[offset + 1] as u32;
        let byte3 = self.instructions[offset + 2] as u32;
        let byte4 = self.instructions[offset + 3] as u32;
        (byte4 << 24) | (byte3 << 16) | (byte2 << 8) | byte1
    }

    pub(crate) fn emit_jump(&mut self, op_code: OpCode, line: u32) -> u32 {
        self.write_op_code(op_code, line);
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
}

impl Block {
    fn add_line(&mut self, offset: usize, line: u32) {
        self.lines.push(Line { offset, line });
    }

    pub(in crate::vm) fn get_line(&self, offset: usize) -> Option<Line> {
        let mut result = Option::default();
        let mut low = 0;
        let mut high = self.lines.len() - 1;

        if offset >= self.instructions.len() {
            return None;
        }

        while low <= high {
            let mid = (low + high) / 2;
            let line = self.lines.get(mid).unwrap();
            if line.offset > offset {
                high = mid - 1;
            } else {
                result = Some(line);
                low = mid + 1;
            }
        }
        result.cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::opcodes::OpCode;
    use crate::vm::{Block, Value};

    #[test]
    fn new_block_is_empty() {
        let block = Block::new("origin");

        assert_eq!("origin", block.name);
        assert_eq!(0, block.instructions.len());
        assert_eq!(0, block.constants.len());
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
            block.write_constant(number!(i as f64), i);
        }

        assert_eq!(2 * 256 + 6, block.instructions.len());
        assert_eq!(
            OpCode::Constant2,
            OpCode::from_u8(block.instructions[2 * 256])
        );

        assert_eq!(256, block.read_u16(2 * 256 + 1));
        assert_eq!(
            OpCode::Constant2,
            OpCode::from_u8(block.instructions[2 * 256 + 3])
        );
        let constant_index = block.read_u16(2 * 256 + 4) as usize;
        assert_eq!(257, constant_index);
        assert_eq!(
            257f64,
            as_number!(block.constants.read_value(constant_index))
        );
    }

    #[test]
    fn can_write_u8() {
        let mut block = Block::new("ruth");
        block.write_u8(123);
        assert_eq!(123, block.read_u8(0));
    }

    #[test]
    fn can_write_u16() {
        let mut block = Block::new("ruth");
        block.write_u16(12345);
        assert_eq!(12345, block.read_u16(0));
    }

    #[test]
    fn can_write_u32() {
        let mut block = Block::new("ruth");
        block.write_u32(12345678);
        assert_eq!(12345678, block.read_u32(0));
    }

    #[test]
    fn can_write_block() {
        let mut block = Block::new("ZeBlock");

        block.write_constant(number!(1234.56), 2);
        block.write_op_code(OpCode::Negate, 3);
        block.write_constant(number!(345.67), 4);
        block.write_op_code(OpCode::Add, 4);
        block.write_constant(number!(1.2), 5);
        block.write_op_code(OpCode::Multiply, 6);
        block.write_op_code(OpCode::Return, 8);
    }

    #[test]
    fn can_read_line_information() {
        let mut block = Block::new("ZeBlock");

        block.write_constant(number!(1234.56), 2);
        block.write_op_code(OpCode::Negate, 3);
        block.write_constant(number!(345.67), 4);
        block.write_op_code(OpCode::Add, 4);
        block.write_constant(number!(1.2), 5);
        block.write_op_code(OpCode::Multiply, 6);
        block.write_op_code(OpCode::Return, 8);

        assert_eq!(2, block.get_line(0).unwrap().line);
        assert_eq!(2, block.get_line(1).unwrap().line);
        assert_eq!(3, block.get_line(2).unwrap().line);
        assert_eq!(4, block.get_line(3).unwrap().line);
        assert_eq!(4, block.get_line(4).unwrap().line);
        assert_eq!(4, block.get_line(5).unwrap().line);
        assert_eq!(5, block.get_line(6).unwrap().line);
        assert_eq!(5, block.get_line(7).unwrap().line);
        assert_eq!(6, block.get_line(8).unwrap().line);
        assert_eq!(8, block.get_line(9).unwrap().line);

        assert_eq!(true, block.get_line(10).is_none());
    }
}
