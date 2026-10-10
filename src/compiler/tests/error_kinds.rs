use super::helpers::compile_errors;
use super::module_graph::TempDir;
use crate::common::errors::{CompilationError, CompilationErrorKind};
use crate::compiler::module_graph::EntryLocation;
use crate::compiler::Compiler;
use std::fs;

fn source_overflowing_a_codegen_limit() -> String {
    let mut body = String::new();
    for i in 0..65536 {
        body.push_str(&i.to_string());
        body.push('\n');
    }
    format!("fn f() {{\n{}\n}}\nf()\n", body)
}

fn source_array_literal_too_large() -> String {
    let elements: Vec<String> = (0..70000).map(|i| i.to_string()).collect();
    format!("val arr = [{}]\n", elements.join(", "))
}

/// Builtin types (8), struct `S`, and field `x` already claim 10 symbol ids,
/// leaving 65,526 free; one more method than that overflows the table.
fn source_too_many_symbols() -> String {
    let mut methods = String::new();
    for i in 0..65527 {
        methods.push_str(&format!("fn m{}(self) {{}}\n", i));
    }
    format!("struct S {{ x }}\nimpl S {{\n{}\n}}\n", methods)
}

/// Like `source_too_many_symbols`, but overflows by 100 names instead of 1,
/// so a test can check the error is reported once, not once per name.
fn source_many_too_many_symbols() -> String {
    let mut methods = String::new();
    for i in 0..65627 {
        methods.push_str(&format!("fn m{}(self) {{}}\n", i));
    }
    format!("struct S {{ x }}\nimpl S {{\n{}\n}}\n", methods)
}

/// One input per known error-construction site for `kind`, each paired with
/// a fragment its message must contain when that fragment distinguishes the
/// site from the kind's other sites (`None` when every site shares wording).
fn sources_for(kind: CompilationErrorKind) -> Vec<(String, Option<&'static str>)> {
    match kind {
        CompilationErrorKind::UnexpectedCharacter => vec![
            ("`\n".to_string(), None),
            ("#\n".to_string(), None),
        ],
        CompilationErrorKind::UnterminatedString => vec![("\"abc".to_string(), None)],
        CompilationErrorKind::InvalidEscapeSequence => vec![("\"\\q\"\n".to_string(), None)],
        CompilationErrorKind::InvalidNumberLiteral => vec![
            ("0b2\n".to_string(), Some("Invalid digit in binary literal")),
            ("0x\n".to_string(), Some("requires at least one digit")),
            (
                "0b1__1\n".to_string(),
                Some("Invalid underscore placement in binary literal"),
            ),
            (
                "0o18\n".to_string(),
                Some("Invalid digit in octal literal"),
            ),
            (
                "1__0\n".to_string(),
                Some("Invalid underscore placement in number literal"),
            ),
            (
                "1.5__0\n".to_string(),
                Some("Invalid underscore placement in number literal"),
            ),
            (
                "1e\n".to_string(),
                Some("Missing digits in number exponent"),
            ),
        ],
        CompilationErrorKind::ExpectedToken => vec![
            ("val 5 = 1\n".to_string(), Some("variable name")),
            (
                "print(\"x${a b c}y\")\n".to_string(),
                Some("interpolated expression"),
            ),
            (
                "print(\"${\")\n".to_string(),
                Some("interpolated expression"),
            ),
            (
                "val x = 1 2\n".to_string(),
                Some("after value declaration"),
            ),
        ],
        CompilationErrorKind::ExpectedExpression => vec![
            ("+\n".to_string(), Some("Expect expression")),
            ("1 +\nbreak\n".to_string(), Some("Expect expression")),
            ("(\nbreak\n".to_string(), Some("Expect expression")),
        ],
        CompilationErrorKind::InvalidAssignmentTarget => {
            vec![("1 = 2\n".to_string(), None)]
        }
        CompilationErrorKind::NumberLiteralTooLarge => {
            vec![(format!("0x{}\n", "F".repeat(20)), None)]
        }
        CompilationErrorKind::TooManyParameters => {
            let params: Vec<String> = (0..256).map(|i| format!("p{i}")).collect();
            vec![(format!("fn f({}) {{\n}}\n", params.join(", ")), None)]
        }
        CompilationErrorKind::TooManyCallArguments => {
            let args: Vec<String> = (0..256).map(|_| "1".to_string()).collect();
            vec![(format!("f({})\n", args.join(", ")), None)]
        }
        CompilationErrorKind::NestingTooDeep => {
            vec![(format!("{}1{}\n", "(".repeat(700), ")".repeat(700)), None)]
        }
        CompilationErrorKind::DuplicateSymbol => {
            vec![("val x = 1\nval x = 2\n".to_string(), None)]
        }
        CompilationErrorKind::UndefinedVariable => vec![
            ("print(z)\n".to_string(), None),
            ("z = 1\n".to_string(), None),
        ],
        CompilationErrorKind::UndefinedType => vec![(
            "impl Ghost {\n    fn boo(self) { return 1 }\n}\n".to_string(),
            None,
        )],
        CompilationErrorKind::ImmutableAssignment => {
            vec![("val x = 1\nx = 2\n".to_string(), Some("Cannot assign to"))]
        }
        CompilationErrorKind::ReadInOwnInitializer => {
            vec![("val x = x\n".to_string(), None)]
        }
        CompilationErrorKind::UseBeforeDeclaration => {
            vec![("print(b)\nval b = 1\n".to_string(), None)]
        }
        CompilationErrorKind::ReservedStructName => {
            vec![("struct Array { x }\n".to_string(), None)]
        }
        CompilationErrorKind::StructNotTopLevel => vec![(
            "fn f() {\n    struct S { x }\n}\nf()\n".to_string(),
            None,
        )],
        CompilationErrorKind::ImplNotTopLevel => vec![(
            "fn f() {\n    impl S { fn m(self) { return 1 } }\n}\nf()\n".to_string(),
            None,
        )],
        CompilationErrorKind::DuplicateMethod => vec![(
            "struct S { x }\nimpl S {\n    fn m(self) { return 1 }\n    fn m(self) { return 2 }\n}\n"
                .to_string(),
            None,
        )],
        CompilationErrorKind::NativeMethodConflict => vec![
            (
                "impl String {\n    fn size(self) { return 1 }\n}\n".to_string(),
                None,
            ),
            (
                "impl Map {\n    fn isEmpty(self) { return true }\n}\n".to_string(),
                None,
            ),
        ],
        CompilationErrorKind::MethodFieldConflict => vec![(
            "struct S { x }\nimpl S {\n    fn x(self) { return 1 }\n}\n".to_string(),
            None,
        )],
        CompilationErrorKind::StaticMethodOnBuiltinType => vec![(
            "impl Array {\n    fn foo(n) { return n }\n}\n".to_string(),
            None,
        )],
        CompilationErrorKind::StaticCallOnBuiltinType => {
            vec![("Map.foo()\n".to_string(), None)]
        }
        CompilationErrorKind::LoopControlOutsideLoop => vec![("break\n".to_string(), None)],
        CompilationErrorKind::NamespaceAsValue => {
            vec![("use \"std/math\"\nval m = math\n".to_string(), None)]
        }
        CompilationErrorKind::UnknownMethod => vec![
            (
                "struct S { x }\nval s = S(1)\ns.bogus()\n".to_string(),
                Some("'S'"),
            ),
            ("\"ab\".bogus()\n".to_string(), Some("'String'")),
        ],
        CompilationErrorKind::UnknownField => {
            vec![("struct S { x }\nval s = S(1)\ns.y\n".to_string(), None)]
        }
        CompilationErrorKind::MethodNeedsInstance => vec![(
            "struct S { x }\nimpl S {\n    fn m(self) { return 1 }\n}\nS.m()\n".to_string(),
            None,
        )],
        CompilationErrorKind::MethodIsStatic => vec![(
            "struct S { x }\nimpl S {\n    fn m(n) { return n }\n}\nval s = S(1)\ns.m(1)\n"
                .to_string(),
            None,
        )],
        CompilationErrorKind::NotCallable => {
            vec![("use \"std/math\"\nmath()\n".to_string(), None)]
        }
        CompilationErrorKind::TooFewArguments => vec![(
            "fn add(a, b) {\n    return a + b\n}\nadd(1)\n".to_string(),
            Some("but got 1"),
        )],
        CompilationErrorKind::TooManyArguments => vec![(
            "fn add(a, b) {\n    return a + b\n}\nadd(1, 2, 3)\n".to_string(),
            Some("but got 3"),
        )],
        CompilationErrorKind::DuplicateField => {
            vec![("struct A { x x }\n".to_string(), Some("'A'"))]
        }
        CompilationErrorKind::LimitExceeded => vec![
            (
                source_overflowing_a_codegen_limit(),
                Some("too many constants"),
            ),
            (
                source_array_literal_too_large(),
                Some("array literal too large"),
            ),
        ],
        CompilationErrorKind::TooManySymbols => {
            vec![(source_too_many_symbols(), Some("limit 65536"))]
        }
        CompilationErrorKind::DuplicateEnumVariant => {
            vec![("enum Color {\n    Red\n    Red\n}\n".to_string(), Some("'Color'"))]
        }
        CompilationErrorKind::UnknownEnumVariant => vec![(
            "enum Color {\n    Red\n}\nprint(Color.Purple)\n".to_string(),
            Some("'Purple'"),
        )],
        CompilationErrorKind::EnumAsValue => {
            vec![("enum Color {\n    Red\n}\nval x = Color\n".to_string(), None)]
        }
        CompilationErrorKind::EnumNotTopLevel => vec![(
            "fn f() {\n    enum Color {\n        Red\n    }\n}\nf()\n".to_string(),
            None,
        )],
        CompilationErrorKind::ImportNotTopLevel => {
            vec![("fn f() {\n    use \"a\"\n}\n".to_string(), None)]
        }
        CompilationErrorKind::ExportNotTopLevel => {
            vec![("if true {\n    pub val x = 1\n}\n".to_string(), None)]
        }
        CompilationErrorKind::ImportCycle => {
            vec![("use \"b\"\n".to_string(), Some("a.n -> b.n -> a.n"))]
        }
        CompilationErrorKind::FileImportUnavailable => vec![(
            "use \"b\"\n".to_string(),
            Some("file imports are not available in the browser build"),
        )],
        CompilationErrorKind::UnknownModule => vec![
            ("use \"missing\"\n".to_string(), Some("missing.n")),
            (
                "use \"std/nope\"\n".to_string(),
                Some("unknown builtin module"),
            ),
        ],
        CompilationErrorKind::UnknownExport => vec![(
            "use \"b\"\nb.nope()\n".to_string(),
            Some("module 'b' has no export 'nope'"),
        )],
        CompilationErrorKind::InvalidImportName => vec![(
            "use \"my-utils\"\n".to_string(),
            Some("cannot bind 'my-utils' as a name"),
        )],
        CompilationErrorKind::ImplOnEnum => vec![(
            "enum Color {\n    Red\n}\nimpl Color {\n    fn m(self) { return 1 }\n}\n"
                .to_string(),
            None,
        )],
        CompilationErrorKind::ImplOutsideOwnModule => vec![(
            "impl a.B {\n}\n".to_string(),
            Some("impl blocks must be in the struct's own module"),
        )],
        CompilationErrorKind::YieldOutsideFunction => vec![(
            "yield 1\n".to_string(),
            Some("'yield' outside a function"),
        )],
        CompilationErrorKind::OptionalDotOnType => {
            vec![(
                "enum E {\n    A\n}\nprint(E?.A)\n".to_string(),
                Some("'?.'"),
            )]
        }
        CompilationErrorKind::NonExhaustiveMatch => vec![(
            "enum Color {\n    Red\n    Green\n}\nval c = Color.Red\nval x = match c {\n    Color.Red -> 1\n}\n"
                .to_string(),
            Some("is missing Green"),
        )],
        CompilationErrorKind::PatternNotInEnum => vec![(
            "enum Color {\n    Red\n    Green\n}\nval c = Color.Red\nval x = match c {\n    Color.Red -> 1\n    3 -> 2\n    _ -> 0\n}\n"
                .to_string(),
            Some("does not belong to enum Color"),
        )],
        CompilationErrorKind::UnreachablePattern => vec![(
            "val x = 1\nval y = match x {\n    1 -> \"a\"\n    1 -> \"b\"\n    _ -> \"c\"\n}\n"
                .to_string(),
            Some("unreachable pattern"),
        )],
        CompilationErrorKind::InvalidMatchPattern => vec![(
            "val y = 1\nval r = match 1 {\n    y + 1 -> 1\n    _ -> 0\n}\n".to_string(),
            Some("Invalid match pattern"),
        )],
        CompilationErrorKind::UnplaceableComment => vec![
            (
                "if (a) {\n} // c\nelse {\n}\n".to_string(),
                Some("line 2"),
            ),
            (
                "print(\"${x // c\n}\")\n".to_string(),
                Some("line 1"),
            ),
        ],
    }
}

/// Kinds that only a file-based build can produce: the `sources_for` input is
/// the entry file `a.n`, and these are the files it imports.
fn file_based_siblings(
    kind: CompilationErrorKind,
) -> Option<&'static [(&'static str, &'static str)]> {
    match kind {
        CompilationErrorKind::ImportCycle => Some(&[("b.n", "val x = 1\nuse \"a\"\n")]),
        CompilationErrorKind::UnknownModule => Some(&[]),
        CompilationErrorKind::UnknownExport => Some(&[("b.n", "pub val x = 1\n")]),
        CompilationErrorKind::InvalidImportName => Some(&[("my-utils.n", "pub val x = 1\n")]),
        _ => None,
    }
}

fn module_graph_errors(
    kind: CompilationErrorKind,
    entry_source: &str,
    siblings: &[(&str, &str)],
) -> Vec<CompilationError> {
    let dir = TempDir::new(&format!("error_kinds_{:?}", kind));
    let entry_path = dir.0.join("a.n");
    fs::write(&entry_path, entry_source).expect("Failed to write a.n");
    for (name, source) in siblings {
        fs::write(dir.0.join(name), source).expect("Failed to write sibling file");
    }
    let mut compiler = Compiler::new();
    compiler.compile_at(entry_source, EntryLocation::File(entry_path));
    compiler.get_structured_errors()
}

#[test]
fn every_error_kind_is_produced_by_some_input() {
    for &kind in CompilationErrorKind::ALL {
        for (source, fragment) in sources_for(kind) {
            let errors = if kind == CompilationErrorKind::UnplaceableComment {
                crate::compiler::format(&source).err().unwrap_or_default()
            } else if let Some(siblings) = file_based_siblings(kind) {
                module_graph_errors(kind, &source, siblings)
            } else {
                compile_errors(&source)
            };
            assert!(
                errors
                    .iter()
                    .any(|e| e.kind == kind
                        && fragment.is_none_or(|f| e.message.contains(f))),
                "expected {:?} ({}) with message containing {:?} to be produced by:\n{}\ngot errors: {:#?}",
                kind,
                kind.code(),
                fragment,
                source,
                errors
            );
        }
    }
}

#[test]
fn all_codes_are_unique_and_well_formed() {
    let mut seen = std::collections::HashSet::new();
    for &kind in CompilationErrorKind::ALL {
        let code = kind.code();
        let bytes = code.as_bytes();
        assert!(
            bytes.len() == 5 && bytes[0] == b'E' && bytes[1..].iter().all(u8::is_ascii_digit),
            "{:?} has malformed code {:?}",
            kind,
            code
        );
        assert!(
            seen.insert(code),
            "code {} is used by more than one variant",
            code
        );
    }
}

#[test]
fn too_many_symbols_reported_once_per_compile() {
    let errors = compile_errors(&source_many_too_many_symbols());
    let count = errors
        .iter()
        .filter(|e| e.kind == CompilationErrorKind::TooManySymbols)
        .count();
    assert_eq!(
        count, 1,
        "expected exactly one TooManySymbols error, got {:#?}",
        errors
    );
}
