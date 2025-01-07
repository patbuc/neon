use crate::compiler::Parser;
use crate::vm::opcodes::OpCode;
use crate::vm::{Block, Value};

impl Parser {
    fn current_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }

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
        self.current_block_mut().write_constant(value, line)
    }

    pub fn define_global(&mut self, name: String) {
        let line = self.previous_token.line;
        self.current_block_mut().define_global(name, line)
    }

    pub fn emit_string(&mut self, value: Value) -> u32 {
        let line = self.previous_token.line;
        self.current_block_mut().write_string(value, line)
    }

    pub fn emit_op_code(&mut self, op_code: OpCode) {
        let line = self.previous_token.line;
        self.current_block_mut().write_op_code(op_code, line);
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
        let current_block: &mut Block = self.current_block_mut();
        current_block.write_op_code(op_code1, line);
        current_block.write_op_code(op_code2, line);
    }
}
