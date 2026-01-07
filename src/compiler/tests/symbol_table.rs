use crate::common::SourceLocation;
use crate::compiler::symbol_table::{MethodSignature, Symbol, SymbolKind, SymbolTable};
use std::collections::HashMap;

fn dummy_location() -> SourceLocation {
    SourceLocation {
        offset: 0,
        line: 1,
        column: 1,
    }
}

#[test]
fn test_symbol_creation() {
    let symbol = Symbol::new(
        "x".to_string(),
        SymbolKind::Value,
        false,
        0,
        dummy_location(),
    );
    assert_eq!(symbol.name, "x");
    assert!(!symbol.is_mutable);
    assert_eq!(symbol.scope_depth, 0);
}

#[test]
fn test_symbol_table_global() {
    let mut table = SymbolTable::new();
    let symbol = Symbol::new(
        "global_var".to_string(),
        SymbolKind::Variable,
        true,
        0,
        dummy_location(),
    );

    assert!(table.define(symbol).is_ok());
    assert!(table.resolve("global_var").is_some());
    assert_eq!(table.current_depth(), 0);
}

#[test]
fn test_symbol_table_nested_scopes() {
    let mut table = SymbolTable::new();

    // Define in global scope
    let global_sym = Symbol::new(
        "x".to_string(),
        SymbolKind::Value,
        false,
        0,
        dummy_location(),
    );
    table.define(global_sym).unwrap();

    // Enter nested scope
    table.enter_scope();
    assert_eq!(table.current_depth(), 1);

    // Define in nested scope
    let local_sym = Symbol::new(
        "y".to_string(),
        SymbolKind::Variable,
        true,
        1,
        dummy_location(),
    );
    table.define(local_sym).unwrap();

    // Can resolve both
    assert!(table.resolve("x").is_some());
    assert!(table.resolve("y").is_some());

    // Exit scope
    table.exit_scope();
    assert_eq!(table.current_depth(), 0);

    // Can still resolve global
    assert!(table.resolve("x").is_some());
    // But not local
    assert!(table.resolve("y").is_none());
}

#[test]
fn test_symbol_shadowing() {
    let mut table = SymbolTable::new();

    // Define x in global scope
    let global_x = Symbol::new(
        "x".to_string(),
        SymbolKind::Value,
        false,
        0,
        dummy_location(),
    );
    table.define(global_x).unwrap();

    // Enter nested scope
    table.enter_scope();

    // Define x again in nested scope (shadowing)
    let local_x = Symbol::new(
        "x".to_string(),
        SymbolKind::Variable,
        true,
        1,
        dummy_location(),
    );
    table.define(local_x).unwrap();

    // Resolve should find the local one
    let resolved = table.resolve("x").unwrap();
    assert_eq!(resolved.scope_depth, 1);
    assert!(resolved.is_mutable);
}

#[test]
fn test_duplicate_definition_error() {
    let mut table = SymbolTable::new();

    let sym1 = Symbol::new(
        "x".to_string(),
        SymbolKind::Value,
        false,
        0,
        dummy_location(),
    );
    table.define(sym1).unwrap();

    // Try to define again in same scope
    let sym2 = Symbol::new(
        "x".to_string(),
        SymbolKind::Variable,
        true,
        0,
        dummy_location(),
    );
    let result = table.define(sym2);
    assert!(result.is_err());
}

#[test]
fn test_struct_with_methods_creation() {
    // Create a struct symbol that can track methods
    let mut methods = HashMap::new();
    methods.insert(
        "distance".to_string(),
        MethodSignature {
            arity: 1, // parameter count excluding self
            is_static: false,
            is_mutating: false,
        },
    );

    let struct_symbol = Symbol::new(
        "Point".to_string(),
        SymbolKind::Struct {
            fields: vec!["x".to_string(), "y".to_string()],
            methods,
        },
        false,
        0,
        dummy_location(),
    );

    assert_eq!(struct_symbol.name, "Point");
    if let SymbolKind::Struct { fields, methods } = &struct_symbol.kind {
        assert_eq!(fields.len(), 2);
        assert_eq!(methods.len(), 1);
        assert!(methods.contains_key("distance"));
    } else {
        panic!("Expected Struct kind");
    }
}

#[test]
fn test_register_method_on_struct() {
    let mut table = SymbolTable::new();

    // Define a struct first
    let struct_symbol = Symbol::new(
        "Point".to_string(),
        SymbolKind::Struct {
            fields: vec!["x".to_string(), "y".to_string()],
            methods: HashMap::new(),
        },
        false,
        0,
        dummy_location(),
    );
    table.define(struct_symbol).unwrap();

    // Register a method on the struct
    let result = table.register_method(
        "Point",
        "distance",
        MethodSignature {
            arity: 1,
            is_static: false,
            is_mutating: false,
        },
    );
    assert!(result.is_ok());

    // Lookup the method
    let method = table.lookup_method("Point", "distance");
    assert!(method.is_some());
    let sig = method.unwrap();
    assert_eq!(sig.arity, 1);
    assert!(!sig.is_static);
    assert!(!sig.is_mutating);
}

#[test]
fn test_register_static_method() {
    let mut table = SymbolTable::new();

    let struct_symbol = Symbol::new(
        "Point".to_string(),
        SymbolKind::Struct {
            fields: vec!["x".to_string(), "y".to_string()],
            methods: HashMap::new(),
        },
        false,
        0,
        dummy_location(),
    );
    table.define(struct_symbol).unwrap();

    let result = table.register_method(
        "Point",
        "origin",
        MethodSignature {
            arity: 0,
            is_static: true,
            is_mutating: false,
        },
    );
    assert!(result.is_ok());

    let method = table.lookup_method("Point", "origin");
    assert!(method.is_some());
    let sig = method.unwrap();
    assert!(sig.is_static);
}

#[test]
fn test_register_mutating_method() {
    let mut table = SymbolTable::new();

    let struct_symbol = Symbol::new(
        "Point".to_string(),
        SymbolKind::Struct {
            fields: vec!["x".to_string(), "y".to_string()],
            methods: HashMap::new(),
        },
        false,
        0,
        dummy_location(),
    );
    table.define(struct_symbol).unwrap();

    let result = table.register_method(
        "Point",
        "translate",
        MethodSignature {
            arity: 2, // dx, dy - excluding self
            is_static: false,
            is_mutating: true,
        },
    );
    assert!(result.is_ok());

    let method = table.lookup_method("Point", "translate");
    assert!(method.is_some());
    let sig = method.unwrap();
    assert!(sig.is_mutating);
    assert!(!sig.is_static);
}

#[test]
fn test_duplicate_method_detection() {
    let mut table = SymbolTable::new();

    let struct_symbol = Symbol::new(
        "Point".to_string(),
        SymbolKind::Struct {
            fields: vec!["x".to_string(), "y".to_string()],
            methods: HashMap::new(),
        },
        false,
        0,
        dummy_location(),
    );
    table.define(struct_symbol).unwrap();

    // Register first method
    table
        .register_method(
            "Point",
            "distance",
            MethodSignature {
                arity: 1,
                is_static: false,
                is_mutating: false,
            },
        )
        .unwrap();

    // Try to register same method again - should fail
    let result = table.register_method(
        "Point",
        "distance",
        MethodSignature {
            arity: 1,
            is_static: false,
            is_mutating: false,
        },
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already defined"));
}

#[test]
fn test_lookup_method_on_nonexistent_struct() {
    let table = SymbolTable::new();
    let method = table.lookup_method("NonExistent", "method");
    assert!(method.is_none());
}

#[test]
fn test_lookup_nonexistent_method() {
    let mut table = SymbolTable::new();

    let struct_symbol = Symbol::new(
        "Point".to_string(),
        SymbolKind::Struct {
            fields: vec!["x".to_string(), "y".to_string()],
            methods: HashMap::new(),
        },
        false,
        0,
        dummy_location(),
    );
    table.define(struct_symbol).unwrap();

    let method = table.lookup_method("Point", "nonexistent");
    assert!(method.is_none());
}

#[test]
fn test_register_method_on_nonexistent_struct() {
    let mut table = SymbolTable::new();

    let result = table.register_method(
        "NonExistent",
        "method",
        MethodSignature {
            arity: 0,
            is_static: false,
            is_mutating: false,
        },
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn test_register_method_on_non_struct_symbol() {
    let mut table = SymbolTable::new();

    // Define a function, not a struct
    let func_symbol = Symbol::new(
        "myFunc".to_string(),
        SymbolKind::Function { arity: 2 },
        false,
        0,
        dummy_location(),
    );
    table.define(func_symbol).unwrap();

    let result = table.register_method(
        "myFunc",
        "method",
        MethodSignature {
            arity: 0,
            is_static: false,
            is_mutating: false,
        },
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not a struct"));
}
