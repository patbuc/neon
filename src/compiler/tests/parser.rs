use crate::compiler::parser::Parser;
use crate::compiler::ast::{Expr, Stmt};

#[test]
fn test_parse_number() {
    let mut parser = Parser::new("42");
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_val_declaration() {
    let mut parser = Parser::new("val x = 5\n");
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Val { name, .. } => assert_eq!(name, "x"),
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_binary_expression() {
    let mut parser = Parser::new("1 + 2\n");
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_parse_function() {
    let mut parser = Parser::new("fn foo(a, b) {\n  print a\n}\n");
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Fn { name, params, .. } => {
            assert_eq!(name, "foo");
            assert_eq!(params.len(), 2);
        }
        _ => panic!("Expected Fn statement"),
    }
}

#[test]
fn test_parse_complex_program() {
    let program = r#"
        val x = 10
        var y = 20
        
        fn add(a, b) {
            return a + b
        }
        
        fn factorial(n) {
            if (n <= 1) {
                return 1
            }
            return n * factorial(n - 1)
        }
        
        val result = add(x, y)
        print result
        print factorial(5)
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();

    if result.is_err() {
        let errors = result.unwrap_err();
        for err in &errors {
            eprintln!(
                "Parse error at {}:{}: {}",
                err.location.line, err.location.column, err.message
            );
        }
        panic!("Parse failed with {} errors", errors.len());
    }

    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 7); // val x, var y, fn add, fn factorial, val result, print result, print factorial
}

#[test]
fn test_parse_struct() {
    let program = r#"
        struct Point {
            x
            y
        }

        val p = Point()
        p.x = 10
        p.y = 20
        print p.x
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert!(!stmts.is_empty());
    match &stmts[0] {
        Stmt::Struct { name, fields, .. } => {
            assert_eq!(name, "Point");
            assert_eq!(fields.len(), 2);
        }
        _ => panic!("Expected Struct statement"),
    }
}

#[test]
fn test_parse_while_loop() {
    let program = r#"
        var i = 0
        while (i < 10) {
            print i
            i = i + 1
        }
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 2);
    match &stmts[1] {
        Stmt::While { .. } => {}
        _ => panic!("Expected While statement"),
    }
}

// ===== Map Literal Tests =====

#[test]
fn test_parse_empty_map() {
    let program = "val m = {}\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Val { initializer, .. } => {
            if let Some(Expr::Map { entries, .. }) = initializer {
                assert_eq!(entries.len(), 0);
            } else {
                panic!("Expected Map expression");
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_map_with_identifier_keys() {
    let program = "val m = {a: 1, b: 2}\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Val { initializer, .. } => {
            if let Some(Expr::Map { entries, .. }) = initializer {
                assert_eq!(entries.len(), 2);
                assert_eq!(entries[0].0, "a");
                assert_eq!(entries[1].0, "b");
            } else {
                panic!("Expected Map expression");
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_map_with_string_keys() {
    let program = "val m = {\"key1\": 10, \"key2\": 20}\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Val { initializer, .. } => {
            if let Some(Expr::Map { entries, .. }) = initializer {
                assert_eq!(entries.len(), 2);
                assert_eq!(entries[0].0, "key1");
                assert_eq!(entries[1].0, "key2");
            } else {
                panic!("Expected Map expression");
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_map_with_mixed_keys() {
    let program = "val m = {a: 1, \"b\": 2, c: 3}\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Val { initializer, .. } => {
            if let Some(Expr::Map { entries, .. }) = initializer {
                assert_eq!(entries.len(), 3);
                assert_eq!(entries[0].0, "a");
                assert_eq!(entries[1].0, "b");
                assert_eq!(entries[2].0, "c");
            } else {
                panic!("Expected Map expression");
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_map_with_trailing_comma() {
    let program = "val m = {a: 1, b: 2,}\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Val { initializer, .. } => {
            if let Some(Expr::Map { entries, .. }) = initializer {
                assert_eq!(entries.len(), 2);
            } else {
                panic!("Expected Map expression");
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_nested_maps() {
    let program = "val m = {a: {b: 1}, c: {d: 2}}\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Val { initializer, .. } => {
            if let Some(Expr::Map { entries, .. }) = initializer {
                assert_eq!(entries.len(), 2);
                // Check that values are also maps
                match &entries[0].1 {
                    Expr::Map { .. } => {}
                    _ => panic!("Expected nested map"),
                }
            } else {
                panic!("Expected Map expression");
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

// ===== Set Literal Tests =====

#[test]
fn test_parse_empty_set() {
    let program = "val s = #{}\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Val { initializer, .. } => {
            if let Some(Expr::Set { elements, .. }) = initializer {
                assert_eq!(elements.len(), 0);
            } else {
                panic!("Expected Set expression");
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_set_with_numbers() {
    let program = "val s = #{1, 2, 3}\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Val { initializer, .. } => {
            if let Some(Expr::Set { elements, .. }) = initializer {
                assert_eq!(elements.len(), 3);
            } else {
                panic!("Expected Set expression");
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_set_with_expressions() {
    let program = "val s = #{1 + 1, 2 * 2, 3}\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Val { initializer, .. } => {
            if let Some(Expr::Set { elements, .. }) = initializer {
                assert_eq!(elements.len(), 3);
            } else {
                panic!("Expected Set expression");
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_set_with_trailing_comma() {
    let program = "val s = #{1, 2, 3,}\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Val { initializer, .. } => {
            if let Some(Expr::Set { elements, .. }) = initializer {
                assert_eq!(elements.len(), 3);
            } else {
                panic!("Expected Set expression");
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

// ===== Method Call Tests =====

#[test]
fn test_parse_method_call_no_args() {
    let program = "map.clear()\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expression { expr, .. } => {
            if let Expr::MethodCall { method, arguments, .. } = expr {
                assert_eq!(method, "clear");
                assert_eq!(arguments.len(), 0);
            } else {
                panic!("Expected MethodCall expression");
            }
        }
        _ => panic!("Expected Expression statement"),
    }
}

#[test]
fn test_parse_method_call_with_args() {
    let program = "map.get(\"key\")\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expression { expr, .. } => {
            if let Expr::MethodCall { method, arguments, .. } = expr {
                assert_eq!(method, "get");
                assert_eq!(arguments.len(), 1);
            } else {
                panic!("Expected MethodCall expression");
            }
        }
        _ => panic!("Expected Expression statement"),
    }
}

#[test]
fn test_parse_method_call_multiple_args() {
    let program = "map.set(\"key\", 42)\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expression { expr, .. } => {
            if let Expr::MethodCall { method, arguments, .. } = expr {
                assert_eq!(method, "set");
                assert_eq!(arguments.len(), 2);
            } else {
                panic!("Expected MethodCall expression");
            }
        }
        _ => panic!("Expected Expression statement"),
    }
}

#[test]
fn test_parse_chained_method_calls() {
    let program = "map.get(\"key\").toString()\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Expression { expr, .. } => {
            if let Expr::MethodCall { method, object, .. } = expr {
                assert_eq!(method, "toString");
                // The object should itself be a MethodCall
                if let Expr::MethodCall { method: inner_method, .. } = &**object {
                    assert_eq!(inner_method, "get");
                } else {
                    panic!("Expected chained MethodCall");
                }
            } else {
                panic!("Expected MethodCall expression");
            }
        }
        _ => panic!("Expected Expression statement"),
    }
}

// ===== Integration Tests =====

#[test]
fn test_parse_map_operations() {
    let program = r#"
        val map = {a: 1, b: 2}
        map.set("c", 3)
        val value = map.get("a")
        print value
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();

    if result.is_err() {
        let errors = result.unwrap_err();
        for err in &errors {
            eprintln!(
                "Parse error at {}:{}: {}",
                err.location.line, err.location.column, err.message
            );
        }
        panic!("Parse failed with {} errors", errors.len());
    }

    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 4);
}

#[test]
fn test_parse_set_operations() {
    let program = r#"
        val s = #{1, 2, 3}
        s.add(4)
        val has = s.contains(2)
        print has
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();

    if result.is_err() {
        let errors = result.unwrap_err();
        for err in &errors {
            eprintln!(
                "Parse error at {}:{}: {}",
                err.location.line, err.location.column, err.message
            );
        }
        panic!("Parse failed with {} errors", errors.len());
    }

    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 4);
}

#[test]
fn test_parse_complex_collections() {
    let program = r#"
        val data = {
            users: #{1, 2, 3},
            config: {
                debug: true,
                port: 8080
            }
        }

        val users = data.get("users")
        users.add(4)

        val port = data.get("config").get("port")
        print port
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();

    if result.is_err() {
        let errors = result.unwrap_err();
        for err in &errors {
            eprintln!(
                "Parse error at {}:{}: {}",
                err.location.line, err.location.column, err.message
            );
        }
        panic!("Parse failed with {} errors", errors.len());
    }

    assert!(result.is_ok());
}
