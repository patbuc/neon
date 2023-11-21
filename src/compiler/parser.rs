use crate::compiler::token::TokenType;
use crate::compiler::{Parser, Scanner, Token};

impl Parser {
    pub(in crate::compiler) fn new(scanner: Scanner) -> Parser {
        Parser {
            scanner,
            previous_token: Token::INVALID,
            current_token: Token::INVALID,
        }
    }

    pub(in crate::compiler) fn advance(&mut self) {
        std::mem::swap(&mut self.previous_token, &mut self.current_token);
        loop {
            self.current_token = self.scanner.scan_token();
            if self.current_token.token_type != TokenType::Error {
                break;
            }
            self.error_at_current();
        }
    }

    pub(in crate::compiler) fn expression(&self) {}

    pub(in crate::compiler) fn consume(&self, token_type: TokenType, message: &str) {}

    fn error_at_current(&self) {
        println!("Error at current token: {:?}", self.current_token);
    }
}
