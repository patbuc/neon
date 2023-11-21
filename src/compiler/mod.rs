use crate::compiler::token::TokenType;

mod compiler;
mod parser;
mod scanner;
mod token;

#[derive(Debug)]
struct Token {
    pub(crate) token_type: TokenType,
    start: usize,
    length: usize,
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
}

pub(crate) struct Compiler {}
