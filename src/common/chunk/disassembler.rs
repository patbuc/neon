use crate::common::opcodes::OpCode;
use crate::common::Chunk;
use std::fmt::Write;

#[cfg(feature = "disassemble")]
impl Chunk {
    #[allow(dead_code)]
    #[allow(clippy::print_stdout)]
    pub(crate) fn disassemble_chunk(&self) {
        print!("\n{}", self.disassemble());
    }
}

#[cfg(any(test, feature = "disassemble"))]
impl Chunk {
    pub(crate) fn disassemble(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "=== <{}>  ===", self.name);

        let mut offset: usize = 0;
        while offset < self.instructions.len() {
            offset = self.disassemble_instruction(offset, &mut out);
        }

        let _ = writeln!(out, "=== </{}> ===", self.name);
        out
    }

    #[allow(clippy::expect_used)]
    pub(crate) fn disassemble_instruction(&self, offset: usize, out: &mut String) -> usize {
        let _ = write!(out, "{:04x} ", offset);

        let line = self
            .get_line_info(offset)
            .expect("offset is the start of an instruction, so the chunk's line table covers it");
        if offset > 0
            && line.line
                == self
                    .get_line_info(offset - 1)
                    .expect("a preceding offset is covered by the line table too")
                    .line
        {
            let _ = write!(out, "     | ");
        } else {
            let _ = write!(out, "{:6} ", line.line);
        }

        let instruction = match OpCode::from_u8(self.instructions[offset]) {
            Some(instruction) => instruction,
            None => {
                let _ = writeln!(out, "Unknown opcode {:#04x}", self.instructions[offset]);
                return offset + 1;
            }
        };
        match instruction {
            OpCode::Return => self.simple_instruction(OpCode::Return, offset, out),
            OpCode::Constant => self.constant_instruction(OpCode::Constant, offset, out),
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
            OpCode::GetLocalField => {
                self.local_field_instruction(OpCode::GetLocalField, offset, out)
            }
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
            OpCode::StoreLocal => self.variable_instruction(OpCode::StoreLocal, offset, out),
            OpCode::StoreField => self.field_instruction(OpCode::StoreField, offset, out),
            OpCode::StoreLocalField => {
                self.local_field_instruction(OpCode::StoreLocalField, offset, out)
            }
            OpCode::AddConstant => self.constant_instruction(OpCode::AddConstant, offset, out),
            OpCode::SubtractConstant => {
                self.constant_instruction(OpCode::SubtractConstant, offset, out)
            }
            OpCode::GreaterConstant => {
                self.constant_instruction(OpCode::GreaterConstant, offset, out)
            }
            OpCode::GreaterEqualConstant => {
                self.constant_instruction(OpCode::GreaterEqualConstant, offset, out)
            }
            OpCode::LessConstant => self.constant_instruction(OpCode::LessConstant, offset, out),
            OpCode::LessEqualConstant => {
                self.constant_instruction(OpCode::LessEqualConstant, offset, out)
            }
        }
    }

    fn define_method_instruction(&self, offset: usize, out: &mut String) -> usize {
        let type_symbol = self.read_u16(offset + 1) as usize;
        let method_symbol = self.read_u16(offset + 3) as usize;
        let takes_self = self.read_u8(offset + 5) != 0;
        let type_name = &self.symbols[type_symbol];
        let method_name = &self.symbols[method_symbol];
        let kind = if takes_self { "instance" } else { "static" };
        let _ = writeln!(
            out,
            "{:?} {}.{} ({})",
            OpCode::DefineMethod,
            type_name,
            method_name,
            kind
        );
        offset + 6
    }

    fn simple_instruction(&self, op_code: OpCode, offset: usize, out: &mut String) -> usize {
        let _ = writeln!(out, "{:?}", op_code);
        offset + 1
    }

    fn field_instruction(&self, op_code: OpCode, offset: usize, out: &mut String) -> usize {
        let symbol = self.read_u16(offset + 1) as usize;
        let name = &self.symbols[symbol];
        let _ = writeln!(out, "{:?} {:02} '{}'", op_code, symbol, name);
        offset + 3
    }

    fn local_field_instruction(&self, op_code: OpCode, offset: usize, out: &mut String) -> usize {
        let slot = self.read_u16(offset + 1);
        let symbol = self.read_u16(offset + 3) as usize;
        let name = &self.symbols[symbol];
        let _ = writeln!(out, "{:?} {:02} {:02} '{}'", op_code, slot, symbol, name);
        offset + 5
    }

    fn constant_instruction(&self, op_code: OpCode, offset: usize, out: &mut String) -> usize {
        let index = self.read_u16(offset + 1) as usize;
        let constant = self.read_constant(index);
        let _ = writeln!(out, "{:?} {:02} '{}'", op_code, index, constant);
        offset + 3
    }

    fn variable_instruction(&self, op_code: OpCode, offset: usize, out: &mut String) -> usize {
        let index = self.read_u16(offset + 1);
        let _ = writeln!(out, "{:?} {:02}", op_code, index);
        offset + 3
    }

    fn jump_instruction(&self, op_code: OpCode, offset: usize, out: &mut String) -> usize {
        let jump = self.read_u32(offset + 1);
        let _ = writeln!(
            out,
            "{:?} {:04x} -> {:04x}",
            op_code,
            offset,
            offset + 5 + jump as usize
        );
        offset + 5
    }

    fn loop_instruction(&self, offset: usize, out: &mut String) -> usize {
        let jump = self.read_u32(offset + 1);
        let _ = writeln!(
            out,
            "{:?} {:04x} -> {:04x}",
            OpCode::Loop,
            offset,
            offset + 5 - jump as usize
        );
        offset + 5
    }

    fn call_instruction(&self, offset: usize, out: &mut String) -> usize {
        let arg_count = self.read_u8(offset + 1);
        let _ = writeln!(out, "Call (args: {})", arg_count);
        offset + 2
    }

    fn invoke_instruction(&self, offset: usize, out: &mut String) -> usize {
        let method_symbol = self.read_u16(offset + 1) as usize;
        let name = &self.symbols[method_symbol];
        let arg_count = self.read_u8(offset + 3);
        let _ = writeln!(out, "Invoke {} (args: {})", name, arg_count);
        offset + 4
    }

    fn create_map_instruction(&self, offset: usize, out: &mut String) -> usize {
        let entry_count = self.read_u16(offset + 1);
        let _ = writeln!(out, "CreateMap (entries: {})", entry_count);
        offset + 3
    }

    fn create_array_instruction(&self, offset: usize, out: &mut String) -> usize {
        let element_count = self.read_u16(offset + 1);
        let _ = writeln!(out, "CreateArray (elements: {})", element_count);
        offset + 3
    }

    fn create_set_instruction(&self, offset: usize, out: &mut String) -> usize {
        let element_count = self.read_u16(offset + 1);
        let _ = writeln!(out, "CreateSet (elements: {})", element_count);
        offset + 3
    }

    fn closure_instruction(&self, offset: usize, out: &mut String) -> usize {
        let index = self.read_u16(offset + 1) as usize;
        let function = self.read_constant(index);
        let _ = writeln!(out, "{:?} {:02} '{}'", OpCode::Closure, index, function);

        let mut cursor = offset + 1 + 2;
        let upvalue_count = self.read_u8(cursor) as usize;
        cursor += 1;
        for _ in 0..upvalue_count {
            let is_local = self.read_u8(cursor) != 0;
            let upvalue_index = self.read_u16(cursor + 1);
            let _ = writeln!(
                out,
                "      |                     {} {:02}",
                if is_local { "local" } else { "upvalue" },
                upvalue_index
            );
            cursor += 3;
        }
        cursor
    }

    fn create_range_instruction(&self, offset: usize, out: &mut String) -> usize {
        let inclusive = self.read_u8(offset + 1);
        let _ = writeln!(out, "CreateRange (inclusive: {})", inclusive != 0);
        offset + 2
    }
}
