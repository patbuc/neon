use crate::common::opcodes::OpCode;
use crate::common::{Bloq, Local, Value};
use crate::compiler::Parser;
use crate::current_bloq_mut;

impl Parser {
    pub fn emit_return(&mut self) {
        // Ensure there's a return value on the stack (nil if no explicit return)
        self.emit_op_code(OpCode::Nil);
        self.emit_op_code(OpCode::Return);
    }

    pub fn emit_constant(&mut self, value: Value) -> u32 {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        current_bloq_mut!(self.bloqs).write_constant(value, line, column)
    }

    pub fn define_value(&mut self, name: String) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        let depth = self.scope_depth;
        current_bloq_mut!(self.bloqs).define_local(Local::new(name, depth, false), line, column)
    }

    pub fn define_variable(&mut self, name: String) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        let depth = self.scope_depth;
        current_bloq_mut!(self.bloqs).define_local(Local::new(name, depth, true), line, column)
    }

    pub fn define_parameter(&mut self, name: String) {
        let depth = self.scope_depth;
        // Parameters are already on the stack (pushed by caller), so we just register them
        // without emitting a SetValue opcode
        current_bloq_mut!(self.bloqs).add_parameter(Local::new(name, depth, false));
    }

    pub fn emit_string(&mut self, value: Value) -> u32 {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        current_bloq_mut!(self.bloqs).write_string(value, line, column)
    }

    pub fn emit_op_code(&mut self, op_code: OpCode) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        current_bloq_mut!(self.bloqs).write_op_code(op_code, line, column);
    }

    pub fn emit_op_code_variant(&mut self, op_code: OpCode, index: u32) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        current_bloq_mut!(self.bloqs).write_op_code_variant(op_code, index, line, column);
    }

    pub fn emit_jump(&mut self, op_code: OpCode) -> u32 {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        current_bloq_mut!(self.bloqs).emit_jump(op_code, line, column)
    }

    pub fn patch_jump(&mut self, offset: u32) {
        current_bloq_mut!(self.bloqs).patch_jump(offset);
    }

    pub fn emit_call(&mut self, argument_count: u8) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        current_bloq_mut!(self.bloqs).write_op_code(OpCode::Call, line, column);
        current_bloq_mut!(self.bloqs).write_u8(argument_count)
    }

    pub fn emit_op_codes(&mut self, op_code1: OpCode, op_code2: OpCode) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        let current_bloq: &mut Bloq = current_bloq_mut!(self.bloqs);
        current_bloq.write_op_code(op_code1, line, column);
        current_bloq.write_op_code(op_code2, line, column);
    }

    pub fn emit_loop(&mut self, loop_start: u32) {
        let line = self.previous_token.line;
        let column = self.previous_token.column;
        current_bloq_mut!(self.bloqs).emit_loop(loop_start, line, column);
    }
}
