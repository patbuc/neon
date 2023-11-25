use crate::compiler::token::TokenType;
use crate::compiler::{Parser, Scanner, Token};

impl Parser {
    pub(in crate::compiler) fn new(scanner: Scanner) -> Parser {
        Parser {
            scanner,
            had_error: false,
            panic_mode: false,
            previous_token: Token::default(),
            current_token: Token::default(),
        }
    }

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

    pub(in crate::compiler) fn expression(&self) {}

    pub(in crate::compiler) fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current_token.token_type == token_type {
            self.advance();
            return;
        }

        self.report_error_at_current(message);
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
}
