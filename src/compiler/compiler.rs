use crate::common::Bloq;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;
use crate::compiler::Compiler;

impl Compiler {
    pub(crate) fn new() -> Compiler {
        Compiler {
            compilation_errors: String::new(),
            structured_errors: Vec::new(),
        }
    }

    pub(crate) fn compile(&mut self, source: &str) -> crate::common::errors::CompilationResult<Bloq> {
        // Multi-pass compilation:
        // Pass 1: Parse source into AST
        // Pass 2: Semantic analysis
        // Pass 3: Code generation

        // Phase 1: Parse
        let mut parser = Parser::new(source);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(errors) => {
                self.record_errors(&errors);
                return Err(errors.clone());
            }
        };

        // Phase 2: Semantic analysis
        let mut analyzer = SemanticAnalyzer::new();
        let _ = match analyzer.analyze(&ast) {
            Ok(table) => table,
            Err(errors) => {
                self.record_errors(&errors);
                return Err(errors.clone());
            }
        };

        // Phase 3: Code generation
        let mut codegen = CodeGenerator::new();
        match codegen.generate(&ast) {
            Ok(bloq) => Ok(bloq),
            Err(errors) => {
                self.record_errors(&errors);
                Err(errors.clone())
            }
        }
    }
}
