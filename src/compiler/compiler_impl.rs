use crate::common::Chunk;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::global_env::GlobalEnv;
use crate::compiler::module_graph::{EntryLocation, Module, ModuleGraph};
use crate::compiler::semantic::SemanticAnalyzer;
use crate::compiler::Compiler;

impl Compiler {
    pub fn new() -> Compiler {
        Compiler::default()
    }

    pub fn compile(&mut self, source: &str) -> Option<Chunk> {
        self.compile_at(source, EntryLocation::None)
    }

    pub(crate) fn compile_at(&mut self, source: &str, entry: EntryLocation) -> Option<Chunk> {
        let env = GlobalEnv::default();
        self.compile_entry(source, entry, &env)
            .map(|(chunk, _)| chunk)
    }

    /// Compiles one REPL line against `env`, the globals earlier lines left
    /// behind. `env` is only ever borrowed, so a failed line leaves the
    /// caller's copy intact.
    #[cfg(test)]
    pub(crate) fn compile_line(
        &mut self,
        source: &str,
        env: &GlobalEnv,
    ) -> Option<(Chunk, GlobalEnv)> {
        self.compile_entry(source, EntryLocation::None, env)
    }

    pub(crate) fn compile_entry(
        &mut self,
        source: &str,
        entry: EntryLocation,
        env: &GlobalEnv,
    ) -> Option<(Chunk, GlobalEnv)> {
        // Pass 1: Resolve the module graph (parses every module)
        // Pass 2: Semantic analysis
        // Pass 3: Code generation

        self.module_sources.clear();
        let graph = match ModuleGraph::build_with_sources(source, entry, &mut self.module_sources) {
            Ok(graph) => graph,
            Err(errors) => return self.fail(errors),
        };
        let entry = Self::entry_module(&graph);
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

    #[allow(clippy::expect_used)]
    fn entry_module(graph: &ModuleGraph) -> &Module {
        graph
            .modules()
            .last()
            .expect("a module graph always contains its entry")
    }

    fn fail<T>(&mut self, errors: Vec<crate::common::errors::CompilationError>) -> Option<T> {
        self.structured_errors = errors;
        None
    }
}
