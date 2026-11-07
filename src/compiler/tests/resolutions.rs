use crate::common::method_registry::BUILTIN_TYPE_NAMES;
use crate::compiler::ast::{Expr, Stmt};
use crate::compiler::exports::ExportTable;
use crate::compiler::global_env::GlobalEnv;
use crate::compiler::parser::Parser;
use crate::compiler::resolutions::{DeclKind, Res, Resolutions};
use crate::compiler::semantic::SemanticAnalyzer;
use std::collections::HashMap;

#[test]
fn builtin_type_names_get_ids_in_registry_order() {
    let resolutions = Resolutions::default();
    let names = resolutions.symbol_names();

    let builtin_names: Vec<&str> = names[..BUILTIN_TYPE_NAMES.len()]
        .iter()
        .map(|n| n.as_ref())
        .collect();
    assert_eq!(builtin_names, BUILTIN_TYPE_NAMES);
}

#[test]
fn interning_an_existing_name_returns_its_id() {
    let mut resolutions = Resolutions::default();
    let first = resolutions.intern_symbol("Point").unwrap();
    let second = resolutions.intern_symbol("Point").unwrap();
    assert_eq!(first, second);

    let array_id = resolutions.intern_symbol("Array").unwrap();
    assert_eq!(array_id, 0);
}

#[test]
fn interning_beyond_65536_names_fails_but_existing_names_still_succeed() {
    let mut resolutions = Resolutions::default();

    let already = BUILTIN_TYPE_NAMES.len();
    for i in 0..(u16::MAX as usize + 1 - already) {
        let name = format!("sym{}", i);
        assert!(
            resolutions.intern_symbol(&name).is_some(),
            "expected symbol {} to fit within the 65536 limit",
            i
        );
    }

    assert!(resolutions.intern_symbol("one_too_many").is_none());

    assert!(resolutions.intern_symbol("sym0").is_some());
    assert!(resolutions.intern_symbol("Array").is_some());
}

#[test]
fn symbol_returns_the_id_a_name_was_interned_under() {
    let mut resolutions = Resolutions::default();
    let id = resolutions.intern_symbol("Point").unwrap();
    assert_eq!(resolutions.symbol("Point"), id);
}

#[test]
#[should_panic(expected = "no symbol interned for")]
fn symbol_panics_for_a_name_never_interned() {
    let resolutions = Resolutions::default();
    resolutions.symbol("nope");
}

fn analyze(source: &str) -> (Vec<Stmt>, Resolutions) {
    let ast = Parser::new(source).parse().expect("parse error");
    let resolutions = SemanticAnalyzer::new()
        .analyze(&ast)
        .expect("semantic error");
    (ast, resolutions)
}

fn kind_of(resolutions: &Resolutions, stmt: &Stmt) -> Option<DeclKind> {
    let id = match stmt {
        Stmt::Val { pattern, .. } | Stmt::Var { pattern, .. } => pattern.bindings()[0].id,
        Stmt::Fn { id, .. } | Stmt::Struct { id, .. } | Stmt::Enum { id, .. } => *id,
        other => panic!("no declaration in {other:?}"),
    };
    resolutions.decl_kind(resolutions.try_decl(id)?)
}

#[test]
fn records_the_kind_of_each_declaration() {
    let (ast, resolutions) =
        analyze("val a = 1\nvar b = 2\nfn f() {}\nstruct S { val x }\nenum E { A }\n");
    let kinds: Vec<_> = ast.iter().map(|s| kind_of(&resolutions, s)).collect();
    assert_eq!(
        kinds,
        vec![
            Some(DeclKind::Value),
            Some(DeclKind::Variable),
            Some(DeclKind::Function),
            Some(DeclKind::Struct),
            Some(DeclKind::Enum),
        ]
    );
}

#[test]
fn records_block_local_kinds() {
    let (ast, resolutions) = analyze("fn outer() {\n    var n = 1\n    fn inner() {}\n}\n");
    let Stmt::Fn { body, .. } = &ast[0] else {
        panic!("expected a function");
    };
    assert_eq!(kind_of(&resolutions, &body[0]), Some(DeclKind::Variable));
    assert_eq!(kind_of(&resolutions, &body[1]), Some(DeclKind::Function));
}

#[test]
fn records_parameters_as_declarations() {
    let (ast, resolutions) = analyze("fn f(a, b) = a\n");
    let Stmt::Fn { params, id, .. } = &ast[0] else {
        panic!("expected a function");
    };
    let decls: Vec<_> = params
        .iter()
        .map(|p| resolutions.try_decl(p.id).expect("param declared"))
        .collect();
    assert_eq!(decls, resolutions.function(*id).params);
    for decl in decls {
        assert_eq!(resolutions.decl_kind(decl), Some(DeclKind::Parameter));
    }
}

#[test]
fn records_the_kind_of_a_pattern_binding() {
    let (ast, resolutions) = analyze("for i in [1] {\n    print(i)\n}\n");
    let Stmt::ForIn { pattern, .. } = &ast[0] else {
        panic!("expected a for loop");
    };
    let decl = resolutions.try_decl(pattern.bindings()[0].id).unwrap();
    assert_eq!(resolutions.decl_kind(decl), Some(DeclKind::Value));
    assert!(resolutions.is_immutable(decl));
}

#[test]
fn try_res_is_none_for_an_unresolved_node() {
    let (ast, resolutions) = analyze("print(1)\n");
    let Stmt::Expression {
        expr: Expr::Call { callee, .. },
        ..
    } = &ast[0]
    else {
        panic!("expected a call");
    };
    let Expr::Variable { id, .. } = callee.as_ref() else {
        panic!("expected a variable callee");
    };
    assert_eq!(resolutions.try_res(*id), None);
}

#[test]
fn try_res_finds_a_resolved_use() {
    let (ast, resolutions) = analyze("val a = 1\nprint(a)\n");
    let Stmt::Expression {
        expr: Expr::Call { arguments, .. },
        ..
    } = &ast[1]
    else {
        panic!("expected a call");
    };
    let Expr::Variable { id, .. } = &arguments[0] else {
        panic!("expected a variable argument");
    };
    assert!(matches!(resolutions.try_res(*id), Some(Res::Local(_))));
}

#[test]
fn records_the_kind_of_a_builtin_module_member() {
    let mut env = GlobalEnv::default();
    let table = ExportTable::builtin("std/math", &mut env.symbols).expect("symbols fit");
    let ast = Parser::new("use \"std/math\"\nmath.sqrt(4)\n")
        .parse()
        .expect("parse error");
    let Stmt::Import { id, .. } = &ast[0] else {
        panic!("expected an import");
    };
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.seed(&env);
    analyzer.seed_imports(HashMap::from([(*id, table)]));
    let resolutions = analyzer.analyze(&ast).expect("semantic error");

    let Stmt::Expression {
        expr: Expr::Call { callee, .. },
        ..
    } = &ast[1]
    else {
        panic!("expected a call");
    };
    let Expr::GetField { object, .. } = callee.as_ref() else {
        panic!("expected a member call");
    };
    let Expr::Variable { id, .. } = object.as_ref() else {
        panic!("expected a module variable");
    };
    assert_eq!(
        resolutions.module_member_kind(*id),
        Some(DeclKind::Function)
    );
}
