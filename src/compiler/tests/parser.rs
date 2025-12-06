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
#[test]
fn test_parse_for_loop() {
    let program = r#"
        for (var i = 0; i < 10; i = i + 1) {
            print i
        }
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    // For loop is desugared to a Block containing a declaration and a While loop
    match &stmts[0] {
        Stmt::Block { statements, .. } => {
            assert_eq!(statements.len(), 2);
            // First statement should be the initializer (var declaration)
            match &statements[0] {
                Stmt::Var { name, .. } => {
                    assert_eq!(name, "i");
                }
                _ => panic!("Expected Var declaration as first statement in for loop"),
            }
            // Second statement should be the while loop
            match &statements[1] {
                Stmt::While { condition, body, .. } => {
                    // Verify condition is a binary comparison
                    match condition {
                        Expr::Binary { .. } => {}
                        _ => panic!("Expected Binary expression for condition"),
                    }
                    // Verify body is a Block containing original body and increment
                    match body.as_ref() {
                        Stmt::Block { statements: while_stmts, .. } => {
                            assert_eq!(while_stmts.len(), 2, "While body should contain original body + increment");
                        }
                        _ => panic!("Expected Block as while body"),
                    }
                }
                _ => panic!("Expected While statement as second statement in for loop"),
            }
        }
        _ => panic!("Expected Block statement for for loop desugaring"),
    }
}

#[test]
fn test_parse_for_loop_with_val() {
    let program = r#"
        for (val i = 0; i < 10; i = i + 1) {
            print i
        }
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    // For loop with val should parse successfully (runtime will catch the error)
    match &stmts[0] {
        Stmt::Block { statements, .. } => {
            assert_eq!(statements.len(), 2);
            // First statement should be the initializer (val declaration)
            match &statements[0] {
                Stmt::Val { name, .. } => {
                    assert_eq!(name, "i");
                }
                _ => panic!("Expected Val declaration as first statement in for loop"),
            }
            // Second statement should be the while loop
            match &statements[1] {
                Stmt::While { .. } => {}
                _ => panic!("Expected While statement as second statement in for loop"),
            }
        }
        _ => panic!("Expected Block statement for for loop desugaring"),
    }
}

#[test]
fn test_parse_for_loop_empty_body() {
    let program = r#"
        for (var i = 0; i < 10; i = i + 1) {
        }
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Block { statements, .. } => {
            assert_eq!(statements.len(), 2);
            // Verify the while loop body contains empty block + increment
            match &statements[1] {
                Stmt::While { body, .. } => {
                    match body.as_ref() {
                        Stmt::Block { statements: while_stmts, .. } => {
                            assert_eq!(while_stmts.len(), 2);
                            // First should be the empty body block
                            match &while_stmts[0] {
                                Stmt::Block { statements: empty_stmts, .. } => {
                                    assert_eq!(empty_stmts.len(), 0, "Body should be empty");
                                }
                                _ => panic!("Expected Block for empty body"),
                            }
                        }
                        _ => panic!("Expected Block as while body"),
                    }
                }
                _ => panic!("Expected While statement"),
            }
        }
        _ => panic!("Expected Block statement"),
    }
}

#[test]
fn test_parse_nested_for_loops() {
    let program = r#"
        for (var i = 0; i < 3; i = i + 1) {
            for (var j = 0; j < 3; j = j + 1) {
                print i
                print j
            }
        }
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    // Outer for loop
    match &stmts[0] {
        Stmt::Block { statements, .. } => {
            assert_eq!(statements.len(), 2);
            // Verify outer var declaration
            match &statements[0] {
                Stmt::Var { name, .. } => assert_eq!(name, "i"),
                _ => panic!("Expected Var declaration for outer loop"),
            }
            // Verify outer while loop
            match &statements[1] {
                Stmt::While { body, .. } => {
                    match body.as_ref() {
                        Stmt::Block { statements: outer_while_stmts, .. } => {
                            assert_eq!(outer_while_stmts.len(), 2);
                            // First statement should be a Block (from the { } in source code)
                            match &outer_while_stmts[0] {
                                Stmt::Block { statements: source_block_stmts, .. } => {
                                    assert_eq!(source_block_stmts.len(), 1, "Source block should contain one statement (the desugared inner for loop)");
                                    // That statement should be the desugared inner for loop (another Block)
                                    match &source_block_stmts[0] {
                                        Stmt::Block { statements: inner_for_stmts, .. } => {
                                            assert_eq!(inner_for_stmts.len(), 2);
                                            // Verify inner var declaration
                                            match &inner_for_stmts[0] {
                                                Stmt::Var { name, .. } => assert_eq!(name, "j"),
                                                _ => panic!("Expected Var declaration for inner loop"),
                                            }
                                            // Verify inner while loop exists
                                            match &inner_for_stmts[1] {
                                                Stmt::While { .. } => {}
                                                _ => panic!("Expected While statement for inner loop"),
                                            }
                                        }
                                        _ => panic!("Expected Block for desugared inner for loop"),
                                    }
                                }
                                _ => panic!("Expected Block from source code braces"),
                            }
                        }
                        _ => panic!("Expected Block as outer while body"),
                    }
                }
                _ => panic!("Expected While statement for outer loop"),
            }
        }
        _ => panic!("Expected Block statement for outer for loop"),
    }
}

#[test]
fn test_parse_for_loop_missing_semicolon_after_init() {
    let program = r#"
        for (var i = 0 i < 10; i = i + 1) {
            print i
        }
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_err(), "Should fail when missing semicolon after init");
    let errors = result.unwrap_err();
    assert!(!errors.is_empty());
    assert!(errors[0].message.contains("';'") || errors[0].message.contains("after loop initializer"));
}

#[test]
fn test_parse_for_loop_missing_semicolon_after_condition() {
    let program = r#"
        for (var i = 0; i < 10 i = i + 1) {
            print i
        }
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_err(), "Should fail when missing semicolon after condition");
    let errors = result.unwrap_err();
    assert!(!errors.is_empty());
    assert!(errors[0].message.contains("';'") || errors[0].message.contains("after loop condition"));
}

#[test]
fn test_parse_for_loop_missing_left_paren() {
    let program = r#"
        for var i = 0; i < 10; i = i + 1) {
            print i
        }
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_err(), "Should fail when missing left parenthesis");
    let errors = result.unwrap_err();
    assert!(!errors.is_empty());
    assert!(errors[0].message.contains("'('") || errors[0].message.contains("after 'for'"));
}

#[test]
fn test_parse_for_loop_missing_right_paren() {
    let program = r#"
        for (var i = 0; i < 10; i = i + 1 {
            print i
        }
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_err(), "Should fail when missing right parenthesis");
    let errors = result.unwrap_err();
    assert!(!errors.is_empty());
    assert!(errors[0].message.contains("')'") || errors[0].message.contains("after for clauses"));
}

#[test]
fn test_parse_for_loop_invalid_init_not_declaration() {
    let program = r#"
        for (i = 0; i < 10; i = i + 1) {
            print i
        }
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_err(), "Should fail when init is not a val/var declaration");
    let errors = result.unwrap_err();
    assert!(!errors.is_empty());
    assert!(
        errors[0].message.contains("val") || errors[0].message.contains("var") || errors[0].message.contains("initializer"),
        "Error should mention val/var requirement, got: {}", errors[0].message
    );
}

#[test]
fn test_parse_for_loop_complex_increment() {
    let program = r#"
        for (var i = 0; i < 10; i = i + 2) {
            print i
        }
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    // Verify the increment expression is part of the while body
    match &stmts[0] {
        Stmt::Block { statements, .. } => {
            match &statements[1] {
                Stmt::While { body, .. } => {
                    match body.as_ref() {
                        Stmt::Block { statements: while_stmts, .. } => {
                            assert_eq!(while_stmts.len(), 2);
                            // Second statement should be the increment
                            match &while_stmts[1] {
                                Stmt::Expression { expr, .. } => {
                                    // Should be an assignment expression
                                    match expr {
                                        Expr::Assign { .. } => {}
                                        _ => panic!("Expected Assign expression for increment"),
                                    }
                                }
                                _ => panic!("Expected Expression statement for increment"),
                            }
                        }
                        _ => panic!("Expected Block as while body"),
                    }
                }
                _ => panic!("Expected While statement"),
            }
        }
        _ => panic!("Expected Block statement"),
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

#[test]
fn test_parse_logical_and() {
    let mut parser = Parser::new("true && false\n");
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_logical_or() {
    let mut parser = Parser::new("true || false\n");
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_parse_logical_and_with_variables() {
    let mut parser = Parser::new("x && y\n");
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_parse_logical_or_with_variables() {
    let mut parser = Parser::new("x || y\n");
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_parse_logical_mixed() {
    let mut parser = Parser::new("a && b || c\n");
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_parse_logical_with_comparisons() {
    let mut parser = Parser::new("x > 5 && y < 10\n");
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_parse_logical_with_parentheses() {
    let mut parser = Parser::new("(x || y) && z\n");
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_parse_logical_in_if() {
    let program = r#"
        if (x > 0 && y > 0) {
            print "both positive"
        }
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
    let stmts = result.unwrap();
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::If { .. } => {}
        _ => panic!("Expected If statement"),
    }
}

#[test]
fn test_parse_complex_logical_expression() {
    let program = r#"
        val result = (a && b) || (c && d) || (e && f)
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();
    assert!(result.is_ok());
}

// ===== Map Literal Tests =====

#[test]
fn test_parse_empty_map() {
    let program = "val m = {}\n";
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
                Expr::MapLiteral { entries, .. } => {
                    assert_eq!(entries.len(), 0);
                }
                _ => panic!("Expected MapLiteral expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_map_with_string_keys() {
    let program = r#"
        val m = {"name": "Alice", "age": 30}
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
                Expr::MapLiteral { entries, .. } => {
                    assert_eq!(entries.len(), 2);

                    // First entry: "name": "Alice"
                    match &entries[0].0 {
                        Expr::String { value, .. } => assert_eq!(value, "name"),
                        _ => panic!("Expected String key"),
                    }
                    match &entries[0].1 {
                        Expr::String { value, .. } => assert_eq!(value, "Alice"),
                        _ => panic!("Expected String value"),
                    }

                    // Second entry: "age": 30
                    match &entries[1].0 {
                        Expr::String { value, .. } => assert_eq!(value, "age"),
                        _ => panic!("Expected String key"),
                    }
                    match &entries[1].1 {
                        Expr::Number { value, .. } => assert_eq!(*value, 30.0),
                        _ => panic!("Expected Number value"),
                    }
                }
                _ => panic!("Expected MapLiteral expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_map_with_variable_keys() {
    let program = r#"
        val m = {x: 10, y: 20}
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
                Expr::MapLiteral { entries, .. } => {
                    assert_eq!(entries.len(), 2);

                    // First entry: x: 10
                    match &entries[0].0 {
                        Expr::Variable { name, .. } => assert_eq!(name, "x"),
                        _ => panic!("Expected Variable key"),
                    }
                    match &entries[0].1 {
                        Expr::Number { value, .. } => assert_eq!(*value, 10.0),
                        _ => panic!("Expected Number value"),
                    }
                }
                _ => panic!("Expected MapLiteral expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_nested_map() {
    let program = r#"
        val m = {"outer": {"inner": 42}}
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
                Expr::MapLiteral { entries, .. } => {
                    assert_eq!(entries.len(), 1);

                    // Value should be another map
                    match &entries[0].1 {
                        Expr::MapLiteral { entries: inner_entries, .. } => {
                            assert_eq!(inner_entries.len(), 1);
                            match &inner_entries[0].1 {
                                Expr::Number { value, .. } => assert_eq!(*value, 42.0),
                                _ => panic!("Expected Number value"),
                            }
                        }
                        _ => panic!("Expected nested MapLiteral"),
                    }
                }
                _ => panic!("Expected MapLiteral expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_map_with_trailing_comma() {
    let program = r#"
        val m = {"a": 1, "b": 2,}
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
                Expr::MapLiteral { entries, .. } => {
                    assert_eq!(entries.len(), 2);
                }
                _ => panic!("Expected MapLiteral expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_map_with_multiline() {
    let program = r#"
        val m = {
            "name": "Bob",
            "age": 25
        }
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
}

// ===== Index Access Tests =====

#[test]
fn test_parse_index_access() {
    let program = r#"
        val x = m["key"]
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
                Expr::Index { object, index, .. } => {
                    match object.as_ref() {
                        Expr::Variable { name, .. } => assert_eq!(name, "m"),
                        _ => panic!("Expected Variable as object"),
                    }
                    match index.as_ref() {
                        Expr::String { value, .. } => assert_eq!(value, "key"),
                        _ => panic!("Expected String as index"),
                    }
                }
                _ => panic!("Expected Index expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_index_assignment() {
    let program = r#"
        m["key"] = 42
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
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::IndexAssign { object, index, value, .. } => {
                    match object.as_ref() {
                        Expr::Variable { name, .. } => assert_eq!(name, "m"),
                        _ => panic!("Expected Variable as object"),
                    }
                    match index.as_ref() {
                        Expr::String { value, .. } => assert_eq!(value, "key"),
                        _ => panic!("Expected String as index"),
                    }
                    match value.as_ref() {
                        Expr::Number { value, .. } => assert_eq!(*value, 42.0),
                        _ => panic!("Expected Number as value"),
                    }
                }
                _ => panic!("Expected IndexAssign expression"),
            }
        }
        _ => panic!("Expected Expression statement"),
    }
}

#[test]
fn test_parse_chained_index_access() {
    let program = r#"
        val x = m["outer"]["inner"]
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
            // Outer index should be ["inner"]
            match expr {
                Expr::Index { object, index, .. } => {
                    match index.as_ref() {
                        Expr::String { value, .. } => assert_eq!(value, "inner"),
                        _ => panic!("Expected String as outer index"),
                    }

                    // Inner object should be m["outer"]
                    match object.as_ref() {
                        Expr::Index { object: inner_obj, index: inner_idx, .. } => {
                            match inner_obj.as_ref() {
                                Expr::Variable { name, .. } => assert_eq!(name, "m"),
                                _ => panic!("Expected Variable as inner object"),
                            }
                            match inner_idx.as_ref() {
                                Expr::String { value, .. } => assert_eq!(value, "outer"),
                                _ => panic!("Expected String as inner index"),
                            }
                        }
                        _ => panic!("Expected Index as inner object"),
                    }
                }
                _ => panic!("Expected Index expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_index_with_variable_key() {
    let program = r#"
        val x = m[key]
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
                Expr::Index { index, .. } => {
                    match index.as_ref() {
                        Expr::Variable { name, .. } => assert_eq!(name, "key"),
                        _ => panic!("Expected Variable as index"),
                    }
                }
                _ => panic!("Expected Index expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_index_with_expression_key() {
    let program = r#"
        val x = m[1 + 2]
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
                Expr::Index { index, .. } => {
                    match index.as_ref() {
                        Expr::Binary { .. } => {}, // Correct, it's a binary expression
                        _ => panic!("Expected Binary expression as index"),
                    }
                }
                _ => panic!("Expected Index expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

// ===== Postfix Increment/Decrement Tests =====

#[test]
fn test_parse_postfix_increment() {
    let program = r#"
        var x = 5
        x++
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

    // Second statement should be the postfix increment
    match &stmts[1] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::PostfixIncrement { operand, .. } => {
                    match operand.as_ref() {
                        Expr::Variable { name, .. } => assert_eq!(name, "x"),
                        _ => panic!("Expected Variable as operand"),
                    }
                }
                _ => panic!("Expected PostfixIncrement expression"),
            }
        }
        _ => panic!("Expected Expression statement"),
    }
}

#[test]
fn test_parse_postfix_decrement() {
    let program = r#"
        var y = 10
        y--
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

    // Second statement should be the postfix decrement
    match &stmts[1] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::PostfixDecrement { operand, .. } => {
                    match operand.as_ref() {
                        Expr::Variable { name, .. } => assert_eq!(name, "y"),
                        _ => panic!("Expected Variable as operand"),
                    }
                }
                _ => panic!("Expected PostfixDecrement expression"),
            }
        }
        _ => panic!("Expected Expression statement"),
    }
}

#[test]
fn test_parse_postfix_in_expression() {
    let program = r#"
        var x = 5
        val y = x++
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

    // Second statement should be val y = x++
    match &stmts[1] {
        Stmt::Val { name, initializer: Some(expr), .. } => {
            assert_eq!(name, "y");
            match expr {
                Expr::PostfixIncrement { operand, .. } => {
                    match operand.as_ref() {
                        Expr::Variable { name, .. } => assert_eq!(name, "x"),
                        _ => panic!("Expected Variable as operand"),
                    }
                }
                _ => panic!("Expected PostfixIncrement expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_postfix_with_field_access() {
    let program = r#"
        obj.counter++
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

    // Should be obj.counter++
    match &stmts[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::PostfixIncrement { operand, .. } => {
                    match operand.as_ref() {
                        Expr::GetField { object, field, .. } => {
                            match object.as_ref() {
                                Expr::Variable { name, .. } => assert_eq!(name, "obj"),
                                _ => panic!("Expected Variable as object"),
                            }
                            assert_eq!(field, "counter");
                        }
                        _ => panic!("Expected GetField as operand"),
                    }
                }
                _ => panic!("Expected PostfixIncrement expression"),
            }
        }
        _ => panic!("Expected Expression statement"),
    }
}

#[test]
fn test_parse_postfix_with_array_index() {
    let program = r#"
        arr[0]++
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

    // Should be arr[0]++
    match &stmts[0] {
        Stmt::Expression { expr, .. } => {
            match expr {
                Expr::PostfixIncrement { operand, .. } => {
                    match operand.as_ref() {
                        Expr::Index { object, index, .. } => {
                            match object.as_ref() {
                                Expr::Variable { name, .. } => assert_eq!(name, "arr"),
                                _ => panic!("Expected Variable as object"),
                            }
                            match index.as_ref() {
                                Expr::Number { value, .. } => assert_eq!(*value, 0.0),
                                _ => panic!("Expected Number as index"),
                            }
                        }
                        _ => panic!("Expected Index as operand"),
                    }
                }
                _ => panic!("Expected PostfixIncrement expression"),
            }
        }
        _ => panic!("Expected Expression statement"),
    }
}

#[test]
fn test_parse_postfix_precedence() {
    // Postfix should bind tighter than binary operators
    let program = r#"
        val result = x++ + 5
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

    // Should parse as (x++) + 5, not x++ (+ 5)
    match &stmts[0] {
        Stmt::Val { initializer: Some(expr), .. } => {
            match expr {
                Expr::Binary { left, right, .. } => {
                    // Left should be x++
                    match left.as_ref() {
                        Expr::PostfixIncrement { operand, .. } => {
                            match operand.as_ref() {
                                Expr::Variable { name, .. } => assert_eq!(name, "x"),
                                _ => panic!("Expected Variable as operand"),
                            }
                        }
                        _ => panic!("Expected PostfixIncrement as left operand"),
                    }
                    // Right should be 5
                    match right.as_ref() {
                        Expr::Number { value, .. } => assert_eq!(*value, 5.0),
                        _ => panic!("Expected Number as right operand"),
                    }
                }
                _ => panic!("Expected Binary expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

// ===== Error Cases =====

#[test]
fn test_parse_map_missing_colon() {
    let program = r#"
        val m = {"key" 42}
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    // With set support, {"key" 42} is ambiguous - could be:
    // 1. A map with missing colon: {"key": 42}
    // 2. A set with missing comma: {"key", 42}
    // Parser treats it as a malformed set, so accept either error message
    assert!(errors[0].message.contains("':'") || errors[0].message.contains("'}'"));
}

#[test]
fn test_parse_map_missing_value() {
    let program = r#"
        val m = {"key":}
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_parse_map_missing_closing_brace() {
    let program = r#"
        val m = {"key": 42
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors[0].message.contains("'}'"));
}

#[test]
fn test_parse_index_missing_closing_bracket() {
    let program = r#"
        val x = m["key"
        "#;
    let mut parser = Parser::new(program);
    let result = parser.parse();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors[0].message.contains("']'"));
}

// ===== Integration Tests =====

#[test]
fn test_parse_map_with_index_access() {
    let program = r#"
        val m = {"name": "Charlie", "age": 28}
        val name = m["name"]
        val age = m["age"]
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
    assert_eq!(stmts.len(), 3);
}

#[test]
fn test_parse_complex_map_program() {
    let program = r#"
        val config = {
            "host": "localhost",
            "port": 8080,
            "settings": {
                "debug": true,
                "timeout": 30
            }
        }

        val host = config["host"]
        config["port"] = 9090
        val debug = config["settings"]["debug"]
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
    assert_eq!(stmts.len(), 4);
}

// ===== Array Literal Tests =====

#[test]
fn test_parse_empty_array() {
    let program = "val arr = []\n";
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
                Expr::ArrayLiteral { elements, .. } => {
                    assert_eq!(elements.len(), 0);
                }
                _ => panic!("Expected ArrayLiteral expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_array_with_numbers() {
    let program = "val arr = [1, 2, 3]\n";
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
                Expr::ArrayLiteral { elements, .. } => {
                    assert_eq!(elements.len(), 3);
                    match &elements[0] {
                        Expr::Number { value, .. } => assert_eq!(*value, 1.0),
                        _ => panic!("Expected Number element"),
                    }
                    match &elements[1] {
                        Expr::Number { value, .. } => assert_eq!(*value, 2.0),
                        _ => panic!("Expected Number element"),
                    }
                    match &elements[2] {
                        Expr::Number { value, .. } => assert_eq!(*value, 3.0),
                        _ => panic!("Expected Number element"),
                    }
                }
                _ => panic!("Expected ArrayLiteral expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_array_with_mixed_types() {
    let program = r#"val arr = [1, "hello", true, nil]"#;
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
                Expr::ArrayLiteral { elements, .. } => {
                    assert_eq!(elements.len(), 4);
                    assert!(matches!(elements[0], Expr::Number { .. }));
                    assert!(matches!(elements[1], Expr::String { .. }));
                    assert!(matches!(elements[2], Expr::Boolean { .. }));
                    assert!(matches!(elements[3], Expr::Nil { .. }));
                }
                _ => panic!("Expected ArrayLiteral expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_array_with_trailing_comma() {
    let program = "val arr = [1, 2, 3,]\n";
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
                Expr::ArrayLiteral { elements, .. } => {
                    assert_eq!(elements.len(), 3);
                }
                _ => panic!("Expected ArrayLiteral expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_nested_arrays() {
    let program = "val arr = [[1, 2], [3, 4]]\n";
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
                Expr::ArrayLiteral { elements, .. } => {
                    assert_eq!(elements.len(), 2);
                    // Check first nested array
                    match &elements[0] {
                        Expr::ArrayLiteral { elements: inner, .. } => {
                            assert_eq!(inner.len(), 2);
                            match &inner[0] {
                                Expr::Number { value, .. } => assert_eq!(*value, 1.0),
                                _ => panic!("Expected Number"),
                            }
                        }
                        _ => panic!("Expected nested ArrayLiteral"),
                    }
                    // Check second nested array
                    match &elements[1] {
                        Expr::ArrayLiteral { elements: inner, .. } => {
                            assert_eq!(inner.len(), 2);
                            match &inner[0] {
                                Expr::Number { value, .. } => assert_eq!(*value, 3.0),
                                _ => panic!("Expected Number"),
                            }
                        }
                        _ => panic!("Expected nested ArrayLiteral"),
                    }
                }
                _ => panic!("Expected ArrayLiteral expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_array_with_multiline() {
    let program = r#"
        val arr = [
            1,
            2,
            3
        ]
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
                Expr::ArrayLiteral { elements, .. } => {
                    assert_eq!(elements.len(), 3);
                }
                _ => panic!("Expected ArrayLiteral expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_array_with_expressions() {
    let program = "val arr = [1 + 2, 3 * 4, 5 - 6]\n";
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
                Expr::ArrayLiteral { elements, .. } => {
                    assert_eq!(elements.len(), 3);
                    // All elements should be binary expressions
                    assert!(matches!(elements[0], Expr::Binary { .. }));
                    assert!(matches!(elements[1], Expr::Binary { .. }));
                    assert!(matches!(elements[2], Expr::Binary { .. }));
                }
                _ => panic!("Expected ArrayLiteral expression"),
            }
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_parse_array_missing_closing_bracket() {
    let program = "val arr = [1, 2, 3\n";
    let mut parser = Parser::new(program);
    let result = parser.parse();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(!errors.is_empty());
    assert!(errors[0].message.contains("']'"));
}
