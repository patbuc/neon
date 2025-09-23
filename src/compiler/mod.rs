use crate::compiler::token::TokenType;
use crate::vm::Block;

pub(crate) mod compiler;
mod parser;
mod scanner;
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
    blocks: Vec<Block>,
    previous_token: Token,
    current_token: Token,
    scope_depth: u32,
    had_error: bool,
    panic_mode: bool,
}

#[derive(Debug)]
pub(crate) struct Compiler {}
