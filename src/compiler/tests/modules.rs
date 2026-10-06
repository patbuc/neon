use super::helpers::disassemble;
use super::module_graph::TempDir;
use crate::common::errors::CompilationError;
use crate::common::{Chunk, Value};
use crate::compiler::module_graph::EntryLocation;
use crate::compiler::Compiler;
use std::fs;

fn compile_files(
    dir_name: &str,
    entry: &str,
    files: &[(&str, &str)],
) -> Result<Chunk, Vec<CompilationError>> {
    let dir = TempDir::new(dir_name);
    for (name, source) in files {
        let path = dir.0.join(name);
        fs::create_dir_all(path.parent().expect("module path has a parent"))
            .expect("Failed to create module dir");
        fs::write(path, source).expect("Failed to write module");
    }
    let main_path = dir.0.join("main.n");
    fs::write(&main_path, entry).expect("Failed to write main.n");
    let mut compiler = Compiler::new();
    compiler
        .compile_at(entry, EntryLocation::File(main_path))
        .ok_or_else(|| compiler.get_structured_errors())
}

#[test]
fn compiles_member_read_to_get_global_of_exports_slot() {
    let chunk = compile_files(
        "member_read",
        "import \"utils\"\nprint(utils.counter)\n",
        &[("utils.n", "export var counter = 0\n")],
    )
    .expect("should compile");

    let disassembly = disassemble(&chunk);
    assert!(
        disassembly.contains("GetGlobal 00"),
        "expected a GetGlobal to counter's slot 0:\n{}",
        disassembly
    );
    assert!(
        !disassembly.contains("GetLocal") && !disassembly.contains("Field"),
        "the member read must not read utils as a value:\n{}",
        disassembly
    );
}

#[test]
fn gives_an_import_binding_no_global_slot() {
    let chunk = compile_files(
        "import_binding_slot",
        "import \"utils\"\nval x = 1\nprint(x)\n",
        &[("utils.n", "export val a = 1\n")],
    )
    .expect("should compile");

    let disassembly = disassemble(&chunk);
    assert!(
        disassembly.contains("SetLocal 01") && disassembly.contains("GetLocal 01"),
        "expected x in slot 1, after utils's a:\n{}",
        disassembly
    );
}

const DOUBLE_MODULE: &str = "export fn double(x) {\n    return x * 2\n}\n";

#[test]
fn compiles_exported_function_call_to_get_global_then_call() {
    let chunk = compile_files(
        "member_call",
        "import \"utils\"\nprint(utils.double(21))\n",
        &[("utils.n", DOUBLE_MODULE)],
    )
    .expect("should compile");

    let disassembly = chunk.disassemble();
    let get_global = disassembly.find("GetGlobal 00");
    let call = disassembly.find("Call (args: 1)");
    assert!(
        get_global.is_some() && call.is_some() && get_global < call,
        "expected GetGlobal 00 before Call (args: 1):\n{}",
        disassembly
    );
    assert!(
        !disassembly.contains("Invoke"),
        "the member call must not be an Invoke:\n{}",
        disassembly
    );
}

#[test]
fn compiles_returned_exported_function_call_to_tail_call() {
    let chunk = compile_files(
        "member_tail_call",
        "import \"utils\"\nfn f(x) {\n    return utils.double(x)\n}\nprint(f(1))\n",
        &[("utils.n", DOUBLE_MODULE)],
    )
    .expect("should compile");

    let f = chunk
        .constants
        .values
        .iter()
        .find_map(|constant| match constant {
            Value::Function(function) if function.name == "f" => Some(function),
            _ => None,
        })
        .expect("f should be in the constant pool");
    let disassembly = f.chunk.disassemble();
    let get_global = disassembly.find("GetGlobal 00");
    let tail_call = disassembly.find("TailCall");
    assert!(
        get_global.is_some() && tail_call.is_some() && get_global < tail_call,
        "expected GetGlobal 00 before TailCall:\n{}",
        disassembly
    );
    assert!(
        !disassembly.contains("TailInvoke"),
        "the member call must not be a TailInvoke:\n{}",
        disassembly
    );
}

#[test]
fn compiles_aliased_import_call_like_the_unaliased_form() {
    let plain = compile_files(
        "alias_plain",
        "import \"lib/utils\"\nprint(utils.double(21))\n",
        &[("lib/utils.n", DOUBLE_MODULE)],
    )
    .expect("should compile");
    let aliased = compile_files(
        "alias_as",
        "import \"lib/utils\" as u\nprint(u.double(21))\n",
        &[("lib/utils.n", DOUBLE_MODULE)],
    )
    .expect("should compile");

    assert_eq!(plain.disassemble(), aliased.disassemble());
}
