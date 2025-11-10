use crate::common::SourceLocation;
use std::collections::HashMap;

/// Kind of symbol in the symbol table
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    /// Immutable value
    Value,
    /// Mutable variable
    Variable,
    /// Function with arity
    Function { arity: u8 },
    /// Struct with field names
    Struct { fields: Vec<String> },
    /// Function parameter
    Parameter,
}

/// Symbol in the symbol table
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    /// Name of the symbol
    pub name: String,
    /// Kind of symbol
    pub kind: SymbolKind,
    /// Whether the symbol is mutable (only for Value/Variable)
    pub is_mutable: bool,
    /// Scope depth where defined
    pub scope_depth: u32,
    /// Source location where defined
    pub location: SourceLocation,
    /// Index in the local/global array (for codegen)
    pub index: Option<u32>,
}

impl Symbol {
    pub fn new(
        name: String,
        kind: SymbolKind,
        is_mutable: bool,
        scope_depth: u32,
        location: SourceLocation,
    ) -> Self {
        Symbol {
            name,
            kind,
            is_mutable,
            scope_depth,
            location,
            index: None,
        }
    }

    pub fn with_index(mut self, index: u32) -> Self {
        self.index = Some(index);
        self
    }
}

/// A scope containing symbols
#[derive(Debug, Clone)]
pub struct Scope {
    /// Symbols defined in this scope
    symbols: HashMap<String, Symbol>,
    /// Parent scope index (None for global scope)
    parent: Option<usize>,
    /// Depth of this scope (0 for global)
    depth: u32,
}

impl Scope {
    pub fn new(parent: Option<usize>, depth: u32) -> Self {
        Scope {
            symbols: HashMap::new(),
            parent,
            depth,
        }
    }

    /// Define a new symbol in this scope
    pub fn define(&mut self, symbol: Symbol) -> Result<(), String> {
        if self.symbols.contains_key(&symbol.name) {
            return Err(format!(
                "Symbol '{}' already defined in this scope",
                symbol.name
            ));
        }
        self.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    /// Look up a symbol in this scope only (not parents)
    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// Get mutable reference to a symbol
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        self.symbols.get_mut(name)
    }

    /// Get all symbols in this scope
    pub fn symbols(&self) -> &HashMap<String, Symbol> {
        &self.symbols
    }
}

/// Symbol table managing all scopes
#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// All scopes (index 0 is global)
    scopes: Vec<Scope>,
    /// Current scope index
    current_scope: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            scopes: vec![Scope::new(None, 0)], // Global scope
            current_scope: 0,
        }
    }

    /// Enter a new scope
    pub fn enter_scope(&mut self) {
        let parent = self.current_scope;
        let depth = self.scopes[parent].depth + 1;
        self.scopes.push(Scope::new(Some(parent), depth));
        self.current_scope = self.scopes.len() - 1;
    }

    /// Exit the current scope
    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    /// Get current scope depth
    pub fn current_depth(&self) -> u32 {
        self.scopes[self.current_scope].depth
    }

    /// Define a symbol in the current scope
    pub fn define(&mut self, symbol: Symbol) -> Result<(), String> {
        self.scopes[self.current_scope].define(symbol)
    }

    /// Resolve a symbol by searching current scope and all parent scopes
    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        let mut scope_idx = self.current_scope;
        loop {
            if let Some(symbol) = self.scopes[scope_idx].get(name) {
                return Some(symbol);
            }
            // Check parent scope
            if let Some(parent) = self.scopes[scope_idx].parent {
                scope_idx = parent;
            } else {
                return None; // Reached global scope and didn't find it
            }
        }
    }

    /// Resolve a symbol mutably
    pub fn resolve_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        let mut scope_idx = self.current_scope;
        loop {
            // We need to check if symbol exists first, then get mutable reference
            let has_symbol = self.scopes[scope_idx].get(name).is_some();
            if has_symbol {
                return self.scopes[scope_idx].get_mut(name);
            }
            // Check parent scope
            if let Some(parent) = self.scopes[scope_idx].parent {
                scope_idx = parent;
            } else {
                return None;
            }
        }
    }

    /// Check if a symbol is defined in the current scope only
    pub fn is_defined_locally(&self, name: &str) -> bool {
        self.scopes[self.current_scope].get(name).is_some()
    }

    /// Get the global scope
    pub fn global_scope(&self) -> &Scope {
        &self.scopes[0]
    }

    /// Get current scope
    pub fn current_scope(&self) -> &Scope {
        &self.scopes[self.current_scope]
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
