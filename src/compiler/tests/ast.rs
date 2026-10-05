use crate::common::SourceLocation;
use crate::compiler::ast::{BinaryOp, Binding, Expr, NodeId, Pattern, Stmt};

fn dummy_location() -> SourceLocation {
    SourceLocation {
        offset: 0,
        line: 1,
        column: 1,
    }
}

#[test]
fn test_expr_binary() {
    let left = Box::new(Expr::Number {
        value: 1.0,
        raw: "1".to_string(),
        location: dummy_location(),
    });
    let right = Box::new(Expr::Number {
        value: 2.0,
        raw: "2".to_string(),
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
        pattern: Pattern::Name(Binding {
            name: "x".to_string(),
            id: NodeId(0),
            location: dummy_location(),
        }),
        initializer: Some(Expr::Number {
            value: 5.0,
            raw: "5".to_string(),
            location: dummy_location(),
        }),
        location: dummy_location(),
    };
    match stmt {
        Stmt::Val { pattern, .. } => {
            assert_eq!(pattern.bindings()[0].name, "x")
        }
        _ => panic!("Expected Val statement"),
    }
}

#[test]
fn test_stmt_fn() {
    let stmt = Stmt::Fn {
        name: "foo".to_string(),
        params: vec!["a".to_string(), "b".to_string()],
        body: vec![],
        id: NodeId(0),
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
