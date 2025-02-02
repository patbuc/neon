use crate::compiler::token::TokenType;
use crate::compiler::{Compiler, Parser, Scanner};
use crate::vm::Block;
use std::cell::RefCell;
use std::rc::Rc;

impl Compiler {
    pub(crate) fn new() -> Compiler {
        Compiler { scope_depth: 0 }
    }

    pub(crate) fn compile(compiler: Rc<RefCell<Compiler>>, source: String) -> Option<Block> {
        let mut parser = Parser::new(compiler, Scanner::new(source));

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
            parser.blocks.pop()
        } else {
            None
        }
    }
}
