use crate::compiler::Parser;
use crate::vm::opcodes::OpCode;
use crate::vm::{Block, Value};

impl Parser {
    pub(super) fn emit_return(&mut self) {
        self.emit_op_code(OpCode::Return);
    }

    pub(super) fn emit_constant(&mut self, value: Value) {
        let line = self.previous_token.line;
        self.current_block().write_constant(value, line)
    }

    pub(super) fn emit_string(&mut self, value: Value) {
        let line = self.previous_token.line;
        self.current_block().write_string(value, line)
    }

    pub(super) fn emit_op_code(&mut self, op_code: OpCode) {
        let line = self.previous_token.line;
        self.current_block().write_op_code(op_code, line);
    }

    pub(super) fn emit_op_codes(&mut self, op_code1: OpCode, op_code2: OpCode) {
        let line = self.previous_token.line;
        let current_block: &mut Block = self.current_block();
        current_block.write_op_code(op_code1, line);
        current_block.write_op_code(op_code2, line);
    }
}
