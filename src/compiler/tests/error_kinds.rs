use crate::common::errors::{CompilationError, CompilationErrorKind};
use crate::compiler::Compiler;

fn compile_to_errors(source: &str) -> Vec<CompilationError> {
    let mut compiler = Compiler::new();
    compiler.compile(source);
    compiler.get_structured_errors()
}

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
        ],
        CompilationErrorKind::ExpectedToken => vec![
            ("val 5 = 1\n".to_string(), Some("variable name")),
            ("for (x < 1) {\n}\n".to_string(), Some("for-in loop")),
            ("for (1; 1; 1) {\n}\n".to_string(), Some("for loop initializer")),
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
            ("z++\n".to_string(), None),
        ],
        CompilationErrorKind::UndefinedType => vec![(
            "impl Ghost {\n    fn boo(self) { return 1 }\n}\n".to_string(),
            None,
        )],
        CompilationErrorKind::ImmutableAssignment => vec![
            ("val x = 1\nx = 2\n".to_string(), Some("Cannot assign to")),
            ("val x = 1\nx++\n".to_string(), Some("Cannot modify")),
        ],
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
        CompilationErrorKind::NativeMethodConflict => vec![(
            "impl String {\n    fn len(self) { return 1 }\n}\n".to_string(),
            None,
        )],
        CompilationErrorKind::MethodFieldConflict => vec![(
            "struct S { x }\nimpl S {\n    fn x(self) { return 1 }\n}\n".to_string(),
            None,
        )],
        CompilationErrorKind::StaticMethodOnBuiltinType => vec![(
            "impl Array {\n    fn foo(n) { return n }\n}\n".to_string(),
            None,
        )],
        CompilationErrorKind::StaticCallOnBuiltinType => {
            vec![("Array.foo()\n".to_string(), None)]
        }
        CompilationErrorKind::LoopControlOutsideLoop => vec![("break\n".to_string(), None)],
        CompilationErrorKind::NamespaceAsValue => vec![("Math\n".to_string(), None)],
        CompilationErrorKind::InvalidIncrementTarget => vec![
            ("1++\n".to_string(), Some("Increment operator")),
            ("1--\n".to_string(), Some("Decrement operator")),
        ],
        CompilationErrorKind::UnknownNamespaceMethod => {
            vec![("Math.bogus()\n".to_string(), None)]
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
        CompilationErrorKind::NotCallable => vec![("Math()\n".to_string(), None)],
        CompilationErrorKind::TooFewArguments => vec![(
            "fn add(a, b) {\n    return a + b\n}\nadd(1)\n".to_string(),
            Some("but got 1"),
        )],
        CompilationErrorKind::TooManyArguments => vec![(
            "fn add(a, b) {\n    return a + b\n}\nadd(1, 2, 3)\n".to_string(),
            Some("but got 3"),
        )],
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
    }
}

#[test]
fn every_error_kind_is_produced_by_some_input() {
    for &kind in CompilationErrorKind::ALL {
        for (source, fragment) in sources_for(kind) {
            let errors = compile_to_errors(&source);
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
