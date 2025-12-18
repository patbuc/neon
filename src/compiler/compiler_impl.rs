use crate::common::{Chunk, Value};
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::module_resolver::ModuleResolver;
use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;
use crate::compiler::Compiler;
use indexmap::IndexMap;
use std::path::PathBuf;

impl Compiler {
    pub fn new(builtin: IndexMap<String, Value>) -> Compiler {
        Compiler {
            compilation_errors: String::new(),
            structured_errors: Vec::new(),
            builtin,
            module_resolver: ModuleResolver::new(),
            current_file_path: None,
        }
    }

    pub(crate) fn compile(&mut self, source: &str) -> Option<Chunk> {
        self.compile_with_path(source, None)
    }

    pub(crate) fn compile_with_path(
        &mut self,
        source: &str,
        file_path: Option<PathBuf>,
    ) -> Option<Chunk> {
        // Store the current file path for relative imports
        self.current_file_path = file_path;

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

        // Extract exports from semantic analyzer
        let exports = analyzer.exports().clone();

        // Phase 3: Code generation
        let mut codegen = CodeGenerator::new(self.builtin.clone());
        match codegen.generate_with_exports(&ast, exports) {
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

    /// Get a reference to the module resolver
    pub(crate) fn module_resolver(&self) -> &ModuleResolver {
        &self.module_resolver
    }

    /// Get a mutable reference to the module resolver
    pub(crate) fn module_resolver_mut(&mut self) -> &mut ModuleResolver {
        &mut self.module_resolver
    }

    /// Get the current file path
    pub(crate) fn current_file_path(&self) -> Option<&PathBuf> {
        self.current_file_path.as_ref()
    }
}
