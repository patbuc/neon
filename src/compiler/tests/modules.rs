use super::helpers::disassemble;
use super::module_graph::TempDir;
use crate::common::errors::CompilationError;
use crate::common::Chunk;
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
        fs::write(dir.0.join(name), source).expect("Failed to write module");
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
