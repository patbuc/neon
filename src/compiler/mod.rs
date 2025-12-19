use crate::compiler::token::TokenType;

pub(crate) mod ast;
pub(crate) mod codegen;
pub(crate) mod compiler_impl;
pub(crate) mod module_resolver;
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
pub struct Compiler {
    compilation_errors: String,
    structured_errors: Vec<crate::common::errors::CompilationError>,
    builtin: indexmap::IndexMap<String, crate::common::Value>,
    /// Exports from the last compilation (for module metadata)
    last_exports: Vec<crate::common::module_types::ExportInfo>,
}

impl Compiler {
    pub fn get_compilation_errors(&self) -> String {
        self.compilation_errors.clone()
    }

    pub fn get_structured_errors(&self) -> Vec<crate::common::errors::CompilationError> {
        self.structured_errors.clone()
    }

    /// Get exports from the last compilation
    pub(crate) fn get_last_exports(&self) -> &Vec<crate::common::module_types::ExportInfo> {
        &self.last_exports
    }
}
