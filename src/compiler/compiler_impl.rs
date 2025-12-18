use crate::common::{Chunk, Value};
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;
use crate::compiler::Compiler;
use indexmap::IndexMap;

impl Compiler {
    pub fn new(builtin: IndexMap<String, Value>) -> Compiler {
        Compiler {
            compilation_errors: String::new(),
            structured_errors: Vec::new(),
            builtin,
        }
    }

    pub fn compile(&mut self, source: &str) -> Option<Chunk> {
        // Multi-pass compilation:
        // Pass 1: Parse source into AST
        // Pass 2: Semantic analysis
        // Pass 3: Code generation

        // Phase 1: Parse
        let mut parser = Parser::new(source);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(errors) => {
                // Store structured errors
                self.structured_errors = errors.clone();
                // Collect all parse errors
                self.compilation_errors = errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
                return None;
            }
        };

        // Phase 2: Semantic analysis
        let mut analyzer = SemanticAnalyzer::new();
        let _ = match analyzer.analyze(&ast) {
            Ok(table) => table,
            Err(errors) => {
                // Store structured errors
                self.structured_errors = errors.clone();
                // Collect all semantic errors
                self.compilation_errors = errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
                return None;
            }
        };

        // Phase 3: Code generation
        let mut codegen = CodeGenerator::new(self.builtin.clone());
        match codegen.generate(&ast) {
            Ok(chunk) => Some(chunk),
            Err(errors) => {
                // Store structured errors
                self.structured_errors = errors.clone();
                // Collect all codegen errors
                self.compilation_errors = errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
                None
            }
        }
    }
}
