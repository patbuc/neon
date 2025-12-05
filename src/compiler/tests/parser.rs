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
fn test_parse_simple_else_if() {
    let program = r#"
        val x = 10
        if (x < 5) {
            print "less than 5"
        } else if (x < 15) {
            print "between 5 and 15"
        } else {
            print "15 or greater"
        }
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
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 2);

    // Check that the second statement is an If statement with an else branch
    match &stmts[1] {
        Stmt::If { else_branch, .. } => {
            assert!(else_branch.is_some());
            // The else branch should contain another If statement (the else-if)
            match else_branch.as_ref().unwrap().as_ref() {
                Stmt::If { else_branch: inner_else, .. } => {
                    // The inner else-if should have a final else branch
                    assert!(inner_else.is_some());
                }
                _ => panic!("Expected nested If statement in else branch"),
            }
        }
        _ => panic!("Expected If statement"),
    }
}

#[test]
fn test_parse_multiple_else_if_branches() {
    let program = r#"
        val score = 85
        if (score >= 90) {
            print "A"
        } else if (score >= 80) {
            print "B"
        } else if (score >= 70) {
            print "C"
        } else if (score >= 60) {
            print "D"
        } else {
            print "F"
        }
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
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 2);

    // Verify the chain of else-if statements
    match &stmts[1] {
        Stmt::If { else_branch, .. } => {
            assert!(else_branch.is_some());
            // First else-if
            match else_branch.as_ref().unwrap().as_ref() {
                Stmt::If { else_branch: else2, .. } => {
                    assert!(else2.is_some());
                    // Second else-if
                    match else2.as_ref().unwrap().as_ref() {
                        Stmt::If { else_branch: else3, .. } => {
                            assert!(else3.is_some());
                            // Third else-if
                            match else3.as_ref().unwrap().as_ref() {
                                Stmt::If { else_branch: else4, .. } => {
                                    assert!(else4.is_some());
                                    // Final else (should be a Block, not another If)
                                    match else4.as_ref().unwrap().as_ref() {
                                        Stmt::Block { .. } => {}
                                        _ => panic!("Expected Block statement in final else"),
                                    }
                                }
                                _ => panic!("Expected nested If statement (3rd else-if)"),
                            }
                        }
                        _ => panic!("Expected nested If statement (2nd else-if)"),
                    }
                }
                _ => panic!("Expected nested If statement (1st else-if)"),
            }
        }
        _ => panic!("Expected If statement"),
    }
}

#[test]
fn test_parse_else_if_without_final_else() {
    let program = r#"
        val x = 10
        if (x < 5) {
            print "less than 5"
        } else if (x < 15) {
            print "between 5 and 15"
        }
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
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 2);

    // Check that the second statement is an If statement with an else branch
    match &stmts[1] {
        Stmt::If { else_branch, .. } => {
            assert!(else_branch.is_some());
            // The else branch should contain another If statement without a final else
            match else_branch.as_ref().unwrap().as_ref() {
                Stmt::If { else_branch: inner_else, .. } => {
                    // The inner else-if should NOT have a final else branch
                    assert!(inner_else.is_none());
                }
                _ => panic!("Expected nested If statement in else branch"),
            }
        }
        _ => panic!("Expected If statement"),
    }
}

#[test]
fn test_parse_nested_if_within_else_if() {
    let program = r#"
        val x = 10
        val y = 20
        if (x < 5) {
            print "x less than 5"
        } else if (x < 15) {
            if (y > 15) {
                print "x between 5 and 15, y greater than 15"
            } else {
                print "x between 5 and 15, y not greater than 15"
            }
        } else {
            print "x is 15 or greater"
        }
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
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 3); // val x, val y, if statement

    // Check the structure of nested if within else-if
    match &stmts[2] {
        Stmt::If { else_branch, .. } => {
            assert!(else_branch.is_some());
            // The else branch should be an else-if (If statement)
            match else_branch.as_ref().unwrap().as_ref() {
                Stmt::If { then_branch, else_branch: outer_else, .. } => {
                    // The then_branch of the else-if should contain a nested if
                    match then_branch.as_ref() {
                        Stmt::Block { statements, .. } => {
                            assert_eq!(statements.len(), 1);
                            match &statements[0] {
                                Stmt::If { .. } => {}
                                _ => panic!("Expected nested If statement inside else-if block"),
                            }
                        }
                        _ => panic!("Expected Block in then_branch of else-if"),
                    }
                    // Verify the outer else exists
                    assert!(outer_else.is_some());
                }
                _ => panic!("Expected nested If statement in else branch"),
            }
        }
        _ => panic!("Expected If statement"),
    }
}
