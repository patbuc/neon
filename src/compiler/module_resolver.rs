use crate::common::{errors::CompilationError, errors::CompilationErrorKind, errors::CompilationPhase, Chunk, SourceLocation};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Symbol exported from a module
#[derive(Debug, Clone, PartialEq)]
pub struct ExportedSymbol {
    pub name: String,
    pub kind: SymbolKind,
    pub global_index: u32,
}

/// Type of exported symbol
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Function { arity: u8 },
    Variable,
    Struct { fields: Vec<String> },
}

/// Compiled module with its exports
#[derive(Debug, Clone)]
pub struct CompiledModule {
    pub path: PathBuf,
    pub chunk: Rc<Chunk>,
    pub exports: HashMap<String, ExportedSymbol>,
}

/// Module resolver for managing module compilation and resolution
#[derive(Debug)]
pub struct ModuleResolver {
    modules: HashMap<PathBuf, Rc<CompiledModule>>,
    compilation_stack: Vec<PathBuf>,
}

impl ModuleResolver {
    /// Create a new module resolver
    pub fn new() -> Self {
        ModuleResolver {
            modules: HashMap::new(),
            compilation_stack: Vec::new(),
        }
    }

    /// Resolve a module path to an absolute canonical path
    ///
    /// Module path resolution rules:
    /// - Explicit relative: "./module" or "../module" - relative to current file
    /// - Absolute: "/path/to/module" - absolute file path (for testing/tooling)
    /// - System/native: "system/io" - contains "/" but not absolute (future: search system paths)
    /// - Local: "math_lib" - simple name, resolves to same directory as current file
    ///
    /// # Arguments
    /// * `module_path` - The module path from the import statement
    /// * `current_file` - The path of the file containing the import (optional)
    ///
    /// # Returns
    /// * `Ok(PathBuf)` - The resolved absolute path with .n extension
    /// * `Err(String)` - Error message if path resolution fails
    pub fn resolve_path(
        &self,
        module_path: &str,
        current_file: Option<&Path>,
    ) -> Result<PathBuf, String> {
        // Determine module path type
        let is_explicit_relative = module_path.starts_with("./") || module_path.starts_with("../");
        let is_absolute = module_path.starts_with('/');
        let is_system_module = module_path.contains('/') && !is_explicit_relative && !is_absolute;

        let mut resolved_path = if is_absolute {
            // Absolute path (e.g., "/path/to/module")
            // Used for testing or direct file imports
            PathBuf::from(module_path)
        } else if is_explicit_relative {
            // Explicit relative path (e.g., "./module" or "../module")
            let current_dir = match current_file {
                Some(path) => path.parent().ok_or_else(|| {
                    format!("Cannot determine parent directory of '{}'", path.display())
                })?,
                None => {
                    return Err(format!(
                        "Relative import '{}' requires current file context",
                        module_path
                    ));
                }
            };
            current_dir.join(module_path)
        } else if is_system_module {
            // System/native module path (e.g., "system/io")
            // For now, treat as error - will be implemented when system modules are added
            return Err(format!(
                "System module imports not yet implemented: '{}'\n\
                 Future: This will search for built-in modules in the Neon standard library",
                module_path
            ));
        } else {
            // Local module - simple name (e.g., "math_lib")
            // Resolve relative to current file's directory
            let current_dir = match current_file {
                Some(path) => path.parent().ok_or_else(|| {
                    format!("Cannot determine parent directory of '{}'", path.display())
                })?,
                None => {
                    return Err(format!(
                        "Module import '{}' requires current file context",
                        module_path
                    ));
                }
            };
            current_dir.join(module_path)
        };

        // Add .n extension if not present
        if resolved_path.extension().is_none() {
            resolved_path.set_extension("n");
        }

        // Canonicalize to absolute path
        // First check if the file exists to provide better error messages
        if !resolved_path.exists() {
            return Err(format!(
                "Module not found: '{}'\nSearched at: {}",
                module_path,
                resolved_path.display()
            ));
        }

        resolved_path.canonicalize().map_err(|e| {
            format!(
                "Cannot resolve module path '{}': {}",
                resolved_path.display(),
                e
            )
        })
    }

    /// Check if a module is currently being compiled (for circular dependency detection)
    ///
    /// # Arguments
    /// * `path` - The absolute path to check
    ///
    /// # Returns
    /// * `true` if the path is in the compilation stack (circular dependency)
    /// * `false` otherwise
    pub fn is_compiling(&self, path: &Path) -> bool {
        self.compilation_stack.contains(&path.to_path_buf())
    }

    /// Begin compiling a module
    ///
    /// # Arguments
    /// * `path` - The absolute path of the module being compiled
    /// * `location` - Source location for error reporting
    ///
    /// # Returns
    /// * `Ok(())` if compilation can proceed
    /// * `Err(CompilationError)` if circular dependency detected
    pub fn begin_compilation(
        &mut self,
        path: PathBuf,
        location: SourceLocation,
    ) -> Result<(), CompilationError> {
        if self.is_compiling(&path) {
            let cycle = self.format_circular_dependency(&path);
            return Err(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::Other,
                format!("Circular dependency detected: {}", cycle),
                location,
            ));
        }

        self.compilation_stack.push(path);
        Ok(())
    }

    /// End compiling a module
    ///
    /// # Arguments
    /// * `path` - The absolute path of the module that finished compilation
    pub fn end_compilation(&mut self, path: &Path) {
        if let Some(pos) = self.compilation_stack.iter().position(|p| p == path) {
            self.compilation_stack.remove(pos);
        }
    }

    /// Register a compiled module
    ///
    /// # Arguments
    /// * `module` - The compiled module to register
    pub fn register_module(&mut self, module: CompiledModule) {
        let path = module.path.clone();
        self.modules.insert(path, Rc::new(module));
    }

    /// Get a compiled module by path
    ///
    /// # Arguments
    /// * `path` - The absolute path of the module
    ///
    /// # Returns
    /// * `Some(Rc<CompiledModule>)` if the module is cached
    /// * `None` if the module has not been compiled yet
    pub fn get_module(&self, path: &Path) -> Option<Rc<CompiledModule>> {
        self.modules.get(path).cloned()
    }

    /// Check if a module has been compiled
    ///
    /// # Arguments
    /// * `path` - The absolute path of the module
    ///
    /// # Returns
    /// * `true` if the module is already compiled
    /// * `false` otherwise
    pub fn has_module(&self, path: &Path) -> bool {
        self.modules.contains_key(path)
    }

    /// Format the circular dependency chain for error reporting
    fn format_circular_dependency(&self, path: &Path) -> String {
        let mut cycle = self
            .compilation_stack
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>();
        cycle.push(path.display().to_string());
        cycle.join(" -> ")
    }

    /// Clear all cached modules (useful for testing)
    #[cfg(test)]
    pub fn clear(&mut self) {
        self.modules.clear();
        self.compilation_stack.clear();
    }
}

impl Default for ModuleResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_absolute_path() {
        let resolver = ModuleResolver::new();
        // Use a file that we know exists in the project
        let test_dir = std::env::current_dir().unwrap().join("src/compiler");
        let module_path = test_dir.join("mod.rs");

        if module_path.exists() {
            let resolved = resolver
                .resolve_path(module_path.to_str().unwrap(), None)
                .unwrap();
            assert_eq!(resolved, module_path.canonicalize().unwrap());
        }
    }

    #[test]
    fn test_resolve_adds_extension() {
        let resolver = ModuleResolver::new();
        // Use existing file without extension
        let test_dir = std::env::current_dir().unwrap().join("src/compiler");
        let module_path = test_dir.join("mod.rs");

        if module_path.exists() {
            // Remove extension and try to resolve
            let path_without_ext = test_dir.join("mod");
            // This will fail because it tries to add .n, but we can test the error
            let result = resolver.resolve_path(path_without_ext.to_str().unwrap(), None);
            // Should fail because mod.n doesn't exist
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_resolve_relative_path_needs_context() {
        let resolver = ModuleResolver::new();
        let test_dir = std::env::current_dir().unwrap().join("src/compiler");
        let parser_path = test_dir.join("parser.rs");
        let mod_path = test_dir.join("mod.rs");

        if parser_path.exists() && mod_path.exists() {
            // From parser.rs, try to import ./mod - should fail because mod.n doesn't exist
            let result = resolver.resolve_path("./mod", Some(&parser_path));
            // Should fail because it will look for mod.n
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_relative_import_without_current_file_fails() {
        let resolver = ModuleResolver::new();
        let result = resolver.resolve_path("./test", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires current file context"));
    }

    #[test]
    fn test_local_module_without_current_file_fails() {
        let resolver = ModuleResolver::new();
        // Local module (simple name) also requires context
        let result = resolver.resolve_path("math_lib", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires current file context"));
    }

    #[test]
    fn test_system_module_not_yet_implemented() {
        let resolver = ModuleResolver::new();
        // System module path (contains /)
        let result = resolver.resolve_path("system/io", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("System module imports not yet implemented"));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut resolver = ModuleResolver::new();
        let a_path = PathBuf::from("/tmp/a.n");
        let location = SourceLocation {
            line: 1,
            column: 1,
            offset: 0,
        };

        // Start compiling a.n
        resolver.begin_compilation(a_path.clone(), location).unwrap();
        assert!(resolver.is_compiling(&a_path));

        // Try to compile a.n again (circular dependency)
        let result = resolver.begin_compilation(a_path.clone(), location);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("Circular dependency"));
    }

    #[test]
    fn test_module_caching() {
        let mut resolver = ModuleResolver::new();
        let module_path = PathBuf::from("/tmp/test.n");

        // Create a dummy chunk
        let chunk = Chunk::new("test");

        // Register module
        let module = CompiledModule {
            path: module_path.clone(),
            chunk: Rc::new(chunk),
            exports: HashMap::new(),
        };
        resolver.register_module(module);

        // Check if module is cached
        assert!(resolver.has_module(&module_path));
        let cached = resolver.get_module(&module_path);
        assert!(cached.is_some());
    }

    #[test]
    fn test_end_compilation() {
        let mut resolver = ModuleResolver::new();
        let a_path = PathBuf::from("/tmp/a.n");
        let location = SourceLocation {
            line: 1,
            column: 1,
            offset: 0,
        };

        resolver.begin_compilation(a_path.clone(), location).unwrap();
        assert!(resolver.is_compiling(&a_path));

        resolver.end_compilation(&a_path);
        assert!(!resolver.is_compiling(&a_path));
    }
}
