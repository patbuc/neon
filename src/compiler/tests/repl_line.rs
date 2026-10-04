use crate::common::errors::CompilationErrorKind;
use crate::common::{Chunk, Value};
use crate::compiler::global_env::GlobalEnv;
use crate::compiler::Compiler;

/// Disassembles `chunk` and every function nested in its constant pool, so
/// an assertion can look for an opcode anywhere in the compiled line.
fn disassemble_program(chunk: &Chunk) -> String {
    let mut out = chunk.disassemble();
    for constant in &chunk.constants.values {
        if let Value::Function(function) = constant {
            out.push_str(&disassemble_program(&function.chunk));
        }
    }
    out
}

#[test]
fn global_read_across_lines() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line("var x = 1\n", &GlobalEnv::default())
        .unwrap();

    let (chunk, _) = compiler.compile_line("print(x)\n", &env).unwrap();

    let disassembly = chunk.disassemble();
    assert!(
        disassembly.contains("GetLocal 00"),
        "expected a GetLocal to slot 0:\n{}",
        disassembly
    );
    assert!(!disassembly.contains("GetGlobal"));
}

#[test]
fn new_global_slot_after_carried() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line("var x = 1\n", &GlobalEnv::default())
        .unwrap();

    let (chunk, _) = compiler.compile_line("var y = 2\n", &env).unwrap();

    let disassembly = chunk.disassemble();
    assert!(
        disassembly.contains("SetLocal 01"),
        "expected y's slot to start after x's, not collide with it:\n{}",
        disassembly
    );
}

#[test]
fn function_reads_earlier_global() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line("var x = 1\n", &GlobalEnv::default())
        .unwrap();

    let (chunk, _) = compiler
        .compile_line("fn get() { return x }\n", &env)
        .unwrap();

    let disassembly = disassemble_program(&chunk);
    assert!(
        disassembly.contains("GetGlobal 00"),
        "expected a GetGlobal to slot 0:\n{}",
        disassembly
    );
}

#[test]
fn struct_instantiated_across_lines() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line("struct Point {\n    x\n    y\n}\n", &GlobalEnv::default())
        .unwrap();

    let result = compiler.compile_line("val p = Point(1, 2)\n", &env);
    assert!(result.is_some(), "{:?}", compiler.get_structured_errors());
}

#[test]
fn field_symbol_ids_stable_across_lines() {
    let mut compiler = Compiler::new();
    let (first_chunk, env) = compiler
        .compile_line("struct Point {\n    x\n    y\n}\n", &GlobalEnv::default())
        .unwrap();

    let (second_chunk, _) = compiler
        .compile_line("val p = Point(1, 2)\n", &env)
        .unwrap();

    assert!(
        second_chunk.symbols.len() >= first_chunk.symbols.len(),
        "line 2 must keep every symbol line 1 interned"
    );
    assert_eq!(
        &first_chunk.symbols[..],
        &second_chunk.symbols[..first_chunk.symbols.len()],
        "line 2's symbol table must keep line 1's ids as a stable prefix"
    );
}

#[test]
fn impl_for_earlier_struct_across_lines() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line("struct Point {\n    x\n    y\n}\n", &GlobalEnv::default())
        .unwrap();

    let result = compiler.compile_line(
        "impl Point {\n    fn sum(self) {\n        return self.x + self.y\n    }\n}\n",
        &env,
    );
    assert!(result.is_some(), "{:?}", compiler.get_structured_errors());
}

#[test]
fn static_method_across_lines() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line(
            "struct Point {\n    x\n}\nimpl Point {\n    fn origin() {\n        return Point(0)\n    }\n}\n",
            &GlobalEnv::default(),
        )
        .unwrap();

    let result = compiler.compile_line("val p = Point.origin()\n", &env);
    assert!(result.is_some(), "{:?}", compiler.get_structured_errors());
}

#[test]
fn duplicate_method_across_lines() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line(
            "struct Point {\n    x\n}\nimpl Point {\n    fn m(self) {\n        return self.x\n    }\n}\n",
            &GlobalEnv::default(),
        )
        .unwrap();

    let result = compiler.compile_line(
        "impl Point {\n    fn m(self) {\n        return 1\n    }\n}\n",
        &env,
    );
    assert!(result.is_none());
    assert!(compiler
        .get_structured_errors()
        .iter()
        .any(|e| e.kind == CompilationErrorKind::DuplicateMethod));
}

#[test]
fn carried_type_rejects_wrong_method() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line("val s = \"hi\"\n", &GlobalEnv::default())
        .unwrap();

    let result = compiler.compile_line("s.push(1)\n", &env);
    assert!(result.is_none());
    assert!(compiler
        .get_structured_errors()
        .iter()
        .any(|e| e.kind == CompilationErrorKind::UnknownMethod));
}

#[test]
fn enum_variant_across_lines() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line(
            "enum Color {\n    Red\n    Green\n}\n",
            &GlobalEnv::default(),
        )
        .unwrap();

    let result = compiler.compile_line("print(Color.Red)\n", &env);
    assert!(result.is_some(), "{:?}", compiler.get_structured_errors());
}

#[test]
fn immutable_val_rejected_across_lines() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line("val x = 1\n", &GlobalEnv::default())
        .unwrap();

    let result = compiler.compile_line("x = 2\n", &env);
    assert!(result.is_none());
    assert!(compiler
        .get_structured_errors()
        .iter()
        .any(|e| e.kind == CompilationErrorKind::ImmutableAssignment));
}

#[test]
fn redeclare_earlier_global_allocates_new_slot() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line("val x = 1\n", &GlobalEnv::default())
        .unwrap();
    let old_slot = env.decl_slots[&env.globals["x"].decl_id];

    let (_, env) = compiler.compile_line("val x = 2\n", &env).unwrap();
    let new_slot = env.decl_slots[&env.globals["x"].decl_id];

    assert_ne!(old_slot, new_slot);
}

#[test]
fn redeclare_struct_name_rejected() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line("struct Point {\n    x\n}\n", &GlobalEnv::default())
        .unwrap();

    let result = compiler.compile_line("val Point = 1\n", &env);
    assert!(result.is_none());
    assert!(compiler
        .get_structured_errors()
        .iter()
        .any(|e| e.kind == CompilationErrorKind::DuplicateSymbol));
}

#[test]
fn same_line_redeclaration_rejected() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line("val x = 1\n", &GlobalEnv::default())
        .unwrap();

    let result = compiler.compile_line("val x = 2\nval x = 3\n", &env);
    assert!(result.is_none());
    assert!(compiler
        .get_structured_errors()
        .iter()
        .any(|e| e.kind == CompilationErrorKind::DuplicateSymbol));
}

#[test]
fn second_line_redeclaring_builtin_value_rejected() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line("val x = 1\n", &GlobalEnv::default())
        .unwrap();

    let result = compiler.compile_line("val args = 5\n", &env);
    assert!(result.is_none());
    assert!(compiler
        .get_structured_errors()
        .iter()
        .any(|e| e.kind == CompilationErrorKind::DuplicateSymbol));
}

#[test]
fn second_line_redeclaring_namespace_rejected() {
    let mut compiler = Compiler::new();
    let (_, env) = compiler
        .compile_line("val x = 1\n", &GlobalEnv::default())
        .unwrap();

    let result = compiler.compile_line("fn Math() { return 7 }\n", &env);
    assert!(result.is_none());
    assert!(compiler
        .get_structured_errors()
        .iter()
        .any(|e| e.kind == CompilationErrorKind::DuplicateSymbol));
}
