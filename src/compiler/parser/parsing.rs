use crate::compiler::parser::rules::{ParseRule, Precedence, PARSE_RULES};
use crate::compiler::token::TokenType;
use crate::compiler::{Parser, Scanner, Token};
use crate::vm::opcodes::OpCode;
use crate::vm::Block;
use crate::vm::Value;
use std::str::FromStr;

use crate::{number, string};
#[cfg(feature = "disassemble")]
use tracing_attributes::instrument;

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

    pub(in crate::compiler) fn start(&mut self) {
        self.blocks.push(Block::new(
            format!("Block no. {}", self.blocks.len()).as_str(),
        ));
    }

    pub(in crate::compiler) fn end(&mut self) {
        self.emit_return();
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

    pub(in crate::compiler) fn match_token(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            return false;
        }
        self.advance();
        return true;
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        return self.current_token.token_type == token_type;
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(in crate::compiler) fn declaration(&mut self) {
        self.statement();
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    fn statement(&mut self) {
        if self.match_token(TokenType::Print) {
            self.print_statement();
        } else {
            self.expression_statement();
        }
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    fn expression(&mut self) {
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
    pub(super) fn number(&mut self) {
        let value = f64::from_str(&*self.previous_token.token).unwrap();
        self.emit_constant(number!(value));
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(super) fn string(&mut self) {
        let value = &*self.previous_token.token;
        let string = value.to_string();
        self.emit_string(string!(string[1..string.len() - 1].to_string()));
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(super) fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect end of expression");
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(super) fn binary(&mut self) {
        let operator_type = self.previous_token.token_type.clone();
        let rule = self.get_rule(operator_type.clone());
        self.parse_precedence(Precedence::from_u8(rule.precedence as u8 + 1));

        match operator_type {
            token_type if token_type == TokenType::BangEqual => {
                self.emit_op_codes(OpCode::Equal, OpCode::Not)
            }
            token_type if token_type == TokenType::EqualEqual => self.emit_op_code(OpCode::Equal),
            token_type if token_type == TokenType::Greater => self.emit_op_code(OpCode::Greater),
            token_type if token_type == TokenType::GreaterEqual => {
                self.emit_op_codes(OpCode::Less, OpCode::Not)
            }
            token_type if token_type == TokenType::Less => self.emit_op_code(OpCode::Less),
            token_type if token_type == TokenType::LessEqual => {
                self.emit_op_codes(OpCode::Greater, OpCode::Not)
            }
            token_type if token_type == TokenType::Plus => self.emit_op_code(OpCode::Add),
            token_type if token_type == TokenType::Minus => self.emit_op_code(OpCode::Subtract),
            token_type if token_type == TokenType::Star => self.emit_op_code(OpCode::Multiply),
            token_type if token_type == TokenType::Slash => self.emit_op_code(OpCode::Divide),
            _ => return, // Unreachable.
        }
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(super) fn literal(&mut self) {
        match self.previous_token.token_type {
            TokenType::False => self.emit_op_code(OpCode::False),
            TokenType::Nil => self.emit_op_code(OpCode::Nil),
            TokenType::True => self.emit_op_code(OpCode::True),
            _ => return, // Unreachable.
        }
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(super) fn unary(&mut self) {
        let operator_type = self.previous_token.token_type.clone();

        // Compile the operand.
        self.parse_precedence(Precedence::Unary);

        // Emit the operator instruction.
        match operator_type {
            TokenType::Bang => self.emit_op_code(OpCode::Not),
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
        #[cfg(feature = "disassemble")]
        return self.get_rule_safe(token_type);
        &(PARSE_RULES.get((token_type as u32) as usize).unwrap().1)
    }

    fn get_rule_safe(&self, token_type: TokenType) -> &ParseRule {
        let parse_rule = PARSE_RULES
            .get((token_type.clone() as u8) as usize)
            .unwrap();
        if parse_rule.0 != token_type {
            panic!("Parsing rules are out of sync with token types.");
        }
        &parse_rule.1
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expecting ';' at end of statement.");
        self.emit_op_code(OpCode::Print);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expecting ';' at end of expression.");
        self.emit_op_code(OpCode::Pop);
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

        eprint!("[line {}, pos {}] Error", token.line + 1, token.start + 1);
        if token.token_type == TokenType::Eof {
            eprint!(" at end");
        } else if token.token_type == TokenType::Error {
            // Nothing.
        } else {
            eprint!(" at '{}'", token.token);
        }
        eprintln!(": {}", message);
    }
}
