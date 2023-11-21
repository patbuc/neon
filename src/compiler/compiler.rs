use crate::compiler::token::TokenType;
use crate::compiler::{Compiler, Parser, Scanner};
use crate::vm::Block;

impl Compiler {
    pub(crate) fn new() -> Compiler {
        Compiler {}
    }

    pub(crate) fn compile(&self, source: String) -> Option<Block> {
        let mut parser = Parser::new(Scanner::new(source));
        parser.advance();
        parser.expression();
        parser.consume(TokenType::Eof, "Expect end of expression.");

        return Option::from(Block::new_no_opt());
    }
}
