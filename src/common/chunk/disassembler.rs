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

        let line = self.get_source_location(offset).unwrap();
        if offset > 0 && line.line == self.get_source_location(offset - 1).unwrap().line {
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
            OpCode::FloorDivide => self.simple_instruction(OpCode::FloorDivide, offset),
            OpCode::Exponent => self.simple_instruction(OpCode::Exponent, offset),
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
            OpCode::Pop => self.simple_instruction(OpCode::Pop, offset),
            OpCode::SetLocal => self.variable_instruction(OpCode::SetLocal, offset),
            OpCode::SetLocal2 => self.variable_instruction(OpCode::SetLocal2, offset),
            OpCode::SetLocal4 => self.variable_instruction(OpCode::SetLocal4, offset),
            OpCode::GetLocal => self.variable_instruction(OpCode::GetLocal, offset),
            OpCode::GetLocal2 => self.variable_instruction(OpCode::GetLocal2, offset),
            OpCode::GetLocal4 => self.variable_instruction(OpCode::GetLocal4, offset),
            OpCode::GetBuiltin => self.variable_instruction(OpCode::GetBuiltin, offset),
            OpCode::GetBuiltin2 => self.variable_instruction(OpCode::GetBuiltin2, offset),
            OpCode::GetBuiltin4 => self.variable_instruction(OpCode::GetBuiltin4, offset),
            OpCode::GetGlobal => self.variable_instruction(OpCode::GetGlobal, offset),
            OpCode::GetGlobal2 => self.variable_instruction(OpCode::GetGlobal2, offset),
            OpCode::GetGlobal4 => self.variable_instruction(OpCode::GetGlobal4, offset),
            OpCode::SetGlobal => self.variable_instruction(OpCode::SetGlobal, offset),
            OpCode::SetGlobal2 => self.variable_instruction(OpCode::SetGlobal2, offset),
            OpCode::SetGlobal4 => self.variable_instruction(OpCode::SetGlobal4, offset),
            OpCode::JumpIfFalse => self.jump_instruction(instruction, offset),
            OpCode::Jump => self.jump_instruction(instruction, offset),
            OpCode::Loop => self.simple_instruction(OpCode::Loop, offset),
            OpCode::Call => self.call_instruction(offset),
            OpCode::Modulo => self.simple_instruction(instruction, offset),
            OpCode::GetField => self.field_instruction(OpCode::GetField, offset),
            OpCode::GetField2 => self.field_instruction(OpCode::GetField2, offset),
            OpCode::GetField4 => self.field_instruction(OpCode::GetField4, offset),
            OpCode::SetField => self.field_instruction(OpCode::SetField, offset),
            OpCode::SetField2 => self.field_instruction(OpCode::SetField2, offset),
            OpCode::SetField4 => self.field_instruction(OpCode::SetField4, offset),
            OpCode::CallMethod => self.call_method_instruction(OpCode::CallMethod, offset),
            OpCode::CallMethod2 => self.call_method_instruction(OpCode::CallMethod2, offset),
            OpCode::CallMethod4 => self.call_method_instruction(OpCode::CallMethod4, offset),
            OpCode::CallStaticMethod => {
                self.call_static_method_instruction(OpCode::CallStaticMethod, offset)
            }
            OpCode::CallStaticMethod2 => {
                self.call_static_method_instruction(OpCode::CallStaticMethod2, offset)
            }
            OpCode::CallStaticMethod4 => {
                self.call_static_method_instruction(OpCode::CallStaticMethod4, offset)
            }
            OpCode::CallConstructor => {
                self.call_constructor_instruction(OpCode::CallConstructor, offset)
            }
            OpCode::CallConstructor2 => {
                self.call_constructor_instruction(OpCode::CallConstructor2, offset)
            }
            OpCode::CallConstructor4 => {
                self.call_constructor_instruction(OpCode::CallConstructor4, offset)
            }
            OpCode::CreateMap => self.create_map_instruction(offset),
            OpCode::CreateArray => self.create_array_instruction(offset),
            OpCode::CreateSet => self.create_set_instruction(offset),
            OpCode::GetIndex => self.simple_instruction(OpCode::GetIndex, offset),
            OpCode::SetIndex => self.simple_instruction(OpCode::SetIndex, offset),
            OpCode::GetIterator => self.simple_instruction(OpCode::GetIterator, offset),
            OpCode::IteratorNext => self.simple_instruction(OpCode::IteratorNext, offset),
            OpCode::IteratorDone => self.simple_instruction(OpCode::IteratorDone, offset),
            OpCode::PopIterator => self.simple_instruction(OpCode::PopIterator, offset),
            OpCode::CreateRange => self.create_range_instruction(offset),
            OpCode::ToString => self.simple_instruction(OpCode::ToString, offset),
            OpCode::BitwiseAnd => self.simple_instruction(OpCode::BitwiseAnd, offset),
            OpCode::BitwiseOr => self.simple_instruction(OpCode::BitwiseOr, offset),
            OpCode::BitwiseXor => self.simple_instruction(OpCode::BitwiseXor, offset),
            OpCode::BitwiseNot => self.simple_instruction(OpCode::BitwiseNot, offset),
            OpCode::LeftShift => self.simple_instruction(OpCode::LeftShift, offset),
            OpCode::RightShift => self.simple_instruction(OpCode::RightShift, offset),
        }
    }

    fn simple_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        println!("{:?}", op_code);
        offset + 1
    }

    fn field_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        fn get_field_index(chunk: &Chunk, op_code: &OpCode, offset: usize) -> (usize, usize) {
            match op_code {
                OpCode::GetField | OpCode::SetField => (chunk.read_u8(offset) as usize, 1),
                OpCode::GetField2 | OpCode::SetField2 => (chunk.read_u16(offset) as usize, 2),
                OpCode::GetField4 | OpCode::SetField4 => (chunk.read_u32(offset) as usize, 4),
                _ => panic!("Invalid OpCode for field instruction"),
            }
        }

        let (index, offset_shift) = get_field_index(self, &op_code, offset + 1);
        let field_name = self.read_string(index);
        println!("{:?} {:02} '{}'", op_code, index, field_name);
        offset + 1 + offset_shift
    }

    fn constant_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        fn get_constant_index(chunk: &Chunk, op_code: &OpCode, offset: usize) -> (usize, usize) {
            match op_code {
                OpCode::Constant => (chunk.read_u8(offset) as usize, 1),
                OpCode::Constant2 => (chunk.read_u16(offset) as usize, 2),
                OpCode::Constant4 => (chunk.read_u32(offset) as usize, 4),
                _ => panic!("Invalid OpCode"),
            }
        }

        let (index, offset_shift) = get_constant_index(self, &op_code, offset + 1);
        let constant = self.read_constant(index);
        println!("{:?} {:02} '{}'", op_code, index, constant);
        offset + 1 + offset_shift
    }

    fn variable_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        fn get_variable_index(chunk: &Chunk, op_code: &OpCode, offset: usize) -> (usize, usize) {
            match op_code {
                OpCode::GetLocal => (chunk.read_u8(offset) as usize, 1),
                OpCode::GetLocal2 => (chunk.read_u16(offset) as usize, 2),
                OpCode::GetLocal4 => (chunk.read_u32(offset) as usize, 4),
                OpCode::SetLocal => (chunk.read_u8(offset) as usize, 1),
                OpCode::SetLocal2 => (chunk.read_u16(offset) as usize, 2),
                OpCode::SetLocal4 => (chunk.read_u32(offset) as usize, 4),
                OpCode::GetGlobal => (chunk.read_u8(offset) as usize, 1),
                OpCode::GetGlobal2 => (chunk.read_u16(offset) as usize, 2),
                OpCode::GetGlobal4 => (chunk.read_u32(offset) as usize, 4),
                OpCode::SetGlobal => (chunk.read_u8(offset) as usize, 1),
                OpCode::SetGlobal2 => (chunk.read_u16(offset) as usize, 2),
                OpCode::SetGlobal4 => (chunk.read_u32(offset) as usize, 4),
                _ => panic!("Invalid OpCode {:?}", op_code),
            }
        }

        let (index, offset_shift) = get_variable_index(self, &op_code, offset + 1);
        let constant = self.read_constant(index);
        println!("{:?} {:02} '{}'", op_code, index, constant);
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

    fn call_instruction(&self, offset: usize) -> usize {
        let arg_count = self.read_u8(offset + 1);
        println!("Call (args: {})", arg_count);
        offset + 2
    }

    fn call_method_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        let arg_count = self.read_u8(offset + 1);

        let (method_index, index_size) = match op_code {
            OpCode::CallMethod => (self.read_u8(offset + 2) as usize, 1),
            OpCode::CallMethod2 => (self.read_u16(offset + 2) as usize, 2),
            OpCode::CallMethod4 => (self.read_u32(offset + 2) as usize, 4),
            _ => panic!("Invalid opcode for call_method_instruction"),
        };

        let method_name = self.read_string(method_index);
        println!(
            "{:?} (args: {}, method: '{}')",
            op_code, arg_count, method_name
        );
        offset + 1 + 1 + index_size
    }

    fn call_static_method_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        let arg_count = self.read_u8(offset + 1);

        let (registry_index, index_size) = match op_code {
            OpCode::CallStaticMethod => (self.read_u8(offset + 2) as usize, 1),
            OpCode::CallStaticMethod2 => (self.read_u16(offset + 2) as usize, 2),
            OpCode::CallStaticMethod4 => (self.read_u32(offset + 2) as usize, 4),
            _ => panic!("Invalid opcode for call_static_method_instruction"),
        };

        println!(
            "{:?} (args: {}, registry_index: {})",
            op_code, arg_count, registry_index
        );
        offset + 1 + 1 + index_size
    }

    fn call_constructor_instruction(&self, op_code: OpCode, offset: usize) -> usize {
        let arg_count = self.read_u8(offset + 1);

        let (registry_index, index_size) = match op_code {
            OpCode::CallConstructor => (self.read_u8(offset + 2) as usize, 1),
            OpCode::CallConstructor2 => (self.read_u16(offset + 2) as usize, 2),
            OpCode::CallConstructor4 => (self.read_u32(offset + 2) as usize, 4),
            _ => panic!("Invalid opcode for call_constructor_instruction"),
        };

        println!(
            "{:?} (args: {}, registry_index: {})",
            op_code, arg_count, registry_index
        );
        offset + 1 + 1 + index_size
    }

    fn create_map_instruction(&self, offset: usize) -> usize {
        let entry_count = self.read_u8(offset + 1);
        println!("CreateMap (entries: {})", entry_count);
        offset + 2
    }

    fn create_array_instruction(&self, offset: usize) -> usize {
        let element_count = self.read_u16(offset + 1);
        println!("CreateArray (elements: {})", element_count);
        offset + 3
    }

    fn create_set_instruction(&self, offset: usize) -> usize {
        let element_count = self.read_u8(offset + 1);
        println!("CreateSet (elements: {})", element_count);
        offset + 2
    }

    fn create_range_instruction(&self, offset: usize) -> usize {
        let inclusive = self.read_u8(offset + 1);
        println!("CreateRange (inclusive: {})", inclusive != 0);
        offset + 2
    }
}
