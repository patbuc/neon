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
        let mut stack = Vec::new();
        load(
            entry_path,
            entry_source.to_string(),
            &mut modules,
            &mut visited,
            &mut stack,
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
    stack: &mut Vec<PathBuf>,
) -> CompilationResult<()> {
    let path = canonicalize(&path)?;
    visited.insert(path.clone());
    stack.push(path.clone());
    let ast = Parser::new(&source).parse()?;
    let dir = path.parent().map(Path::to_path_buf).unwrap_or_default();

    let mut dependencies = Vec::new();
    for stmt in &ast {
        if let Stmt::Import {
            path: import,
            location,
            ..
        } = stmt
        {
            let file = if import.ends_with(".n") {
                import.to_string()
            } else {
                format!("{import}.n")
            };
            let tried = dir.join(file);
            let dependency = tried
                .canonicalize()
                .map_err(|_| unknown_module_error(import, &tried, *location))?;
            if let Some(start) = stack.iter().position(|p| *p == dependency) {
                return Err(cycle_error(&stack[start..], &dependency, *location));
            }
            if !visited.contains(&dependency) {
                let dependency_source =
                    std::fs::read_to_string(&dependency).map_err(|e| io_error(&dependency, &e))?;
                load(
                    dependency.clone(),
                    dependency_source,
                    modules,
                    visited,
                    stack,
                )?;
            }
            dependencies.push(dependency);
        }
    }

    stack.pop();
    modules.push(Module {
        path,
        source,
        ast,
        dependencies,
    });
    Ok(())
}

fn cycle_error(
    cycle: &[PathBuf],
    closing: &Path,
    location: SourceLocation,
) -> Vec<CompilationError> {
    let names: Vec<String> = cycle
        .iter()
        .map(PathBuf::as_path)
        .chain(std::iter::once(closing))
        .map(|p| {
            p.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    vec![CompilationError::new(
        CompilationPhase::Parse,
        CompilationErrorKind::ImportCycle,
        format!("import cycle: {}", names.join(" -> ")),
        location,
    )]
}

fn unknown_module_error(
    import: &str,
    tried: &Path,
    location: SourceLocation,
) -> Vec<CompilationError> {
    vec![CompilationError::new(
        CompilationPhase::Parse,
        CompilationErrorKind::UnknownModule,
        format!("cannot find module '{import}' (tried {})", tried.display()),
        location,
    )]
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
