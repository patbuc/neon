use crate::compiler::token::TokenType;
use crate::compiler::{Compiler, Parser, Scanner};
use crate::vm::opcodes::OpCode;
use crate::vm::{Block, Value};

impl Compiler {
    pub(crate) fn new() -> Compiler {
        Compiler {
            blocks: Vec::default(),
            parser: None,
        }
    }

    pub(crate) fn compile(&mut self, source: String) -> Option<Block> {
        self.parser = Option::from(Parser::new(Scanner::new(source)));
        self.start_compiler();

        let had_error = match &mut self.parser {
            Some(ref mut parser) => {
                parser.advance();
                parser.expression();
                parser.consume(TokenType::Eof, "Expect end of expression");
                parser.had_error
            }
            None => true,
        };

        self.end_compiler();

        return if !had_error {
            Some(self.blocks.pop().unwrap())
        } else {
            None
        };
    }

    fn start_compiler(&mut self) {
        self.blocks.push(Block::new(
            format!("Block no. {}", self.blocks.len()).as_str(),
        ));
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn number(&mut self) {
        if let Some(parser) = &mut self.parser {
            let value = parser.previous_token.token.parse::<f64>().unwrap();
            self.emit_constant(value);
        }
    }

    fn current_block(&mut self) -> &mut Block {
        self.blocks.last_mut().unwrap()
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn emit_constant(&mut self, value: Value) {
        self.current_block().write_constant(value, 0)
    }

    fn emit_byte(&mut self, byte: u8) {
        self.current_block().write_u8(byte);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.current_block().write_u8(byte1);
        self.current_block().write_u8(byte2);
    }
}
