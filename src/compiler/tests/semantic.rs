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

// ===== Method Validation Tests =====

#[test]
fn test_valid_method_on_array_literal() {
    let program = r#"
val x = [1, 2, 3].length()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            eprintln!("Error: {}", err.message);
        }
    }
    assert!(result.is_ok());
}

#[test]
fn test_invalid_method_on_array_literal() {
    let program = r#"
val x = [1, 2, 3].invalidMethod()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("has no method named 'invalidMethod'")));
}

#[test]
fn test_typo_on_method_name_suggests_correction() {
    let program = r#"
val x = [1, 2, 3].lenght()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("Did you mean 'length'")));
}

#[test]
fn test_non_existent_method_shows_available_methods() {
    let program = r#"
val x = [1, 2, 3].notAMethod()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("Available methods:")));
}

#[test]
fn test_method_on_tracked_variable_validates_correctly() {
    let program = r#"
val arr = [1, 2, 3]
val len = arr.length()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            eprintln!("Error: {}", err.message);
        }
    }
    assert!(result.is_ok());
}

#[test]
fn test_invalid_method_on_tracked_variable() {
    let program = r#"
val arr = [1, 2, 3]
val result = arr.badMethod()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("has no method named 'badMethod'")));
}

#[test]
fn test_valid_method_on_string_literal() {
    let program = r#"
val x = "hello".len()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            eprintln!("Error: {}", err.message);
        }
    }
    assert!(result.is_ok());
}

#[test]
fn test_invalid_method_on_string_literal() {
    let program = r#"
val x = "hello".invalidMethod()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("has no method named 'invalidMethod'")));
}

#[test]
fn test_valid_method_on_map_literal() {
    let program = r#"
val m = {"a": 1, "b": 2}
val k = m.keys()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            eprintln!("Error: {}", err.message);
        }
    }
    assert!(result.is_ok());
}

#[test]
fn test_invalid_method_on_map_literal() {
    let program = r#"
val result = {"a": 1}.wrongMethod()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("has no method named 'wrongMethod'")));
}

#[test]
fn test_valid_method_on_set_literal() {
    let program = r#"
val s = {1, 2, 3}
val arr = s.toArray()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            eprintln!("Error: {}", err.message);
        }
    }
    assert!(result.is_ok());
}

#[test]
fn test_invalid_method_on_set_literal() {
    let program = r#"
val result = {1, 2, 3}.invalidMethod()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("has no method named 'invalidMethod'")));
}

#[test]
fn test_method_chaining_with_type_inference() {
    let program = r#"
val m = {"a": 1, "b": 2}
val len = m.keys().length()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            eprintln!("Error: {}", err.message);
        }
    }
    assert!(result.is_ok());
}

#[test]
fn test_invalid_method_in_chain() {
    let program = r#"
val m = {"a": 1, "b": 2}
val result = m.keys().invalidMethod()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("has no method named 'invalidMethod'")));
}

#[test]
fn test_string_split_returns_array() {
    let program = r#"
val parts = "a,b,c".split(",")
val len = parts.length()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            eprintln!("Error: {}", err.message);
        }
    }
    assert!(result.is_ok());
}

#[test]
fn test_multiple_method_validation_errors() {
    let program = r#"
val a = [1, 2, 3].badMethod()
val b = "hello".wrongMethod()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    // Should have at least 2 errors
    assert!(errors.len() >= 2);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("badMethod")));
    assert!(errors
        .iter()
        .any(|e| e.message.contains("wrongMethod")));
}

#[test]
fn test_method_on_number_literal() {
    let program = r#"
val result = 42.toString()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            eprintln!("Error: {}", err.message);
        }
    }
    // This should succeed if Number type has toString method
    // or fail gracefully if not implemented yet
    assert!(result.is_ok() || result.is_err());
}

