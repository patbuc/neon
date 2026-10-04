use crate::common::SourceLocation;
use crate::compiler::resolutions::DeclId;
use std::collections::{HashMap, HashSet};

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
    /// Plain enum with variant names, in declaration order
    Enum { variants: Vec<String> },
    /// Function parameter
    Parameter,
    /// Built-in namespace (e.g. Math, File); usable only as `Name.method(...)`
    /// or, for namespaces with a constructor, as a call `Name(...)`
    Namespace,
    /// Runtime builtin value (e.g. `args`); index into `stdlib::BUILTIN_VALUES`
    Builtin { index: u32 },
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
    /// Identity of the declaration this symbol names
    pub decl_id: DeclId,
    /// Function-nesting level where this symbol was declared (0 = the script)
    pub function_level: u32,
}

impl Symbol {
    pub fn new(
        name: String,
        kind: SymbolKind,
        is_mutable: bool,
        scope_depth: u32,
        location: SourceLocation,
        decl_id: DeclId,
        function_level: u32,
    ) -> Self {
        Symbol {
            name,
            kind,
            is_mutable,
            scope_depth,
            location,
            decl_id,
            function_level,
        }
    }
}

/// A scope containing symbols
#[derive(Debug, Clone)]
pub struct Scope {
    /// Symbols defined in this scope
    symbols: HashMap<String, Symbol>,
    /// Names seeded from an earlier REPL line's globals, eligible to be
    /// redeclared by a `val`/`var`/`fn` in this line rather than rejected
    /// as a duplicate. Cleared for a name as soon as this line redeclares
    /// it, so a second redeclaration of the same name in the same line
    /// still conflicts.
    replaceable: HashSet<String>,
    /// Parent scope index (None for global scope)
    parent: Option<usize>,
    /// Depth of this scope (0 for global)
    depth: u32,
}

impl Scope {
    pub fn new(parent: Option<usize>, depth: u32) -> Self {
        Scope {
            symbols: HashMap::new(),
            replaceable: HashSet::new(),
            parent,
            depth,
        }
    }

    /// Define a new symbol in this scope. A name seeded from an earlier
    /// REPL line is replaced rather than rejected, unless the existing or
    /// the new binding is a struct or enum - type redefinition stays a
    /// `DuplicateSymbol` error.
    pub fn define(&mut self, symbol: Symbol) -> Result<(), String> {
        if let Some(existing) = self.symbols.get(&symbol.name) {
            let is_type = |kind: &SymbolKind| {
                matches!(kind, SymbolKind::Struct { .. } | SymbolKind::Enum { .. })
            };
            let can_replace = self.replaceable.contains(&symbol.name)
                && !is_type(&existing.kind)
                && !is_type(&symbol.kind);
            if can_replace {
                self.replaceable.remove(&symbol.name);
                self.symbols.insert(symbol.name.clone(), symbol);
                return Ok(());
            }
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
            // Global scope has no parent, so `?` ends the search
            scope_idx = self.scopes[scope_idx].parent?;
        }
    }

    /// Moves out every symbol defined in the global scope, for the
    /// `GlobalEnv` a later REPL line compiles against.
    #[allow(clippy::expect_used)]
    pub(crate) fn into_global_symbols(self) -> HashMap<String, Symbol> {
        self.scopes
            .into_iter()
            .next()
            .expect("scopes always starts with the global scope")
            .symbols
    }

    /// Defines symbols carried over from an earlier REPL line directly in
    /// the global scope, bypassing the duplicate check `define` applies,
    /// and marks each name replaceable by a `val`/`var`/`fn` later in this
    /// line.
    pub(crate) fn seed_globals(&mut self, symbols: impl IntoIterator<Item = Symbol>) {
        for symbol in symbols {
            self.scopes[0].replaceable.insert(symbol.name.clone());
            self.scopes[0].symbols.insert(symbol.name.clone(), symbol);
        }
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
