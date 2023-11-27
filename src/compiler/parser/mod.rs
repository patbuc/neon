use crate::compiler::parser::rules::PARSE_RULES;
use crate::compiler::token::TokenType;
use crate::compiler::{Parser, Scanner, Token};
use crate::vm::opcodes::OpCode;
use crate::vm::{Block, Value};
use rules::{ParseRule, Precedence};
use std::str::FromStr;
use tracing_attributes::instrument;

mod rules;

impl Parser {
    pub(in crate::compiler) fn new(scanner: Scanner) -> Parser {
        Parser {
            scanner,
            blocks: Vec::default(),
            had_error: false,
            panic_mode: false,
            previous_token: Token::default(),
            current_token: Token::default(),
        }
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(in crate::compiler) fn advance(&mut self) {
        std::mem::swap(&mut self.previous_token, &mut self.current_token);
        loop {
            self.current_token = self.scanner.scan_token();
            if self.current_token.token_type != TokenType::Error {
                break;
            }
            self.report_error_at_current(self.current_token.token.clone().as_str());
        }
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(in crate::compiler) fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self, precedence)))]
    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self.get_rule(self.previous_token.token_type.clone()).prefix;
        if prefix_rule.is_none() {
            self.report_error_at_current("Expect expression");
            return;
        }
        prefix_rule.unwrap()(self);

        while precedence as u8
            <= self
                .get_rule(self.current_token.token_type.clone())
                .precedence as u8
        {
            self.advance();
            let infix_rule = self.get_rule(self.previous_token.token_type.clone()).infix;
            infix_rule.unwrap()(self);
        }
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    fn number(&mut self) {
        let value = f64::from_str(&*self.previous_token.token).unwrap();
        self.emit_constant(Value::from_number(value));
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect end of expression");
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    fn binary(&mut self) {
        let operator_type = self.previous_token.token_type.clone();
        let rule = self.get_rule(operator_type.clone());
        self.parse_precedence(Precedence::from_u8(rule.precedence as u8 + 1));

        match operator_type {
            token_type if token_type == TokenType::Plus => self.emit_op_code(OpCode::Add),
            token_type if token_type == TokenType::Minus => self.emit_op_code(OpCode::Subtract),
            token_type if token_type == TokenType::Star => self.emit_op_code(OpCode::Multiply),
            token_type if token_type == TokenType::Slash => self.emit_op_code(OpCode::Divide),
            _ => return, // Unreachable.
        }
    }

    fn literal(&mut self) {
        match self.previous_token.token_type {
            TokenType::False => self.emit_op_code(OpCode::False),
            TokenType::Nil => self.emit_op_code(OpCode::Nil),
            TokenType::True => self.emit_op_code(OpCode::True),
            _ => return, // Unreachable.
        }
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    fn unary(&mut self) {
        let operator_type = self.previous_token.token_type.clone();

        // Compile the operand.
        self.parse_precedence(Precedence::Unary);

        // Emit the operator instruction.
        match operator_type {
            TokenType::Minus => self.emit_op_code(OpCode::Negate),
            _ => return, // Unreachable.
        }
    }

    pub(in crate::compiler) fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current_token.token_type == token_type {
            self.advance();
            return;
        }

        self.report_error_at_current(message);
    }

    fn get_rule(&self, token_type: TokenType) -> &ParseRule {
        PARSE_RULES.get(&token_type).unwrap().clone()
    }

    fn report_error_at_current(&mut self, message: &str) {
        self.report_error(&self.current_token.clone(), message);
    }

    fn report_error(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }

        self.had_error = true;
        self.panic_mode = true;

        eprint!("[line {}] Error", token.line);
        if token.token_type == TokenType::Eof {
            eprint!(" at end");
        } else if token.token_type == TokenType::Error {
            // Nothing.
        } else {
            eprint!(" at '{}'", token.token);
        }
        eprintln!(": {}", message);
    }

    pub(in crate::compiler) fn start(&mut self) {
        self.blocks.push(Block::new(
            format!("Block no. {}", self.blocks.len()).as_str(),
        ));
    }

    pub(in crate::compiler) fn end(&mut self) {
        self.emit_return();
    }

    fn current_block(&mut self) -> &mut Block {
        self.blocks.last_mut().unwrap()
    }

    fn emit_return(&mut self) {
        self.emit_op_code(OpCode::Return);
    }

    fn emit_constant(&mut self, value: Value) {
        self.current_block().write_constant(value, 0)
    }

    fn emit_op_code(&mut self, op_code: OpCode) {
        let line = self.previous_token.line;
        self.current_block().write_op_code(op_code, line);
    }
}
