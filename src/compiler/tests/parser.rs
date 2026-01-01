use crate::compiler::parser::Parser;
use crate::compiler::ast::Stmt;

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
fn test_parse_loop() {
    let program = r#"
        loop {
            print 1
        }
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Loop { .. } => {}
        _ => panic!("Expected Loop statement"),
    }
}

#[test]
fn test_parse_break() {
    let program = r#"
        break
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Break { .. } => {}
        _ => panic!("Expected Break statement"),
    }
}

#[test]
fn test_parse_loop_with_break() {
    let program = r#"
        var i = 0
        loop {
            if (i > 5) {
                break
            }
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
        Stmt::Loop { body, .. } => {
            match body.as_ref() {
                Stmt::Block { statements, .. } => {
                    assert_eq!(statements.len(), 3); // if, print, assignment
                    match &statements[0] {
                        Stmt::If { then_branch, .. } => {
                            match then_branch.as_ref() {
                                Stmt::Block { statements, .. } => {
                                    assert_eq!(statements.len(), 1);
                                    match &statements[0] {
                                        Stmt::Break { .. } => {}
                                        _ => panic!("Expected Break statement in if body"),
                                    }
                                }
                                _ => panic!("Expected Block statement in if branch"),
                            }
                        }
                        _ => panic!("Expected If statement in loop body"),
                    }
                }
                _ => panic!("Expected Block statement in loop body"),
            }
        }
        _ => panic!("Expected Loop statement"),
    }
}
