use crate::compiler::token::TokenType;

pub(crate) mod ast;
pub(crate) mod codegen;
pub(crate) mod compiler_impl;
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
    /// Exact source text the token spans, before any decoding (e.g. a
    /// string's escapes are undecoded here but decoded in `token`).
    pub raw: String,
    pub column: u32,
    pub line: u32,
    pub offset: usize,
}

#[derive(Debug)]
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
