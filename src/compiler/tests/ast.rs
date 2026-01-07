use crate::common::SourceLocation;
use crate::compiler::ast::{BinaryOp, Expr, FunctionParam, ImplMethod, Stmt};

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

#[test]
fn test_function_param() {
    let param = FunctionParam {
        name: "x".to_string(),
        is_mutable: false,
    };
    assert_eq!(param.name, "x");
    assert!(!param.is_mutable);

    let mut_param = FunctionParam {
        name: "self".to_string(),
        is_mutable: true,
    };
    assert_eq!(mut_param.name, "self");
    assert!(mut_param.is_mutable);
}

#[test]
fn test_impl_method_instance() {
    // Instance method: fn distance(self, other) { ... }
    let method = ImplMethod {
        name: "distance".to_string(),
        params: vec![
            FunctionParam {
                name: "self".to_string(),
                is_mutable: false,
            },
            FunctionParam {
                name: "other".to_string(),
                is_mutable: false,
            },
        ],
        body: vec![],
        is_static: false,
        is_mutating: false,
        location: dummy_location(),
    };

    assert_eq!(method.name, "distance");
    assert_eq!(method.params.len(), 2);
    assert!(!method.is_static);
    assert!(!method.is_mutating);
    assert_eq!(method.params[0].name, "self");
}

#[test]
fn test_impl_method_mutating() {
    // Mutating method: fn translate(mut self, dx, dy) { ... }
    let method = ImplMethod {
        name: "translate".to_string(),
        params: vec![
            FunctionParam {
                name: "self".to_string(),
                is_mutable: true,
            },
            FunctionParam {
                name: "dx".to_string(),
                is_mutable: false,
            },
            FunctionParam {
                name: "dy".to_string(),
                is_mutable: false,
            },
        ],
        body: vec![],
        is_static: false,
        is_mutating: true,
        location: dummy_location(),
    };

    assert_eq!(method.name, "translate");
    assert_eq!(method.params.len(), 3);
    assert!(!method.is_static);
    assert!(method.is_mutating);
    assert!(method.params[0].is_mutable);
}

#[test]
fn test_impl_method_static() {
    // Static method: fn origin() { ... }
    let method = ImplMethod {
        name: "origin".to_string(),
        params: vec![],
        body: vec![],
        is_static: true,
        is_mutating: false,
        location: dummy_location(),
    };

    assert_eq!(method.name, "origin");
    assert!(method.params.is_empty());
    assert!(method.is_static);
    assert!(!method.is_mutating);
}

#[test]
fn test_stmt_impl() {
    let instance_method = ImplMethod {
        name: "distance".to_string(),
        params: vec![
            FunctionParam {
                name: "self".to_string(),
                is_mutable: false,
            },
            FunctionParam {
                name: "other".to_string(),
                is_mutable: false,
            },
        ],
        body: vec![],
        is_static: false,
        is_mutating: false,
        location: dummy_location(),
    };

    let static_method = ImplMethod {
        name: "origin".to_string(),
        params: vec![],
        body: vec![],
        is_static: true,
        is_mutating: false,
        location: dummy_location(),
    };

    let stmt = Stmt::Impl {
        struct_name: "Point".to_string(),
        methods: vec![instance_method, static_method],
        location: dummy_location(),
    };

    match stmt {
        Stmt::Impl {
            struct_name,
            methods,
            location,
        } => {
            assert_eq!(struct_name, "Point");
            assert_eq!(methods.len(), 2);
            assert_eq!(methods[0].name, "distance");
            assert!(!methods[0].is_static);
            assert_eq!(methods[1].name, "origin");
            assert!(methods[1].is_static);
            assert_eq!(location.line, 1);
        }
        _ => panic!("Expected Impl statement"),
    }
}

#[test]
fn test_stmt_impl_location() {
    let stmt = Stmt::Impl {
        struct_name: "Point".to_string(),
        methods: vec![],
        location: SourceLocation {
            offset: 10,
            line: 5,
            column: 3,
        },
    };

    let loc = stmt.location();
    assert_eq!(loc.line, 5);
    assert_eq!(loc.column, 3);
    assert_eq!(loc.offset, 10);
}
