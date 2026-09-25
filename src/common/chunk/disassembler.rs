use crate::common::opcodes::OpCode;
use crate::common::Chunk;
use std::fmt::Write;

#[cfg(feature = "disassemble")]
impl Chunk {
    #[allow(dead_code)]
    pub(crate) fn disassemble_chunk(&self) {
        print!("\n{}", self.disassemble());
    }
}

#[cfg(any(test, feature = "disassemble"))]
impl Chunk {
    pub(crate) fn disassemble(&self) -> String {
        let mut out = String::new();
        writeln!(out, "=== <{}>  ===", self.name).unwrap();

        let mut offset: usize = 0;
        while offset < self.instructions.len() {
            offset = self.disassemble_instruction(offset, &mut out);
        }

        writeln!(out, "=== </{}> ===", self.name).unwrap();
        out
    }

    pub(crate) fn disassemble_instruction(&self, offset: usize, out: &mut String) -> usize {
        write!(out, "{:04x} ", offset).unwrap();

        let line = self.get_line_info(offset).unwrap();
        if offset > 0 && line.line == self.get_line_info(offset - 1).unwrap().line {
            write!(out, "     | ").unwrap();
        } else {
            write!(out, "{:6} ", line.line).unwrap();
        }

        let instruction = match OpCode::from_u8(self.instructions[offset]) {
            Some(instruction) => instruction,
            None => {
                writeln!(out, "Unknown opcode {:#04x}", self.instructions[offset]).unwrap();
                return offset + 1;
            }
        };
        match instruction {
            OpCode::Return => self.simple_instruction(OpCode::Return, offset, out),
            OpCode::Constant => self.constant_instruction(offset, out),
            OpCode::Negate => self.simple_instruction(OpCode::Negate, offset, out),
            OpCode::Add => self.simple_instruction(OpCode::Add, offset, out),
            OpCode::Subtract => self.simple_instruction(OpCode::Subtract, offset, out),
            OpCode::Multiply => self.simple_instruction(OpCode::Multiply, offset, out),
            OpCode::Divide => self.simple_instruction(OpCode::Divide, offset, out),
            OpCode::Exponent => self.simple_instruction(OpCode::Exponent, offset, out),
            OpCode::Nil => self.simple_instruction(OpCode::Nil, offset, out),
            OpCode::True => self.simple_instruction(OpCode::True, offset, out),
            OpCode::False => self.simple_instruction(OpCode::False, offset, out),
            OpCode::Equal => self.simple_instruction(OpCode::Equal, offset, out),
            OpCode::Greater => self.simple_instruction(OpCode::Greater, offset, out),
            OpCode::GreaterEqual => self.simple_instruction(OpCode::GreaterEqual, offset, out),
            OpCode::Less => self.simple_instruction(OpCode::Less, offset, out),
            OpCode::LessEqual => self.simple_instruction(OpCode::LessEqual, offset, out),
            OpCode::Not => self.simple_instruction(OpCode::Not, offset, out),
            OpCode::Pop => self.simple_instruction(OpCode::Pop, offset, out),
            OpCode::SetLocal => self.variable_instruction(OpCode::SetLocal, offset, out),
            OpCode::GetLocal => self.variable_instruction(OpCode::GetLocal, offset, out),
            OpCode::GetBuiltin => self.variable_instruction(OpCode::GetBuiltin, offset, out),
            OpCode::GetGlobal => self.variable_instruction(OpCode::GetGlobal, offset, out),
            OpCode::SetGlobal => self.variable_instruction(OpCode::SetGlobal, offset, out),
            OpCode::JumpIfFalse => self.jump_instruction(instruction, offset, out),
            OpCode::Jump => self.jump_instruction(instruction, offset, out),
            OpCode::Loop => self.loop_instruction(offset, out),
            OpCode::Call => self.call_instruction(offset, out),
            OpCode::Invoke => self.invoke_instruction(offset, out),
            OpCode::Modulo => self.simple_instruction(instruction, offset, out),
            OpCode::GetField => self.field_instruction(OpCode::GetField, offset, out),
            OpCode::SetField => self.field_instruction(OpCode::SetField, offset, out),
            OpCode::CreateMap => self.create_map_instruction(offset, out),
            OpCode::CreateArray => self.create_array_instruction(offset, out),
            OpCode::CreateSet => self.create_set_instruction(offset, out),
            OpCode::GetIndex => self.simple_instruction(OpCode::GetIndex, offset, out),
            OpCode::SetIndex => self.simple_instruction(OpCode::SetIndex, offset, out),
            OpCode::GetIterator => self.simple_instruction(OpCode::GetIterator, offset, out),
            OpCode::IteratorNext => self.variable_instruction(OpCode::IteratorNext, offset, out),
            OpCode::IteratorDone => self.variable_instruction(OpCode::IteratorDone, offset, out),
            OpCode::CreateRange => self.create_range_instruction(offset, out),
            OpCode::ToString => self.simple_instruction(OpCode::ToString, offset, out),
            OpCode::BitwiseAnd => self.simple_instruction(OpCode::BitwiseAnd, offset, out),
            OpCode::BitwiseOr => self.simple_instruction(OpCode::BitwiseOr, offset, out),
            OpCode::BitwiseXor => self.simple_instruction(OpCode::BitwiseXor, offset, out),
            OpCode::BitwiseNot => self.simple_instruction(OpCode::BitwiseNot, offset, out),
            OpCode::LeftShift => self.simple_instruction(OpCode::LeftShift, offset, out),
            OpCode::RightShift => self.simple_instruction(OpCode::RightShift, offset, out),
            OpCode::Closure => self.closure_instruction(offset, out),
            OpCode::GetUpvalue => self.variable_instruction(OpCode::GetUpvalue, offset, out),
            OpCode::SetUpvalue => self.variable_instruction(OpCode::SetUpvalue, offset, out),
            OpCode::CloseUpvalue => self.simple_instruction(OpCode::CloseUpvalue, offset, out),
            OpCode::CloseUpvalueInPlace => {
                self.simple_instruction(OpCode::CloseUpvalueInPlace, offset, out)
            }
            OpCode::DefineMethod => self.define_method_instruction(offset, out),
            OpCode::CheckInitialized => {
                self.simple_instruction(OpCode::CheckInitialized, offset, out)
            }
        }
    }

    fn define_method_instruction(&self, offset: usize, out: &mut String) -> usize {
        let type_index = self.read_u16(offset + 1) as usize;
        let method_index = self.read_u16(offset + 3) as usize;
        let takes_self = self.read_u8(offset + 5) != 0;
        let type_name = self.read_constant(type_index);
        let method_name = self.read_constant(method_index);
        let kind = if takes_self { "instance" } else { "static" };
        writeln!(
            out,
            "{:?} {}.{} ({})",
            OpCode::DefineMethod,
            type_name,
            method_name,
            kind
        )
        .unwrap();
        offset + 6
    }

    fn simple_instruction(&self, op_code: OpCode, offset: usize, out: &mut String) -> usize {
        writeln!(out, "{:?}", op_code).unwrap();
        offset + 1
    }

    fn field_instruction(&self, op_code: OpCode, offset: usize, out: &mut String) -> usize {
        let index = self.read_u16(offset + 1) as usize;
        let field_name = self.read_constant(index);
        writeln!(out, "{:?} {:02} '{}'", op_code, index, field_name).unwrap();
        offset + 3
    }

    fn constant_instruction(&self, offset: usize, out: &mut String) -> usize {
        let index = self.read_u16(offset + 1) as usize;
        let constant = self.read_constant(index);
        writeln!(out, "{:?} {:02} '{}'", OpCode::Constant, index, constant).unwrap();
        offset + 3
    }

    fn variable_instruction(&self, op_code: OpCode, offset: usize, out: &mut String) -> usize {
        let index = self.read_u16(offset + 1);
        writeln!(out, "{:?} {:02}", op_code, index).unwrap();
        offset + 3
    }

    fn jump_instruction(&self, op_code: OpCode, offset: usize, out: &mut String) -> usize {
        let jump = self.read_u32(offset + 1);
        writeln!(
            out,
            "{:?} {:04x} -> {:04x}",
            op_code,
            offset,
            offset + 5 + jump as usize
        )
        .unwrap();
        offset + 5
    }

    fn loop_instruction(&self, offset: usize, out: &mut String) -> usize {
        let jump = self.read_u32(offset + 1);
        writeln!(
            out,
            "{:?} {:04x} -> {:04x}",
            OpCode::Loop,
            offset,
            offset + 5 - jump as usize
        )
        .unwrap();
        offset + 5
    }

    fn call_instruction(&self, offset: usize, out: &mut String) -> usize {
        let arg_count = self.read_u8(offset + 1);
        writeln!(out, "Call (args: {})", arg_count).unwrap();
        offset + 2
    }

    fn invoke_instruction(&self, offset: usize, out: &mut String) -> usize {
        let name_index = self.read_u16(offset + 1) as usize;
        let name = self.read_constant(name_index);
        let arg_count = self.read_u8(offset + 3);
        writeln!(out, "Invoke {} (args: {})", name, arg_count).unwrap();
        offset + 4
    }

    fn create_map_instruction(&self, offset: usize, out: &mut String) -> usize {
        let entry_count = self.read_u16(offset + 1);
        writeln!(out, "CreateMap (entries: {})", entry_count).unwrap();
        offset + 3
    }

    fn create_array_instruction(&self, offset: usize, out: &mut String) -> usize {
        let element_count = self.read_u16(offset + 1);
        writeln!(out, "CreateArray (elements: {})", element_count).unwrap();
        offset + 3
    }

    fn create_set_instruction(&self, offset: usize, out: &mut String) -> usize {
        let element_count = self.read_u16(offset + 1);
        writeln!(out, "CreateSet (elements: {})", element_count).unwrap();
        offset + 3
    }

    fn closure_instruction(&self, offset: usize, out: &mut String) -> usize {
        let index = self.read_u16(offset + 1) as usize;
        let function = self.read_constant(index);
        writeln!(out, "{:?} {:02} '{}'", OpCode::Closure, index, function).unwrap();

        let mut cursor = offset + 1 + 2;
        let upvalue_count = self.read_u8(cursor) as usize;
        cursor += 1;
        for _ in 0..upvalue_count {
            let is_local = self.read_u8(cursor) != 0;
            let upvalue_index = self.read_u16(cursor + 1);
            writeln!(
                out,
                "      |                     {} {:02}",
                if is_local { "local" } else { "upvalue" },
                upvalue_index
            )
            .unwrap();
            cursor += 3;
        }
        cursor
    }

    fn create_range_instruction(&self, offset: usize, out: &mut String) -> usize {
        let inclusive = self.read_u8(offset + 1);
        writeln!(out, "CreateRange (inclusive: {})", inclusive != 0).unwrap();
        offset + 2
    }
}
