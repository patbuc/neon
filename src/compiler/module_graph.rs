use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};
use crate::common::method_registry::builtin_modules;
use crate::common::SourceLocation;
use crate::compiler::ast::{NodeId, Stmt};
use crate::compiler::parser::Parser;
use std::collections::{HashMap, HashSet};
use std::path::{Component, Path, PathBuf};

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
    /// The resolved path of each `Stmt::Import`, by its node id.
    pub imports: HashMap<NodeId, PathBuf>,
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
        Self::build_with_sources(entry_source, entry, &mut HashMap::new())
    }

    /// Like `build`, also recording the source of every imported module it
    /// reads, so errors attributed to one can be rendered after a failed build.
    pub(crate) fn build_with_sources(
        entry_source: &str,
        entry: EntryLocation,
        sources: &mut HashMap<PathBuf, String>,
    ) -> CompilationResult<ModuleGraph> {
        let (entry_path, entry_dir) = match entry {
            _ if cfg!(target_arch = "wasm32") => (PathBuf::new(), None),
            EntryLocation::File(path) => {
                let path = path.canonicalize().map_err(|e| {
                    vec![CompilationError::new(
                        CompilationPhase::Parse,
                        CompilationErrorKind::UnknownModule,
                        format!("cannot read '{}': {}", path.display(), e),
                        SourceLocation::default(),
                    )]
                })?;
                let dir = path.parent().map(Path::to_path_buf);
                (path, dir)
            }
            EntryLocation::Directory(dir) => (PathBuf::new(), Some(dir)),
            EntryLocation::None => (PathBuf::new(), None),
        };
        let mut modules = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        load(
            entry_path,
            entry_dir,
            entry_source.to_string(),
            sources,
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
    dir: Option<PathBuf>,
    source: String,
    sources: &mut HashMap<PathBuf, String>,
    modules: &mut Vec<Module>,
    visited: &mut HashSet<PathBuf>,
    stack: &mut Vec<PathBuf>,
) -> CompilationResult<()> {
    let is_entry = stack.is_empty();
    let file = (!is_entry).then_some(path.as_path());
    if !is_entry {
        sources.insert(path.clone(), source.clone());
    }
    visited.insert(path.clone());
    stack.push(path.clone());
    let mut parser = Parser::new(&source);
    let ast = parser.parse().map_err(|errors| attach_file(errors, file))?;
    let eof_location = parser.eof_location();
    let end_locations = parser.end_locations().clone();
    let mut dependencies = Vec::new();
    let mut imports = HashMap::new();
    for stmt in &ast {
        if let Stmt::Import {
            path: import,
            id,
            location,
            ..
        } = stmt
        {
            if import.starts_with("std/") {
                if !builtin_modules().contains(&import.as_str()) {
                    return Err(unknown_builtin_module_error(import, *location, file));
                }
                let builtin = PathBuf::from(import.as_str());
                if visited.insert(builtin.clone()) {
                    modules.push(Module {
                        path: builtin.clone(),
                        source: String::new(),
                        ast: Vec::new(),
                        dependencies: Vec::new(),
                        imports: HashMap::new(),
                        builtin: true,
                        eof_location: SourceLocation::default(),
                        end_locations: HashMap::new(),
                    });
                }
                imports.insert(*id, builtin.clone());
                if !dependencies.contains(&builtin) {
                    dependencies.push(builtin);
                }
                continue;
            }
            let Some(dir) = &dir else {
                return Err(file_import_unavailable_error(*location, file));
            };
            let file_name = if import.ends_with(".n") {
                import.to_string()
            } else {
                format!("{import}.n")
            };
            let relative: PathBuf = Path::new(&file_name)
                .components()
                .filter(|c| *c != Component::CurDir)
                .collect();
            let tried = dir.join(relative);
            let dependency = tried
                .canonicalize()
                .map_err(|_| unknown_module_error(import, &tried, *location, file))?;
            if let Some(start) = stack.iter().position(|p| *p == dependency) {
                return Err(cycle_error(&stack[start..], &dependency, *location, file));
            }
            if !visited.contains(&dependency) {
                let dependency_source = std::fs::read_to_string(&dependency).map_err(|e| {
                    unreadable_module_error(import, &dependency, &e, *location, file)
                })?;
                let dependency_dir = dependency.parent().map(Path::to_path_buf);
                load(
                    dependency.clone(),
                    dependency_dir,
                    dependency_source,
                    sources,
                    modules,
                    visited,
                    stack,
                )?;
            }
            imports.insert(*id, dependency.clone());
            if !dependencies.contains(&dependency) {
                dependencies.push(dependency);
            }
        }
    }

    stack.pop();
    modules.push(Module {
        path,
        source,
        ast,
        dependencies,
        imports,
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
    file: Option<&Path>,
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
    attach_file(
        vec![CompilationError::new(
            CompilationPhase::Parse,
            CompilationErrorKind::ImportCycle,
            format!("import cycle: {}", names.join(" -> ")),
            location,
        )],
        file,
    )
}

fn file_import_unavailable_error(
    location: SourceLocation,
    file: Option<&Path>,
) -> Vec<CompilationError> {
    attach_file(
        vec![CompilationError::new(
            CompilationPhase::Parse,
            CompilationErrorKind::FileImportUnavailable,
            "file imports are not available in the browser build".to_string(),
            location,
        )],
        file,
    )
}

fn unknown_module_error(
    import: &str,
    tried: &Path,
    location: SourceLocation,
    file: Option<&Path>,
) -> Vec<CompilationError> {
    attach_file(
        vec![CompilationError::new(
            CompilationPhase::Parse,
            CompilationErrorKind::UnknownModule,
            format!("cannot find module '{import}' (tried {})", tried.display()),
            location,
        )],
        file,
    )
}

fn unknown_builtin_module_error(
    import: &str,
    location: SourceLocation,
    file: Option<&Path>,
) -> Vec<CompilationError> {
    attach_file(
        vec![CompilationError::new(
            CompilationPhase::Parse,
            CompilationErrorKind::UnknownModule,
            format!(
                "unknown builtin module '{import}' (expected one of {})",
                builtin_modules().join(", ")
            ),
            location,
        )],
        file,
    )
}

fn attach_file(errors: Vec<CompilationError>, file: Option<&Path>) -> Vec<CompilationError> {
    match file {
        Some(file) => errors.into_iter().map(|e| e.with_file(file)).collect(),
        None => errors,
    }
}

fn unreadable_module_error(
    import: &str,
    path: &Path,
    error: &std::io::Error,
    location: SourceLocation,
    file: Option<&Path>,
) -> Vec<CompilationError> {
    attach_file(
        vec![CompilationError::new(
            CompilationPhase::Parse,
            CompilationErrorKind::UnknownModule,
            format!(
                "cannot read module '{import}' ({}): {error}",
                path.display()
            ),
            location,
        )],
        file,
    )
}
