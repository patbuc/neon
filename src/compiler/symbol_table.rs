use crate::common::SourceLocation;
use std::collections::HashMap;

/// Method signature stored in struct symbols
#[derive(Debug, Clone, PartialEq)]
pub struct MethodSignature {
    /// Parameter count excluding self
    pub arity: u8,
    /// True if no self parameter (called via Type.method())
    pub is_static: bool,
    /// True if first parameter is `mut self`
    pub is_mutating: bool,
}

/// Kind of symbol in the symbol table
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    /// Immutable value
    Value,
    /// Mutable variable
    Variable,
    /// Function with arity
    Function { arity: u8 },
    /// Struct with field names and methods
    Struct {
        fields: Vec<String>,
        methods: HashMap<String, MethodSignature>,
    },
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
        }
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

    /// Register a method on a struct symbol
    pub fn register_method(
        &mut self,
        struct_name: &str,
        method_name: &str,
        signature: MethodSignature,
    ) -> Result<(), String> {
        // Find the struct in all scopes
        let mut scope_idx = self.current_scope;
        loop {
            if let Some(symbol) = self.scopes[scope_idx].symbols.get_mut(struct_name) {
                if let SymbolKind::Struct { methods, .. } = &mut symbol.kind {
                    if methods.contains_key(method_name) {
                        return Err(format!(
                            "Method '{}' already defined on struct '{}'",
                            method_name, struct_name
                        ));
                    }
                    methods.insert(method_name.to_string(), signature);
                    return Ok(());
                } else {
                    return Err(format!("'{}' is not a struct", struct_name));
                }
            }
            if let Some(parent) = self.scopes[scope_idx].parent {
                scope_idx = parent;
            } else {
                return Err(format!("Struct '{}' not found", struct_name));
            }
        }
    }

    /// Look up a method signature on a struct
    pub fn lookup_method(&self, struct_name: &str, method_name: &str) -> Option<&MethodSignature> {
        let symbol = self.resolve(struct_name)?;
        if let SymbolKind::Struct { methods, .. } = &symbol.kind {
            methods.get(method_name)
        } else {
            None
        }
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
