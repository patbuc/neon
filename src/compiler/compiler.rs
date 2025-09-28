use crate::common::Brick;
use crate::compiler::token::TokenType;
use crate::compiler::{Compiler, Parser};

impl Compiler {
    pub(crate) fn new() -> Compiler {
        Compiler {
            compilation_errors: String::new(),
        }
    }

    pub(crate) fn compile(&mut self, source: String) -> Option<Brick> {
        let mut parser = Parser::new(source);

        parser.start();
        parser.advance();

        loop {
            if parser.match_token(TokenType::Eof) {
                break;
            }
            parser.declaration();
        }

        parser.consume(TokenType::Eof, "Expect end of expression");
        parser.end();

        if !(parser.had_error) {
            parser.bricks.pop()
        } else {
            self.compilation_errors = parser.compilation_errors.clone();
            None
        }
    }
}
