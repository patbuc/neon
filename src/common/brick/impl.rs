use crate::common::opcodes::OpCode;
use crate::common::{Brick, Constants, Local, SourceLocation, Value};

impl Brick {
    pub(crate) fn new(name: &str) -> Self {
        Brick {
            name: String::from(name),
            constants: Constants::new(),
            strings: Constants::new(),
            instructions: Vec::new(),
            source_locations: Vec::new(),
            locals: Vec::new(),
        }
    }
}

impl Brick {
    pub(crate) fn write_op_code(&mut self, op_code: OpCode, line: u32, column: u32) {
        self.source_locations.push(SourceLocation {
            offset: self.instructions.len(),
            line,
            column,
        });
        self.instructions.push(op_code as u8)
    }

    pub(crate) fn write_op_code_variant(
        &mut self,
        op_code: OpCode,
        index: u32,
        line: u32,
        column: u32,
    ) {
        let offset = if index <= 0xFF {
            0
        } else if index <= 0xFFFF {
            1
        } else {
            2
        };

        // SAFETY: Only use this if the variants are consecutive and valid
        let op_code_variant =
            unsafe { std::mem::transmute::<u8, OpCode>((op_code as u8) + offset) };

        if offset == 0 {
            self.write_op_code(op_code_variant, line, column);
            self.write_u8(index as u8);
        } else if offset == 1 {
            self.write_op_code(op_code_variant, line, column);
            self.write_u16(index as u16);
        } else {
            self.write_op_code(op_code_variant, line, column);
            self.write_u32(index);
        }
    }

    pub(crate) fn add_constant(&mut self, value: Value) -> u32 {
        self.constants.write_value(value)
    }

    pub(crate) fn write_constant(&mut self, value: Value, line: u32, column: u32) -> u32 {
        let constant_index = self.add_constant(value);
        self.write_op_code_variant(OpCode::Constant, constant_index, line, column);
        constant_index
    }

    pub(crate) fn add_parameter(&mut self, local: Local) {
        // Parameters are already on the stack, just register them
        self.locals.push(local);
    }

    pub(crate) fn define_local(&mut self, local: Local, line: u32, column: u32) {
        self.locals.push(local);
        let index = (self.locals.len() - 1) as u32;
        self.write_op_code_variant(OpCode::SetLocal, index, line, column);
    }

    pub(crate) fn write_string(&mut self, value: Value, line: u32, column: u32) -> u32 {
        let string_index = self.strings.write_value(value);
        self.write_op_code_variant(OpCode::String, string_index, line, column);
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

    pub(crate) fn get_local_index(&self, name: &str) -> (Option<u32>, bool) {
        if self.locals.is_empty() {
            return (None, false);
        }

        let mut index = self.locals.len() - 1;
        loop {
            if self.locals[index].name == name {
                let local = &self.locals[index];
                return (Some(index as u32), local.is_mutable);
            }
            if index == 0 {
                break;
            }
            index -= 1;
        }
        (None, false)
    }
}

impl Brick {
    pub(crate) fn get_source_location(&self, offset: usize) -> Option<SourceLocation> {
        let mut result = Option::default();
        let mut low = 0;
        let mut high = self.source_locations.len() - 1;

        if offset >= self.instructions.len() {
            return None;
        }

        while low <= high {
            let mid = (low + high) / 2;
            let line = self.source_locations.get(mid).unwrap();
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

impl Local {
    pub(crate) fn new(name: String, depth: u32, readonly: bool) -> Self {
        Local {
            name,
            depth,
            is_mutable: readonly,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::opcodes::OpCode;
    use crate::common::Brick;
    use crate::{as_number, number};

    #[test]
    fn new_brick_is_empty() {
        let brick = Brick::new("origin");

        assert_eq!("origin", brick.name);
        assert_eq!(0, brick.instructions.len());
        assert_eq!(0, brick.constants.len());
    }

    #[test]
    fn op_code_can_be_pushed_to_an_brick() {
        let mut brick = Brick::new("jenny");
        brick.write_op_code(OpCode::Return, 123, 42);

        assert_eq!(1, brick.instructions.len());
        assert_eq!(OpCode::Return as u8, brick.instructions[0]);
    }

    #[test]
    fn can_write_more_then_256_constants() {
        let mut brick = Brick::new("maggie");
        for i in 0..258 {
            brick.write_constant(number!(i as f64), i, 42);
        }

        assert_eq!(2 * 256 + 6, brick.instructions.len());
        assert_eq!(
            OpCode::Constant2,
            OpCode::from_u8(brick.instructions[2 * 256])
        );

        assert_eq!(256, brick.read_u16(2 * 256 + 1));
        assert_eq!(
            OpCode::Constant2,
            OpCode::from_u8(brick.instructions[2 * 256 + 3])
        );
        let constant_index = brick.read_u16(2 * 256 + 4) as usize;
        assert_eq!(257, constant_index);
        assert_eq!(
            257f64,
            as_number!(brick.constants.read_value(constant_index))
        );
    }

    #[test]
    fn can_write_u8() {
        let mut brick = Brick::new("ruth");
        brick.write_u8(123);
        assert_eq!(123, brick.read_u8(0));
    }

    #[test]
    fn can_write_u16() {
        let mut brick = Brick::new("ruth");
        brick.write_u16(12345);
        assert_eq!(12345, brick.read_u16(0));
    }

    #[test]
    fn can_write_u32() {
        let mut brick = Brick::new("ruth");
        brick.write_u32(12345678);
        assert_eq!(12345678, brick.read_u32(0));
    }

    #[test]
    fn can_write_brick() {
        let mut brick = Brick::new("Zebrick");

        brick.write_constant(number!(1234.56), 2, 0);
        brick.write_op_code(OpCode::Negate, 3, 0);
        brick.write_constant(number!(345.67), 4, 0);
        brick.write_op_code(OpCode::Add, 4, 0);
        brick.write_constant(number!(1.2), 5, 0);
        brick.write_op_code(OpCode::Multiply, 6, 0);
        brick.write_op_code(OpCode::Return, 8, 0);
    }

    #[test]
    fn can_read_line_information() {
        let mut brick = Brick::new("Zebrick");

        brick.write_constant(number!(1234.56), 2, 0);
        brick.write_op_code(OpCode::Negate, 3, 0);
        brick.write_constant(number!(345.67), 4, 0);
        brick.write_op_code(OpCode::Add, 4, 0);
        brick.write_constant(number!(1.2), 5, 0);
        brick.write_op_code(OpCode::Multiply, 6, 0);
        brick.write_op_code(OpCode::Return, 8, 0);

        assert_eq!(2, brick.get_source_location(0).unwrap().line);
        assert_eq!(2, brick.get_source_location(1).unwrap().line);
        assert_eq!(3, brick.get_source_location(2).unwrap().line);
        assert_eq!(4, brick.get_source_location(3).unwrap().line);
        assert_eq!(4, brick.get_source_location(4).unwrap().line);
        assert_eq!(4, brick.get_source_location(5).unwrap().line);
        assert_eq!(5, brick.get_source_location(6).unwrap().line);
        assert_eq!(5, brick.get_source_location(7).unwrap().line);
        assert_eq!(6, brick.get_source_location(8).unwrap().line);
        assert_eq!(8, brick.get_source_location(9).unwrap().line);

        assert!(brick.get_source_location(10).is_none());
    }
}
