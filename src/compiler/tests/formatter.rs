use crate::compiler::format;
use crate::compiler::parser::Parser;
use crate::compiler::Comment;
use crate::compiler::CommentKind;
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

/// Asserts `format(source)` fails with exactly the errors `Compiler::compile`
/// reports for the same source.
fn check_errors_match_compiler(source: &str) {
    let format_errors = format(source).unwrap_err();

    let mut compiler = Compiler::new();
    compiler.compile(source);
    assert_eq!(format_errors, compiler.get_structured_errors());
}

#[test]
fn test_format_propagates_parse_errors() {
    check_errors_match_compiler("val = 1\n");
}

#[test]
fn test_format_propagates_multiple_parse_errors() {
    check_errors_match_compiler("val = 1\nval = 2\n");
}

#[test]
fn test_format_propagates_scanner_errors() {
    check_errors_match_compiler("val s = \"abc\n");
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
fn test_compound_assign_field_spacing() {
    check("o.n+=1\n", "o.n += 1\n");
}

#[test]
fn test_compound_assign_index_spacing() {
    check("a[i]*=2\n", "a[i] *= 2\n");
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
fn test_range_unchanged() {
    check("1 .. 5\n", "1..5\n");
    check("1 ..= 5\n", "1..=5\n");
}

#[test]
fn test_match_arms_one_per_line() {
    check(
        "val x = match c { 1, 2 -> \"a\"\n _ -> \"b\" }\n",
        "val x = match c {\n    1, 2 -> \"a\"\n    _ -> \"b\"\n}\n",
    );
}

#[test]
fn test_match_arm_spacing() {
    check(
        "val x = match c {\n1,2->\"a\"\n_->\"b\"\n}\n",
        "val x = match c {\n    1, 2 -> \"a\"\n    _ -> \"b\"\n}\n",
    );
}

#[test]
fn test_match_block_arm() {
    check(
        "val x = match c {\nColor.Red -> {\nprint(1)\n}\n_ -> {\nprint(2)\n}\n}\n",
        "val x = match c {\n    Color.Red -> {\n        print(1)\n    }\n    _ -> {\n        print(2)\n    }\n}\n",
    );
}

#[test]
fn test_match_comment_between_arms_kept() {
    check(
        "val x = match c {\n1 -> \"a\"\n// second\n_ -> \"b\"\n}\n",
        "val x = match c {\n    1 -> \"a\"\n    // second\n    _ -> \"b\"\n}\n",
    );
}

#[test]
fn test_match_blank_line_between_arms_kept() {
    check(
        "val x = match c {\n1 -> \"a\"\n\n_ -> \"b\"\n}\n",
        "val x = match c {\n    1 -> \"a\"\n\n    _ -> \"b\"\n}\n",
    );
}

#[test]
fn test_match_trailing_comment_on_arm_kept() {
    check(
        "val x = match c {\n1 -> \"one\" // c\n_ -> \"b\"\n}\n",
        "val x = match c {\n    1 -> \"one\" // c\n    _ -> \"b\"\n}\n",
    );
}

#[test]
fn test_match_range_patterns() {
    check(
        "val x = match c {\n400..500 -> \"a\"\n-5..=-1 -> \"b\"\n_ -> \"c\"\n}\n",
        "val x = match c {\n    400..500 -> \"a\"\n    -5..=-1 -> \"b\"\n    _ -> \"c\"\n}\n",
    );
}

#[test]
fn test_get_field_unchanged() {
    check("a . b\n", "a.b\n");
}

#[test]
fn test_index_unchanged() {
    check("a[ 0 ]\n", "a[0]\n");
}

#[test]
fn test_call_args_spacing() {
    check("f ( a , b )\n", "f(a, b)\n");
}

#[test]
fn test_map_entry_spacing() {
    check("val m = { \"a\" : 1 }\n", "val m = {\"a\": 1}\n");
}

#[test]
fn test_set_literal_spacing() {
    check("val s = #{ 1 , 2 }\n", "val s = #{1, 2}\n");
}

#[test]
fn test_array_trailing_comma_dropped_when_inline() {
    check("val a = [ 1 , 2 , ]\n", "val a = [1, 2]\n");
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
    check("( a )\n", "(a)\n");
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

#[test]
fn test_if_expression_body_expands() {
    check(
        "val s = if a { 1 } else { 2 }\n",
        "val s = if a {\n    1\n} else {\n    2\n}\n",
    );
}

#[test]
fn test_if_expression_else_if_chain_expands() {
    check(
        "val s = if a { 1 } else if b { 2 } else { 3 }\n",
        "val s = if a {\n    1\n} else if b {\n    2\n} else {\n    3\n}\n",
    );
}

#[test]
fn test_if_expression_as_call_argument_expands() {
    check(
        "f(if a { 1 } else { 2 })\n",
        "f(if a {\n    1\n} else {\n    2\n})\n",
    );
}

#[test]
fn test_if_else_blocks_expand_and_cuddle() {
    check(
        "if (c) { a } else { b }\n",
        "if c {\n    a\n} else {\n    b\n}\n",
    );
}

#[test]
fn test_else_on_its_own_line_is_cuddled() {
    check(
        "if (c) {\n    a\n}\nelse {\n    b\n}\n",
        "if c {\n    a\n} else {\n    b\n}\n",
    );
}

#[test]
fn test_empty_if_block() {
    check("if(c){}\n", "if c {}\n");
}

#[test]
fn test_else_if_chain() {
    check(
        "if (a) { x() } else if (b) { y() } else { z() }\n",
        "if a {\n    x()\n} else if b {\n    y()\n} else {\n    z()\n}\n",
    );
}

#[test]
fn test_while_with_block_body() {
    check("while (c) { x() }\n", "while c {\n    x()\n}\n");
}

#[test]
fn test_if_condition_with_operand_parens_is_kept() {
    check("if (a) && (b) { x() }\n", "if (a) && (b) {\n    x()\n}\n");
}

#[test]
fn test_while_missing_space_before_paren_is_added() {
    check("while(c) { x() }\n", "while c {\n    x()\n}\n");
}

#[test]
fn test_nested_condition_parens_are_removed() {
    check(
        "if ((x > 0)) {\n    print(x)\n}\n",
        "if x > 0 {\n    print(x)\n}\n",
    );
    check("while ((c)) { x() }\n", "while c {\n    x()\n}\n");
}

#[test]
fn test_for_in_loop() {
    check(
        "for x in xs {\n    print(x)\n}\n",
        "for x in xs {\n    print(x)\n}\n",
    );
}

#[test]
fn test_for_in_collection_parens_are_removed() {
    check(
        "for x in (xs) {\n    print(x)\n}\n",
        "for x in xs {\n    print(x)\n}\n",
    );
}

#[test]
fn test_condition_parens_with_comment_are_kept() {
    check(
        "if ( // c\n    x > 0) {\n    print(x)\n}\n",
        "if ( // c\n    x > 0\n) {\n    print(x)\n}\n",
    );
}

#[test]
fn test_struct_one_field_per_line() {
    check("struct P { x y }\n", "struct P {\n    x\n    y\n}\n");
}

#[test]
fn test_empty_struct() {
    check("struct E {}\n", "struct E {}\n");
}

#[test]
fn test_enum_one_variant_per_line() {
    check(
        "enum Color { Red Green Blue }\n",
        "enum Color {\n    Red\n    Green\n    Blue\n}\n",
    );
}

#[test]
fn test_enum_payload_variants_unchanged() {
    let src = "enum Shape {\n    Circle(radius)\n    Rect(w, h)\n    Square\n}\n";
    check(src, src);
}

#[test]
fn test_impl_with_two_methods() {
    check(
        "impl Point {\nfn len(self) { return self.x }\nfn origin() { return Point(0, 0) }\n}\n",
        "impl Point {\n    fn len(self) {\n        return self.x\n    }\n    fn origin() {\n        return Point(0, 0)\n    }\n}\n",
    );
}

#[test]
fn test_expression_bodied_fn_stays_one_line() {
    check("fn sq(x) = x * x\n", "fn sq(x) = x * x\n");
}

#[test]
fn test_expression_bodied_fn_spacing_normalized() {
    check("fn sq(x)=x*x\n", "fn sq(x) = x * x\n");
}

#[test]
fn test_expression_bodied_impl_method_stays_one_line() {
    check(
        "impl P {\n    fn len(self) = self.x\n}\n",
        "impl P {\n    fn len(self) = self.x\n}\n",
    );
}

#[test]
fn test_expression_bodied_fn_long_body_wraps() {
    check("fn f(a) = a +\n    1\n", "fn f(a) = a +\n    1\n");
}

#[test]
fn test_corpus_nested_fn_returning_lambda() {
    check(
        "fn getF() { print(\"callee\")\nreturn fn(a) { return a } }\n",
        "fn getF() {\n    print(\"callee\")\n    return fn(a) {\n        return a\n    }\n}\n",
    );
}

#[test]
fn test_call_args_broken_after_open_paren() {
    check("foo(\na, b)\n", "foo(\n    a,\n    b,\n)\n");
}

#[test]
fn test_call_args_broken_between_args() {
    check("foo(a,\nb)\n", "foo(\n    a,\n    b,\n)\n");
}

#[test]
fn test_call_args_broken_before_close_paren() {
    check("foo(a, b\n)\n", "foo(\n    a,\n    b,\n)\n");
}

#[test]
fn test_call_args_empty_broken_collapses() {
    check("foo(\n)\n", "foo()\n");
}

#[test]
fn test_lambda_call_arg_already_broken_stays_broken() {
    check(
        "g.map(fn(x) {\nreturn x\n})\n",
        "g.map(fn(x) {\n    return x\n})\n",
    );
}

#[test]
fn test_trailing_block_one_line_stays_one_line() {
    check(
        "print([1, 2].map { it * 2 })\n",
        "print([1, 2].map { it * 2 })\n",
    );
}

#[test]
fn test_trailing_block_with_params_one_line_stays_one_line() {
    check(
        "[1, 2].reduce(0) { acc, x -> acc + x }\n",
        "[1, 2].reduce(0) { acc, x -> acc + x }\n",
    );
}

#[test]
fn test_trailing_block_multi_line_stays_multi_line() {
    check(
        "xs.forEach {\n    print(it)\n}\n",
        "xs.forEach {\n    print(it)\n}\n",
    );
}

#[test]
fn test_empty_trailing_block_stays_empty() {
    check("call {}\n", "call {}\n");
}

#[test]
fn test_trailing_block_empty_body_with_params_stays_one_line() {
    check("f { a -> }\n", "f { a -> }\n");
}

#[test]
fn test_trailing_block_with_return_stays_one_line() {
    check("[1].map { return it }\n", "[1].map { return it }\n");
}

#[test]
fn test_trailing_block_with_val_stays_one_line() {
    check("call { val y = 1 }\n", "call { val y = 1 }\n");
}

#[test]
fn test_map_literal_one_entry_per_line_stays() {
    check(
        "val m = {\n    \"a\": 1,\n    \"b\": 2,\n}\n",
        "val m = {\n    \"a\": 1,\n    \"b\": 2,\n}\n",
    );
}

#[test]
fn test_set_literal_broken_with_trailing_comma_stays() {
    check("val s = #{\n1,\n2,\n}\n", "val s = #{\n    1,\n    2,\n}\n");
}

#[test]
fn test_fn_params_broken_across_lines() {
    check("fn f(a,\nb) {}\n", "fn f(\n    a,\n    b,\n) {}\n");
}

#[test]
fn test_grouping_broken_across_lines() {
    check("val x = (\n1 + 2\n)\n", "val x = (\n    1 + 2\n)\n");
}

#[test]
fn test_val_initializer_continuation_reindents_to_one_level() {
    check("val x = 1 +\n        2\n", "val x = 1 +\n    2\n");
}

#[test]
fn test_val_without_space_before_break() {
    check("val z =\n5\n", "val z =\n    5\n");
}

#[test]
fn test_ternary_continuation() {
    check("val x = c ?\n1 : 2\n", "val x = c ?\n    1 : 2\n");
}

#[test]
fn test_map_entry_value_continuation() {
    check("val m = {\"a\":\n 1}\n", "val m = {\"a\":\n    1}\n");
}

#[test]
fn test_binary_chain_continuation_is_flat() {
    check("val x = a +\nb +\nc\n", "val x = a +\n    b +\n    c\n");
}

#[test]
fn test_continuation_inside_block_does_not_leak_to_next_statement() {
    check(
        "fn f() {\n    val x = 1 +\n        2\n    val y = 3\n}\n",
        "fn f() {\n    val x = 1 +\n        2\n    val y = 3\n}\n",
    );
}

#[test]
fn test_top_level_continuation_does_not_leak_to_next_statement() {
    check(
        "val x = 1 +\n2\nval y = 3\n",
        "val x = 1 +\n    2\nval y = 3\n",
    );
}

#[test]
fn test_continuation_inside_an_unbroken_call_arg_starts_its_own_scope() {
    check(
        "val x = 1 +\nf(a +\nb)\n",
        "val x = 1 +\n    f(a +\n        b)\n",
    );
}

#[test]
fn test_continuation_inside_an_unbroken_grouping_starts_its_own_scope() {
    check(
        "val x = 1 +\n(a +\nb)\n",
        "val x = 1 +\n    (a +\n        b)\n",
    );
}

#[test]
fn test_fmt_leading_dot_chain() {
    check(
        "val r = [1, 2, 3, 4]\n.reverse()\n        .slice(0, 2)\nprint(r)\n",
        "val r = [1, 2, 3, 4]\n    .reverse()\n    .slice(0, 2)\nprint(r)\n",
    );
}

#[test]
fn test_fmt_leading_dot_assign() {
    check("p\n    .x\n    .y = 2\n", "p\n    .x\n    .y = 2\n");
}

#[test]
fn test_fmt_single_line_chain() {
    check(
        "val r = [1, 2, 3, 4].reverse().slice(0, 2)\n",
        "val r = [1, 2, 3, 4].reverse().slice(0, 2)\n",
    );
}

#[test]
fn test_fmt_leading_dot_trailing_comment() {
    check(
        "val r = [1, 2, 3, 4]\n    .reverse() // t\n    .slice(0, 2)\n",
        "val r = [1, 2, 3, 4]\n    .reverse() // t\n    .slice(0, 2)\n",
    );
}

#[test]
fn test_compound_assign_continuation() {
    check("x +=\n1\n", "x +=\n    1\n");
}

#[test]
fn test_compound_assign_field_continuation() {
    check("o.n +=\n1\n", "o.n +=\n    1\n");
}

#[test]
fn test_compound_assign_index_continuation() {
    check("a[i] +=\n1\n", "a[i] +=\n    1\n");
}

#[test]
fn test_set_field_continuation() {
    check("a.b =\n1\n", "a.b =\n    1\n");
}

#[test]
fn test_index_assign_continuation() {
    check("a[0] =\n1\n", "a[0] =\n    1\n");
}

#[test]
fn test_nested_broken_lists_indent_one_level_each() {
    check("foo(bar(\na,\nb\n))\n", "foo(bar(\n    a,\n    b,\n))\n");
}

#[test]
fn test_broken_call_arg_continuation_does_not_leak_to_next_arg() {
    check(
        "f(\na +\nb,\nc\n)\n",
        "f(\n    a +\n        b,\n    c,\n)\n",
    );
}

#[test]
fn test_broken_if_condition_continuation_does_not_leak_into_block_body() {
    check(
        "if (a &&\nb) {\nx()\n}\nval q = 1\n",
        "if a &&\n    b {\n    x()\n}\nval q = 1\n",
    );
}

#[test]
fn test_broken_if_condition_indent_does_not_leak() {
    check(
        "if (a &&\nb) {\nx()\n}\nelse {\ny()\n}\n",
        "if a &&\n    b {\n    x()\n} else {\n    y()\n}\n",
    );
}

#[test]
fn test_broken_while_condition_continuation_does_not_leak_into_body() {
    check(
        "while (a &&\nb) {\nx()\n}\n",
        "while a &&\n    b {\n    x()\n}\n",
    );
}

#[test]
fn test_broken_for_in_collection_continuation_does_not_leak_into_body() {
    check(
        "for x in a +\nb {\ny()\n}\n",
        "for x in a +\n    b {\n    y()\n}\n",
    );
}

#[test]
fn test_val_continuation_into_lambda_body_does_not_leak_to_next_statement() {
    check(
        "val f =\nfn(x) {\nreturn x\n}\nval q = 2\n",
        "val f =\n    fn(x) {\n        return x\n    }\nval q = 2\n",
    );
}

#[test]
fn test_several_top_level_statements() {
    check(
        "val x = 1\nvar y = 2\nprint(x)\nprint(y)\n",
        "val x = 1\nvar y = 2\nprint(x)\nprint(y)\n",
    );
}

#[test]
fn test_set_field_one_line() {
    check("a.b = 1\n", "a.b = 1\n");
}

#[test]
fn test_index_assign_one_line() {
    check("a[0] = 1\n", "a[0] = 1\n");
}

#[test]
fn test_boolean_and_nil_literals() {
    check("true\n", "true\n");
    check("false\n", "false\n");
    check("nil\n", "nil\n");
}

#[test]
fn test_plain_string_literal() {
    check("\"hello\"\n", "\"hello\"\n");
}

#[test]
fn test_multiple_blank_lines_collapse_to_one() {
    check("val a = 1\n\n\n\nval b = 2\n", "val a = 1\n\nval b = 2\n");
}

#[test]
fn test_leading_blank_lines_are_dropped() {
    check("\n\n\nval a = 1\n", "val a = 1\n");
}

#[test]
fn test_blank_after_open_brace_is_dropped() {
    check("fn f() {\n\n    a()\n}\n", "fn f() {\n    a()\n}\n");
}

#[test]
fn test_blank_before_close_brace_is_dropped() {
    check("fn f() {\n    a()\n\n}\n", "fn f() {\n    a()\n}\n");
}

#[test]
fn test_trailing_blank_at_eof_is_dropped() {
    check("val a = 1\n\n\n", "val a = 1\n");
}

#[test]
fn test_whitespace_only_line_counts_as_blank() {
    check("val a = 1\n   \nval b = 2\n", "val a = 1\n\nval b = 2\n");
}

#[test]
fn test_blank_between_list_items_is_kept() {
    check(
        "val a = [\n    1,\n\n    2,\n]\n",
        "val a = [\n    1,\n\n    2,\n]\n",
    );
}

#[test]
fn test_blank_after_open_bracket_is_dropped() {
    check(
        "val a = [\n\n    1,\n    2,\n]\n",
        "val a = [\n    1,\n    2,\n]\n",
    );
}

#[test]
fn test_blank_in_continuation_is_dropped() {
    check("val x = 1 +\n\n    2\n", "val x = 1 +\n    2\n");
}

#[test]
fn test_no_trailing_whitespace_on_any_line() {
    let source = "fn f(a, b) {\n    val x = 1 +\n        2\n    return x\n}\n";
    let formatted = crate::compiler::format(source).unwrap();
    for line in formatted.lines() {
        assert!(!line.ends_with(' '), "trailing whitespace in {line:?}");
    }
}

#[test]
fn test_expected_header_then_blank_then_code() {
    check(
        "// Expected:\n// 1\n\nprint(1)\n",
        "// Expected:\n// 1\n\nprint(1)\n",
    );
}

#[test]
fn test_trailing_comment_trimmed_and_one_space() {
    check("print(1)    // x  \n", "print(1) // x\n");
}

#[test]
fn test_own_line_comment_at_col_zero_gets_indented() {
    check(
        "fn f() {\n// hi\n    a()\n}\n",
        "fn f() {\n    // hi\n    a()\n}\n",
    );
}

#[test]
fn test_comments_after_open_and_before_close_kept_with_blank_rules() {
    check(
        "fn f() {\n    // open\n\n    a()\n\n    // close\n}\n",
        "fn f() {\n    // open\n\n    a()\n\n    // close\n}\n",
    );
}

#[test]
fn test_trailing_comment_after_open_brace_stays_on_that_line() {
    check(
        "if (c) { // why\n    a()\n}\n",
        "if c { // why\n    a()\n}\n",
    );
}

#[test]
fn test_trailing_comment_after_expanded_one_line_block() {
    check("if (c) { a } // x\n", "if c {\n    a\n} // x\n");
}

#[test]
fn test_broken_list_with_trailing_and_own_line_comment() {
    check(
        "foo(\n    a, // a\n    // b\n    b,\n)\n",
        "foo(\n    a, // a\n    // b\n    b,\n)\n",
    );
}

#[test]
fn test_own_line_comment_inside_empty_block() {
    check("fn f() {\n// todo\n}\n", "fn f() {\n    // todo\n}\n");
}

#[test]
fn test_own_line_comment_inside_broken_call_with_no_args() {
    check("foo(\n// c\n)\n", "foo(\n    // c\n)\n");
}

#[test]
fn test_trailing_comment_after_binary_operator() {
    check("1 + // c\n2\n", "1 + // c\n    2\n");
}

#[test]
fn test_trailing_comment_after_multiline_expr_bodied_fn_binary() {
    check("fn f() = 1 +\n    2 // c\n", "fn f() = 1 +\n    2 // c\n");
}

#[test]
fn test_trailing_comment_after_multiline_expr_bodied_fn_array() {
    check(
        "fn f(a) = [\n    a,\n] // c\n",
        "fn f(a) = [\n    a,\n] // c\n",
    );
}

#[test]
fn test_expr_bodied_fn_body_on_next_line_kept_as_is() {
    check("fn f() =\n    1\n", "fn f() =\n    1\n");
}

#[test]
fn test_eof_comment_after_last_statement() {
    check("print(1)\n// eof\n", "print(1)\n// eof\n");
}

#[test]
fn test_comments_only_file() {
    check("// just a comment\n", "// just a comment\n");
}

#[test]
fn test_consecutive_comments_stay_contiguous() {
    check(
        "// one\n// two\n// three\nprint(1)\n",
        "// one\n// two\n// three\nprint(1)\n",
    );
}

#[test]
fn test_unplaceable_comment_between_block_and_else_is_an_error() {
    let source = "if (a) {\n} // c\nelse {\n}\n";
    let errors = crate::compiler::format(source).unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("line 2"), "{:?}", errors[0]);
    assert_eq!(errors[0].location.line, 2);
    assert_eq!(errors[0].location.column, 3);
}

#[test]
fn test_unplaceable_comment_inside_interpolation_is_an_error() {
    let source = "print(\"${x // c\n}\")\n";
    let errors = crate::compiler::format(source).unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("line 1"), "{:?}", errors[0]);
}

fn corpus_comments(source: &str) -> Vec<Comment> {
    let mut parser = Parser::new(source);
    let _ = parser.parse();
    parser.trivia().comments.clone()
}

#[test]
fn test_corpus_formats_idempotently_and_preserves_comments() {
    for dir in ["tests/scripts", "benches"] {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("n") {
                continue;
            }
            let source = std::fs::read_to_string(&path).unwrap();

            let formatted =
                format(&source).unwrap_or_else(|e| panic!("format({path:?}) failed: {e:?}"));
            let reformatted = format(&formatted)
                .unwrap_or_else(|e| panic!("format(format({path:?})) failed: {e:?}"));
            assert_eq!(formatted, reformatted, "not idempotent: {path:?}");

            let input_comments = corpus_comments(&source);
            let output_comments = corpus_comments(&formatted);
            let input_signature: Vec<_> = input_comments
                .iter()
                .map(|c| (c.text.trim_end().to_string(), c.kind))
                .collect();
            let output_signature: Vec<_> = output_comments
                .iter()
                .map(|c| (c.text.trim_end().to_string(), c.kind))
                .collect();
            assert_eq!(
                input_signature, output_signature,
                "comments changed: {path:?}"
            );

            for i in 0..input_comments.len().saturating_sub(1) {
                if input_comments[i + 1].kind == CommentKind::OwnLine
                    && input_comments[i + 1].line == input_comments[i].line + 1
                {
                    assert_eq!(
                        output_comments[i + 1].line,
                        output_comments[i].line + 1,
                        "adjacent comments drifted apart in {path:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn test_match_guards_and_binding_patterns() {
    check(
        "val x = match c {\nn if n>1 ->\"a\"\n1,2 if  ok(1)->\"b\"\nm->m\n}\n",
        "val x = match c {\n    n if n > 1 -> \"a\"\n    1, 2 if ok(1) -> \"b\"\n    m -> m\n}\n",
    );
}

#[test]
fn test_match_array_patterns_unchanged() {
    let source = "val x = match c {\n    [] -> 0\n    [a, _] -> a\n    [1, [b, c]] -> b + c\n    _ -> 1\n}\n";
    check(source, source);
}

#[test]
fn test_match_underscore_rest_formats_as_bare_rest() {
    check(
        "val x = match c {\n    [.._, 1] -> 1\n    _ -> 0\n}\n",
        "val x = match c {\n    [.., 1] -> 1\n    _ -> 0\n}\n",
    );
}
