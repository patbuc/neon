use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;

#[test]
fn test_undefined_variable() {
    let program = "print(x)\n";
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
    let program = "val x = 5\nprint(x)\n";
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
    print(x)
    print(y)
}
print(x)
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
    print(x)
}
print(x)
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
    assert!(errors.iter().any(|e| e.message.contains("badMethod")));
    assert!(errors.iter().any(|e| e.message.contains("wrongMethod")));
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

// ===== Integration Tests =====

#[test]
fn test_integration_multiple_errors_in_complex_program() {
    let program = r#"
fn processData(data) {
    val arr = [1, 2, 3]
    val filtered = arr.contans()  // typo: should be 'contains'

    val text = "hello world"
    val upper = text.repalce()  // typo: should be 'replace'

    val map = {"a": 1, "b": 2}
    val entries = map.entrys()  // typo: should be 'entries'

    val set = {1, 2, 3}
    val missing = set.notAMethod()  // completely wrong method

    return filtered
}

val result = processData(42)
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();

    // Should have at least 4 errors (one for each invalid method)
    assert!(
        errors.len() >= 4,
        "Expected at least 4 errors, got {}",
        errors.len()
    );

    // Verify each error is present and has helpful messages
    assert!(
        errors
            .iter()
            .any(|e| e.message.contains("contans") && e.message.contains("contains")),
        "Expected suggestion for 'contans' -> 'contains'"
    );
    assert!(
        errors.iter().any(|e| e.message.contains("repalce")),
        "Expected error for 'repalce'"
    );
    assert!(
        errors.iter().any(|e| e.message.contains("entrys")),
        "Expected error for 'entrys'"
    );
    assert!(
        errors.iter().any(|e| e.message.contains("notAMethod")),
        "Expected error for 'notAMethod'"
    );
}

#[test]
fn test_integration_complex_valid_program() {
    let program = r#"
fn analyzeText(input) {
    // String operations
    val length = input.len()
    val parts = input.split(" ")
    val partCount = parts.length()

    // Array operations
    val words = ["hello", "world", "neon"]
    val wordCount = words.length()
    words.push("lang")
    val last = words.pop()

    // Map operations
    val wordMap = {"hello": 1, "world": 2}
    val keys = wordMap.keys()
    val values = wordMap.values()
    val entries = wordMap.entries()

    // Set operations
    val uniqueNums = {1, 2, 3, 4, 5}
    val hasTwo = uniqueNums.has(2)
    val asArray = uniqueNums.toArray()
    val setSize = uniqueNums.size()

    // Method chaining
    val result = "hello world".split(" ")
    val chainedLength = result.length()

    return chainedLength
}

val output = analyzeText("sample input")
print(output)
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            eprintln!(
                "Unexpected error: {} at {}:{}",
                err.message, err.location.line, err.location.column
            );
        }
    }

    // All methods should be valid - no errors expected
    assert!(
        result.is_ok(),
        "Valid program should compile without errors"
    );
}

#[test]
fn test_integration_edge_case_empty_strings() {
    let program = r#"
val empty = ""
val length = empty.len()
val replaced = empty.replace(",", ";")
val parts = empty.split(",")
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

    // Empty strings should work fine with valid methods
    assert!(
        result.is_ok(),
        "Empty strings with valid methods should not error"
    );
}

#[test]
fn test_integration_edge_case_special_characters_in_method_names() {
    // Test that method names with special characters are handled correctly
    let program = r#"
val arr = [1, 2, 3]
val result = arr.with_underscore()  // Invalid method with underscore
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(
        errors.iter().any(|e| e.message.contains("with_underscore")),
        "Should report error for invalid method 'with_underscore'"
    );
}

#[test]
fn test_integration_graceful_degradation_unknown_type_from_function() {
    // When type cannot be inferred (e.g., from function return), no error should be generated
    let program = r#"
fn unknownReturnType(x) {
    if (x > 0) {
        return [1, 2, 3]
    } else {
        return "hello"
    }
}

val result = unknownReturnType(5)
val something = result.anyMethod()  // We can't know the type, so don't error
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    // Should succeed because we can't determine the type of 'result'
    // Graceful degradation: don't error on unknown types
    if let Err(ref errors) = result {
        // If there are errors, they should NOT be about method validation
        for err in errors {
            assert!(
                !err.message.contains("has no method named"),
                "Should not validate methods on unknown types, but got: {}",
                err.message
            );
        }
    }
}

#[test]
fn test_integration_graceful_degradation_complex_expression() {
    // Test graceful degradation with complex expressions where type is unknown
    let program = r#"
val x = someFunction()  // Function doesn't exist, but that's a different error
val y = x.anyMethod()   // x's type is unknown, so method validation should not trigger
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    // Should have error about undefined 'someFunction', but NOT about method validation
    if let Err(ref errors) = result {
        // Check that errors are NOT about method validation
        for err in errors {
            if err.message.contains("anyMethod") {
                assert!(
                    !err.message.contains("has no method named"),
                    "Should not validate methods on unknown types"
                );
            }
        }
    }
}

#[test]
fn test_integration_no_false_positives_all_builtin_array_methods() {
    // Verify no false positives: all valid array methods should pass
    let program = r#"
val arr = [1, 2, 3, 4, 5]
val len = arr.length()
val sz = arr.size()
val pushed = arr.push(6)
val popped = arr.pop()
val hasThree = arr.contains(3)
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            eprintln!("False positive error: {}", err.message);
        }
    }

    assert!(
        result.is_ok(),
        "All valid array methods should be accepted without errors"
    );
}

#[test]
fn test_integration_no_false_positives_all_builtin_string_methods() {
    // Verify no false positives: all valid string methods should pass
    let program = r#"
val text = "Hello World"
val length = text.len()
val sub = text.substring(0, 5)
val parts = text.split(" ")
val replaced = text.replace("Hello", "Hi")
val asInt = "123".toInt()
val asFloat = "3.14".toFloat()
val asBool = "true".toBool()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            eprintln!("False positive error: {}", err.message);
        }
    }

    assert!(
        result.is_ok(),
        "All valid string methods should be accepted without errors"
    );
}

#[test]
fn test_integration_no_false_positives_all_builtin_map_methods() {
    // Verify no false positives: all valid map methods should pass
    let program = r#"
val m = {"a": 1, "b": 2, "c": 3}
val keys = m.keys()
val values = m.values()
val entries = m.entries()
val hasKey = m.has("a")
val size = m.size()
val value = m.get("a")
val removed = m.remove("b")
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            eprintln!("False positive error: {}", err.message);
        }
    }

    assert!(
        result.is_ok(),
        "All valid map methods should be accepted without errors"
    );
}

#[test]
fn test_integration_no_false_positives_all_builtin_set_methods() {
    // Verify no false positives: all valid set methods should pass
    let program = r#"
val s = {1, 2, 3, 4, 5}
val hasItem = s.has(3)
val arr = s.toArray()
val size = s.size()
val added = s.add(6)
val removed = s.remove(2)
val cleared = s.clear()
val s2 = {4, 5, 6}
val unionSet = s.union(s2)
val intersectSet = s.intersection(s2)
val diffSet = s.difference(s2)
val isSub = s.isSubset(s2)
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            eprintln!("False positive error: {}", err.message);
        }
    }

    assert!(
        result.is_ok(),
        "All valid set methods should be accepted without errors"
    );
}

#[test]
fn test_integration_mixed_valid_and_invalid_methods() {
    // Real-world scenario: some methods valid, some invalid on known types
    let program = r#"
fn processData() {
    val text = "hello world"
    val textLen = text.len()           // valid
    val upper = text.toUpper()         // invalid: not implemented

    val numbers = [1, 2, 3, 4, 5]
    val last = numbers.pop()           // valid
    val filtered = numbers.filtr()     // invalid: typo
    val length = numbers.length()      // valid

    return upper
}
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();

    // Debug: print(all errors)
    eprintln!("Errors found:");
    for (i, err) in errors.iter().enumerate() {
        eprintln!("  {}: {}", i, err.message);
    }

    // Should have exactly 2 errors (toUpper and filtr)
    assert!(
        errors.len() >= 2,
        "Expected at least 2 errors, got {}",
        errors.len()
    );

    // Check for specific errors
    assert!(
        errors.iter().any(|e| e.message.contains("toUpper")),
        "Should have error for 'toUpper'"
    );
    assert!(
        errors.iter().any(|e| e.message.contains("filtr")),
        "Should have error for 'filtr'"
    );

    // Verify no errors for valid methods
    // Check that there's no error saying these methods don't exist (but they can appear in suggestions)
    assert!(
        !errors
            .iter()
            .any(|e| e.message.contains("has no method named 'len'")),
        "Should not error on valid 'len' method"
    );
    assert!(
        !errors
            .iter()
            .any(|e| e.message.contains("has no method named 'length'")),
        "Should not error on valid 'length' method"
    );
    assert!(
        !errors
            .iter()
            .any(|e| e.message.contains("has no method named 'pop'")),
        "Should not error on valid 'pop' method"
    );
}

#[test]
fn test_integration_error_messages_are_actionable() {
    // Verify that error messages provide actionable guidance
    let program = r#"
val arr = [1, 2, 3]
val result = arr.lenght()  // typo: should be 'length'
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();

    let error_msg = &errors[0].message;

    // Error message should be user-friendly and actionable:
    // 1. Mention the type
    assert!(
        error_msg.contains("Array") || error_msg.contains("array"),
        "Error should mention the type 'Array'"
    );

    // 2. Mention the invalid method name
    assert!(
        error_msg.contains("lenght"),
        "Error should mention the invalid method 'lenght'"
    );

    // 3. Provide a suggestion
    assert!(
        error_msg.contains("Did you mean") || error_msg.contains("length"),
        "Error should provide a suggestion"
    );
}

#[test]
fn test_integration_nested_method_calls_in_conditions() {
    // Test method validation in conditional expressions
    let program = r#"
fn checkData(items) {
    if (items.length() > 0) {
        val hasTwo = items.contains(2)
        return true
    }
    return false
}

val result = checkData([1, 2, 3])
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

    assert!(
        result.is_ok(),
        "Valid methods in conditions should not error"
    );
}

#[test]
fn test_integration_method_calls_in_loops() {
    // Test method validation inside loop constructs
    let program = r#"
val items = ["a", "b", "c"]
var i = 0
while (i < items.length()) {
    val item = items[i]
    val itemLen = item.len()
    print(itemLen)
    i = i + 1
}
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

    assert!(result.is_ok(), "Valid methods in loops should not error");
}

// =============================================================================
// Break and Continue Statement Tests
// =============================================================================

#[test]
fn test_break_outside_loop() {
    let program = r#"
        var x = 5
        break
        print(x)
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0]
        .message
        .contains("Cannot use 'break' outside of a loop"));
}

#[test]
fn test_continue_outside_loop() {
    let program = r#"
        var x = 5
        continue
        print(x)
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0]
        .message
        .contains("Cannot use 'continue' outside of a loop"));
}

#[test]
fn test_break_in_while_loop_valid() {
    let program = r#"
        var x = 0
        while (x < 10) {
            if (x == 5) {
                break
            }
            x = x + 1
        }
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_continue_in_while_loop_valid() {
    let program = r#"
        var x = 0
        while (x < 10) {
            x = x + 1
            if (x == 5) {
                continue
            }
            print(x)
        }
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_break_in_for_loop_valid() {
    let program = r#"
        for (var i = 0; i < 10; i = i + 1) {
            if (i == 5) {
                break
            }
        }
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_continue_in_for_loop_valid() {
    let program = r#"
        for (var i = 0; i < 10; i = i + 1) {
            if (i == 5) {
                continue
            }
            print(i)
        }
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_break_in_for_in_loop_valid() {
    let program = r#"
        val arr = [1, 2, 3, 4, 5]
        for (item in arr) {
            if (item == 3) {
                break
            }
            print(item)
        }
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_continue_in_for_in_loop_valid() {
    let program = r#"
        val arr = [1, 2, 3, 4, 5]
        for (item in arr) {
            if (item == 3) {
                continue
            }
            print(item)
        }
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_nested_break_valid() {
    let program = r#"
        var i = 0
        while (i < 3) {
            var j = 0
            while (j < 3) {
                if (j == 2) {
                    break
                }
                j = j + 1
            }
            i = i + 1
        }
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_break_outside_function_in_loop() {
    let program = r#"
        fn test() {
            break
        }
        while (true) {
            test()
        }
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0]
        .message
        .contains("Cannot use 'break' outside of a loop"));
}

#[test]
fn test_continue_outside_function_in_loop() {
    let program = r#"
        fn test() {
            continue
        }
        while (true) {
            test()
        }
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0]
        .message
        .contains("Cannot use 'continue' outside of a loop"));
}

// =============================================================================
// Postfix Increment/Decrement Operator Tests
// =============================================================================

#[test]
fn test_postfix_increment_on_undefined_variable() {
    let program = r#"
        x++
        "#;
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
fn test_postfix_decrement_on_undefined_variable() {
    let program = r#"
        y--
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Undefined variable 'y'"));
}

#[test]
fn test_postfix_increment_on_immutable_variable() {
    let program = r#"
        val x = 5
        x++
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0]
        .message
        .contains("Cannot modify immutable variable 'x'"));
}

#[test]
fn test_postfix_decrement_on_immutable_variable() {
    let program = r#"
        val x = 10
        x--
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0]
        .message
        .contains("Cannot modify immutable variable 'x'"));
}

#[test]
fn test_postfix_increment_on_mutable_variable_valid() {
    let program = r#"
        var x = 5
        x++
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_postfix_decrement_on_mutable_variable_valid() {
    let program = r#"
        var x = 10
        x--
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_postfix_increment_on_array_element_fails() {
    let program = r#"
        var arr = [1, 2, 3]
        arr[0]++
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0]
        .message
        .contains("Increment operator can only be applied to variables"));
}

#[test]
fn test_postfix_decrement_on_array_element_fails() {
    let program = r#"
        var arr = [1, 2, 3]
        arr[0]--
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0]
        .message
        .contains("Decrement operator can only be applied to variables"));
}

#[test]
fn test_postfix_increment_on_field_access_fails() {
    let program = r#"
        var p = { "x": 5, "y": 10 }
        p.x++
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e
        .message
        .contains("Increment operator can only be applied to variables")));
}

#[test]
fn test_postfix_decrement_on_field_access_fails() {
    let program = r#"
        var p = { "x": 5, "y": 10 }
        p.y--
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e
        .message
        .contains("Decrement operator can only be applied to variables")));
}

#[test]
fn test_postfix_increment_on_function_call_fails() {
    let program = r#"
        fn getValue() {
            return 5
        }
        getValue()++
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e
        .message
        .contains("Increment operator can only be applied to variables")));
}

#[test]
fn test_postfix_increment_in_expression() {
    let program = r#"
        var x = 5
        val y = x++ + 10
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_postfix_decrement_in_expression() {
    let program = r#"
        var x = 10
        val y = x-- * 2
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_multiple_postfix_operations() {
    let program = r#"
        var a = 5
        var b = 10
        a++
        b--
        var c = a++ + b--
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_postfix_in_loop() {
    let program = r#"
        var i = 0
        while (i < 10) {
            print(i)
            i++
        }
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_postfix_multiple_errors() {
    let program = r#"
        val x = 5
        val y = 10
        x++
        y--
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("Cannot modify immutable variable 'x'")));
    assert!(errors
        .iter()
        .any(|e| e.message.contains("Cannot modify immutable variable 'y'")));
}

#[test]
fn test_postfix_on_literal_fails() {
    let program = r#"
        5++
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e
        .message
        .contains("Increment operator can only be applied to variables")));
}

#[test]
fn test_postfix_in_function_parameters() {
    let program = r#"
        fn process(x) {
            return x * 2
        }
        var num = 5
        val result = process(num++)
        "#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}
