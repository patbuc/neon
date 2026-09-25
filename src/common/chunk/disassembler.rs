use crate::common::opcodes::OpCode;
use crate::common::Chunk;

#[cfg(feature = "disassemble")]
impl Chunk {
    #[allow(dead_code)]
    pub(crate) fn disassemble_chunk(&self) {
        println!();
        println!("=== <{}>  ===", self.name);

        let mut offset: usize = 0;
        while offset < self.instructions.len() {
            offset = self.disassemble_instruction(offset);
        }

        println!("=== </{}> ===", self.name);
    }

    pub(crate) fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04x} ", offset);

        let line = self.get_line_info(offset).unwrap();
        if offset > 0 && line.line == self.get_line_info(offset - 1).unwrap().line {
            print!("     | ");
        } else {
            print!("{:6} ", line.line);
        }

        let instruction = match OpCode::from_u8(self.instructions[offset]) {
            Some(instruction) => instruction,
            None => {
                println!("Unknown opcode {:#04x}", self.instructions[offset]);
                return offset + 1;
            }
        };
        match instruction {
            OpCode::Return => self.simple_instruction(OpCode::Return, offset),
            OpCode::Constant => self.constant_instruction(offset),
            OpCode::Negate => self.simple_instruction(OpCode::Negate, offset),
            OpCode::Add => self.simple_instruction(OpCode::Add, offset),
            OpCode::Subtract => self.simple_instruction(OpCode::Subtract, offset),
            OpCode::Multiply => self.simple_instruction(OpCode::Multiply, offset),
            OpCode::Divide => self.simple_instruction(OpCode::Divide, offset),
            OpCode::Exponent => self.simple_instruction(OpCode::Exponent, offset),
            OpCode::Nil => self.simple_instruction(OpCode::Nil, offset),
            OpCode::True => self.simple_instruction(OpCode::True, offset),
            OpCode::False => self.simple_instruction(OpCode::False, offset),
            OpCode::Equal => self.simple_instruction(OpCode::Equal, offset),
            OpCode::Greater => self.simple_instruction(OpCode::Greater, offset),
            OpCode::GreaterEqual => self.simple_instruction(OpCode::GreaterEqual, offset),
            OpCode::Less => self.simple_instruction(OpCode::Less, offset),
            OpCode::LessEqual => self.simple_instruction(OpCode::LessEqual, offset),
            OpCode::Not => self.simple_instruction(OpCode::Not, offset),
            OpCode::String => self.string_instruction(offset),
            OpCode::Pop => self.simple_instruction(OpCode::Pop, offset),
            OpCode::SetLocal => self.variable_instruction(OpCode::SetLocal, offset),
            OpCode::GetLocal => self.variable_instruction(OpCode::GetLocal, offset),
            OpCode::GetBuiltin => self.variable_instruction(OpCode::GetBuiltin, offset),
            OpCode::GetGlobal => self.variable_instruction(OpCode::GetGlobal, offset),
            OpCode::SetGlobal => self.variable_instruction(OpCode::SetGlobal, offset),
            OpCode::JumpIfFalse => self.jump_instruction(instruction, offset),
            OpCode::Jump => self.jump_instruction(instruction, offset),
            OpCode::Loop => self.loop_instruction(offset),
            OpCode::Call => self.call_instruction(offset),
            OpCode::Invoke => self.invoke_instruction(offset),
            OpCode::Modulo => self.simple_instruction(instruction, offset),
            OpCode::GetField => self.field_instruction(OpCode::GetField, offset),
            OpCode::SetField => self.field_instruction(OpCode::SetField, offset),
            OpCode::CreateMap => self.create_map_instruction(offset),
            OpCode::CreateArray => self.create_array_instruction(offset),
            OpCode::CreateSet => self.create_set_instruction(offset),
            OpCode::GetIndex => self.simple_instruction(OpCode::GetIndex, offset),
            OpCode::SetIndex => self.simple_instruction(OpCode::SetIndex, offset),
            OpCode::GetIterator => self.simple_instruction(OpCode::GetIterator, offset),
            OpCode::IteratorNext => self.variable_instruction(OpCode::IteratorNext, offset),
            OpCode::IteratorDone => self.variable_instruction(OpCode::IteratorDone, offset),
            OpCode::CreateRange => self.create_range_instruction(offset),
            OpCode::ToString => self.simple_instruction(OpCode::ToString, offset),
            OpCode::BitwiseAnd => self.simple_instruction(OpCode::BitwiseAnd, offset),
            OpCode::BitwiseOr => self.simple_instruction(OpCode::BitwiseOr, offset),
            OpCode::BitwiseXor => self.simple_instruction(OpCode::BitwiseXor, offset),
            OpCode::BitwiseNot => self.simple_instruction(OpCode::BitwiseNot, offset),
            OpCode::LeftShift => self.simple_instruction(OpCode::LeftShift, offset),
            OpCode::RightShift => self.simple_instruction(OpCode::RightShift, offset),
            OpCode::Closure => self.closure_instruction(offset),
            OpCode::GetUpvalue => self.variable_instruction(OpCode::GetUpvalue, offset),
            OpCode::SetUpvalue => self.variable_instruction(OpCode::SetUpvalue, offset),
            OpCode::CloseUpvalue => self.simple_instruction(OpCode::CloseUpvalue, offset),
            OpCode::CloseUpvalueInPlace => {
                self.simple_instruction(OpCode::CloseUpvalueInPlace, offset)
            }
            OpCode::DefineMethod => self.define_method_instruction(offset),
            OpCode::CheckInitialized => self.simple_instruction(OpCode::CheckInitialized, offset),
        }
    }

    fn define_method_instruction(&self, offset: usize) -> usize {
        let type_index = self.read_u16(offset + 1) as usize;
        let method_index = self.read_u16(offset + 3) as usize;
        let takes_self = self.read_u8(offset + 5) != 0;
        let type_name = self.read_string(type_index);
        let method_name = self.read_string(method_index);
        let kind = if takes_self { "instance" } else { "static" };
        println!(
            "{:?} {}.{} ({})",
            OpCode::DefineMethod,
            type_name,
            method_name,
            kind
        );
        offset + 6
    }

    fn simple_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        println!("{:?}", op_code);
        offset + 1
    }

    fn field_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        let index = self.read_u16(offset + 1) as usize;
        let field_name = self.read_string(index);
        println!("{:?} {:02} '{}'", op_code, index, field_name);
        offset + 3
    }

    fn constant_instruction(&self, offset: usize) -> usize {
        let index = self.read_u16(offset + 1) as usize;
        let constant = self.read_constant(index);
        println!("{:?} {:02} '{}'", OpCode::Constant, index, constant);
        offset + 3
    }

    fn variable_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        let index = self.read_u16(offset + 1);
        println!("{:?} {:02}", op_code, index);
        offset + 3
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

    fn loop_instruction(&self, offset: usize) -> usize {
        let jump = self.read_u32(offset + 1);
        println!(
            "{:?} {:04x} -> {:04x}",
            OpCode::Loop,
            offset,
            offset + 5 - jump as usize
        );
        offset + 5
    }

    fn string_instruction(&self, offset: usize) -> usize {
        let index = self.read_u16(offset + 1) as usize;
        let string = self.read_string(index);
        println!("{:?} {:02} '{}'", OpCode::String, index, string);
        offset + 3
    }

    fn call_instruction(&self, offset: usize) -> usize {
        let arg_count = self.read_u8(offset + 1);
        println!("Call (args: {})", arg_count);
        offset + 2
    }

    fn invoke_instruction(&self, offset: usize) -> usize {
        let name_index = self.read_u16(offset + 1) as usize;
        let name = self.read_string(name_index);
        let arg_count = self.read_u8(offset + 3);
        println!("Invoke {} (args: {})", name, arg_count);
        offset + 4
    }

    fn create_map_instruction(&self, offset: usize) -> usize {
        let entry_count = self.read_u16(offset + 1);
        println!("CreateMap (entries: {})", entry_count);
        offset + 3
    }

    fn create_array_instruction(&self, offset: usize) -> usize {
        let element_count = self.read_u16(offset + 1);
        println!("CreateArray (elements: {})", element_count);
        offset + 3
    }

    fn create_set_instruction(&self, offset: usize) -> usize {
        let element_count = self.read_u16(offset + 1);
        println!("CreateSet (elements: {})", element_count);
        offset + 3
    }

    fn closure_instruction(&self, offset: usize) -> usize {
        let index = self.read_u16(offset + 1) as usize;
        let function = self.read_constant(index);
        println!("{:?} {:02} '{}'", OpCode::Closure, index, function);

        let mut cursor = offset + 1 + 2;
        let upvalue_count = self.read_u8(cursor) as usize;
        cursor += 1;
        for _ in 0..upvalue_count {
            let is_local = self.read_u8(cursor) != 0;
            let upvalue_index = self.read_u16(cursor + 1);
            println!(
                "      |                     {} {:02}",
                if is_local { "local" } else { "upvalue" },
                upvalue_index
            );
            cursor += 3;
        }
        cursor
    }

    fn create_range_instruction(&self, offset: usize) -> usize {
        let inclusive = self.read_u8(offset + 1);
        println!("CreateRange (inclusive: {})", inclusive != 0);
        offset + 2
    }
}
