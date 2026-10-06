use crate::common::Chunk;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::global_env::GlobalEnv;
use crate::compiler::module_graph::{EntryLocation, ModuleGraph};
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
        // Pass 1: Resolve the module graph (parses every module)
        // Pass 2: Semantic analysis
        // Pass 3: Code generation

        let graph = match ModuleGraph::build(source, EntryLocation::None) {
            Ok(graph) => graph,
            Err(errors) => return self.fail(errors),
        };
        let entry = graph.modules().last()?;
        let ast = &entry.ast;
        let eof_location = entry.eof_location;

        // Phase 2: Semantic analysis
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.seed(env);
        let resolutions = match analyzer.analyze(ast) {
            Ok(resolutions) => resolutions,
            Err(errors) => return self.fail(errors),
        };

        // Phase 3: Code generation
        let mut codegen = CodeGenerator::new(&resolutions, &entry.end_locations);
        codegen.seed(env);
        let chunk = match codegen.generate(ast, eof_location) {
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
