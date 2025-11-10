use crate::compiler::ast::{Expr, Stmt, BinaryOp};
use crate::common::SourceLocation;

fn dummy_location() -> SourceLocation {
    SourceLocation {
        offset: 0,
        line: 1,
        column: 1,
    }
}

#[test]
fn test_expr_number() {
    let expr = Expr::Number {
        value: 42.0,
        location: dummy_location(),
    };
    assert_eq!(expr.location().line, 1);
}

#[test]
fn test_expr_binary() {
    let left = Box::new(Expr::Number {
        value: 1.0,
        location: dummy_location(),
    });
    let right = Box::new(Expr::Number {
        value: 2.0,
        location: dummy_location(),
    });
    let expr = Expr::Binary {
        left,
        operator: BinaryOp::Add,
        right,
        location: dummy_location(),
    };
    
    match expr {
        Expr::Binary { operator, .. } => {
            assert_eq!(operator, BinaryOp::Add);
        }
        _ => panic!("Expected Binary expression"),
    }
}

#[test]
fn test_stmt_val() {
    let stmt = Stmt::Val {
        name: "x".to_string(),
        initializer: Some(Expr::Number {
            value: 5.0,
            location: dummy_location(),
        }),
        location: dummy_location(),
    };
    assert_eq!(stmt.location().line, 1);
}

#[test]
fn test_stmt_fn() {
    let stmt = Stmt::Fn {
        name: "foo".to_string(),
        params: vec!["a".to_string(), "b".to_string()],
        body: vec![],
        location: dummy_location(),
    };
    
    match stmt {
        Stmt::Fn { name, params, .. } => {
            assert_eq!(name, "foo");
            assert_eq!(params.len(), 2);
        }
        _ => panic!("Expected Fn statement"),
    }
}
