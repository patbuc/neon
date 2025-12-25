use crate::common::{Chunk, Value};
use crate::compiler::codegen::CodeGenerator;
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
            last_exports: Vec::new(),
            module_resolver: crate::compiler::module_resolver::ModuleResolver::new(),
        }
    }

    pub fn compile(&mut self, source: &str) -> Option<Chunk> {
        self.compile_with_path(source, None)
    }

    pub(crate) fn compile_with_path(
        &mut self,
        source: &str,
        file_path: Option<PathBuf>,
    ) -> Option<Chunk> {
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
        let _ = match analyzer.analyze(&ast, file_path.clone()) {
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

        // Store exports for later retrieval (e.g., for module metadata creation)
        self.last_exports = exports.clone();

        // Phase 3: Code generation
        let mut codegen = CodeGenerator::new(self.builtin.clone());
        match codegen.generate_with_exports(&ast, exports) {
            Ok(chunk) => {
                // Note: Module metadata (exports and source path) is now attached
                // to ObjFunction at the VM level when creating modules
                Some(chunk)
            }
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
