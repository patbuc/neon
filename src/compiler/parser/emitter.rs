use crate::common::opcodes::OpCode;
use crate::common::{Brick, Local, Value};
use crate::compiler::Parser;
use crate::current_brick_mut;

impl Parser {
    pub fn emit_return(&mut self) {
        self.emit_op_code(OpCode::Return);
    }

    pub fn emit_constant(&mut self, value: Value) -> u32 {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        current_brick_mut!(self.bricks).write_constant(value, line, column)
    }

    pub fn define_value(&mut self, name: String) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        let depth = self.scope_depth;
        current_brick_mut!(self.bricks).define_value(Local::new(name, depth), line, column)
    }

    pub fn define_variable(&mut self, name: String) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        let depth = self.scope_depth;

        current_brick_mut!(self.bricks).define_variable(Local::new(name, depth), line, column)
    }

    pub fn emit_string(&mut self, value: Value) -> u32 {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        current_brick_mut!(self.bricks).write_string(value, line, column)
    }

    pub fn emit_op_code(&mut self, op_code: OpCode) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        current_brick_mut!(self.bricks).write_op_code(op_code, line, column);
    }

    pub fn emit_op_code_variant(&mut self, op_code: OpCode, index: u32) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        current_brick_mut!(self.bricks).write_op_code_variant(op_code, index, line, column);
    }

    pub fn emit_jump(&mut self, op_code: OpCode) -> u32 {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        current_brick_mut!(self.bricks).emit_jump(op_code, line, column)
    }

    pub fn patch_jump(&mut self, offset: u32) {
        current_brick_mut!(self.bricks).patch_jump(offset);
    }

    pub fn emit_op_codes(&mut self, op_code1: OpCode, op_code2: OpCode) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        let current_brick: &mut Brick = current_brick_mut!(self.bricks);
        current_brick.write_op_code(op_code1, line, column);
        current_brick.write_op_code(op_code2, line, column);
    }

    pub fn emit_loop(&mut self, loop_start: u32) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        current_brick_mut!(self.bricks).emit_loop(loop_start, line, column);
    }
}
