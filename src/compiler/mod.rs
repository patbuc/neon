use crate::compiler::token::TokenType;

pub(crate) mod ast;
pub(crate) mod codegen;
pub(crate) mod compiler_impl;
mod formatter;
pub use formatter::format;
pub(crate) mod global_env;
pub(crate) mod parser;
pub(crate) mod resolutions;
mod scanner;
pub(crate) mod semantic;
pub(crate) mod symbol_table;
mod token;

#[cfg(test)]
mod tests;

// Scanner and Token types used by ast_parser
#[derive(Debug, Clone, Default)]
pub(crate) struct Token {
    pub token_type: TokenType,
    pub token: String,
    /// For a string segment, the source text between its delimiters with
    /// escapes undecoded; empty for every other token.
    pub raw: String,
    pub column: u32,
    pub line: u32,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: u32,
    column: u32,
    start_line: u32,
    start_column: u32,
    previous_token_type: TokenType,
    /// Open `${...}` interpolations, innermost last.
    interpolations: Vec<scanner::Interpolation>,
    trivia: Trivia,
}

/// Whether a comment is alone on its line or follows code on the same line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommentKind {
    OwnLine,
    Trailing,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Comment {
    pub(crate) line: u32,
    pub(crate) column: u32,
    /// Verbatim comment text, including the leading `//`, excluding any
    /// trailing newline or CRLF's `\r`.
    pub(crate) text: String,
    pub(crate) kind: CommentKind,
}

/// Source detail dropped by the token stream: comments and the line
/// numbers of blank lines.
#[derive(Debug, Clone, Default)]
pub(crate) struct Trivia {
    pub(crate) comments: Vec<Comment>,
    pub(crate) blank_lines: Vec<u32>,
}

#[derive(Debug, Default)]
pub struct Compiler {
    structured_errors: Vec<crate::common::errors::CompilationError>,
}

impl Compiler {
    pub fn get_structured_errors(&self) -> Vec<crate::common::errors::CompilationError> {
        self.structured_errors.clone()
    }
}

/// `lexeme` is the raw source slice, not the decoded value.
pub fn tokens_to_json(source: &str) -> String {
    let mut scanner = Scanner::new(source);
    let mut out = String::from("[");
    let mut first = true;
    loop {
        let token = scanner.scan_token();
        if token.token_type == TokenType::Eof {
            break;
        }
        if !first {
            out.push(',');
        }
        first = false;
        let lexeme: String = scanner.source[scanner.start..scanner.current]
            .iter()
            .collect();
        out.push_str(&format!(
            "{{\"kind\":\"{:?}\",\"line\":{},\"column\":{},\"lexeme\":{}}}",
            token.token_type,
            token.line,
            token.column,
            json_string(&lexeme)
        ));
    }
    out.push(']');
    out
}

fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
