use crate::compiler::Parser;
use crate::vm::opcodes::OpCode;
use crate::vm::{Block, Value};

impl Parser {
    fn current_block_mut(&mut self) -> &mut Block {
        self.blocks.last_mut().unwrap()
    }

    pub fn emit_return(&mut self) {
        self.emit_op_code(OpCode::Return);
    }

    pub fn add_constant(&mut self, value: Value) -> u32 {
        self.current_block_mut().add_constant(value)
    }

    pub fn emit_constant(&mut self, value: Value) -> u32 {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        self.current_block_mut().write_constant(value, line, column)
    }

    pub fn define_global(&mut self, name: String) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        self.current_block_mut().define_global(name, line, column)
    }

    pub fn emit_string(&mut self, value: Value) -> u32 {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        self.current_block_mut().write_string(value, line, column)
    }

    pub fn emit_op_code(&mut self, op_code: OpCode) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        self.current_block_mut()
            .write_op_code(op_code, line, column);
    }

    pub fn emit_jump(&mut self, op_code: OpCode) -> u32 {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        self.current_block_mut().emit_jump(op_code, line, column)
    }

    pub fn patch_jump(&mut self, offset: u32) {
        self.current_block_mut().patch_jump(offset);
    }

    pub fn emit_u8(&mut self, value: u8) {
        self.current_block_mut().write_u8(value);
    }

    pub fn emit_u16(&mut self, value: u16) {
        self.current_block_mut().write_u16(value);
    }

    pub fn emit_u32(&mut self, value: u32) {
        self.current_block_mut().write_u32(value);
    }

    pub fn emit_op_codes(&mut self, op_code1: OpCode, op_code2: OpCode) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        let current_block: &mut Block = self.current_block_mut();
        current_block.write_op_code(op_code1, line, column);
        current_block.write_op_code(op_code2, line, column);
    }
}
