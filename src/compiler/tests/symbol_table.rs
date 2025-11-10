use crate::compiler::symbol_table::{Symbol, SymbolKind, SymbolTable};
use crate::common::SourceLocation;

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
