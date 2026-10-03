use crate::compiler::format;
use crate::compiler::Compiler;

/// Asserts `format(input) == Ok(expected)` and that `expected` is already
/// in canonical form (`format(expected) == Ok(expected)`).
fn check(input: &str, expected: &str) {
    match format(input) {
        Ok(actual) => assert_eq!(actual, expected, "formatting {input:?}"),
        Err(errors) => panic!("formatting {input:?} failed: {errors:?}"),
    }
    match format(expected) {
        Ok(actual) => assert_eq!(actual, expected, "idempotence for {expected:?}"),
        Err(errors) => panic!("formatting {expected:?} failed: {errors:?}"),
    }
}

#[test]
fn test_empty_input_formats_to_empty_string() {
    assert_eq!(format("").unwrap(), "");
}

#[test]
fn test_trailing_newline_is_added_and_not_doubled() {
    check("val x = 1", "val x = 1\n");
}

#[test]
fn test_crlf_is_normalized_to_lf() {
    let input = "val s = \"a\r\nb\"\r\n";
    assert_eq!(format(input).unwrap(), "val s = \"a\nb\"\n");
}

#[test]
fn test_format_propagates_parse_errors() {
    let source = "val = 1\n";
    let format_errors = format(source).unwrap_err();

    let mut compiler = Compiler::new();
    compiler.compile(source);
    assert_eq!(format_errors, compiler.get_structured_errors());
}

#[test]
fn test_binary_spacing() {
    check("1+2\n", "1 + 2\n");
}

#[test]
fn test_compound_assign_spacing() {
    check("x+=1\n", "x += 1\n");
}

#[test]
fn test_ternary_spacing() {
    check("c?1:2\n", "c ? 1 : 2\n");
}

#[test]
fn test_unary_operators_no_space() {
    check("-x\n", "-x\n");
    check("!a\n", "!a\n");
    check("~b\n", "~b\n");
}

#[test]
fn test_double_negate_keeps_separating_space() {
    check("-  -x\n", "- -x\n");
}

#[test]
fn test_postfix_increment() {
    check("x++\n", "x++\n");
}

#[test]
fn test_range_unchanged() {
    check("1..5\n", "1..5\n");
    check("1..=5\n", "1..=5\n");
}

#[test]
fn test_get_field_unchanged() {
    check("a.b\n", "a.b\n");
}

#[test]
fn test_index_unchanged() {
    check("a[0]\n", "a[0]\n");
}

#[test]
fn test_call_args_spacing() {
    check("f(a,b)\n", "f(a, b)\n");
}

#[test]
fn test_map_entry_spacing() {
    check("val m = {\"a\":1}\n", "val m = {\"a\": 1}\n");
}

#[test]
fn test_set_literal_spacing() {
    check("val s = #{1,2}\n", "val s = #{1, 2}\n");
}

#[test]
fn test_array_trailing_comma_dropped_when_inline() {
    check("val a = [1,2,]\n", "val a = [1, 2]\n");
}

#[test]
fn test_empty_literals() {
    check("val a = []\n", "val a = []\n");
    check("val m = {}\n", "val m = {}\n");
    check("val s = #{}\n", "val s = #{}\n");
}

#[test]
fn test_raw_number_spellings_are_kept() {
    check("0xFF\n", "0xFF\n");
    check("1_000\n", "1_000\n");
    check("1e3\n", "1e3\n");
}

#[test]
fn test_raw_string_escapes_are_kept() {
    check("\"a\\tb\"\n", "\"a\\tb\"\n");
}

#[test]
fn test_interpolation_braces_are_tight() {
    check("\"x ${ a + 1 } y\"\n", "\"x ${a + 1} y\"\n");
}

#[test]
fn test_grouping_unchanged() {
    check("((a))\n", "((a))\n");
    check("(a + b) * c\n", "(a + b) * c\n");
}

#[test]
fn test_fn_declaration_body_expands() {
    check("fn f(n) { return n }\n", "fn f(n) {\n    return n\n}\n");
}

#[test]
fn test_lambda_argument_body_expands() {
    check(
        "print(apply(fn(x) { return x + 1 }, 3))\n",
        "print(apply(fn(x) {\n    return x + 1\n}, 3))\n",
    );
}

#[test]
fn test_break_and_continue_in_lambda_body() {
    check("val f = fn() { break }\n", "val f = fn() {\n    break\n}\n");
}
