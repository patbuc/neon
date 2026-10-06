use crate::common::errors::CompilationResult;
use crate::common::Chunk;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::exports::ExportTable;
use crate::compiler::global_env::GlobalEnv;
use crate::compiler::module_graph::{EntryLocation, Module, ModuleGraph};
use crate::compiler::semantic::SemanticAnalyzer;
use crate::compiler::Compiler;
use std::collections::HashMap;
use std::path::PathBuf;

/// A compiled program: one chunk per imported file module, in graph order,
/// then the entry's chunk and the globals it leaves behind.
pub(crate) struct Compiled {
    pub(crate) modules: Vec<Chunk>,
    /// The global slot count after each module in `modules` has run.
    pub(crate) module_slot_counts: Vec<u32>,
    pub(crate) entry: Chunk,
    pub(crate) env: GlobalEnv,
}

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
            .map(|compiled| compiled.entry)
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
            .map(|compiled| (compiled.entry, compiled.env))
    }

    pub(crate) fn compile_entry(
        &mut self,
        source: &str,
        entry: EntryLocation,
        env: &GlobalEnv,
    ) -> Option<Compiled> {
        // Pass 1: Resolve the module graph (parses every module)
        // Pass 2: Semantic analysis
        // Pass 3: Code generation

        self.module_sources.clear();
        let graph = match ModuleGraph::build_with_sources(source, entry, &mut self.module_sources) {
            Ok(graph) => graph,
            Err(errors) => return self.fail(errors),
        };

        // Each imported module compiles against a fresh env sharing only
        // the symbols, decl ids and slot count, so its globals follow the
        // previous module's and stay invisible to the others.
        let mut env = env.clone();
        let mut exports = std::mem::take(&mut env.modules);
        let mut modules = Vec::new();
        let mut module_slot_counts = Vec::new();
        let (entry_module, imported) = Self::split_entry(&graph);
        for module in imported.iter().filter(|module| !module.builtin) {
            if exports.contains_key(&module.path) {
                continue;
            }
            let module_env = GlobalEnv {
                symbols: env.symbols.clone(),
                next_decl_id: env.next_decl_id,
                slot_count: env.slot_count,
                ..GlobalEnv::default()
            };
            let (mut chunk, module_env, table) =
                match self.compile_unit(module, &module_env, &exports) {
                    Ok(unit) => unit,
                    Err(errors) => {
                        let errors = errors
                            .into_iter()
                            .map(|error| error.with_file(&module.path))
                            .collect();
                        return self.fail(errors);
                    }
                };
            env.symbols = module_env.symbols;
            env.next_decl_id = module_env.next_decl_id;
            env.slot_count = module_env.slot_count;
            exports.insert(module.path.clone(), table);
            chunk.name = module.path.display().to_string();
            modules.push(chunk);
            module_slot_counts.push(env.slot_count);
        }

        let (entry, mut env, _) = match self.compile_unit(entry_module, &env, &exports) {
            Ok(unit) => unit,
            Err(errors) => return self.fail(errors),
        };
        env.modules = exports;
        Some(Compiled {
            modules,
            module_slot_counts,
            entry,
            env,
        })
    }

    fn compile_unit(
        &mut self,
        module: &Module,
        env: &GlobalEnv,
        exports: &HashMap<PathBuf, ExportTable>,
    ) -> CompilationResult<(Chunk, GlobalEnv, ExportTable)> {
        let imports = module
            .imports
            .iter()
            .filter_map(|(id, path)| Some((*id, exports.get(path)?.clone())))
            .collect();

        // Phase 2: Semantic analysis
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.seed(env);
        analyzer.seed_imports(imports);
        let resolutions = analyzer.analyze(&module.ast)?;

        // Phase 3: Code generation
        let mut codegen = CodeGenerator::new(&resolutions, &module.end_locations);
        codegen.seed(env);
        let chunk = codegen.generate(&module.ast, module.eof_location)?;
        let decl_slots = codegen.into_decl_slots();

        let table = ExportTable::build(&module.ast, &resolutions, &decl_slots);
        let new_env = analyzer.snapshot_env(resolutions, decl_slots, env.slot_count);
        Ok((chunk, new_env, table))
    }

    #[allow(clippy::expect_used)]
    fn split_entry(graph: &ModuleGraph) -> (&Module, &[Module]) {
        graph
            .modules()
            .split_last()
            .expect("a module graph always contains its entry")
    }

    fn fail<T>(&mut self, errors: Vec<crate::common::errors::CompilationError>) -> Option<T> {
        self.structured_errors = errors;
        None
    }
}
