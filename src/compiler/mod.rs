use crate::compiler::token::TokenType;

pub(crate) mod ast;
pub(crate) mod codegen;
pub(crate) mod compiler;
pub(crate) mod parser;
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
    offset: usize,
    previous_token_type: TokenType,
}

#[derive(Debug)]
pub(crate) struct Compiler {
    compilation_errors: String,
    structured_errors: Vec<crate::common::errors::CompilationError>,
}

impl Compiler {
    pub(crate) fn get_compilation_errors(&self) -> String {
        self.compilation_errors.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn get_structured_errors(&self) -> Vec<crate::common::errors::CompilationError> {
        self.structured_errors.clone()
    }

    pub(crate) fn record_errors(&mut self, errors: &[crate::common::errors::CompilationError]) {
        self.structured_errors = errors.to_vec();
        self.compilation_errors = errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n");
    }
}
