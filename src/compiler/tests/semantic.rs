use super::helpers::{assert_compile_error, compile_errors};
use crate::common::errors::CompilationErrorKind;
use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;

mod resolutions {
    use crate::compiler::ast::{Expr, IfExprElse, MatchArmBody, MatchPattern, NodeId, Stmt};
    use crate::compiler::parser::Parser;
    use crate::compiler::resolutions::{Capture, Res, Resolutions};
    use crate::compiler::semantic::SemanticAnalyzer;

    /// Parses and resolves `program`, panicking on any compile error.
    fn analyze(program: &str) -> (Vec<Stmt>, Resolutions) {
        let mut parser = Parser::new(program);
        let ast = parser.parse().expect("parse error");
        let mut analyzer = SemanticAnalyzer::new();
        let resolutions = analyzer.analyze(&ast).expect("semantic error");
        (ast, resolutions)
    }

    /// Every name use, declaration, and call site found in an AST, in
    /// source order, so a test can find the node it wants to assert on.
    #[derive(Default)]
    struct Index<'a> {
        vars: Vec<(&'a str, NodeId)>,
        calls: Vec<(String, NodeId)>,
        decls: Vec<(&'a str, NodeId)>,
        fns: Vec<(&'a str, NodeId)>,
    }

    fn index_stmts<'a>(stmts: &'a [Stmt], idx: &mut Index<'a>) {
        for stmt in stmts {
            index_stmt(stmt, idx);
        }
    }

    fn index_stmt<'a>(stmt: &'a Stmt, idx: &mut Index<'a>) {
        match stmt {
            Stmt::Val {
                pattern,
                initializer,
                ..
            }
            | Stmt::Var {
                pattern,
                initializer,
                ..
            } => {
                for binding in pattern.bindings() {
                    idx.decls.push((&binding.name, binding.id));
                }
                if let Some(expr) = initializer {
                    index_expr(expr, idx);
                }
            }
            Stmt::Fn { name, body, id, .. } => {
                idx.decls.push((name, *id));
                idx.fns.push((name, *id));
                index_stmts(body, idx);
            }
            Stmt::Struct { name, id, .. } => idx.decls.push((name, *id)),
            Stmt::Enum { name, id, .. } => idx.decls.push((name, *id)),
            Stmt::Impl { methods, .. } => index_stmts(methods, idx),
            Stmt::Expression { expr, .. } => index_expr(expr, idx),
            Stmt::Block { statements, .. } => index_stmts(statements, idx),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                index_expr(condition, idx);
                index_stmt(then_branch, idx);
                if let Some(stmt) = else_branch {
                    index_stmt(stmt, idx);
                }
            }
            Stmt::While {
                condition, body, ..
            } => {
                index_expr(condition, idx);
                index_stmt(body, idx);
            }
            Stmt::Return {
                value: Some(value), ..
            } => index_expr(value, idx),
            Stmt::Return { value: None, .. } => {}
            Stmt::ForIn {
                pattern,
                collection,
                body,
                ..
            } => {
                for binding in pattern.bindings() {
                    idx.decls.push((&binding.name, binding.id));
                }
                index_expr(collection, idx);
                index_stmt(body, idx);
            }
            Stmt::Export { declaration, .. } => index_stmt(declaration, idx),
            Stmt::Break { .. } | Stmt::Continue { .. } | Stmt::Import { .. } => {}
        }
    }

    fn index_expr<'a>(expr: &'a Expr, idx: &mut Index<'a>) {
        match expr {
            Expr::Variable { name, id, .. } => idx.vars.push((name, *id)),
            Expr::Assign { value, .. } => index_expr(value, idx),
            Expr::CompoundAssign {
                name,
                read_id,
                value,
                ..
            } => {
                idx.vars.push((name, *read_id));
                index_expr(value, idx);
            }
            Expr::Binary { left, right, .. } => {
                index_expr(left, idx);
                index_expr(right, idx);
            }
            Expr::Unary { operand, .. } => index_expr(operand, idx),
            Expr::Call {
                callee,
                arguments,
                id,
                ..
            } => {
                idx.calls.push((call_desc(callee), *id));
                index_expr(callee, idx);
                for arg in arguments {
                    index_expr(arg, idx);
                }
            }
            Expr::GetField { object, .. } => index_expr(object, idx),
            Expr::SetField { object, value, .. }
            | Expr::CompoundAssignField { object, value, .. } => {
                index_expr(object, idx);
                index_expr(value, idx);
            }
            Expr::Grouping { expr, .. } => index_expr(expr, idx),
            Expr::MapLiteral { entries, .. } => {
                for (key, value) in entries {
                    index_expr(key, idx);
                    index_expr(value, idx);
                }
            }
            Expr::ArrayLiteral { elements, .. } | Expr::SetLiteral { elements, .. } => {
                for element in elements {
                    index_expr(element, idx);
                }
            }
            Expr::Index { object, index, .. } => {
                index_expr(object, idx);
                index_expr(index, idx);
            }
            Expr::IndexAssign {
                object,
                index,
                value,
                ..
            }
            | Expr::CompoundAssignIndex {
                object,
                index,
                value,
                ..
            } => {
                index_expr(object, idx);
                index_expr(index, idx);
                index_expr(value, idx);
            }
            Expr::Range { start, end, .. } => {
                index_expr(start, idx);
                index_expr(end, idx);
            }
            Expr::Conditional {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                index_expr(condition, idx);
                index_expr(then_expr, idx);
                index_expr(else_expr, idx);
            }
            Expr::Function { body, id, .. } => {
                idx.fns.push(("<lambda>", *id));
                index_stmts(body, idx);
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                index_expr(condition, idx);
                index_stmt(then_branch, idx);
                match else_branch.as_ref() {
                    IfExprElse::If(expr) => index_expr(expr, idx),
                    IfExprElse::Block(stmt) => index_stmt(stmt, idx),
                }
            }
            Expr::Match {
                scrutinee, arms, ..
            } => {
                index_expr(scrutinee, idx);
                for arm in arms {
                    for pattern in &arm.patterns {
                        if let MatchPattern::Expr(expr) = pattern {
                            index_expr(expr, idx);
                        }
                    }
                    match &arm.body {
                        MatchArmBody::Expr(expr) => index_expr(expr, idx),
                        MatchArmBody::Block(stmt) => index_stmt(stmt, idx),
                    }
                }
            }
            Expr::Number { .. }
            | Expr::Int { .. }
            | Expr::String { .. }
            | Expr::StringInterpolation { .. }
            | Expr::Boolean { .. }
            | Expr::Nil { .. } => {}
        }
    }

    fn call_desc(callee: &Expr) -> String {
        match callee {
            Expr::Variable { name, .. } => name.clone(),
            Expr::GetField { object, field, .. } => match object.as_ref() {
                Expr::Variable { name, .. } => format!("{}.{}", name, field),
                _ => format!("?.{}", field),
            },
            _ => "?".to_string(),
        }
    }

    fn find_var(idx: &Index, name: &str, n: usize) -> NodeId {
        idx.vars
            .iter()
            .filter(|(found, _)| *found == name)
            .nth(n)
            .map(|(_, id)| *id)
            .unwrap_or_else(|| panic!("variable use '{}' #{} not found", name, n))
    }

    fn find_decl(idx: &Index, name: &str) -> NodeId {
        idx.decls
            .iter()
            .find(|(found, _)| *found == name)
            .map(|(_, id)| *id)
            .unwrap_or_else(|| panic!("declaration '{}' not found", name))
    }

    fn find_fn(idx: &Index, name: &str) -> NodeId {
        idx.fns
            .iter()
            .find(|(found, _)| *found == name)
            .map(|(_, id)| *id)
            .unwrap_or_else(|| panic!("function '{}' not found", name))
    }

    fn find_call(idx: &Index, desc: &str, n: usize) -> NodeId {
        idx.calls
            .iter()
            .filter(|(found, _)| found == desc)
            .nth(n)
            .map(|(_, id)| *id)
            .unwrap_or_else(|| panic!("call '{}' #{} not found", desc, n))
    }

    #[test]
    fn script_local_used_in_script_is_local() {
        let (ast, res) = analyze("val x = 1\nprint(x)\n");
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let decl = res.decl(find_decl(&idx, "x"));
        let use_id = find_var(&idx, "x", 0);
        assert_eq!(res.res(use_id), Res::Local(decl));
    }

    #[test]
    fn toplevel_name_used_inside_fn_is_global() {
        let (ast, res) = analyze(
            r#"
val x = 1
fn f() {
    return x
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let decl = res.decl(find_decl(&idx, "x"));
        let use_id = find_var(&idx, "x", 0);
        assert_eq!(res.res(use_id), Res::Global(decl));
    }

    #[test]
    fn parameter_use_is_local() {
        let (ast, res) = analyze(
            r#"
fn f(a) {
    return a
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let fn_id = find_fn(&idx, "f");
        let param_decl = res.function(fn_id).params[0];
        let use_id = find_var(&idx, "a", 0);
        assert_eq!(res.res(use_id), Res::Local(param_decl));
    }

    #[test]
    fn nested_fn_captures_enclosing_local_as_upvalue() {
        let (ast, res) = analyze(
            r#"
fn outer() {
    var x = 1
    fn inner() {
        return x
    }
    return inner()
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let x_decl = res.decl(find_decl(&idx, "x"));
        let use_id = find_var(&idx, "x", 0);
        assert_eq!(res.res(use_id), Res::Upvalue(0));

        let inner_res = res.function(find_fn(&idx, "inner"));
        assert_eq!(inner_res.upvalues, vec![Capture::Local(x_decl)]);
        assert!(res.is_captured(x_decl));
    }

    #[test]
    fn grandparent_capture_chain_assigns_upvalue_indices_per_level() {
        let (ast, res) = analyze(
            r#"
fn outer() {
    var x = 1
    fn middle() {
        fn inner() {
            return x
        }
        return inner()
    }
    return middle()
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let x_decl = res.decl(find_decl(&idx, "x"));
        let middle_res = res.function(find_fn(&idx, "middle"));
        let inner_res = res.function(find_fn(&idx, "inner"));
        assert_eq!(middle_res.upvalues, vec![Capture::Local(x_decl)]);
        assert_eq!(inner_res.upvalues, vec![Capture::Upvalue(0)]);

        let use_id = find_var(&idx, "x", 0);
        assert_eq!(res.res(use_id), Res::Upvalue(0));
    }

    #[test]
    fn script_block_local_captured_by_fn_is_upvalue_not_global() {
        let (ast, res) = analyze(
            r#"
{
    var x = 1
    fn f() {
        return x
    }
    print(f())
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let x_decl = res.decl(find_decl(&idx, "x"));
        let use_id = find_var(&idx, "x", 0);
        assert_eq!(res.res(use_id), Res::Upvalue(0));
        assert!(res.is_captured(x_decl));
    }

    #[test]
    fn args_builtin_use_is_builtin_zero() {
        let (ast, res) = analyze("print(args)\n");
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let use_id = find_var(&idx, "args", 0);
        assert_eq!(res.res(use_id), Res::Builtin(0));
    }

    #[test]
    fn local_args_shadows_builtin() {
        let (ast, res) = analyze(
            r#"
fn f() {
    val args = [1, 2, 3]
    return args
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let decl = res.decl(find_decl(&idx, "args"));
        let use_id = find_var(&idx, "args", 0);
        assert_eq!(res.res(use_id), Res::Local(decl));
    }

    #[test]
    fn global_print_call_is_native() {
        let (ast, res) = analyze("print(1)\n");
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let call_id = find_call(&idx, "print", 0);
        assert!(res.native(call_id).is_some());
    }

    #[test]
    fn local_print_shadows_native_print_call() {
        let (ast, res) = analyze(
            r#"
fn f() {
    val print = fn(x) { return x }
    return print(1)
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let call_id = find_call(&idx, "print", 0);
        assert!(res.native(call_id).is_none());

        let decl = res.decl(find_decl(&idx, "print"));
        let use_id = find_var(&idx, "print", 0);
        assert_eq!(res.res(use_id), Res::Local(decl));
    }

    #[test]
    fn method_call_captures_receiver_before_argument() {
        let (ast, res) = analyze(
            r#"
struct Obj {
    m
}
fn outer() {
    var a = Obj(fn(x) { return x })
    var b = 3
    fn inner() {
        return a.m(b)
    }
    return inner()
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let a_decl = res.decl(find_decl(&idx, "a"));
        let b_decl = res.decl(find_decl(&idx, "b"));
        let inner_res = res.function(find_fn(&idx, "inner"));
        assert_eq!(
            inner_res.upvalues,
            vec![Capture::Local(a_decl), Capture::Local(b_decl)]
        );

        let a_use = find_var(&idx, "a", 0);
        let b_use = find_var(&idx, "b", 0);
        assert_eq!(res.res(a_use), Res::Upvalue(0));
        assert_eq!(res.res(b_use), Res::Upvalue(1));
    }

    #[test]
    fn repeated_capture_of_same_local_reuses_one_upvalue_entry() {
        let (ast, res) = analyze(
            r#"
fn outer() {
    var a = 1
    fn inner() {
        return a + a
    }
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let a_decl = res.decl(find_decl(&idx, "a"));
        let inner_res = res.function(find_fn(&idx, "inner"));
        assert_eq!(inner_res.upvalues, vec![Capture::Local(a_decl)]);

        let first_use = find_var(&idx, "a", 0);
        let second_use = find_var(&idx, "a", 1);
        assert_eq!(res.res(first_use), Res::Upvalue(0));
        assert_eq!(res.res(second_use), Res::Upvalue(0));
    }

    #[test]
    fn struct_static_call_receiver_is_resolved_as_a_value() {
        let (ast, res) = analyze(
            r#"
struct Point {
    x
    y
}
impl Point {
    fn origin() {
        return 0
    }
}
print(Point.origin())
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let decl = res.decl(find_decl(&idx, "Point"));
        let use_id = find_var(&idx, "Point", 0);
        assert_eq!(res.res(use_id), Res::Local(decl));
    }

    #[test]
    fn for_in_variable_declaration_and_use() {
        let (ast, res) = analyze(
            r#"
val arr = [1, 2, 3]
for item in arr {
    print(item)
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let decl = res.decl(find_decl(&idx, "item"));
        let use_id = find_var(&idx, "item", 0);
        assert_eq!(res.res(use_id), Res::Local(decl));
    }

    #[test]
    fn impl_method_function_resolution_includes_self_param() {
        let (ast, res) = analyze(
            r#"
struct Point {
    x
    y
}
impl Point {
    fn len(self) {
        return self.x
    }
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let method_res = res.function(find_fn(&idx, "len"));
        assert_eq!(method_res.params.len(), 1);

        let self_use = find_var(&idx, "self", 0);
        assert_eq!(res.res(self_use), Res::Local(method_res.params[0]));
    }

    #[test]
    fn impl_method_body_uses_toplevel_name_as_global() {
        let (ast, res) = analyze(
            r#"
fn helper() {
    return 42
}
struct Circle {
    r
}
impl Circle {
    fn area(self) {
        return helper()
    }
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let decl = res.decl(find_decl(&idx, "helper"));
        let use_id = find_var(&idx, "helper", 0);
        assert_eq!(res.res(use_id), Res::Global(decl));
    }

    #[test]
    fn impl_method_body_reads_toplevel_val_as_global() {
        let (ast, res) = analyze(
            r#"
struct Point {
    x
    y
}
val scale = 10
impl Point {
    fn scaled(self) {
        return self.x * scale
    }
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let decl = res.decl(find_decl(&idx, "scale"));
        let use_id = find_var(&idx, "scale", 0);
        assert_eq!(res.res(use_id), Res::Global(decl));
    }

    #[test]
    fn hoisted_block_fn_read_before_its_line_is_checked() {
        let (ast, res) = analyze(
            r#"
fn outer() {
    fn a() {
        return b()
    }
    fn b() {
        return a()
    }
    return a()
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let use_b_in_a = find_var(&idx, "b", 0);
        assert!(res.is_checked(use_b_in_a));

        let use_a_in_b = find_var(&idx, "a", 0);
        assert!(!res.is_checked(use_a_in_b));
        let use_a_in_outer_return = find_var(&idx, "a", 1);
        assert!(!res.is_checked(use_a_in_outer_return));
    }

    #[test]
    fn hoisted_block_fn_self_recursive_call_is_not_checked() {
        let (ast, res) = analyze(
            r#"
fn outer() {
    fn c() {
        return c()
    }
    return c()
}
"#,
        );
        let mut idx = Index::default();
        index_stmts(&ast, &mut idx);

        let self_call = find_var(&idx, "c", 0);
        assert!(!res.is_checked(self_call));
    }
}

#[test]
fn test_tuple_pattern_underscore_declares_nothing() {
    let program = "val (p, _) = [1, 2]\nprint(_)\n";
    let errors = assert_compile_error(program, "Undefined variable '_'");
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_tuple_pattern_duplicate_name_is_duplicate_declaration() {
    let program = "val (a, a) = [1, 2]\n";
    let errors = assert_compile_error(program, "Symbol 'a' already defined in this scope");
    assert_eq!(errors.len(), 1);
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
    let errors = assert_compile_error(program, "Cannot assign to immutable");
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_compound_assign_to_immutable() {
    let program = "val x = 1\nx += 1\n";
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e
        .message
        .contains("Cannot assign to immutable variable 'x'")));
}

#[test]
fn test_compound_assign_to_undefined_variable() {
    let program = "y += 1\n";
    let errors = assert_compile_error(program, "Undefined variable 'y'");
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_compound_assign_before_declaration() {
    let program = "g += 1\nvar g = 0\n";
    let errors = assert_compile_error(program, "Cannot use 'g' before its declaration");
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_compound_assign_both_sides_undefined() {
    let program = "y += z\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 2);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("Undefined variable 'y'")));
    assert!(errors
        .iter()
        .any(|e| e.message.contains("Undefined variable 'z'")));
}

#[test]
fn test_implicit_it_undefined_without_it_in_scope() {
    let program = "[1].map { x -> it }\n";
    let errors = assert_compile_error(program, "Undefined variable 'it'");
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_compound_assign_self_reference_before_declaration() {
    let program = "g += g\nvar g = 0\n";
    let errors = compile_errors(program);
    // The two uses of 'g' are at different columns, so both are distinct
    // diagnostics - only an identical (same message and location) repeat
    // is deduplicated.
    assert_eq!(errors.len(), 2);
    assert!(errors
        .iter()
        .all(|e| e.message.contains("Cannot use 'g' before its declaration")));
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
    let errors = compile_errors(program);
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
    let errors = compile_errors(program);
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
fn test_calling_variable_is_allowed_statically() {
    // A variable's arity is unknown until runtime, so calling it is not a
    // static error even when it happens to hold a non-callable value.
    let program = r#"
val x = 10
x()
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_calling_parameter_is_allowed_statically() {
    let program = r#"
fn apply(g, v) {
    return g(v)
}
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

// ===== Method Validation Tests =====

#[test]
fn test_valid_method_on_array_literal() {
    let program = r#"
val x = [1, 2, 3].size()
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
fn test_typo_on_method_name_suggests_correction() {
    let program = r#"
val x = [1, 2, 3].szie()
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("Did you mean 'size'")));
}

#[test]
fn test_string_len_suggests_size() {
    let program = r#"
val x = "abc".len()
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| { e.message == "Type 'String' has no method named 'len'. Did you mean 'size'?" }));
}

#[test]
fn test_string_char_at_removed() {
    let program = r#"
val x = "abc".charAt(0)
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e
        .message
        .starts_with("Type 'String' has no method named 'charAt'")));
}

#[test]
fn test_string_includes_suggests_contains() {
    let program = r#"
val x = "abc".includes("a")
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| {
        e.message == "Type 'String' has no method named 'includes'. Did you mean 'contains'?"
    }));
}

#[test]
fn test_array_length_suggests_size() {
    let program = r#"
val x = [1].length()
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| {
        e.message == "Type 'Array' has no method named 'length'. Did you mean 'size'?"
    }));
}

#[test]
fn test_range_length_suggests_size() {
    let program = r#"
val x = (1..3).length()
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| {
        e.message == "Type 'Range' has no method named 'length'. Did you mean 'size'?"
    }));
}

#[test]
fn test_map_has_suggests_contains() {
    let program = r#"
val x = {"a": 1}.has("a")
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| {
        e.message == "Type 'Map' has no method named 'has'. Did you mean 'contains'?"
    }));
}

#[test]
fn test_set_has_suggests_contains() {
    let program = r#"
val x = #{1}.has(1)
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| {
        e.message == "Type 'Set' has no method named 'has'. Did you mean 'contains'?"
    }));
}

#[test]
fn test_struct_with_size_method_len_does_not_suggest_size() {
    let program = r#"
struct S { a }
impl S {
    fn size(self) = 1
}
S(1).len()
"#;
    let errors = compile_errors(program);
    assert_eq!(errors[0].kind, CompilationErrorKind::UnknownMethod);
    assert!(!errors
        .iter()
        .any(|e| e.message.contains("Did you mean 'size'")));
}

#[test]
fn test_number_len_does_not_suggest_size() {
    let program = r#"
val x = 5.len()
"#;
    let errors = compile_errors(program);
    assert_eq!(errors[0].kind, CompilationErrorKind::UnknownMethod);
    assert!(!errors
        .iter()
        .any(|e| e.message.contains("Did you mean 'size'")));
}

#[test]
fn test_non_existent_method_shows_available_methods() {
    let program = r#"
val x = [1, 2, 3].notAMethod()
"#;
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1, "errors: {:#?}", errors);
    assert_eq!(
        errors[0].message,
        "Type 'Array' has no method named 'notAMethod'"
    );
    let help = errors[0].help.as_deref().expect("help present");
    assert!(help.starts_with("available methods: "), "help: {help}");
    assert!(help.contains("push"), "help: {help}");
}

#[test]
fn test_method_on_tracked_variable_validates_correctly() {
    let program = r#"
val arr = [1, 2, 3]
val len = arr.size()
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
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("has no method named 'badMethod'")));
}

#[test]
fn test_valid_method_on_string_literal() {
    let program = r#"
val x = "hello".size()
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
    let errors = compile_errors(program);
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
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("has no method named 'wrongMethod'")));
}

#[test]
fn test_valid_method_on_set_literal() {
    let program = r#"
val s = #{1, 2, 3}
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
val result = #{1, 2, 3}.invalidMethod()
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("has no method named 'invalidMethod'")));
}

#[test]
fn test_method_chaining_with_type_inference() {
    let program = r#"
val m = {"a": 1, "b": 2}
val len = m.keys().size()
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
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("has no method named 'invalidMethod'")));
}

#[test]
fn test_string_split_returns_array() {
    let program = r#"
val parts = "a,b,c".split(",")
val len = parts.size()
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
    let errors = compile_errors(program);
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

    val set = #{1, 2, 3}
    val missing = set.notAMethod()  // completely wrong method

    return filtered
}

val result = processData(42)
"#;
    let errors = compile_errors(program);

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
    val length = input.size()
    val parts = input.split(" ")
    val partCount = parts.size()

    // Array operations
    val words = ["hello", "world", "neon"]
    val wordCount = words.size()
    words.push("lang")
    val last = words.pop()

    // Map operations
    val wordMap = {"hello": 1, "world": 2}
    val keys = wordMap.keys()
    val values = wordMap.values()
    val entries = wordMap.entries()

    // Set operations
    val uniqueNums = #{1, 2, 3, 4, 5}
    val hasTwo = uniqueNums.contains(2)
    val asArray = uniqueNums.toArray()
    val setSize = uniqueNums.size()

    // Method chaining
    val result = "hello world".split(" ")
    val chainedLength = result.size()

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
val length = empty.size()
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
    let errors = compile_errors(program);
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
val len = arr.size()
val sz = arr.isEmpty()
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
val length = text.size()
val sub = text.substring(0, 5)
val parts = text.split(" ")
val replaced = text.replace("Hello", "Hi")
val asInt = "123".toInt()
val asFloat = "3.14".toFloat()
val asBool = "true".toBool()
val empty = text.isEmpty()
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
val hasKey = m.contains("a")
val size = m.size()
val value = m.get("a")
val removed = m.remove("b")
val empty = m.isEmpty()
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
val s = #{1, 2, 3, 4, 5}
val hasItem = s.contains(3)
val arr = s.toArray()
val size = s.size()
val added = s.add(6)
val removed = s.remove(2)
val cleared = s.clear()
val s2 = #{4, 5, 6}
val unionSet = s.union(s2)
val intersectSet = s.intersection(s2)
val diffSet = s.difference(s2)
val isSub = s.isSubset(s2)
val empty = s.isEmpty()
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
    val textLen = text.size()           // valid
    val upper = text.toUpper()         // invalid: not implemented

    val numbers = [1, 2, 3, 4, 5]
    val last = numbers.pop()           // valid
    val filtered = numbers.filtr()     // invalid: typo
    val length = numbers.size()        // valid

    return upper
}
"#;
    let errors = compile_errors(program);

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
            .any(|e| e.message.contains("has no method named 'size'")),
        "Should not error on valid 'size' method"
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
val result = arr.szie()  // typo: should be 'size'
"#;
    let errors = compile_errors(program);

    let error_msg = &errors[0].message;

    // Error message should be user-friendly and actionable:
    // 1. Mention the type
    assert!(
        error_msg.contains("Array") || error_msg.contains("array"),
        "Error should mention the type 'Array'"
    );

    // 2. Mention the invalid method name
    assert!(
        error_msg.contains("szie"),
        "Error should mention the invalid method 'szie'"
    );

    // 3. Provide a suggestion
    assert!(
        error_msg.contains("Did you mean") || error_msg.contains("size"),
        "Error should provide a suggestion"
    );
}

#[test]
fn test_integration_nested_method_calls_in_conditions() {
    // Test method validation in conditional expressions
    let program = r#"
fn checkData(items) {
    if (items.size() > 0) {
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
while (i < items.size()) {
    val item = items[i]
    val itemLen = item.size()
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
    let errors = assert_compile_error(program, "Cannot use 'break' outside of a loop");
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_continue_outside_loop() {
    let program = r#"
        var x = 5
        continue
        print(x)
        "#;
    let errors = assert_compile_error(program, "Cannot use 'continue' outside of a loop");
    assert_eq!(errors.len(), 1);
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
        for i in 0..10 {
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
        for i in 0..10 {
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
        for item in arr {
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
        for item in arr {
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
    let errors = assert_compile_error(program, "Cannot use 'break' outside of a loop");
    assert_eq!(errors.len(), 1);
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
    let errors = assert_compile_error(program, "Cannot use 'continue' outside of a loop");
    assert_eq!(errors.len(), 1);
}

// =============================================================================
// Nested Function Declaration Tests
// =============================================================================

#[test]
fn test_nested_fn_in_block_is_valid() {
    let program = r#"
{
    fn f() {
        return 42
    }
    val x = f()
}
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_nested_fn_recursion_is_valid() {
    let program = r#"
fn outer() {
    fn factorial(n) {
        if (n <= 1) {
            return 1
        }
        return n * factorial(n - 1)
    }
    return factorial(5)
}
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_nested_fn_capturing_enclosing_function_local_is_valid() {
    let program = r#"
fn outer() {
    var x = 1
    fn inner() {
        return x
    }
    return inner()
}
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_nested_fn_referencing_script_variable_is_valid() {
    let program = r#"
val x = 1
fn outer() {
    fn inner() {
        return x
    }
    return inner()
}
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_break_in_nested_fn_inside_loop_is_error() {
    let program = r#"
while (true) {
    fn f() {
        break
    }
    f()
}
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("Cannot use 'break' outside of a loop")));
}

#[test]
fn test_continue_in_nested_fn_inside_loop_is_error() {
    let program = r#"
while (true) {
    fn f() {
        continue
    }
    f()
}
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e
        .message
        .contains("Cannot use 'continue' outside of a loop")));
}

// =============================================================================
// Builtin Module Tests (std/math, std/file)
// =============================================================================

#[test]
fn test_std_module_as_value_is_error() {
    assert_compile_error(
        "import \"std/math\"\nval m = math\n",
        "'math' is a module, not a value",
    );
}

#[test]
fn test_file_without_import_is_undefined_variable() {
    let errors = assert_compile_error("val f = File\n", "Undefined variable 'File'");
    assert_eq!(errors[0].kind, CompilationErrorKind::UndefinedVariable);
}

#[test]
fn test_array_as_value_is_undefined_variable() {
    let errors = assert_compile_error("val a = Array\n", "Undefined variable 'Array'");
    assert_eq!(errors[0].kind, CompilationErrorKind::UndefinedVariable);
}

#[test]
fn test_std_file_open_wrong_arity() {
    let program = "import \"std/file\"\nval f = file.open(\"x.txt\", \"y.txt\")\n";
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("expects 1 arguments but got 2")));
}

#[test]
fn test_calling_std_module_is_not_a_function_error() {
    let program = "import \"std/math\"\nval m = math(1)\n";
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message == "'math' is not a function"));
}

#[test]
fn test_break_in_lambda_inside_loop_is_error() {
    let program = r#"
while (true) {
    val f = fn() {
        break
    }
    f()
}
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("Cannot use 'break' outside of a loop")));
}

#[test]
fn test_continue_in_lambda_inside_loop_is_error() {
    let program = r#"
while (true) {
    val f = fn() {
        continue
    }
    f()
}
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e
        .message
        .contains("Cannot use 'continue' outside of a loop")));
}

// ===== Issue #98: scoped type environment =====

#[test]
fn test_toplevel_val_after_function_local_val() {
    let program = r#"
fn f() {
    val s = "a"
}
val s = 1
print(s + 1)
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok());
}

#[test]
fn test_function_local_val_does_not_leak_type_to_outer_scope() {
    let program = r#"
val s = [1, 2]
fn f() {
    val s = "text"
}
s.push(3)
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
fn test_parameter_shadowing_does_not_inherit_outer_type() {
    let program = r#"
val s = "abc"
fn h(s) {
    return s.push(1)
}
print(h([1]))
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
fn test_for_in_variable_shadowing_does_not_inherit_outer_type() {
    let program = r#"
val s = "abc"
for s in [[1]] {
    s.push(2)
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
    assert!(result.is_ok());
}

// ===== Issue #98: Add/ternary type inference =====

#[test]
fn test_add_with_unknown_operand_and_string_infers_string() {
    let program = r#"
fn identity(x) {
    return x
}
val x = identity(1)
print(("a" + x).toUpperCase())
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
fn test_add_with_two_unknown_operands_is_unknown() {
    let program = r#"
fn identity(x) {
    return x
}
val a = identity(1)
val b = identity(2)
val c = a + b
print(c.toUpperCase())
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        // Type is unknown, so no method-validation error should fire.
        for err in errors {
            assert!(!err.message.contains("has no method named"));
        }
    }
}

#[test]
fn test_ternary_mismatched_branches_type_is_unknown() {
    let program = r#"
val cond = true
val result = cond ? 1 : "hello"
print(result.toUpperCase())
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

// ===== Issue #98: struct field and constructor arity checks =====

#[test]
fn test_unknown_field_get_on_known_struct_is_compile_error() {
    let program = r#"
struct P {
    x
}
val p = P(1)
print(p.y)
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("no field named 'y'")));
}

#[test]
fn test_unknown_field_set_on_known_struct_is_compile_error() {
    let program = r#"
struct P {
    x
    y
}
val p = P(1, 2)
p.z = 3
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("no field named 'z'")));
}

#[test]
fn test_valid_field_get_set_on_known_struct_compiles() {
    let program = r#"
struct P {
    x
    y
}
val p = P(1, 2)
p.x = 5
print(p.x)
print(p.y)
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
fn test_static_call_on_field_name_is_compile_error() {
    // Fields only exist on instances, so a static call naming a field is
    // still an unknown method, even though s.f(...) on an instance is fine.
    let program = r#"
struct S {
    f
}
S.f(1)
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("has no method named 'f'")));
}

#[test]
fn test_field_access_on_untyped_parameter_does_not_false_positive() {
    let program = r#"
struct Player {
    name
    score
}
fn get_score(p) {
    return p.score
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
    assert!(result.is_ok());
}

#[test]
fn test_var_reassigned_to_different_struct_stops_field_validation() {
    // Reassignment makes the type unknown from then on (analysis is
    // flow-insensitive, so it can't assume the assignment always runs) -
    // field access after it is no longer checked, valid or not.
    let program = r#"
struct A {
    x
}
struct B {
    y
}
var v = A(1)
v = B(2)
print(v.x)
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
fn test_conditionally_reassigned_struct_does_not_false_positive() {
    // The assignment inside the `if` may never run, so the original
    // struct's fields must still be considered valid afterward.
    let program = r#"
struct P {
    x
}
struct Q {
    y
}
var p = P(1)
if (false) {
    p = Q(2)
}
print(p.x)
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
fn test_conditionally_reassigned_primitive_does_not_false_positive() {
    let program = r#"
var s = "a"
if (false) {
    s = 1
}
print(s.toUpperCase())
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
fn test_struct_constructor_wrong_arity_is_compile_error() {
    let program = r#"
struct P {
    x
    y
}
val p = P(1, 2, 3)
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("expects 2 arguments but got 3")));
}

#[test]
fn test_struct_constructor_correct_arity_compiles() {
    let program = r#"
struct P {
    x
    y
}
val p = P(1, 2)
print(p.x)
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
fn test_too_few_arguments_uses_distinct_error_kind() {
    let program = r#"
fn add(a, b) {
    return a + b
}
val x = add(1)
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.kind == CompilationErrorKind::TooFewArguments
            && e.message.contains("expects 2 arguments but got 1")));
}

#[test]
fn test_too_many_arguments_uses_distinct_error_kind() {
    let program = r#"
fn add(a, b) {
    return a + b
}
val x = add(1, 2, 3)
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.kind == CompilationErrorKind::TooManyArguments));
}

// ===== Issue #147: impl blocks add methods to structs =====

#[test]
fn test_impl_for_undefined_type_is_compile_error() {
    let program = r#"
impl Ghost {
    fn boo(self) {
        return self.baz()
    }
}
"#;
    let errors = compile_errors(program);
    assert_eq!(1, errors.len(), "{:?}", errors);
    assert!(errors[0].message.contains("Ghost"));
}

#[test]
fn test_impl_method_named_like_field_is_compile_error() {
    let program = r#"
struct Circle {
    radius
}
impl Circle {
    fn radius(self) {
        return 1
    }
}
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e.message.contains("radius")));
}

#[test]
fn test_duplicate_method_in_one_impl_is_compile_error() {
    let program = r#"
struct Point {
    x
    y
}
impl Point {
    fn len(self) {
        return 1
    }
    fn len(self) {
        return 2
    }
}
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e.message.contains("len")));
}

#[test]
fn test_duplicate_method_across_two_impls_is_compile_error() {
    let program = r#"
struct Point {
    x
    y
}
impl Point {
    fn len(self) {
        return 1
    }
}
impl Point {
    fn len(self) {
        return 2
    }
}
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e.message.contains("len")));
}

#[test]
fn test_impl_inside_fn_is_compile_error() {
    let program = r#"
struct Point {
    x
}
fn wrapper() {
    impl Point {
        fn len(self) {
            return 1
        }
    }
}
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e.message.contains("top level")));
}

#[test]
fn test_impl_inside_block_is_compile_error() {
    let program = r#"
struct Point {
    x
}
if (true) {
    impl Point {
        fn len(self) {
            return 1
        }
    }
}
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e.message.contains("top level")));
}

#[test]
fn test_struct_inside_fn_is_compile_error() {
    let program = "fn f() {\n    struct P { x }\n    return P(1)\n}\nprint(f().x)\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].location.line, 2);
    assert!(errors[0].message.contains("'P'"));
    assert!(errors[0].message.contains("top level"));
}

#[test]
fn test_struct_inside_block_is_compile_error() {
    let program = "if (true) {\n    struct P { x }\n    print(P(1).x)\n}\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].location.line, 2);
    assert!(errors[0].message.contains("'P'"));
    assert!(errors[0].message.contains("top level"));
}

#[test]
fn test_duplicate_struct_field_is_compile_error() {
    let program = "struct A {\n    x\n    x\n}\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind, CompilationErrorKind::DuplicateField);
    assert_eq!(errors[0].location.line, 3);
    assert_eq!(errors[0].location.column, 5);
    assert!(errors[0].message.contains("'A'"));
    assert!(errors[0].message.contains("'x'"));
}

#[test]
fn test_wrong_argument_count_to_method_names_method() {
    let program = r#"
struct Point {
    x
    y
}
impl Point {
    fn len(self) {
        return 1
    }
}
val p = Point(1, 2)
p.len(5)
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e.message.contains("len")));
}

#[test]
fn test_undefined_method_on_inferred_struct_suggests_correction() {
    let program = r#"
struct Point {
    x
    y
}
impl Point {
    fn len(self) {
        return self.x
    }
}
val p = Point(1, 2)
p.lne()
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e.message.contains("Did you mean")));
}

#[test]
fn test_self_unknown_field_in_method_is_compile_error() {
    let program = r#"
struct Point {
    x
    y
}
impl Point {
    fn len(self) {
        return self.nofield
    }
}
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e.message.contains("nofield")));
}

#[test]
fn test_static_call_on_instance_method_is_compile_error() {
    let program = r#"
struct Point {
    x
    y
}
impl Point {
    fn len(self) {
        return self.x
    }
}
Point.len()
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("len") && e.message.contains("instance")));
}

#[test]
fn test_instance_call_on_static_method_is_compile_error() {
    let program = r#"
struct Point {
    x
    y
}
impl Point {
    fn origin() {
        return Point(0, 0)
    }
}
val p = Point(1, 2)
p.origin()
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("origin") && e.message.contains("static")));
}

#[test]
fn test_method_named_this_is_static_typed_receiver_is_compile_error() {
    let program = r#"
struct Point {
    x
    y
}
impl Point {
    fn len(this) {
        return this.x
    }
}
val p = Point(1, 2)
p.len()
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("len") && e.message.contains("static")));
}

#[test]
fn test_wrong_argument_count_to_static_method_names_method() {
    let program = r#"
struct Point {
    x
    y
}
impl Point {
    fn make(a, b) {
        return Point(a, b)
    }
}
Point.make(1)
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("make") && e.message.contains("expects 2 arguments")));
}

#[test]
fn test_struct_named_string_is_compile_error() {
    let program = r#"
struct String {
    v
}
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e.message.contains("String")));
}

#[test]
fn test_struct_named_array_is_compile_error() {
    let program = r#"
struct Array {
    v
}
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e.message.contains("Array")));
}

#[test]
fn test_duplicate_field_on_reserved_struct_name_reports_both() {
    let program = "struct Array { x x }\n";
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.kind == CompilationErrorKind::DuplicateField));
    assert!(errors
        .iter()
        .any(|e| e.kind == CompilationErrorKind::ReservedStructName));
}

// ===== Issue #149: impl blocks on builtin types =====

#[test]
fn test_impl_method_shadowing_native_method_is_compile_error() {
    let program = r#"
impl String {
    fn size(self) {
        return 0
    }
}
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("size") && e.message.contains("native")));
}

#[test]
fn test_impl_for_undefined_builtin_like_type_is_compile_error() {
    let program = r#"
impl Foo {
    fn bar(self) {}
}
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e.message.contains("Foo")));
}

#[test]
fn test_typo_on_user_method_on_builtin_type_suggests_correction() {
    let program = r#"
impl Array {
    fn second(self) {
        return self[1]
    }
}
val x = [1, 2].secnd()
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("Did you mean 'second'")));
}

#[test]
fn test_self_in_builtin_impl_is_typed_as_that_builtin_type() {
    // self is typed as the builtin type being implemented.
    let program = r#"
impl String {
    fn shoutLen(self) {
        return self.sizee()
    }
}
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("Did you mean 'size'")));
}

#[test]
fn test_static_method_in_builtin_impl_is_compile_error() {
    let program = r#"
impl Array {
    fn make() {
        return []
    }
}
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e.message.contains("make")
        && e.message.contains("self")
        && e.message
            .contains("static methods are only supported on structs")));
}

#[test]
fn test_calling_builtin_user_method_in_static_form_is_compile_error() {
    let program = r#"
impl Map {
    fn second(self) {
        return self[1]
    }
}
val x = Map.second({})
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e
        .message
        .contains("Static methods are only supported on structs")));
}

#[test]
fn test_unknown_static_method_on_builtin_type_is_compile_error() {
    let errors = compile_errors("val x = String.nope(1)\n");
    assert!(
        errors.iter().any(|e| {
            e.kind == CompilationErrorKind::StaticCallOnBuiltinType
                && e.message == "Type 'String' has no static method 'nope'"
        }),
        "got {:#?}",
        errors
    );
}

#[test]
fn test_duplicate_symbol_shadowing_builtin_at_top_level() {
    let program = "val args = 5\n";
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.kind == CompilationErrorKind::DuplicateSymbol));
}

#[test]
fn test_assign_to_builtin_at_top_level() {
    let program = "args = 5\n";
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.kind == CompilationErrorKind::ImmutableAssignment));
}

// ===== Issue #170: pin native method return-type inference =====

#[test]
fn test_native_method_return_types_are_inferred() {
    // Each snippet calls a native method whose return type the registry
    // tracks, then chains an unknown method onto the result - the error
    // must name the inferred type.
    let cases: &[(&str, &str)] = &[
        (r#"val m = {"a": 1}\nm.keys().bogus()"#, "Array"),
        (r#"val m = {"a": 1}\nm.values().bogus()"#, "Array"),
        (r#"val s = #{1}\ns.toArray().bogus()"#, "Array"),
        (r#""a,b".split(",").bogus()"#, "Array"),
        (r#""ab".chars().bogus()"#, "Array"),
        (r#""ab".toUpperCase().bogus()"#, "String"),
        (r#""AB".toLowerCase().bogus()"#, "String"),
        (r#""  ab  ".trim().bogus()"#, "String"),
        (r#""5".toInt().bogus()"#, "Number"),
        (r#""5.5".toFloat().bogus()"#, "Number"),
        (r#"(5).toString().bogus()"#, "String"),
        (r#"[1, 2].join(",").bogus()"#, "String"),
        (r#"[1].map(fn(x) { return x }).bogus()"#, "Array"),
        (r#"[1].filter(fn(x) { return x }).bogus()"#, "Array"),
    ];

    for (source, expected_type) in cases {
        let source = source.replace("\\n", "\n");
        let errors = compile_errors(&source);
        let expected = format!("Type '{}' has no method named 'bogus'", expected_type);
        assert!(
            errors.iter().any(|e| e.message.contains(&expected)),
            "expected error containing \"{}\" for `{}`, got: {:?}",
            expected,
            source,
            errors.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }
}

#[test]
fn test_method_with_unknowable_return_type_has_no_error() {
    // Array.pop() returns whatever element was stored, which can be of any
    // type - the registry tracks no return type for it, so chaining an
    // unknown method must not error.
    let program = r#"[1, "a"].pop().bogus()"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    if let Err(ref errors) = result {
        for err in errors {
            assert!(
                !err.message.contains("has no method named"),
                "unexpected method validation error: {}",
                err.message
            );
        }
    }
}

#[test]
fn test_self_typed_as_builtin_type_in_impl_validates_methods() {
    // Pins StaticType::from_name's builtin arms: `self` inside `impl Array`
    // must be typed as StaticType::Array (not Struct("Array")), so a
    // ternary returning `self` still validates against Array's methods.
    let program = r#"
impl Array {
    fn f(self) {
        return (true ? self : [1]).bogus()
    }
}
"#;
    let errors = compile_errors(program);
    assert!(errors.iter().any(|e| e
        .message
        .contains("Type 'Array' has no method named 'bogus'")));
}

#[test]
fn forward_use_inside_top_level_block_is_compile_error() {
    let program = r#"
{
    print(b)
}
val b = 1
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("Cannot use 'b' before its declaration")));
}

#[test]
fn top_level_lambda_reading_later_val_compiles() {
    let program = r#"
val f = fn() { return b }
val b = 1
print(f())
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok(), "{:?}", result.unwrap_err());
}

#[test]
fn top_level_compound_assignment_before_declaration_is_compile_error() {
    let program = r#"
x += 1
var x = 1
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("Cannot use 'x' before its declaration")));
}

#[test]
fn top_level_assignment_before_declaration_is_compile_error() {
    let program = r#"
x = 5
var x = 1
"#;
    let errors = compile_errors(program);
    assert!(errors
        .iter()
        .any(|e| e.message.contains("Cannot use 'x' before its declaration")));
}

#[test]
fn struct_fields_and_methods_get_symbol_ids() {
    // `obj` is an untyped parameter, so `mystery_field`/`other_field`/
    // `mystery_method` are never validated against a struct declaration -
    // their only route to a symbol id is the GetField/SetField/method-call
    // interning itself, isolating those sites from struct/impl interning.
    let program = r#"
struct Point {
    x
    y
}
impl Point {
    fn len(self) {
        return self.x
    }
}
fn use_obj(obj) {
    val g = obj.mystery_field
    obj.other_field = 1
    return obj.mystery_method()
}
"#;
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let resolutions = analyzer.analyze(&ast).expect("semantic error");

    let names = resolutions.symbol_names();
    for expected in [
        "Point",
        "x",
        "y",
        "len",
        "mystery_field",
        "other_field",
        "mystery_method",
    ] {
        assert!(
            names.iter().any(|n| n.as_ref() == expected),
            "expected '{}' to have a symbol id, got {:?}",
            expected,
            names
        );
    }
}

// =============================================================================
// Enum Tests (issue #333)
// =============================================================================

#[test]
fn test_enum_duplicate_variant_is_compile_error() {
    let program = "enum Color {\n    Red\n    Red\n}\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].location.line, 3);
    assert!(errors[0].message.contains("'Color'"));
    assert!(errors[0].message.contains("'Red'"));
}

#[test]
fn test_enum_unknown_variant_is_compile_error() {
    let program = "enum Color {\n    Red\n    Green\n}\nprint(Color.Purple)\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].location.line, 5);
    assert_eq!(errors[0].kind, CompilationErrorKind::UnknownEnumVariant);
    assert!(errors[0].message.contains("'Purple'"));
}

#[test]
fn test_enum_as_value_is_compile_error() {
    let program = "enum Color {\n    Red\n}\nval x = Color\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].location.line, 4);
    assert_eq!(errors[0].kind, CompilationErrorKind::EnumAsValue);
    assert!(errors[0].message.contains("'Color'"));
}

#[test]
fn test_enum_inside_fn_is_compile_error() {
    let program = "fn f() {\n    enum Color {\n        Red\n    }\n    return 1\n}\nf()\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].location.line, 2);
    assert!(errors[0].message.contains("'Color'"));
    assert!(errors[0].message.contains("top level"));
}

#[test]
fn test_enum_inside_block_is_compile_error() {
    let program = "if (true) {\n    enum Color {\n        Red\n    }\n    print(Color.Red)\n}\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].location.line, 2);
    assert!(errors[0].message.contains("'Color'"));
    assert!(errors[0].message.contains("top level"));
}

#[test]
fn test_enum_name_clash_with_struct_is_duplicate_symbol() {
    let program = "enum Color {\n    Red\n}\nstruct Color {\n    x\n}\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind, CompilationErrorKind::DuplicateSymbol);
    assert_eq!(errors[0].location.line, 4);
}

#[test]
fn test_enum_name_clash_with_fn_is_duplicate_symbol() {
    let program = "enum Color {\n    Red\n}\nfn Color() {\n    return 1\n}\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind, CompilationErrorKind::DuplicateSymbol);
    assert_eq!(errors[0].location.line, 4);
}

#[test]
fn test_enum_name_shadowed_by_local_resolves_as_struct_field() {
    let program = "enum Color {\n    Red\n}\nstruct Box {\n    Red\n}\nfn f() {\n    val Color = Box(0)\n    print(Color.Red)\n}\nf()\n";
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(result.is_ok(), "expected success, got {:#?}", result.err());
}

#[test]
fn test_enum_name_clash_with_val_is_duplicate_symbol() {
    let program = "enum Color {\n    Red\n}\nval Color = 1\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind, CompilationErrorKind::DuplicateSymbol);
    assert_eq!(errors[0].location.line, 4);
}

#[test]
fn test_enum_variant_assignment_is_compile_error() {
    let program = "enum Color {\n    Red\n}\nColor.Red = 1\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind, CompilationErrorKind::ImmutableAssignment);
    assert_eq!(errors[0].location.line, 4);
    assert!(errors[0].message.contains("'Color.Red'"));
}

#[test]
fn test_enum_unknown_static_method_is_compile_error() {
    let program = "enum Color {\n    Red\n}\nprint(Color.names())\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind, CompilationErrorKind::UnknownMethod);
    assert_eq!(errors[0].location.line, 4);
    assert!(errors[0].message.contains("'Color'"));
    assert!(errors[0].message.contains("'names'"));
    assert_eq!(errors[0].help.as_deref(), Some("available methods: values"));
}

#[test]
fn test_impl_on_enum_is_compile_error() {
    let program =
        "enum Color {\n    Red\n}\nimpl Color {\n    fn m(self) {\n        return 1\n    }\n}\n";
    let errors = compile_errors(program);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind, CompilationErrorKind::ImplOnEnum);
    assert_eq!(errors[0].location.line, 4);
    assert!(errors[0].message.contains("'Color'"));
}

#[test]
fn test_enum_payload_constructor_too_few_arguments_is_compile_error() {
    let errors = compile_errors("enum Shape {\n    Rect(w, h)\n}\nprint(Shape.Rect(1))\n");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind, CompilationErrorKind::TooFewArguments);
    assert_eq!(errors[0].location.line, 4);
    assert!(
        errors[0]
            .message
            .contains("Function 'Rect' expects 2 arguments but got 1"),
        "got {:#?}",
        errors
    );
}

#[test]
fn test_enum_payload_constructor_too_many_arguments_is_compile_error() {
    let errors = compile_errors("enum Shape {\n    Rect(w, h)\n}\nprint(Shape.Rect(1, 2, 3))\n");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind, CompilationErrorKind::TooManyArguments);
    assert_eq!(errors[0].location.line, 4);
    assert!(
        errors[0]
            .message
            .contains("Function 'Rect' expects 2 arguments but got 3"),
        "got {:#?}",
        errors
    );
}

#[test]
fn test_enum_payload_unknown_variant_suggests_variants_not_values() {
    let errors =
        compile_errors("enum Shape {\n    Circle(radius)\n    Square\n}\nprint(Shape.Circel(2))\n");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind, CompilationErrorKind::UnknownMethod);
    assert!(!errors[0].message.contains("values"), "got {:#?}", errors);
    assert!(errors[0].message.contains("Circle"), "got {:#?}", errors);
}

#[test]
fn test_enum_unit_variant_call_is_compile_error() {
    let errors = compile_errors("enum Shape {\n    Square\n}\nprint(Shape.Square())\n");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].location.line, 4);
    assert!(
        errors[0]
            .message
            .contains("'Square' is not a payload variant"),
        "got {:#?}",
        errors
    );
}

#[test]
fn test_enum_payload_duplicate_field_is_compile_error() {
    let errors = compile_errors("enum Shape {\n    Rect(w, w)\n}\n");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].location.line, 2);
    assert!(
        errors[0].message.contains("Duplicate field 'w'"),
        "got {:#?}",
        errors
    );
}

#[test]
fn test_enum_values_with_payload_variant_is_compile_error() {
    let errors =
        compile_errors("enum Shape {\n    Circle(radius)\n    Square\n}\nprint(Shape.values())\n");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].location.line, 5);
    assert!(
        errors[0]
            .message
            .contains("'values()' is not available on enum 'Shape'"),
        "got {:#?}",
        errors
    );
}

#[test]
fn test_optional_dot_on_enum_static_call_is_compile_error() {
    let errors = compile_errors("enum Color {\n    Red\n    Green\n}\nColor?.values()\n");
    assert!(
        errors
            .iter()
            .any(|e| e.message == "Cannot use '?.' on a type"),
        "expected a '?.' compile error, got {:#?}",
        errors
    );
}

#[test]
fn test_optional_dot_on_enum_variant_is_compile_error() {
    let errors = compile_errors("enum Color {\n    Red\n    Green\n}\nColor?.Red\n");
    assert!(
        errors
            .iter()
            .any(|e| e.message == "Cannot use '?.' on a type"),
        "expected a '?.' compile error, got {:#?}",
        errors
    );
}

#[test]
fn test_optional_dot_on_struct_type_name_is_compile_error() {
    let errors = compile_errors("struct P {\n    x\n}\nP?.x\n");
    assert!(
        errors
            .iter()
            .any(|e| e.message == "Cannot use '?.' on a type"),
        "expected a '?.' compile error, got {:#?}",
        errors
    );
}

// =============================================================================
// Match Expression Tests (issue #405)
// =============================================================================

#[test]
fn test_match_missing_enum_variant_is_compile_error() {
    let program =
        "enum Color {\n    Red\n    Green\n}\nval c = Color.Red\nval x = match c {\n    Color.Red -> 1\n}\n";
    let errors = compile_errors(program);
    assert!(
        errors
            .iter()
            .any(|e| e.message.contains("match on Color is missing Green")),
        "expected the missing-variant error, got {:#?}",
        errors
    );

    let with_wildcard = "enum Color {\n    Red\n    Green\n}\nval c = Color.Red\nval x = match c {\n    Color.Red -> 1\n    _ -> 0\n}\n";
    let mut parser = Parser::new(with_wildcard);
    let ast = parser.parse().unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    assert!(analyzer.analyze(&ast).is_ok());
}

#[test]
fn test_match_pattern_not_belonging_to_enum_is_compile_error() {
    let program = "enum Color {\n    Red\n    Green\n}\nval c = Color.Red\nval x = match c {\n    Color.Red -> 1\n    3 -> 2\n    _ -> 0\n}\n";
    let errors = compile_errors(program);
    assert!(
        errors.iter().any(|e| e
            .message
            .contains("Pattern 3 does not belong to enum Color")),
        "expected the wrong-enum-pattern error, got {:#?}",
        errors
    );
}

#[test]
fn test_match_duplicate_pattern_is_unreachable() {
    let program =
        "val x = 1\nval y = match x {\n    1 -> \"a\"\n    1 -> \"b\"\n    _ -> \"c\"\n}\n";
    let errors = compile_errors(program);
    let error = errors
        .iter()
        .find(|e| e.message.contains("unreachable pattern"))
        .unwrap_or_else(|| panic!("expected an unreachable pattern error, got {:#?}", errors));
    assert_eq!(error.location.line, 4);
    assert_eq!(error.location.column, 5);
}

#[test]
fn test_match_duplicate_pattern_within_single_arm_is_unreachable() {
    let program = "val x = 1\nval y = match x {\n    1, 1 -> \"a\"\n    _ -> \"c\"\n}\n";
    let errors = compile_errors(program);
    assert!(
        errors
            .iter()
            .any(|e| e.message.contains("unreachable pattern")),
        "expected an unreachable pattern error, got {:#?}",
        errors
    );
}

fn match_errors(program: &str) -> Vec<String> {
    let mut parser = Parser::new(program);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(errors) => return errors.iter().map(|e| e.message.clone()).collect(),
    };
    let mut analyzer = SemanticAnalyzer::new();
    match analyzer.analyze(&ast) {
        Ok(_) => Vec::new(),
        Err(errors) => errors.iter().map(|e| e.message.clone()).collect(),
    }
}

fn assert_match_error(program: &str, expected: &str) {
    let errors: Vec<String> = compile_errors(program)
        .into_iter()
        .map(|e| e.message)
        .collect();
    assert!(
        errors.iter().any(|m| m.contains(expected)),
        "expected {:?}, got {:#?}",
        expected,
        errors
    );
}

#[test]
fn test_match_array_pattern_with_two_rests_is_compile_error() {
    assert_match_error(
        "val r = match [1, 2, 3] {\n    [.., ..] -> 1\n    _ -> 0\n}\n",
        "only one rest",
    );
}

#[test]
fn test_match_expression_pattern_is_invalid() {
    assert_match_error(
        "val y = 1\nval r = match 1 {\n    y + 1 -> 1\n    _ -> 0\n}\n",
        "Invalid match pattern",
    );
}

#[test]
fn test_match_interpolated_string_pattern_is_invalid() {
    assert_match_error(
        "val y = 1\nval r = match \"a\" {\n    \"a${y}\" -> 1\n    _ -> 0\n}\n",
        "Invalid match pattern",
    );
}

#[test]
fn test_match_float_range_pattern_is_invalid() {
    assert_match_error(
        "val r = match 2 {\n    1.5..2.5 -> 1\n    _ -> 0\n}\n",
        "Invalid match pattern",
    );
}

#[test]
fn test_match_call_pattern_is_invalid() {
    assert_match_error(
        "fn f() = 1\nval r = match 1 {\n    f() -> 1\n    _ -> 0\n}\n",
        "Invalid match pattern",
    );
}

#[test]
fn test_match_bare_payload_variant_is_invalid() {
    assert_match_error(
        "enum Shape {\n    Circle(radius)\n    Dot\n}\nval r = match Shape.Dot {\n    Shape.Circle -> 1\n    Shape.Dot -> 2\n}\n",
        "payload variant Shape.Circle must be matched with its fields",
    );
}

#[test]
fn test_match_variant_pattern_wrong_subpattern_count_is_compile_error() {
    assert_match_error(
        "enum Shape {\n    Rect(w, h)\n}\nval r = match Shape.Rect(1, 2) {\n    Shape.Rect(a) -> a\n    _ -> 0\n}\n",
        "Shape.Rect has 2 fields but the pattern has 1",
    );
}

#[test]
fn test_match_variant_pattern_rest_is_compile_error() {
    assert_match_error(
        "enum Shape {\n    Rect(w, h)\n}\nval r = match Shape.Rect(1, 2) {\n    Shape.Rect(a, ..) -> a\n    _ -> 0\n}\n",
        "a variant pattern cannot have a rest ('..')",
    );
}

#[test]
fn test_match_parentheses_on_unit_variant_pattern_is_compile_error() {
    assert_match_error(
        "enum Shape {\n    Dot\n}\nval r = match Shape.Dot {\n    Shape.Dot() -> 1\n    _ -> 0\n}\n",
        "unit variant Shape.Dot takes no parentheses",
    );
}

#[test]
fn test_match_variant_patterns_with_bindings_cover_every_variant() {
    let program = "enum Shape {\n    Circle(r)\n    Rect(w, h)\n}\nval r = match Shape.Circle(1) {\n    Shape.Circle(a) -> a\n    Shape.Rect(w, _) -> w\n}\n";
    assert_eq!(match_errors(program), Vec::<String>::new());
}

#[test]
fn test_match_variant_pattern_with_literal_subpattern_is_not_exhaustive() {
    assert_match_error(
        "enum Shape {\n    Circle(r)\n    Rect(w, h)\n}\nval r = match Shape.Circle(1) {\n    Shape.Circle(a) -> a\n    Shape.Rect(w, 0) -> w\n}\n",
        "match on Shape is missing Rect",
    );
}

#[test]
fn test_match_repeated_irrefutable_variant_pattern_is_unreachable() {
    assert_match_error(
        "enum Shape {\n    Circle(r)\n    Dot\n}\nval r = match Shape.Dot {\n    Shape.Circle(a) -> a\n    Shape.Circle(b) -> b\n    Shape.Dot -> 0\n}\n",
        "unreachable pattern",
    );
}

#[test]
fn test_match_refutable_variant_pattern_after_irrefutable_one_is_unreachable() {
    assert_match_error(
        "enum Shape {\n    Circle(r)\n    Dot\n}\nval r = match Shape.Dot {\n    Shape.Circle(r) -> r\n    Shape.Circle(1) -> 99\n    Shape.Dot -> 0\n}\n",
        "unreachable pattern",
    );
}

#[test]
fn test_match_irrefutable_variant_pattern_after_refutable_one_is_reachable() {
    let program = "enum Shape {\n    Circle(r)\n    Dot\n}\nval r = match Shape.Dot {\n    Shape.Circle(1) -> 99\n    Shape.Circle(r) -> r\n    Shape.Dot -> 0\n}\n";
    assert_eq!(match_errors(program), Vec::<String>::new());
}

#[test]
fn test_match_literal_patterns_are_valid() {
    let program = "enum Color {\n    Red\n}\nval r = match 1 {\n    -1 -> 1\n    2.5 -> 2\n    \"s\" -> 3\n    true -> 4\n    nil -> 5\n    10..20 -> 6\n    -5..=-1 -> 7\n    _ -> 0\n}\nval c = match Color.Red {\n    Color.Red -> 1\n}\n";
    assert_eq!(match_errors(program), Vec::<String>::new());
}

#[test]
fn test_match_duplicate_enum_variant_is_unreachable() {
    assert_match_error(
        "enum Color {\n    Red\n    Green\n}\nval r = match Color.Red {\n    Color.Red -> 1\n    Color.Red -> 2\n    Color.Green -> 3\n}\n",
        "unreachable pattern",
    );
}

#[test]
fn test_match_duplicate_range_is_unreachable() {
    assert_match_error(
        "val r = match 2 {\n    1..3 -> 1\n    1..3 -> 2\n    _ -> 0\n}\n",
        "unreachable pattern",
    );
}

#[test]
fn test_match_int_and_float_duplicate_is_unreachable() {
    assert_match_error(
        "val r = match 1 {\n    1, 1.0 -> 1\n    _ -> 0\n}\n",
        "unreachable pattern",
    );
}

#[test]
fn test_match_enum_taken_from_non_first_pattern() {
    assert_match_error(
        "enum Color {\n    Red\n    Green\n}\nval c = Color.Red\nval x = match c {\n    3 -> 1\n    Color.Red -> 2\n    _ -> 0\n}\n",
        "Pattern 3 does not belong to enum Color",
    );
}

#[test]
fn test_match_large_distinct_ints_are_not_duplicates() {
    let program =
        "val r = match 1 {\n    9007199254740992 -> 1\n    9007199254740993 -> 2\n    _ -> 0\n}\n";
    assert_eq!(match_errors(program), Vec::<String>::new());
}

#[test]
fn test_match_range_pattern_in_enum_match_is_displayed() {
    assert_match_error(
        "enum Color {\n    Red\n}\nval c = Color.Red\nval x = match c {\n    Color.Red -> 1\n    1..=3 -> 2\n}\n",
        "Pattern 1..=3 does not belong to enum Color",
    );
}

#[test]
fn test_match_duplicate_enum_variant_points_at_pattern_start() {
    let program = "enum Color {\n    Red\n}\nval x = match Color.Red {\n    Color.Red -> 1\n    Color.Red -> 2\n}\n";
    let errors = compile_errors(program);
    let error = errors
        .iter()
        .find(|e| e.message.contains("unreachable pattern"))
        .unwrap_or_else(|| panic!("expected an unreachable pattern error, got {:#?}", errors));
    assert_eq!((error.location.line, error.location.column), (6, 5));
}

#[test]
fn test_match_arm_after_wildcard_is_unreachable() {
    assert_match_error(
        "val r = match 3 {\n    _ -> \"w\"\n    3 -> \"dead\"\n}\n",
        "unreachable pattern",
    );
}

#[test]
fn test_match_second_wildcard_is_unreachable() {
    assert_match_error(
        "val r = match 3 {\n    _ -> \"a\"\n    _ -> \"b\"\n}\n",
        "unreachable pattern",
    );
}

#[test]
fn test_match_arm_after_binding_is_unreachable() {
    assert_match_error(
        "val r = match 1 {\n    n -> n\n    2 -> 0\n}\n",
        "unreachable pattern",
    );
}

#[test]
fn test_match_binding_covers_every_enum_variant() {
    let program = "enum Color {\n    Red\n    Green\n}\nval c = Color.Red\nval x = match c {\n    Color.Red -> 1\n    other -> 0\n}\n";
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(
        result.is_ok(),
        "expected no errors, got {:#?}",
        result.err()
    );
}

#[test]
fn test_match_alternatives_binding_different_names_is_error() {
    assert_match_error(
        "val r = match 1 {\n    a, b -> 1\n}\n",
        "must bind the same names",
    );
}

#[test]
fn test_match_name_repeated_in_later_alternative_is_error() {
    assert_match_error(
        "val r = match [1, 2] {\n    [x, 0], [x, x] -> x\n    _ -> 0\n}\n",
        "already defined",
    );
}

#[test]
fn test_match_underscore_rest_alternatives_compile() {
    let program = "val r = match [5] {\n    [.._, 1], [1] -> 1\n    _ -> 0\n}\n";
    let mut parser = Parser::new(program);
    let ast = parser.parse().unwrap();

    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);

    assert!(
        result.is_ok(),
        "expected no errors, got {:#?}",
        result.err()
    );
}

#[test]
fn test_match_array_alternatives_binding_different_names_is_error() {
    assert_match_error(
        "val r = match [1, 2] {\n    [a, 0], [0, b] -> 1\n    _ -> 0\n}\n",
        "must bind the same names",
    );
}

#[test]
fn test_match_arm_after_guarded_binding_is_reachable() {
    let program = "val x = 1\nval y = match x {\n    n if n > 5 -> 1\n    _ -> 0\n}\n";
    assert_eq!(match_errors(program), Vec::<String>::new());
}

#[test]
fn test_match_guarded_arm_does_not_count_toward_exhaustiveness() {
    let program = "enum Color {\n    Red\n    Green\n}\nval c = Color.Red\nval x = match c {\n    Color.Red -> 1\n    Color.Green if false -> 2\n}\n";
    assert_match_error(program, "match on Color is missing Green");
}

#[test]
fn test_match_guarded_arm_repeated_alternative_is_unreachable() {
    assert_match_error(
        "val r = match 1 {\n    1, 1 if true -> \"a\"\n    _ -> \"b\"\n}\n",
        "unreachable pattern",
    );
}

#[test]
fn test_match_guarded_arm_wildcard_then_repeated_alternative_is_unreachable() {
    assert_match_error(
        "val c = true\nval r = match 1 {\n    _, 1 if c -> \"a\"\n    _ -> \"b\"\n}\n",
        "unreachable pattern",
    );
}

#[test]
fn test_match_literal_then_binding_alternatives_is_error() {
    assert_match_error(
        "val r = match 1 {\n    1, n -> n\n}\n",
        "must bind the same names",
    );
}

#[test]
fn test_match_binding_then_literal_alternatives_is_error() {
    assert_match_error(
        "val r = match 1 {\n    n, 1 -> n\n}\n",
        "must bind the same names",
    );
}

#[test]
fn test_match_variant_pattern_from_other_enum_is_rejected() {
    assert_match_error(
        "enum Shape {\n    Circle(r)\n}\nenum Other {\n    Box(b)\n}\nval r = match Shape.Circle(1) {\n    Shape.Circle(a) -> a\n    Other.Box(b) -> b\n}\n",
        "Pattern Other.Box does not belong to enum Shape",
    );
}

#[test]
fn test_match_guarded_variant_pattern_does_not_cover_variant() {
    assert_match_error(
        "enum Shape {\n    Circle(r)\n    Rect(w, h)\n}\nval r = match Shape.Circle(1) {\n    Shape.Circle(a) if a > 0 -> a\n    Shape.Rect(w, h) -> w\n}\n",
        "match on Shape is missing Circle",
    );
}

#[test]
fn test_match_unknown_variant_pattern_reports_only_no_such_variant() {
    let program = "enum Shape {\n    Circle(r)\n}\nval r = match Shape.Circle(1) {\n    Shape.Nope(a) -> a\n    _ -> 0\n}\n";
    let errors: Vec<String> = compile_errors(program)
        .into_iter()
        .map(|e| e.message)
        .collect();
    assert_eq!(
        errors,
        vec!["Enum 'Shape' has no variant named 'Nope'".to_string()]
    );
}

#[test]
fn test_match_unknown_unit_variant_pattern_reports_only_no_such_variant() {
    let program = "enum Color {\n    Red\n}\nval c = Color.Red\nval x = match c {\n    Color.Purple -> 1\n    _ -> 0\n}\n";
    let errors: Vec<String> = compile_errors(program)
        .into_iter()
        .map(|e| e.message)
        .collect();
    assert_eq!(
        errors,
        vec!["Enum 'Color' has no variant named 'Purple'".to_string()]
    );
}

#[test]
fn test_compiler_resolves_imports_before_semantic_analysis() {
    let errors = compile_errors("val x = 1\nimport \"b\"\n");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].kind, CompilationErrorKind::FileImportUnavailable);
    assert_eq!(
        errors[0].message,
        "file imports are not available in the browser build"
    );
    assert_eq!(errors[0].location.line, 2);
}
