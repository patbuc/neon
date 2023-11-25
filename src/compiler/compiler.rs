use crate::compiler::token::TokenType;
use crate::compiler::{Compiler, Parser, Scanner};
use crate::vm::Block;

impl Compiler {
    pub(crate) fn new() -> Compiler {
        Compiler { parser: None }
    }

    pub(crate) fn compile(&mut self, source: String) -> Option<Block> {
        let mut parser = Parser::new(Scanner::new(source));

        parser.start();
        parser.advance();
        parser.expression();
        parser.consume(TokenType::Eof, "Expect end of expression");
        parser.end();

        return if parser.had_error {
            None
        } else {
            parser.blocks.pop()
        };
    }
}
