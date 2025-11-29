use crate::compiler::semantic::SemanticAnalyzer;
use crate::compiler::parser::Parser;

#[test]
fn test_undefined_variable() {
    let program = "print x\n";
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Undefined variable 'x'"));
}

#[test]
fn test_defined_variable() {
    let program = "val x = 5\nprint x\n";
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_assign_to_immutable() {
    let program = "val x = 5\nx = 10\n";
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Cannot assign to immutable"));
}

#[test]
fn test_assign_to_mutable() {
    let program = "var x = 5\nx = 10\n";
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_function_scope() {
    let program = r#"
fn foo(a, b) {
    val c = a + b
    return c
}
val x = foo(1, 2)
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            eprintln!(
                "Error: {} at {}:{}",
                err.message, err.location.line, err.location.column
            );
        }
    }
    assert!(result.is_ok());
}

#[test]
fn test_function_arity_mismatch() {
    let program = r#"
fn add(a, b) {
    return a + b
}
val x = add(1, 2, 3)
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("expects 2 arguments but got 3")));
}

#[test]
fn test_nested_scopes() {
    let program = r#"
val x = 10
{
    val y = 20
    print x
    print y
}
print x
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_variable_shadowing() {
    let program = r#"
val x = 10
{
    val x = 20
    print x
}
print x
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_duplicate_declaration() {
    let program = r#"
val x = 10
val x = 20
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("already defined")));
}

#[test]
fn test_forward_function_reference() {
    let program = r#"
fn foo() {
    return bar()
}

fn bar() {
    return 42
}
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    // This should work because we collect all declarations first
    assert!(result.is_ok());
}

#[test]
fn test_calling_non_function() {
    let program = r#"
val x = 10
x()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("is not a function")));
}

// ===== Array Expression Tests =====

#[test]
fn test_array_literal_with_valid_elements() {
    let program = r#"
val x = 5
val y = 10
val arr = [x, y, 15]
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_array_literal_with_undefined_variable() {
    let program = "val arr = [1, 2, undefined_var]\n";
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Undefined variable 'undefined_var'"));
}

#[test]
fn test_array_literal_with_multiple_undefined_variables() {
    let program = "val arr = [foo, 2, bar]\n";
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2);
    assert!(errors.iter().any(|e| e.message.contains("Undefined variable 'foo'")));
    assert!(errors.iter().any(|e| e.message.contains("Undefined variable 'bar'")));
}

#[test]
fn test_array_literal_with_expressions() {
    let program = r#"
val a = 5
val b = 10
val arr = [a + b, a * 2, b - 3]
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_array_indexing_with_valid_expressions() {
    let program = r#"
val arr = [1, 2, 3]
val i = 0
val x = arr[i]
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_array_indexing_with_undefined_array() {
    let program = "val x = undefined_arr[0]\n";
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Undefined variable 'undefined_arr'"));
}

#[test]
fn test_array_indexing_with_undefined_index() {
    let program = r#"
val arr = [1, 2, 3]
val x = arr[undefined_idx]
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Undefined variable 'undefined_idx'"));
}

#[test]
fn test_array_indexing_with_both_undefined() {
    let program = "val x = undefined_arr[undefined_idx]\n";
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2);
    assert!(errors.iter().any(|e| e.message.contains("Undefined variable 'undefined_arr'")));
    assert!(errors.iter().any(|e| e.message.contains("Undefined variable 'undefined_idx'")));
}

#[test]
fn test_array_set_index_with_valid_expressions() {
    let program = r#"
var arr = [1, 2, 3]
val i = 1
val v = 42
arr[i] = v
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_array_set_index_with_undefined_array() {
    let program = "undefined_arr[0] = 42\n";
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Undefined variable 'undefined_arr'"));
}

#[test]
fn test_array_set_index_with_undefined_index() {
    let program = r#"
var arr = [1, 2, 3]
arr[undefined_idx] = 42
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Undefined variable 'undefined_idx'"));
}

#[test]
fn test_array_set_index_with_undefined_value() {
    let program = r#"
var arr = [1, 2, 3]
arr[0] = undefined_val
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Undefined variable 'undefined_val'"));
}

#[test]
fn test_array_set_index_with_all_undefined() {
    let program = "undefined_arr[undefined_idx] = undefined_val\n";
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 3);
    assert!(errors.iter().any(|e| e.message.contains("Undefined variable 'undefined_arr'")));
    assert!(errors.iter().any(|e| e.message.contains("Undefined variable 'undefined_idx'")));
    assert!(errors.iter().any(|e| e.message.contains("Undefined variable 'undefined_val'")));
}

#[test]
fn test_nested_array_literals() {
    let program = r#"
val x = 1
val y = 2
val arr = [[x, y], [3, 4], [5, 6]]
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_nested_array_indexing() {
    let program = r#"
val arr = [[1, 2], [3, 4]]
val i = 0
val j = 1
val x = arr[i][j]
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_array_in_function_call() {
    let program = r#"
fn process(arr) {
    return arr[0]
}
val result = process([1, 2, 3])
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_array_with_undefined_in_nested_expression() {
    let program = r#"
val arr = [1 + undefined_var, 2]
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Undefined variable 'undefined_var'"));
}
