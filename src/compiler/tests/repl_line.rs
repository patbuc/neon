use super::helpers::disassemble;
use super::module_graph::TempDir;
use crate::common::errors::CompilationErrorKind;
use crate::compiler::global_env::GlobalEnv;
use crate::compiler::module_graph::EntryLocation;
use crate::compiler::Compiler;
use std::fs;

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

    let disassembly = disassemble(&chunk);
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

#[test]
fn resolves_module_member_on_line_after_import() {
    let dir = TempDir::new("repl_import");
    fs::write(
        dir.0.join("utils.n"),
        "export fn double(x) { return x * 2 }\n",
    )
    .expect("Failed to write utils.n");
    let mut compiler = Compiler::new();
    let entry = || EntryLocation::Directory(dir.0.clone());

    let first = compiler
        .compile_entry("import \"utils\"\n", entry(), &GlobalEnv::default())
        .unwrap_or_else(|| panic!("{:?}", compiler.get_structured_errors()));

    let second = compiler
        .compile_entry("print(utils.double(1))\n", entry(), &first.env)
        .unwrap_or_else(|| panic!("{:?}", compiler.get_structured_errors()));
    let disassembly = disassemble(&second.entry);
    assert!(
        disassembly.contains("GetGlobal 00") && disassembly.contains("Call"),
        "expected a call through the export's slot:\n{}",
        disassembly
    );

    let third = compiler
        .compile_entry("val y = 1\n", entry(), &second.env)
        .unwrap_or_else(|| panic!("{:?}", compiler.get_structured_errors()));
    let disassembly = third.entry.disassemble();
    assert!(
        disassembly.contains("SetLocal 01"),
        "expected y's slot after the module's globals:\n{}",
        disassembly
    );
}

#[test]
fn reimporting_a_module_on_a_later_line_reuses_its_slots() {
    let dir = TempDir::new("repl_reimport");
    fs::write(dir.0.join("a.n"), "export var counter = 0\n").expect("Failed to write a.n");
    let mut compiler = Compiler::new();
    let entry = || EntryLocation::Directory(dir.0.clone());

    let first = compiler
        .compile_entry("import \"a\"\n", entry(), &GlobalEnv::default())
        .unwrap_or_else(|| panic!("{:?}", compiler.get_structured_errors()));
    let second = compiler
        .compile_entry("import \"a\"\n", entry(), &first.env)
        .unwrap_or_else(|| panic!("{:?}", compiler.get_structured_errors()));
    let third = compiler
        .compile_entry("val y = 1\nprint(a.counter)\n", entry(), &second.env)
        .unwrap_or_else(|| panic!("{:?}", compiler.get_structured_errors()));

    let disassembly = disassemble(&third.entry);
    assert!(
        disassembly.contains("SetLocal 01") && disassembly.contains("GetGlobal 00"),
        "expected y in slot 1 and a.counter read from slot 0:\n{}",
        disassembly
    );
}

#[test]
fn module_importing_an_earlier_lines_module_reads_its_slots() {
    let dir = TempDir::new("repl_transitive_import");
    fs::write(dir.0.join("a.n"), "export var counter = 0\n").expect("Failed to write a.n");
    fs::write(
        dir.0.join("b.n"),
        "import \"a\"\nexport fn get() { return a.counter }\n",
    )
    .expect("Failed to write b.n");
    let mut compiler = Compiler::new();
    let entry = || EntryLocation::Directory(dir.0.clone());

    let first = compiler
        .compile_entry("import \"a\"\n", entry(), &GlobalEnv::default())
        .unwrap_or_else(|| panic!("{:?}", compiler.get_structured_errors()));
    let second = compiler
        .compile_entry("import \"b\"\n", entry(), &first.env)
        .unwrap_or_else(|| panic!("{:?}", compiler.get_structured_errors()));

    let b_chunk = second.modules.last().expect("b should be compiled");
    let disassembly = disassemble(b_chunk);
    assert!(
        disassembly.contains("GetGlobal 00"),
        "expected b to read a.counter from slot 0:\n{}",
        disassembly
    );
}
