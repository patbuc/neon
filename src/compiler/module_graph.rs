use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};
use crate::common::SourceLocation;
use crate::compiler::ast::{NodeId, Stmt};
use crate::compiler::parser::Parser;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub enum EntryLocation {
    File(PathBuf),
    Directory(PathBuf),
    None,
}

pub struct Module {
    pub path: PathBuf,
    pub source: String,
    pub ast: Vec<Stmt>,
    pub dependencies: Vec<PathBuf>,
    pub builtin: bool,
    pub(crate) eof_location: SourceLocation,
    pub(crate) end_locations: HashMap<NodeId, SourceLocation>,
}

pub struct ModuleGraph {
    modules: Vec<Module>,
}

impl ModuleGraph {
    /// Modules come in dependency order: every module follows the modules it
    /// imports, so the entry module is last.
    pub fn build(entry_source: &str, entry: EntryLocation) -> CompilationResult<ModuleGraph> {
        let (entry_path, entry_dir) = match entry {
            EntryLocation::File(path) if !cfg!(target_arch = "wasm32") => {
                let path = canonicalize(&path)?;
                let dir = path.parent().map(Path::to_path_buf).unwrap_or_default();
                (path, dir)
            }
            EntryLocation::Directory(dir) => (PathBuf::new(), dir),
            _ => (PathBuf::new(), PathBuf::new()),
        };
        let mut modules = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        load(
            entry_path,
            entry_dir,
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
    dir: PathBuf,
    source: String,
    modules: &mut Vec<Module>,
    visited: &mut HashSet<PathBuf>,
    stack: &mut Vec<PathBuf>,
) -> CompilationResult<()> {
    visited.insert(path.clone());
    stack.push(path.clone());
    let mut parser = Parser::new(&source);
    let ast = parser
        .parse()
        .map_err(|errors| attach_file(errors, &path))?;
    let eof_location = parser.eof_location();
    let end_locations = parser.end_locations().clone();
    let mut dependencies = Vec::new();
    for stmt in &ast {
        if let Stmt::Import {
            path: import,
            location,
            ..
        } = stmt
        {
            if import.starts_with("std/") {
                let builtin = PathBuf::from(import.as_str());
                if visited.insert(builtin.clone()) {
                    modules.push(Module {
                        path: builtin.clone(),
                        source: String::new(),
                        ast: Vec::new(),
                        dependencies: Vec::new(),
                        builtin: true,
                        eof_location: SourceLocation::default(),
                        end_locations: HashMap::new(),
                    });
                }
                dependencies.push(builtin);
                continue;
            }
            if dir.as_os_str().is_empty() {
                return Err(file_import_unavailable_error(*location, &path));
            }
            let file = if import.ends_with(".n") {
                import.to_string()
            } else {
                format!("{import}.n")
            };
            let tried = dir.join(file);
            let dependency = tried
                .canonicalize()
                .map_err(|_| unknown_module_error(import, &tried, *location, &path))?;
            if let Some(start) = stack.iter().position(|p| *p == dependency) {
                return Err(cycle_error(&stack[start..], &dependency, *location, &path));
            }
            if !visited.contains(&dependency) {
                let dependency_source =
                    std::fs::read_to_string(&dependency).map_err(|e| io_error(&dependency, &e))?;
                let dependency_dir = dependency
                    .parent()
                    .map(Path::to_path_buf)
                    .unwrap_or_default();
                load(
                    dependency.clone(),
                    dependency_dir,
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
        builtin: false,
        eof_location,
        end_locations,
    });
    Ok(())
}

fn cycle_error(
    cycle: &[PathBuf],
    closing: &Path,
    location: SourceLocation,
    file: &Path,
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
    )
    .with_file(file)]
}

fn file_import_unavailable_error(location: SourceLocation, file: &Path) -> Vec<CompilationError> {
    vec![CompilationError::new(
        CompilationPhase::Parse,
        CompilationErrorKind::FileImportUnavailable,
        "file imports are not available in the browser build".to_string(),
        location,
    )
    .with_file(file)]
}

fn unknown_module_error(
    import: &str,
    tried: &Path,
    location: SourceLocation,
    file: &Path,
) -> Vec<CompilationError> {
    vec![CompilationError::new(
        CompilationPhase::Parse,
        CompilationErrorKind::UnknownModule,
        format!("cannot find module '{import}' (tried {})", tried.display()),
        location,
    )
    .with_file(file)]
}

fn attach_file(errors: Vec<CompilationError>, file: &Path) -> Vec<CompilationError> {
    if file.as_os_str().is_empty() {
        return errors;
    }
    errors.into_iter().map(|e| e.with_file(file)).collect()
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
