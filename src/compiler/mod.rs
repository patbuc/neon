use crate::compiler::token::TokenType;
use crate::vm::Block;

mod compiler;
mod parser;
mod scanner;
mod token;

#[derive(Debug, Clone, Default)]
struct Token {
    pub(crate) token_type: TokenType,
    token: String,
    start: usize,
    line: u32,
}

struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: u32,
}

struct Parser {
    scanner: Scanner,
    previous_token: Token,
    current_token: Token,
    had_error: bool,
    panic_mode: bool,
}

pub(crate) struct Compiler {
    blocks: Vec<Block>,
    parser: Option<Parser>,
}
