/// Module system types - unified export representation
///
/// This module contains the single source of truth for module export information,
/// separating module metadata from bytecode representation.

use std::path::PathBuf;

/// Kind of exported symbol
#[derive(Debug, Clone, PartialEq)]
pub enum ExportKind {
    Function { arity: u8 },
    Variable,
    Struct { fields: Vec<String> },
}

/// Information about a single exported symbol
#[derive(Debug, Clone, PartialEq)]
pub struct ExportInfo {
    pub name: String,
    pub kind: ExportKind,
    pub global_index: usize,
}

/// Module metadata separate from bytecode
/// This contains all module-specific information that isn't part of the executable bytecode
#[derive(Debug, Clone)]
pub struct ModuleMetadata {
    /// Canonical path to the module source file
    pub source_path: PathBuf,
    /// List of symbols exported by this module
    pub exports: Vec<ExportInfo>,
}

impl ModuleMetadata {
    /// Create new module metadata
    pub fn new(source_path: PathBuf, exports: Vec<ExportInfo>) -> Self {
        ModuleMetadata {
            source_path,
            exports,
        }
    }

    /// Get an export by name
    pub fn get_export(&self, name: &str) -> Option<&ExportInfo> {
        self.exports.iter().find(|e| e.name == name)
    }
}
