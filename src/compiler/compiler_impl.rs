use crate::common::Chunk;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;
use crate::compiler::Compiler;

impl Compiler {
    pub fn new() -> Compiler {
        Compiler::default()
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
            Err(errors) => return self.fail(errors),
        };

        // Phase 2: Semantic analysis
        let mut analyzer = SemanticAnalyzer::new();
        let resolutions = match analyzer.analyze(&ast) {
            Ok(resolutions) => resolutions,
            Err(errors) => return self.fail(errors),
        };

        // Phase 3: Code generation
        let mut codegen = CodeGenerator::new(&resolutions);
        match codegen.generate(&ast) {
            Ok(chunk) => Some(chunk),
            Err(errors) => self.fail(errors),
        }
    }

    fn fail(&mut self, errors: Vec<crate::common::errors::CompilationError>) -> Option<Chunk> {
        self.structured_errors = errors;
        None
    }
}
