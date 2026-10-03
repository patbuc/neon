use crate::compiler::ast::Stmt;
use crate::compiler::formatter::SourceMap;
use crate::compiler::parser::Parser;
use crate::compiler::token::TokenType;

#[test]
fn test_partners_of_nested_brackets() {
    // f  (  a  ,  [  b  ,  {  c  :  d  }  ]  ,  #{ e  }  )
    // 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17
    let source = "f(a, [b, {c: d}], #{e})\n";
    let map = SourceMap::new(source);

    assert_eq!(*map.kind(1), TokenType::LeftParen);
    assert_eq!(map.partner(1), 17);
    assert_eq!(*map.kind(17), TokenType::RightParen);

    assert_eq!(*map.kind(4), TokenType::LeftBracket);
    assert_eq!(map.partner(4), 12);
    assert_eq!(*map.kind(12), TokenType::RightBracket);

    assert_eq!(*map.kind(7), TokenType::LeftBrace);
    assert_eq!(map.partner(7), 11);
    assert_eq!(*map.kind(11), TokenType::RightBrace);

    assert_eq!(*map.kind(14), TokenType::HashLeftBrace);
    assert_eq!(map.partner(14), 16);
    assert_eq!(*map.kind(16), TokenType::RightBrace);
}

#[test]
fn test_nested_interpolation_pairs_string_start_and_end() {
    // StringStart(outer) StringStart(inner) x StringEnd(inner) StringEnd(outer)
    //      0                   1             2       3               4
    let source = "\"a ${\"b ${x}\"} c\"\n";
    let map = SourceMap::new(source);

    assert_eq!(*map.kind(0), TokenType::StringStart);
    assert_eq!(*map.kind(1), TokenType::StringStart);
    assert_eq!(*map.kind(3), TokenType::StringEnd);
    assert_eq!(*map.kind(4), TokenType::StringEnd);

    assert_eq!(
        map.partner(1),
        3,
        "inner string pairs with the nearer StringEnd"
    );
    assert_eq!(
        map.partner(0),
        4,
        "outer string pairs with the final StringEnd"
    );
}

#[test]
fn test_grouping_statement_spans_its_source_lines() {
    let source = "(\n1 + 2\n) * 3\n";
    let mut parser = Parser::new(source);
    let stmts = parser.parse().expect("should parse");
    let map = SourceMap::new(source);

    assert_eq!(map.stmt_first_line(&stmts[0]), 1);
    assert_eq!(map.stmt_last_line(&stmts[0]), 3);
}

#[test]
fn test_call_closing_paren_on_its_own_line() {
    let source = "f(\n    1,\n    2\n)\n";
    let mut parser = Parser::new(source);
    let stmts = parser.parse().expect("should parse");
    let map = SourceMap::new(source);

    assert_eq!(map.stmt_last_line(&stmts[0]), 4);
}

#[test]
fn test_multiline_string_end_line() {
    let source = "val s = \"a\nb\"\n";
    let mut parser = Parser::new(source);
    let stmts = parser.parse().expect("should parse");
    let map = SourceMap::new(source);

    assert_eq!(map.stmt_last_line(&stmts[0]), 2);
}

#[test]
fn test_fn_with_broken_params_lines() {
    let source = "fn f(\n    a,\n    b\n) {\n    return a\n}\n";
    let mut parser = Parser::new(source);
    let stmts = parser.parse().expect("should parse");
    let map = SourceMap::new(source);

    match &stmts[0] {
        Stmt::Fn { location, .. } => {
            let fn_tokens = map.fn_tokens(location);
            assert_eq!(map.param_lines(&fn_tokens), vec![2, 3]);
            assert_eq!(map.line(fn_tokens.body_open), 4);
            assert_eq!(map.line(fn_tokens.body_close), 6);
        }
        _ => panic!("Expected Fn statement"),
    }
}

#[test]
fn test_struct_close_brace_line() {
    let source = "struct P {\n    x\n    y\n}\n";
    let mut parser = Parser::new(source);
    let stmts = parser.parse().expect("should parse");
    let map = SourceMap::new(source);

    assert_eq!(map.stmt_last_line(&stmts[0]), 4);
}

#[test]
fn test_if_else_last_line_is_else_branch() {
    let source = "if (c) {\n    a\n} else {\n    b\n}\n";
    let mut parser = Parser::new(source);
    let stmts = parser.parse().expect("should parse");
    let map = SourceMap::new(source);

    assert_eq!(map.stmt_last_line(&stmts[0]), 5);
}

#[test]
fn test_lambda_last_line_is_body_close() {
    let source = "val f = fn(x) {\n    return x\n}\n";
    let mut parser = Parser::new(source);
    let stmts = parser.parse().expect("should parse");
    let map = SourceMap::new(source);

    assert_eq!(map.stmt_last_line(&stmts[0]), 3);
}

#[test]
fn test_binary_expression_first_and_last_line() {
    let source = "1 +\n2\n";
    let mut parser = Parser::new(source);
    let stmts = parser.parse().expect("should parse");
    let map = SourceMap::new(source);

    match &stmts[0] {
        Stmt::Expression { expr, .. } => {
            assert_eq!(map.first_line(expr), 1);
            assert_eq!(map.last_line(expr), 2);
        }
        _ => panic!("Expected Expression statement"),
    }
}

#[test]
fn test_get_field_last_token_is_field_name() {
    let source = "a.b\n";
    let mut parser = Parser::new(source);
    let stmts = parser.parse().expect("should parse");
    let map = SourceMap::new(source);

    match &stmts[0] {
        Stmt::Expression { expr, .. } => {
            let last = map.last_token(expr);
            assert_eq!(*map.kind(last), TokenType::Identifier);
            assert_eq!(map.text(last), "b");
        }
        _ => panic!("Expected Expression statement"),
    }
}
