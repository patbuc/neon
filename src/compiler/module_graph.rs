// Not consumed by Compiler until the plumbing unit lands.
#![allow(dead_code)]

use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};
use crate::common::SourceLocation;
use crate::compiler::ast::Stmt;
use crate::compiler::parser::Parser;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub(crate) enum EntryLocation {
    File(PathBuf),
}

pub(crate) struct Module {
    pub path: PathBuf,
    pub source: String,
    pub ast: Vec<Stmt>,
    pub dependencies: Vec<PathBuf>,
}

pub(crate) struct ModuleGraph {
    modules: Vec<Module>,
}

impl ModuleGraph {
    /// Modules come in dependency order: every module follows the modules it
    /// imports, so the entry module is last.
    pub fn build(entry_source: &str, entry: EntryLocation) -> CompilationResult<ModuleGraph> {
        let EntryLocation::File(entry_path) = entry;
        let mut modules = Vec::new();
        let mut visited = HashSet::new();
        load(
            entry_path,
            entry_source.to_string(),
            &mut modules,
            &mut visited,
        )?;
        Ok(ModuleGraph { modules })
    }

    pub fn modules(&self) -> &[Module] {
        &self.modules
    }
}

fn load(
    path: PathBuf,
    source: String,
    modules: &mut Vec<Module>,
    visited: &mut HashSet<PathBuf>,
) -> CompilationResult<()> {
    let path = canonicalize(&path)?;
    visited.insert(path.clone());
    let ast = Parser::new(&source).parse()?;
    let dir = path.parent().map(Path::to_path_buf).unwrap_or_default();

    let mut dependencies = Vec::new();
    for stmt in &ast {
        if let Stmt::Import { path: import, .. } = stmt {
            let file = if import.ends_with(".n") {
                import.to_string()
            } else {
                format!("{import}.n")
            };
            let dependency = canonicalize(&dir.join(file))?;
            if !visited.contains(&dependency) {
                let dependency_source =
                    std::fs::read_to_string(&dependency).map_err(|e| io_error(&dependency, &e))?;
                load(dependency.clone(), dependency_source, modules, visited)?;
            }
            dependencies.push(dependency);
        }
    }

    modules.push(Module {
        path,
        source,
        ast,
        dependencies,
    });
    Ok(())
}

fn canonicalize(path: &Path) -> CompilationResult<PathBuf> {
    path.canonicalize().map_err(|e| io_error(path, &e))
}

fn io_error(path: &Path, error: &std::io::Error) -> Vec<CompilationError> {
    vec![CompilationError::new(
        CompilationPhase::Parse,
        CompilationErrorKind::ModulesUnsupported,
        format!("cannot read '{}': {}", path.display(), error),
        SourceLocation::default(),
    )]
}
