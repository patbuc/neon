use crate::compiler::Parser;
use crate::vm::opcodes::OpCode;
use crate::vm::{Block, Local, Value};

impl Parser {
    pub(super) fn current_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }

    fn current_block_mut(&mut self) -> &mut Block {
        self.blocks.last_mut().unwrap()
    }

    pub fn emit_return(&mut self) {
        self.emit_op_code(OpCode::Return);
    }

    pub fn emit_constant(&mut self, value: Value) -> u32 {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        self.current_block_mut().write_constant(value, line, column)
    }

    pub fn define_value(&mut self, name: String) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        let depth = self.scope_depth;
        self.current_block_mut()
            .define_value(Local::new(name, depth), line, column)
    }

    pub fn define_variable(&mut self, name: String) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        let depth = self.scope_depth;
        self.current_block_mut()
            .define_variable(Local::new(name, depth), line, column)
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

    pub fn emit_op_code_variant(&mut self, op_code: OpCode, index: u32) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        self.current_block_mut()
            .write_op_code_variant(op_code, index, line, column);
    }

    pub fn emit_jump(&mut self, op_code: OpCode) -> u32 {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        self.current_block_mut().emit_jump(op_code, line, column)
    }

    pub fn patch_jump(&mut self, offset: u32) {
        self.current_block_mut().patch_jump(offset);
    }

    pub fn emit_op_codes(&mut self, op_code1: OpCode, op_code2: OpCode) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        let current_block: &mut Block = self.current_block_mut();
        current_block.write_op_code(op_code1, line, column);
        current_block.write_op_code(op_code2, line, column);
    }
}
