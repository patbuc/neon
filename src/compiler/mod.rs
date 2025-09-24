use crate::compiler::token::TokenType;
use crate::vm::Brick;

pub(crate) mod compiler;
mod parser;
mod token;

#[derive(Debug, Clone, Default)]
struct Token {
    token_type: TokenType,
    token: String,
    column: u32,
    line: u32,
}

#[derive(Debug)]
struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: u32,
    column: u32,
    previous_token_type: TokenType,
}

#[derive(Debug)]
struct Parser {
    scanner: Scanner,
    bricks: Vec<Brick>,
    previous_token: Token,
    current_token: Token,
    scope_depth: u32,
    had_error: bool,
    panic_mode: bool,
    compilation_errors: String,
}

#[derive(Debug)]
pub(crate) struct Compiler {
    compilation_errors: String,
}

impl Compiler {
    pub(crate) fn get_compilation_errors(&self) -> String {
        self.compilation_errors.clone()
    }
}
