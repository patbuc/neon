use super::module_graph::TempDir;
use crate::compiler::module_graph::EntryLocation;
use crate::compiler::semantic_tokens::SemanticToken;
use crate::compiler::{Compiler, semantic_tokens_to_json};
use std::fs;

fn describe(token: &SemanticToken) -> String {
    let m = token.modifiers;
    let modifiers: Vec<&str> = [
        (m.declaration, "declaration"),
        (m.readonly, "readonly"),
        (m.default_library, "defaultLibrary"),
    ]
    .into_iter()
    .filter(|(on, _)| *on)
    .map(|(_, name)| name)
    .collect();
    format!(
        "{}:{} {} {} {}",
        token.line,
        token.column,
        token.length,
        token.kind.name(),
        modifiers.join(",")
    )
    .trim_end()
    .to_string()
}

fn tokens_of(source: &str, entry: EntryLocation) -> Option<Vec<String>> {
    let tokens = Compiler::new().semantic_tokens(source, entry)?;
    Some(tokens.iter().map(describe).collect())
}

fn tokens(source: &str) -> Vec<String> {
    tokens_of(source, EntryLocation::None).expect("source should produce tokens")
}

fn module_tokens(dir_name: &str, files: &[(&str, &str)], main: &str) -> Option<Vec<String>> {
    let dir = TempDir::new(dir_name);
    for (name, source) in files {
        fs::write(dir.0.join(name), source).expect("Failed to write module");
    }
    let main_path = dir.0.join("main.n");
    fs::write(&main_path, main).expect("Failed to write main.n");
    tokens_of(main, EntryLocation::File(main_path))
}

#[test]
fn types_a_function_and_its_call() {
    assert_eq!(
        tokens("fn add(a, b) = a + b\nval n = add(1, 2)\n"),
        vec![
            "1:4 3 function declaration",
            "1:8 1 parameter declaration",
            "1:11 1 parameter declaration",
            "1:16 1 parameter",
            "1:20 1 parameter",
            "2:5 1 variable declaration,readonly",
            "2:9 3 function",
        ]
    );
}

#[test]
fn types_the_implicit_it_as_a_parameter() {
    assert_eq!(
        tokens("val xs = [1, 2]\nxs.map { it * 2 }\n"),
        vec![
            "1:5 2 variable declaration,readonly",
            "2:1 2 variable readonly",
            "2:10 2 parameter",
        ]
    );
}

#[test]
fn leaves_it_undeclared() {
    let found = tokens("[1].map { it }\n");
    assert_eq!(found, vec!["1:11 2 parameter"]);
}

#[test]
fn types_trailing_block_and_lambda_parameters() {
    assert_eq!(
        tokens("val f = fn(a) { a }\nval g = [1].map { x -> x }\n"),
        vec![
            "1:5 1 variable declaration,readonly",
            "1:12 1 parameter declaration",
            "1:17 1 parameter",
            "2:5 1 variable declaration,readonly",
            "2:19 1 parameter declaration",
            "2:24 1 parameter",
        ]
    );
}

#[test]
fn marks_val_readonly_and_var_not() {
    assert_eq!(
        tokens("val a = 1\nvar b = 2\nb = a\n"),
        vec![
            "1:5 1 variable declaration,readonly",
            "2:5 1 variable declaration",
            "3:1 1 variable",
            "3:5 1 variable readonly",
        ]
    );
}

#[test]
fn types_struct_enum_and_variant_names() {
    let source =
        "struct P {\n    val x\n}\nenum Color {\n    Red\n}\nval p = P(1)\nval c = Color.Red\n";
    assert_eq!(
        tokens(source),
        vec![
            "1:8 1 struct declaration",
            "4:6 5 enum declaration",
            "5:5 3 enumMember declaration",
            "7:5 1 variable declaration,readonly",
            "7:9 1 struct",
            "8:5 1 variable declaration,readonly",
            "8:9 5 enum",
            "8:15 3 enumMember",
        ]
    );
}

#[test]
fn types_payload_variants_and_match_bindings() {
    let source = "enum Shape {\n    Circle(r)\n    Dot\n}\nval s = Shape.Circle(2)\nval a = match s {\n    Shape.Circle(r) -> r\n    Shape.Dot -> 0\n}\n";
    assert_eq!(
        tokens(source),
        vec![
            "1:6 5 enum declaration",
            "2:5 6 enumMember declaration",
            "3:5 3 enumMember declaration",
            "5:5 1 variable declaration,readonly",
            "5:9 5 enum",
            "5:15 6 enumMember",
            "6:5 1 variable declaration,readonly",
            "6:15 1 variable readonly",
            "7:5 5 enum",
            "7:11 6 enumMember",
            "7:18 1 variable declaration,readonly",
            "7:24 1 variable readonly",
            "8:5 5 enum",
            "8:11 3 enumMember",
        ]
    );
}

#[test]
fn types_enum_values_call() {
    assert_eq!(
        tokens("enum C {\n    A\n}\nC.values()\n"),
        vec![
            "1:6 1 enum declaration",
            "2:5 1 enumMember declaration",
            "4:1 1 enum",
        ]
    );
}

#[test]
fn types_for_catch_and_tuple_bindings_as_variables() {
    let source = "for i in [1] {\n    print(i)\n}\nval (a, b) = [1, 2]\ntry {\n    throw 1\n} catch (e) {\n    print(e)\n}\n";
    assert_eq!(
        tokens(source),
        vec![
            "1:5 1 variable declaration,readonly",
            "2:5 5 function defaultLibrary",
            "2:11 1 variable readonly",
            "4:6 1 variable declaration,readonly",
            "4:9 1 variable declaration,readonly",
            "7:10 1 variable declaration,readonly",
            "8:5 5 function defaultLibrary",
            "8:11 1 variable readonly",
        ]
    );
}

#[test]
fn marks_the_builtins_default_library() {
    assert_eq!(
        tokens("print(args)\nsleep(1)\n"),
        vec![
            "1:1 5 function defaultLibrary",
            "1:7 4 variable defaultLibrary",
            "2:1 5 function defaultLibrary",
        ]
    );
}

#[test]
fn types_a_local_print_as_a_plain_variable() {
    assert_eq!(
        tokens("var print = 1\nval x = print + 1\n"),
        vec![
            "1:5 5 variable declaration",
            "2:5 1 variable declaration,readonly",
            "2:9 5 variable",
        ]
    );
}

#[test]
fn types_a_called_local_print_as_a_variable() {
    assert_eq!(
        tokens("fn run(print) {\n    print(1)\n}\n"),
        vec![
            "1:4 3 function declaration",
            "1:8 5 parameter declaration",
            "2:5 5 parameter",
        ]
    );
}

#[test]
fn types_a_captured_variable_like_its_declaration() {
    let source =
        "fn a() {\n    var n = 1\n    fn b() {\n        fn c() = n\n        c()\n    }\n}\n";
    assert_eq!(
        tokens(source),
        vec![
            "1:4 1 function declaration",
            "2:9 1 variable declaration",
            "3:8 1 function declaration",
            "4:12 1 function declaration",
            "4:18 1 variable",
            "5:9 1 function",
        ]
    );
}

#[test]
fn types_a_captured_parameter() {
    assert_eq!(
        tokens("fn f(x) {\n    fn g() = x\n}\n"),
        vec![
            "1:4 1 function declaration",
            "1:6 1 parameter declaration",
            "2:8 1 function declaration",
            "2:14 1 parameter",
        ]
    );
}

#[test]
fn leaves_fields_and_methods_out() {
    let source = "struct P {\n    var x\n}\nimpl P {\n    fn get(self) = self.x\n}\nval p = P(1)\np.x = 2\np.get()\n";
    assert_eq!(
        tokens(source),
        vec![
            "1:8 1 struct declaration",
            "5:12 4 parameter declaration",
            "5:20 4 parameter",
            "7:5 1 variable declaration,readonly",
            "7:9 1 struct",
            "8:1 1 variable readonly",
            "9:1 1 variable readonly",
        ]
    );
}

#[test]
fn leaves_builtin_constructors_out() {
    assert_eq!(
        tokens("val a = Array(3)\n"),
        vec!["1:5 1 variable declaration,readonly"]
    );
}

#[test]
fn counts_columns_and_length_in_characters() {
    assert_eq!(
        tokens("val name = 1\nprint(\"日本é${name}\")\n"),
        vec![
            "1:5 4 variable declaration,readonly",
            "2:1 5 function defaultLibrary",
            "2:13 4 variable readonly",
        ]
    );
}

#[test]
fn sorts_by_line_then_column() {
    let source = "fn f(x) = x\nval a = f(1)\nval b = [a, f(a)]\nfor i in b { print(i) }\n";
    let found = Compiler::new()
        .semantic_tokens(source, EntryLocation::None)
        .expect("tokens");
    let positions: Vec<(u32, u32)> = found.iter().map(|t| (t.line, t.column)).collect();
    let mut sorted = positions.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(positions, sorted);
    assert!(positions.len() > 10);
}

#[test]
fn emits_one_token_for_a_compound_assignment() {
    assert_eq!(
        tokens("var n = 1\nn += 1\n"),
        vec!["1:5 1 variable declaration", "2:1 1 variable"]
    );
}

#[test]
fn keeps_tokens_for_names_that_resolved_despite_an_undefined_one() {
    assert_eq!(
        tokens("val a = 1\nval b = a + missing\n"),
        vec![
            "1:5 1 variable declaration,readonly",
            "2:5 1 variable declaration,readonly",
            "2:9 1 variable readonly",
        ]
    );
}

#[test]
fn yields_nothing_for_a_source_that_does_not_parse() {
    assert_eq!(tokens_of("val = \n", EntryLocation::None), None);
}

#[test]
fn yields_nothing_for_a_failing_file_import() {
    assert_eq!(tokens_of("use \"nope\"\n", EntryLocation::None), None);
}

#[test]
fn types_a_module_alias_and_its_exports_by_kind() {
    let utils = "pub fn helper() = 1\npub var counter = 0\npub struct Point {\n    val x\n}\npub enum Color {\n    Red\n}\n";
    let main = "use \"utils\" as u\nval a = u.helper()\nu.counter = 2\nval p = u.Point(1)\nval c = u.Color.Red\nprint(u.counter)\n";
    assert_eq!(
        module_tokens("tokens_alias", &[("utils.n", utils)], main).expect("tokens"),
        vec![
            "1:16 1 namespace declaration",
            "2:5 1 variable declaration,readonly",
            "2:9 1 namespace",
            "2:11 6 function",
            "3:1 1 namespace",
            "3:3 7 variable",
            "4:5 1 variable declaration,readonly",
            "4:9 1 namespace",
            "4:11 5 struct",
            "5:5 1 variable declaration,readonly",
            "5:9 1 namespace",
            "5:11 5 enum",
            "5:17 3 enumMember",
            "6:1 5 function defaultLibrary",
            "6:7 1 namespace",
            "6:9 7 variable",
        ]
    );
}

#[test]
fn types_a_module_named_by_its_file_stem() {
    assert_eq!(
        module_tokens(
            "tokens_stem",
            &[("utils.n", "pub fn helper() = 1\n")],
            "use \"utils\"\nutils.helper()\n"
        )
        .expect("tokens"),
        vec!["2:1 5 namespace", "2:7 6 function"]
    );
}

#[test]
fn types_a_builtin_module_function() {
    assert_eq!(
        tokens("use \"std/math\"\nmath.sqrt(4)\n"),
        vec!["2:1 4 namespace", "2:6 4 function"]
    );
}

#[test]
fn types_a_payload_variant_of_a_module_enum() {
    let utils = "pub enum Shape {\n    Circle(r)\n}\n";
    let main = "use \"utils\"\nutils.Shape.Circle(2)\n";
    assert_eq!(
        module_tokens("tokens_enum", &[("utils.n", utils)], main).expect("tokens"),
        vec!["2:1 5 namespace", "2:7 5 enum", "2:13 6 enumMember"]
    );
}

#[test]
fn yields_nothing_when_an_import_fails_to_compile() {
    assert_eq!(
        module_tokens(
            "tokens_bad_import",
            &[("utils.n", "val x = missing\n")],
            "use \"utils\"\n"
        ),
        None
    );
}

#[test]
fn renders_tokens_as_json() {
    let json = semantic_tokens_to_json("val n = 1\n", EntryLocation::None).expect("json");
    assert_eq!(
        json,
        r#"[{"line":1,"column":5,"length":1,"type":"variable","modifiers":["declaration","readonly"]}]"#
    );
}

fn on_line(tokens: Vec<String>, line: u32) -> Vec<String> {
    let prefix = format!("{line}:");
    tokens
        .into_iter()
        .filter(|token| token.starts_with(&prefix))
        .collect()
}

#[test]
fn types_a_name_in_a_match_guard() {
    let source = "enum Shape {\n    Circle(r)\n}\nval s = Shape.Circle(2)\nval a = match s {\n    Shape.Circle(k) if k > 1 -> k\n    _ -> 0\n}\n";
    assert_eq!(
        on_line(tokens(source), 6),
        vec![
            "6:5 5 enum",
            "6:11 6 enumMember",
            "6:18 1 variable declaration,readonly",
            "6:24 1 variable readonly",
            "6:33 1 variable readonly",
        ]
    );
}

#[test]
fn types_the_base_of_a_struct_update() {
    let source = "struct P {\n    val x\n    val y\n}\nval p = P(1, 2)\nval q = P(..p, y: 2)\n";
    assert_eq!(
        on_line(tokens(source), 6),
        vec![
            "6:5 1 variable declaration,readonly",
            "6:9 1 struct",
            "6:13 1 variable readonly",
        ]
    );
}
