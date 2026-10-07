use super::helpers::{compile, compile_errors, disassemble};
use super::module_graph::TempDir;
use crate::common::errors::{CompilationError, CompilationErrorKind};
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

#[test]
fn assigns_a_modules_globals_the_slots_after_the_module_compiled_before_it() {
    let chunk = compile_files(
        "module_slot_offsets",
        "import \"a\"\nimport \"b\"\nprint(b.z)\n",
        &[
            ("a.n", "export val x = 1\nexport val y = 2\n"),
            ("b.n", "export val z = 3\n"),
        ],
    )
    .expect("should compile");

    let disassembly = disassemble(&chunk);
    assert!(
        disassembly.contains("GetGlobal 02"),
        "expected b.z in slot 2, after a's two globals:\n{}",
        disassembly
    );
}

#[test]
fn accepts_two_modules_declaring_the_same_top_level_name() {
    let chunk = compile_files(
        "duplicate_top_level_names",
        "import \"a\"\nimport \"b\"\nprint(a.x)\nprint(b.x)\n",
        &[
            ("a.n", "export val x = 1\nval hidden = 2\n"),
            ("b.n", "export val x = 3\nval hidden = 4\n"),
        ],
    )
    .expect("should compile");

    let disassembly = disassemble(&chunk);
    assert!(
        disassembly.contains("GetGlobal 00") && disassembly.contains("GetGlobal 02"),
        "expected a.x and b.x in different slots:\n{}",
        disassembly
    );
}

#[test]
fn interns_a_field_name_used_in_two_modules_to_one_symbol_id() {
    let chunk = compile_files(
        "shared_field_symbol",
        "import \"a\"\nimport \"b\"\nprint(a.p.x)\nprint(b.p.x)\n",
        &[
            (
                "a.n",
                "struct P {\n    x\n}\nexport val p = P(1)\nprint(p.x)\n",
            ),
            (
                "b.n",
                "struct P {\n    x\n}\nexport val p = P(2)\nprint(p.x)\n",
            ),
        ],
    )
    .expect("should compile");

    let count = chunk.symbols.iter().filter(|s| &***s == "x").count();
    assert_eq!(1, count, "symbols: {:?}", chunk.symbols);
}

fn compile_errors_of(dir_name: &str, entry: &str, files: &[(&str, &str)]) -> Vec<CompilationError> {
    match compile_files(dir_name, entry, files) {
        Ok(_) => panic!("expected compile errors"),
        Err(errors) => errors,
    }
}

#[test]
fn rejects_an_unknown_export_call_with_a_suggestion() {
    let errors = compile_errors_of(
        "unknown_export_suggestion",
        "import \"utils\"\nutils.doubel(1)\n",
        &[("utils.n", DOUBLE_MODULE)],
    );

    assert_eq!(1, errors.len(), "errors: {:#?}", errors);
    let error = &errors[0];
    assert_eq!(CompilationErrorKind::UnknownExport, error.kind);
    assert_eq!(
        "module 'utils' has no export 'doubel'. Did you mean 'double'?",
        error.message
    );
    assert_eq!(2, error.location.line);
    assert_eq!(None, error.file);
}

#[test]
fn rejects_an_unknown_export_call_without_a_suggestion_when_nothing_is_similar() {
    let errors = compile_errors_of(
        "unknown_export_plain",
        "import \"utils\"\nutils.nope()\n",
        &[("utils.n", DOUBLE_MODULE)],
    );

    assert_eq!(1, errors.len(), "errors: {:#?}", errors);
    assert_eq!(CompilationErrorKind::UnknownExport, errors[0].kind);
    assert!(
        errors[0]
            .message
            .starts_with("module 'utils' has no export 'nope'"),
        "message: {}",
        errors[0].message
    );
    assert!(
        !errors[0].message.contains("Did you mean"),
        "message: {}",
        errors[0].message
    );
}

#[test]
fn rejects_a_call_to_a_non_exported_function_as_an_unknown_export() {
    let errors = compile_errors_of(
        "unknown_export_hidden_fn",
        "import \"utils\"\nutils.hidden()\n",
        &[("utils.n", "fn hidden() {}\nexport fn shown() {}\n")],
    );

    assert_eq!(1, errors.len(), "errors: {:#?}", errors);
    assert_eq!(CompilationErrorKind::UnknownExport, errors[0].kind);
    assert!(
        errors[0]
            .message
            .starts_with("module 'utils' has no export 'hidden'"),
        "message: {}",
        errors[0].message
    );
}

#[test]
fn rejects_a_read_of_a_non_exported_val_as_an_unknown_export() {
    let errors = compile_errors_of(
        "unknown_export_hidden_val",
        "import \"utils\"\nprint(utils.hidden_val)\n",
        &[("utils.n", "val hidden_val = 1\nexport val shown = 2\n")],
    );

    assert_eq!(1, errors.len(), "errors: {:#?}", errors);
    assert_eq!(CompilationErrorKind::UnknownExport, errors[0].kind);
    assert!(
        errors[0]
            .message
            .starts_with("module 'utils' has no export 'hidden_val'"),
        "message: {}",
        errors[0].message
    );
}

fn assert_one_error(
    errors: &[CompilationError],
    kind: CompilationErrorKind,
    message: &str,
    line: u32,
) {
    assert_eq!(1, errors.len(), "errors: {:#?}", errors);
    assert_eq!(kind, errors[0].kind);
    assert_eq!(message, errors[0].message);
    assert_eq!(line, errors[0].location.line);
}

#[test]
fn rejects_assignment_to_an_exported_val() {
    let errors = compile_errors_of(
        "assign_export_val",
        "import \"utils\"\nutils.VERSION = 2\n",
        &[("utils.n", "export val VERSION = 1\n")],
    );

    assert_one_error(
        &errors,
        CompilationErrorKind::ImmutableAssignment,
        "exports are read-only",
        2,
    );
}

#[test]
fn rejects_compound_assignment_to_an_exported_var() {
    let errors = compile_errors_of(
        "assign_export_var",
        "import \"utils\"\nutils.count += 1\n",
        &[("utils.n", "export var count = 0\n")],
    );

    assert_one_error(
        &errors,
        CompilationErrorKind::ImmutableAssignment,
        "exports are read-only",
        2,
    );
}

#[test]
fn rejects_a_module_bound_to_a_val() {
    let errors = compile_errors_of(
        "module_as_val",
        "import \"utils\"\nval m = utils\n",
        &[("utils.n", "export val a = 1\n")],
    );

    assert_one_error(
        &errors,
        CompilationErrorKind::NamespaceAsValue,
        "'utils' is a module, not a value",
        2,
    );
}

#[test]
fn rejects_a_module_passed_as_an_argument() {
    let errors = compile_errors_of(
        "module_as_argument",
        "import \"utils\"\nprint(utils)\n",
        &[("utils.n", "export val a = 1\n")],
    );

    assert_one_error(
        &errors,
        CompilationErrorKind::NamespaceAsValue,
        "'utils' is a module, not a value",
        2,
    );
}

#[test]
fn rejects_an_exported_function_call_with_the_wrong_arity_like_a_local_function() {
    let local = compile_errors("fn double(x) {\n    return x * 2\n}\ndouble(1, 2)\n");
    assert_eq!(1, local.len(), "errors: {:#?}", local);

    let errors = compile_errors_of(
        "export_call_arity",
        "import \"utils\"\nutils.double(1, 2)\n",
        &[("utils.n", DOUBLE_MODULE)],
    );

    assert_one_error(&errors, local[0].kind, &local[0].message, 2);
}

#[test]
fn rejects_two_imports_binding_the_same_name() {
    let local = compile_errors("val utils = 1\nval utils = 2\n");
    assert_eq!(1, local.len(), "errors: {:#?}", local);

    let errors = compile_errors_of(
        "duplicate_imports",
        "import \"a/utils\"\nimport \"b/utils\"\n",
        &[
            ("a/utils.n", "export val a = 1\n"),
            ("b/utils.n", "export val b = 1\n"),
        ],
    );

    assert_one_error(
        &errors,
        CompilationErrorKind::DuplicateSymbol,
        &local[0].message,
        2,
    );
}

#[test]
fn rejects_an_import_and_a_val_binding_the_same_name() {
    let local = compile_errors("val utils = 1\nval utils = 2\n");
    assert_eq!(1, local.len(), "errors: {:#?}", local);

    let errors = compile_errors_of(
        "import_and_val",
        "import \"utils\"\nval utils = 1\n",
        &[("utils.n", "export val a = 1\n")],
    );

    assert_one_error(
        &errors,
        CompilationErrorKind::DuplicateSymbol,
        &local[0].message,
        2,
    );
}

const POINT_MODULE: &str = "export struct Point {\n    x\n    y\n}\n";

#[test]
fn validates_the_fields_of_an_exported_struct_constructed_from_the_importer() {
    compile_files(
        "struct_field_ok",
        "import \"utils\"\nval p = utils.Point(1, 2)\nprint(p.x)\n",
        &[("utils.n", POINT_MODULE)],
    )
    .expect("should compile");

    let local_field =
        compile_errors("struct Point {\n    x\n    y\n}\nval p = Point(1, 2)\nprint(p.z)\n");
    assert_eq!(1, local_field.len(), "errors: {:#?}", local_field);
    let qualified_field = local_field[0].message.replace("'Point'", "'utils.Point'");
    assert_eq!(
        "Struct 'utils.Point' has no field named 'z'",
        qualified_field
    );
    let errors = compile_errors_of(
        "struct_field_unknown",
        "import \"utils\"\nval p = utils.Point(1, 2)\nprint(p.z)\n",
        &[("utils.n", POINT_MODULE)],
    );
    assert_one_error(&errors, local_field[0].kind, &qualified_field, 3);
    let errors = compile_errors_of(
        "struct_field_unknown_shadowed",
        "import \"utils\"\nstruct Point {\n    z\n}\nval p = utils.Point(1, 2)\nprint(p.z)\n",
        &[("utils.n", POINT_MODULE)],
    );
    assert_one_error(&errors, local_field[0].kind, &qualified_field, 6);

    let local_arity = compile_errors("struct Point {\n    x\n    y\n}\nval p = Point(1)\n");
    assert_eq!(1, local_arity.len(), "errors: {:#?}", local_arity);
    let errors = compile_errors_of(
        "struct_ctor_arity",
        "import \"utils\"\nval p = utils.Point(1)\n",
        &[("utils.n", POINT_MODULE)],
    );
    assert_one_error(&errors, local_arity[0].kind, &local_arity[0].message, 2);
}

const POINT_METHODS_MODULE: &str = "export struct Point {\n    x\n}\nimpl Point {\n    fn make() {\n        return Point(1)\n    }\n    fn tag(self) {\n        return 1\n    }\n}\n";
const LOCAL_POINT_METHODS: &str = "struct Point {\n    x\n}\nimpl Point {\n    fn make() {\n        return Point(1)\n    }\n    fn tag(self) {\n        return 1\n    }\n}\n";

#[test]
fn validates_method_calls_on_an_exported_struct() {
    compile_files(
        "struct_methods_ok",
        "import \"utils\"\nprint(utils.Point.make().x)\nprint(utils.Point(1).tag())\n",
        &[("utils.n", POINT_METHODS_MODULE)],
    )
    .expect("should compile");

    let local_static = compile_errors(&format!("{}Point.nope()\n", LOCAL_POINT_METHODS));
    assert_eq!(1, local_static.len(), "errors: {:#?}", local_static);
    let expected = local_static[0].message.replace("'Point'", "'utils.Point'");
    assert_eq!("Type 'utils.Point' has no method named 'nope'", expected);
    let errors = compile_errors_of(
        "struct_static_unknown",
        "import \"utils\"\nutils.Point.nope()\n",
        &[("utils.n", POINT_METHODS_MODULE)],
    );
    assert_eq!(1, errors.len(), "errors: {:#?}", errors);
    assert_eq!(local_static[0].kind, errors[0].kind);
    assert_eq!(expected, errors[0].message);
    assert!(local_static[0].help.is_some());
    assert_eq!(local_static[0].help, errors[0].help);
    assert_eq!(2, errors[0].location.line);

    let local_instance = compile_errors(&format!("{}Point(1).nope()\n", LOCAL_POINT_METHODS));
    assert_eq!(1, local_instance.len(), "errors: {:#?}", local_instance);
    let errors = compile_errors_of(
        "struct_instance_unknown",
        "import \"utils\"\nutils.Point(1).nope()\n",
        &[("utils.n", POINT_METHODS_MODULE)],
    );
    assert_eq!(1, errors.len(), "errors: {:#?}", errors);
    assert_eq!(local_instance[0].kind, errors[0].kind);
    assert_eq!(expected, errors[0].message);
    assert_eq!(local_instance[0].help, errors[0].help);
    assert_eq!(2, errors[0].location.line);

    let local_arity = compile_errors(&format!("{}Point.make(1)\n", LOCAL_POINT_METHODS));
    assert_eq!(1, local_arity.len(), "errors: {:#?}", local_arity);
    let errors = compile_errors_of(
        "struct_static_arity",
        "import \"utils\"\nutils.Point.make(1)\n",
        &[("utils.n", POINT_METHODS_MODULE)],
    );
    assert_one_error(&errors, local_arity[0].kind, &local_arity[0].message, 2);
}

const ENUM_MODULE: &str =
    "export enum Color {\n    Red\n    Green\n}\nexport enum Shape {\n    Circle(r)\n}\n";

fn instructions(chunk: &Chunk) -> Vec<String> {
    chunk
        .disassemble()
        .lines()
        .filter(|line| !line.starts_with("==="))
        .map(|line| {
            let tokens: Vec<&str> = line.split_whitespace().skip(2).collect();
            let text = tokens.join(" ");
            match text.find('\'') {
                Some(quote) => format!("{} {}", tokens[0], &text[quote..]),
                None => text,
            }
        })
        .collect()
}

#[test]
fn resolves_utils_color_red_for_an_exported_enum() {
    let local = compile(&format!(
        "{}print(Color.Red)\nprint(Shape.Circle(2))\n",
        ENUM_MODULE.replace("export ", "")
    ))
    .expect("should compile");
    let chunk = compile_files(
        "enum_variant",
        "import \"utils\"\nprint(utils.Color.Red)\nprint(utils.Shape.Circle(2))\n",
        &[("utils.n", ENUM_MODULE)],
    )
    .expect("should compile");
    assert_eq!(instructions(&local), instructions(&chunk));

    let local_unknown =
        compile_errors("enum Color {\n    Red\n    Green\n}\nprint(Color.Purple)\n");
    assert_eq!(1, local_unknown.len(), "errors: {:#?}", local_unknown);
    let errors = compile_errors_of(
        "enum_unknown_variant",
        "import \"utils\"\nprint(utils.Color.Purple)\n",
        &[("utils.n", ENUM_MODULE)],
    );
    assert_one_error(&errors, local_unknown[0].kind, &local_unknown[0].message, 2);

    let local_value = compile_errors("enum Color {\n    Red\n    Green\n}\nval c = Color\n");
    assert_eq!(1, local_value.len(), "errors: {:#?}", local_value);
    let errors = compile_errors_of(
        "enum_as_value",
        "import \"utils\"\nval c = utils.Color\n",
        &[("utils.n", ENUM_MODULE)],
    );
    assert_one_error(&errors, local_value[0].kind, &local_value[0].message, 2);
}

#[test]
fn suggests_the_alphabetically_first_of_equally_close_exports() {
    let errors = compile_errors_of(
        "unknown_export_tie",
        "import \"ties\"\nprint(ties.hat)\n",
        &[(
            "ties.n",
            "export val rat = 1\nexport val mat = 2\nexport val cat = 3\nexport val bat = 4\n",
        )],
    );

    assert_one_error(
        &errors,
        CompilationErrorKind::UnknownExport,
        "module 'ties' has no export 'hat'. Did you mean 'bat'?",
        2,
    );
}

fn assert_same_errors(local: &[CompilationError], errors: &[CompilationError]) {
    assert!(!local.is_empty());
    let kinds_and_messages = |errors: &[CompilationError]| -> Vec<(CompilationErrorKind, String)> {
        errors
            .iter()
            .map(|error| (error.kind, error.message.clone()))
            .collect()
    };
    assert_eq!(kinds_and_messages(local), kinds_and_messages(errors));
}

#[test]
fn rejects_calling_an_exported_enum_like_a_local_enum() {
    let local_enums = ENUM_MODULE.replace("export ", "");
    let cases = [
        ("enum_call_no_args", "Color()\n", "utils.Color()\n"),
        ("enum_call_one_arg", "Color(1)\n", "utils.Color(1)\n"),
        (
            "enum_call_returned",
            "fn f() {\n    return Shape()\n}\n",
            "fn f() {\n    return utils.Shape()\n}\n",
        ),
    ];
    for (dir_name, local_call, member_call) in cases {
        let local = compile_errors(&format!("{}{}", local_enums, local_call));
        let errors = compile_errors_of(
            dir_name,
            &format!("import \"utils\"\n{}", member_call),
            &[("utils.n", ENUM_MODULE)],
        );
        assert_same_errors(&local, &errors);
    }
}

#[test]
fn rejects_optional_dot_on_an_exported_type_like_a_local_type() {
    let local_types = format!("{}{}", POINT_MODULE, ENUM_MODULE).replace("export ", "");
    let cases = [
        ("optional_dot_struct", "print(Point?.x)\n"),
        ("optional_dot_enum", "print(Color?.values())\n"),
    ];
    for (dir_name, local_use) in cases {
        let local = compile_errors(&format!("{}{}", local_types, local_use));
        assert_eq!(CompilationErrorKind::OptionalDotOnType, local[0].kind);
        let errors = compile_errors_of(
            dir_name,
            &format!(
                "import \"utils\"\n{}",
                local_use.replace("print(", "print(utils.")
            ),
            &[("utils.n", &format!("{}{}", POINT_MODULE, ENUM_MODULE))],
        );
        assert_same_errors(&local, &errors);
    }
}

#[test]
fn rejects_a_file_stem_that_is_not_an_identifier_unless_aliased() {
    let errors = compile_errors_of(
        "invalid_import_name",
        "import \"my-utils\"\n",
        &[("my-utils.n", "export val x = 1\n")],
    );
    assert_one_error(
        &errors,
        CompilationErrorKind::InvalidImportName,
        "cannot bind 'my-utils' as a name; use `import \"my-utils\" as <name>`",
        1,
    );
    assert_eq!("E0058", CompilationErrorKind::InvalidImportName.code());

    compile_files(
        "invalid_import_name_aliased",
        "import \"my-utils\" as mu\nprint(mu.x)\n",
        &[("my-utils.n", "export val x = 1\n")],
    )
    .expect("should compile");
}

#[test]
fn rejects_a_std_native_call_with_too_many_arguments() {
    let errors = compile_errors_of("std_arity", "import \"std/math\"\nmath.abs(1, 2)\n", &[]);

    assert_eq!(1, errors.len(), "errors: {:#?}", errors);
    assert_eq!(CompilationErrorKind::TooManyArguments, errors[0].kind);
    assert_eq!(
        "Function 'abs' expects 1 arguments but got 2",
        errors[0].message
    );
}

#[test]
fn compiles_a_variadic_std_native_call() {
    compile_files(
        "std_variadic",
        "import \"std/math\"\nprint(math.max(1, 2, 3, 4))\n",
        &[],
    )
    .expect("should compile");
}

#[test]
fn rejects_an_unknown_std_export_with_a_suggestion() {
    let errors = compile_errors_of(
        "std_unknown_export",
        "import \"std/math\"\nmath.abss(1)\n",
        &[],
    );

    assert_eq!(1, errors.len(), "errors: {:#?}", errors);
    assert_eq!(CompilationErrorKind::UnknownExport, errors[0].kind);
    assert_eq!(
        "module 'math' has no export 'abss'. Did you mean 'abs'?",
        errors[0].message
    );
}

#[test]
fn loads_a_std_native_export_as_a_native_function_constant() {
    let chunk = compile_files(
        "std_native_value",
        "import \"std/math\" as m\nval abs = m.abs\n",
        &[],
    )
    .expect("should compile");

    let disassembly = disassemble(&chunk);
    assert!(
        disassembly
            .lines()
            .any(|line| line.contains("Constant") && line.ends_with("'<native fn abs>'")),
        "got:\n{disassembly}"
    );
}
