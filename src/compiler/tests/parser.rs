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

#[test]
fn test_parse_method_call_no_args() {
    let program = r#"
        val result = str.len()
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();

    if result.is_err() {
        let errors = result.unwrap_err();
        for err in &errors {
            eprintln!("Parse error at {}:{}: {}", err.location.line, err.location.column, err.message);
        }
        panic!("Parse failed with {} errors", errors.len());
    }

    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);

    match &stmts[0] {
        Stmt::Val { initializer: Some(expr), .. } => {
            match expr {
                Expr::MethodCall { object, method, arguments, .. } => {
                    assert_eq!(method, "len");
                    assert_eq!(arguments.len(), 0);
                    match object.as_ref() {
                        Expr::Variable { name, .. } => assert_eq!(name, "str"),
                        _ => panic!("Expected Variable as object"),
                    }
                }
                _ => panic!("Expected MethodCall expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_method_call_one_arg() {
    let program = r#"
        val parts = str.split(",")
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();

    if result.is_err() {
        let errors = result.unwrap_err();
        for err in &errors {
            eprintln!("Parse error at {}:{}: {}", err.location.line, err.location.column, err.message);
        }
        panic!("Parse failed with {} errors", errors.len());
    }

    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);

    match &stmts[0] {
        Stmt::Val { initializer: Some(expr), .. } => {
            match expr {
                Expr::MethodCall { object, method, arguments, .. } => {
                    assert_eq!(method, "split");
                    assert_eq!(arguments.len(), 1);
                    match &arguments[0] {
                        Expr::String { value, .. } => assert_eq!(value, ","),
                        _ => panic!("Expected String argument"),
                    }
                    match object.as_ref() {
                        Expr::Variable { name, .. } => assert_eq!(name, "str"),
                        _ => panic!("Expected Variable as object"),
                    }
                }
                _ => panic!("Expected MethodCall expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_method_call_multiple_args() {
    let program = r#"
        val sub = str.substring(0, 5)
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();

    if result.is_err() {
        let errors = result.unwrap_err();
        for err in &errors {
            eprintln!("Parse error at {}:{}: {}", err.location.line, err.location.column, err.message);
        }
        panic!("Parse failed with {} errors", errors.len());
    }

    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);

    match &stmts[0] {
        Stmt::Val { initializer: Some(expr), .. } => {
            match expr {
                Expr::MethodCall { object, method, arguments, .. } => {
                    assert_eq!(method, "substring");
                    assert_eq!(arguments.len(), 2);
                    match &arguments[0] {
                        Expr::Number { value, .. } => assert_eq!(*value, 0.0),
                        _ => panic!("Expected Number argument"),
                    }
                    match &arguments[1] {
                        Expr::Number { value, .. } => assert_eq!(*value, 5.0),
                        _ => panic!("Expected Number argument"),
                    }
                    match object.as_ref() {
                        Expr::Variable { name, .. } => assert_eq!(name, "str"),
                        _ => panic!("Expected Variable as object"),
                    }
                }
                _ => panic!("Expected MethodCall expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_chained_method_calls() {
    let program = r#"
        val result = str.substring(0, 5).len()
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();

    if result.is_err() {
        let errors = result.unwrap_err();
        for err in &errors {
            eprintln!("Parse error at {}:{}: {}", err.location.line, err.location.column, err.message);
        }
        panic!("Parse failed with {} errors", errors.len());
    }

    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);

    match &stmts[0] {
        Stmt::Val { initializer: Some(expr), .. } => {
            // Outer method call should be .len()
            match expr {
                Expr::MethodCall { object, method, arguments, .. } => {
                    assert_eq!(method, "len");
                    assert_eq!(arguments.len(), 0);

                    // Inner object should be .substring(0, 5)
                    match object.as_ref() {
                        Expr::MethodCall { object: inner_obj, method: inner_method, arguments: inner_args, .. } => {
                            assert_eq!(inner_method, "substring");
                            assert_eq!(inner_args.len(), 2);

                            // Innermost object should be the variable 'str'
                            match inner_obj.as_ref() {
                                Expr::Variable { name, .. } => assert_eq!(name, "str"),
                                _ => panic!("Expected Variable as innermost object"),
                            }
                        }
                        _ => panic!("Expected MethodCall as inner object"),
                    }
                }
                _ => panic!("Expected MethodCall expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_method_call_vs_field_access() {
    let program = r#"
        val a = obj.field
        val b = obj.method()
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();

    if result.is_err() {
        let errors = result.unwrap_err();
        for err in &errors {
            eprintln!("Parse error at {}:{}: {}", err.location.line, err.location.column, err.message);
        }
        panic!("Parse failed with {} errors", errors.len());
    }

    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 2);

    // First should be field access
    match &stmts[0] {
        Stmt::Val { initializer: Some(expr), .. } => {
            match expr {
                Expr::GetField { field, .. } => assert_eq!(field, "field"),
                _ => panic!("Expected GetField expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }

    // Second should be method call
    match &stmts[1] {
        Stmt::Val { initializer: Some(expr), .. } => {
            match expr {
                Expr::MethodCall { method, .. } => assert_eq!(method, "method"),
                _ => panic!("Expected MethodCall expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}
