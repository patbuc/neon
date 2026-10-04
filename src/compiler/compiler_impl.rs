use crate::common::Chunk;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::global_env::GlobalEnv;
use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;
use crate::compiler::Compiler;

impl Compiler {
    pub fn new() -> Compiler {
        Compiler::default()
    }

    pub fn compile(&mut self, source: &str) -> Option<Chunk> {
        let env = GlobalEnv::default();
        self.compile_line(source, &env).map(|(chunk, _)| chunk)
    }

    /// Compiles one REPL line against `env`, the globals earlier lines left
    /// behind. `env` is only ever borrowed, so a failed line leaves the
    /// caller's copy intact.
    pub(crate) fn compile_line(
        &mut self,
        source: &str,
        env: &GlobalEnv,
    ) -> Option<(Chunk, GlobalEnv)> {
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
        let eof_location = parser.eof_location();

        // Phase 2: Semantic analysis
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.seed(env);
        let resolutions = match analyzer.analyze(&ast) {
            Ok(resolutions) => resolutions,
            Err(errors) => return self.fail(errors),
        };

        // Phase 3: Code generation
        let mut codegen = CodeGenerator::new(&resolutions, parser.end_locations());
        codegen.seed(env);
        let chunk = match codegen.generate(&ast, eof_location) {
            Ok(chunk) => chunk,
            Err(errors) => return self.fail(errors),
        };
        let decl_slots = codegen.into_decl_slots();

        let new_env = analyzer.snapshot_env(resolutions, decl_slots, env.slot_count);
        Some((chunk, new_env))
    }

    fn fail<T>(&mut self, errors: Vec<crate::common::errors::CompilationError>) -> Option<T> {
        self.structured_errors = errors;
        None
    }
}
