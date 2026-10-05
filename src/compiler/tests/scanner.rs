use crate::common::errors::CompilationErrorKind;
use crate::compiler::scanner::KEYWORDS;
use crate::compiler::token::TokenType;
use crate::compiler::CommentKind;
use crate::compiler::Scanner;
use crate::compiler::Token;
use std::collections::BTreeSet;

fn scan_to_eof(scanner: &mut Scanner) {
    loop {
        if scanner.scan_token().token_type == TokenType::Eof {
            break;
        }
    }
}

fn collect_tokens(mut scanner: Scanner) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let token = scanner.scan_token();
        if token.token_type == TokenType::Eof {
            tokens.push(token);
            break;
        }
        if token.token_type == TokenType::NewLine
            && (tokens.is_empty() || tokens[tokens.len() - 1].token_type == TokenType::NewLine)
        {
            continue;
        }
        tokens.push(token);
    }
    tokens
}

fn assert_first_token(source: &str, token_type: TokenType, lexeme: &str) {
    let tokens = collect_tokens(Scanner::new(source));
    assert_eq!(tokens[0].token_type, token_type, "source {source:?}");
    assert_eq!(tokens[0].token, lexeme, "source {source:?}");
    assert_eq!(tokens.len(), 2, "source {source:?}");
}

#[test]
fn can_scan_simple_statement() {
    let script = "var a = 1;";

    let scanner = Scanner::new(script);
    let x: Vec<Token> = collect_tokens(scanner);

    assert_eq!(x.len(), 6);

    assert_eq!(x[0].token_type, TokenType::Var);
    assert_eq!(x[0].column, 1);
    assert_eq!(x[0].token, "var");
    assert_eq!(x[0].line, 1);

    assert_eq!(x[1].token_type, TokenType::Identifier);
    assert_eq!(x[1].column, 5);
    assert_eq!(x[1].token, "a");
    assert_eq!(x[1].line, 1);

    assert_eq!(x[2].token_type, TokenType::Equal);
    assert_eq!(x[3].token_type, TokenType::Number);
    assert_eq!(x[4].token_type, TokenType::Semicolon);
    assert_eq!(x[5].token_type, TokenType::Eof);
}

#[test]
fn can_scan_interpolated_string() {
    let script = "var a = \"This is an ${interpolated} string\";";

    let scanner = Scanner::new(script);
    let x: Vec<Token> = collect_tokens(scanner);

    assert_eq!(x.len(), 8);

    assert_eq!(x[0].token_type, TokenType::Var);
    assert_eq!(x[0].column, 1);
    assert_eq!(x[0].token, "var");
    assert_eq!(x[0].line, 1);

    assert_eq!(x[1].token_type, TokenType::Identifier);
    assert_eq!(x[1].column, 5);
    assert_eq!(x[1].token, "a");
    assert_eq!(x[1].line, 1);

    assert_eq!(x[2].token_type, TokenType::Equal);

    assert_eq!(x[3].token_type, TokenType::StringStart);
    assert_eq!(x[3].column, 9);
    assert_eq!(x[3].token, "This is an ");

    assert_eq!(x[4].token_type, TokenType::Identifier);
    assert_eq!(x[4].column, 23);
    assert_eq!(x[4].token, "interpolated");

    assert_eq!(x[5].token_type, TokenType::StringEnd);
    assert_eq!(x[5].column, 35);
    assert_eq!(x[5].token, " string");

    assert_eq!(x[6].token_type, TokenType::Semicolon);
    assert_eq!(x[7].token_type, TokenType::Eof);
}

#[test]
fn can_scan_logical_and_operator() {
    let script = "true && false";

    let scanner = Scanner::new(script);
    let x: Vec<Token> = collect_tokens(scanner);

    assert_eq!(x.len(), 4);
    assert_eq!(x[0].token_type, TokenType::True);
    assert_eq!(x[1].token_type, TokenType::AndAnd);
    assert_eq!(x[1].token, "&&");
    assert_eq!(x[2].token_type, TokenType::False);
    assert_eq!(x[3].token_type, TokenType::Eof);
}

#[test]
fn can_scan_logical_or_operator() {
    let script = "true || false";

    let scanner = Scanner::new(script);
    let x: Vec<Token> = collect_tokens(scanner);

    assert_eq!(x.len(), 4);
    assert_eq!(x[0].token_type, TokenType::True);
    assert_eq!(x[1].token_type, TokenType::OrOr);
    assert_eq!(x[1].token, "||");
    assert_eq!(x[2].token_type, TokenType::False);
    assert_eq!(x[3].token_type, TokenType::Eof);
}

#[test]
fn can_scan_complex_logical_expression() {
    let script = "x > 5 && y < 10 || z == 0";

    let scanner = Scanner::new(script);
    let x: Vec<Token> = collect_tokens(scanner);

    assert_eq!(x.len(), 12);
    assert_eq!(x[0].token_type, TokenType::Identifier);
    assert_eq!(x[1].token_type, TokenType::Greater);
    assert_eq!(x[2].token_type, TokenType::Number);
    assert_eq!(x[3].token_type, TokenType::AndAnd);
    assert_eq!(x[4].token_type, TokenType::Identifier);
    assert_eq!(x[5].token_type, TokenType::Less);
    assert_eq!(x[6].token_type, TokenType::Number);
    assert_eq!(x[7].token_type, TokenType::OrOr);
    assert_eq!(x[8].token_type, TokenType::Identifier);
    assert_eq!(x[9].token_type, TokenType::EqualEqual);
    assert_eq!(x[10].token_type, TokenType::Number);
    assert_eq!(x[11].token_type, TokenType::Eof);
}

#[test]
fn plus_plus_scans_as_two_plus_tokens() {
    let script = "x++";

    let scanner = Scanner::new(script);
    let x: Vec<Token> = collect_tokens(scanner);

    assert_eq!(x.len(), 4);
    assert_eq!(x[0].token_type, TokenType::Identifier);
    assert_eq!(x[0].token, "x");
    assert_eq!(x[1].token_type, TokenType::Plus);
    assert_eq!(x[1].token, "+");
    assert_eq!(x[2].token_type, TokenType::Plus);
    assert_eq!(x[2].token, "+");
    assert_eq!(x[3].token_type, TokenType::Eof);
}

#[test]
fn minus_minus_scans_as_two_minus_tokens() {
    let script = "x--";

    let scanner = Scanner::new(script);
    let x: Vec<Token> = collect_tokens(scanner);

    assert_eq!(x.len(), 4);
    assert_eq!(x[0].token_type, TokenType::Identifier);
    assert_eq!(x[0].token, "x");
    assert_eq!(x[1].token_type, TokenType::Minus);
    assert_eq!(x[1].token, "-");
    assert_eq!(x[2].token_type, TokenType::Minus);
    assert_eq!(x[2].token, "-");
    assert_eq!(x[3].token_type, TokenType::Eof);
}

#[test]
fn can_scan_compound_assignment_operators() {
    let script = "x += y -= z *= w /= v %= u **= t";

    let scanner = Scanner::new(script);
    let x: Vec<Token> = collect_tokens(scanner);

    let operators: Vec<&TokenType> = x
        .iter()
        .map(|t| &t.token_type)
        .filter(|t| **t != TokenType::Identifier)
        .collect();
    assert_eq!(
        operators,
        vec![
            &TokenType::PlusEqual,
            &TokenType::MinusEqual,
            &TokenType::StarEqual,
            &TokenType::SlashEqual,
            &TokenType::PercentEqual,
            &TokenType::StarStarEqual,
            &TokenType::Eof,
        ]
    );
}

#[test]
fn can_scan_question_dot_vs_ternary_with_space() {
    let scanner = Scanner::new("x ? .y");
    let x: Vec<Token> = collect_tokens(scanner);
    let types: Vec<&TokenType> = x.iter().map(|t| &t.token_type).collect();
    assert_eq!(
        types,
        vec![
            &TokenType::Identifier,
            &TokenType::Question,
            &TokenType::Dot,
            &TokenType::Identifier,
            &TokenType::Eof,
        ]
    );

    let scanner = Scanner::new("x?.y");
    let x: Vec<Token> = collect_tokens(scanner);
    let types: Vec<&TokenType> = x.iter().map(|t| &t.token_type).collect();
    assert_eq!(
        types,
        vec![
            &TokenType::Identifier,
            &TokenType::QuestionDot,
            &TokenType::Identifier,
            &TokenType::Eof,
        ]
    );
}

#[test]
fn scans_number_literals() {
    let cases = [
        "0xff",
        "0XFF",
        "0xAbCdEf",
        "0b1010",
        "0B11110000",
        "0o755",
        "0O77",
        "1_000_000",
        "0xFF_FF",
        "0b1111_0000",
        "0o7_5_5",
        "1_234.567_89",
        "1.5e3",
        "1e3",
        "2E+2",
        "1.5e-3",
        "1_0e1_0",
        "0xE",
        "0xE1",
    ];
    for source in cases {
        assert_first_token(source, TokenType::Number, source);
    }
}

#[test]
fn rejects_malformed_input() {
    let invalid_number = CompilationErrorKind::InvalidNumberLiteral;
    let cases = [
        ("1e", 0, invalid_number, "Missing digits in number exponent"),
        (
            "1e+",
            0,
            invalid_number,
            "Missing digits in number exponent",
        ),
        (
            "1e_5",
            0,
            invalid_number,
            "Missing digits in number exponent",
        ),
        (
            "0b123",
            0,
            invalid_number,
            "Invalid digit in binary literal",
        ),
        (
            "0b2",
            0,
            invalid_number,
            "Invalid digit in binary literal (only 0 and 1 allowed)",
        ),
        ("0o89", 0, invalid_number, "Invalid digit in octal literal"),
        ("0x", 0, invalid_number, "requires at least one digit"),
        ("0b", 0, invalid_number, "requires at least one digit"),
        ("123_", 0, invalid_number, "underscore"),
        (
            "val s = # {1}",
            3,
            CompilationErrorKind::UnexpectedCharacter,
            "Unexpected character",
        ),
    ];
    for (source, index, kind, message) in cases {
        let tokens = collect_tokens(Scanner::new(source));
        let error = &tokens[index];
        assert_eq!(
            error.token_type,
            TokenType::Error(kind),
            "source {source:?}"
        );
        assert!(
            error.token.contains(message),
            "source {source:?}: {:?} does not contain {message:?}",
            error.token
        );
    }
}

#[test]
fn can_scan_hash_left_brace() {
    let script = "#{1}";

    let scanner = Scanner::new(script);
    let x: Vec<Token> = collect_tokens(scanner);

    assert_eq!(x.len(), 4);
    assert_eq!(x[0].token_type, TokenType::HashLeftBrace);
    assert_eq!(x[0].token, "#{");
    assert_eq!(x[1].token_type, TokenType::Number);
    assert_eq!(x[1].column, 3);
    assert_eq!(x[2].token_type, TokenType::RightBrace);
    assert_eq!(x[3].token_type, TokenType::Eof);
}

#[test]
fn can_scan_comment_immediately_after_slashes() {
    let scanner = Scanner::new("1//note");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "1");
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn can_scan_comment_with_space_after_slashes() {
    let scanner = Scanner::new("1 // note");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "1");
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn bare_comment_at_end_of_line_does_not_swallow_next_line() {
    let scanner = Scanner::new("1 //\n2");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "1");
    assert_eq!(tokens[1].token_type, TokenType::NewLine);
    assert_eq!(tokens[2].token_type, TokenType::Number);
    assert_eq!(tokens[2].token, "2");
    assert_eq!(tokens[3].token_type, TokenType::Eof);
}

#[test]
fn can_scan_number_then_comment_for_double_slash() {
    let scanner = Scanner::new("7 // 2");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "7");
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn trivia_records_comments_and_blank_lines() {
    let source = "// leading\nvar a = 1 // trailing\n\n    // indented\nvar b = 2\n   \nvar c = 3";
    let mut scanner = Scanner::new(source);
    scan_to_eof(&mut scanner);
    let trivia = scanner.trivia();

    assert_eq!(trivia.comments.len(), 3);

    assert_eq!(trivia.comments[0].line, 1);
    assert_eq!(trivia.comments[0].column, 1);
    assert_eq!(trivia.comments[0].text, "// leading");
    assert_eq!(trivia.comments[0].kind, CommentKind::OwnLine);

    assert_eq!(trivia.comments[1].line, 2);
    assert_eq!(trivia.comments[1].column, 11);
    assert_eq!(trivia.comments[1].text, "// trailing");
    assert_eq!(trivia.comments[1].kind, CommentKind::Trailing);

    assert_eq!(trivia.comments[2].line, 4);
    assert_eq!(trivia.comments[2].column, 5);
    assert_eq!(trivia.comments[2].text, "// indented");
    assert_eq!(trivia.comments[2].kind, CommentKind::OwnLine);

    assert_eq!(trivia.blank_lines, vec![3, 6]);
}

#[test]
fn trivia_comment_text_excludes_crlf_carriage_return() {
    let source = "// hi\r\nvar a = 1";
    let mut scanner = Scanner::new(source);
    scan_to_eof(&mut scanner);
    let trivia = scanner.trivia();

    assert_eq!(trivia.comments.len(), 1);
    assert_eq!(trivia.comments[0].text, "// hi");
}

#[test]
fn trivia_ignores_double_slash_inside_string_literal() {
    let source = "var s = \"http://example.com\";";
    let mut scanner = Scanner::new(source);
    scan_to_eof(&mut scanner);

    assert!(scanner.trivia().comments.is_empty());
}

#[test]
fn trivia_does_not_count_a_comment_only_line_before_a_blank_line() {
    let source = "x\n// c\n\ny";
    let mut scanner = Scanner::new(source);
    scan_to_eof(&mut scanner);

    assert_eq!(scanner.trivia().blank_lines, vec![3]);
}

#[test]
fn trivia_ignores_blank_line_inside_multiline_string_literal() {
    let source = "val s = \"a\n\nb\"\nx";
    let mut scanner = Scanner::new(source);
    scan_to_eof(&mut scanner);

    assert!(scanner.trivia().blank_lines.is_empty());
}

#[test]
fn trivia_records_crlf_blank_line() {
    let source = "x\r\n\r\ny";
    let mut scanner = Scanner::new(source);
    scan_to_eof(&mut scanner);

    assert_eq!(scanner.trivia().blank_lines, vec![2]);
}

#[test]
fn can_scan_all_keywords() {
    for (keyword, _) in KEYWORDS {
        let tokens = collect_tokens(Scanner::new(keyword));

        assert_eq!(tokens.len(), 2, "{keyword} should scan as one token");
        assert_eq!(tokens[0].token, *keyword, "{keyword} lexeme");
        assert_ne!(
            tokens[0].token_type,
            TokenType::Identifier,
            "{keyword} should scan as a keyword"
        );
        assert!(
            format!("{:?}", tokens[0].token_type).eq_ignore_ascii_case(keyword),
            "{keyword} scanned as {:?}",
            tokens[0].token_type
        );
    }
}

#[test]
fn identifiers_with_keyword_prefixes_scan_as_identifiers() {
    for src in [
        "fname", "iffy", "valid", "variable", "format", "returned", "nilly", "implicit",
    ] {
        let scanner = Scanner::new(src);
        let tokens = collect_tokens(scanner);

        assert_eq!(
            tokens[0].token_type,
            TokenType::Identifier,
            "{src} should scan as Identifier"
        );
        assert_eq!(tokens[0].token, src);
    }
}

#[test]
fn and_or_super_this_scan_as_identifiers() {
    for src in ["and", "or", "super", "this"] {
        let scanner = Scanner::new(src);
        let tokens = collect_tokens(scanner);

        assert_eq!(
            tokens[0].token_type,
            TokenType::Identifier,
            "{src} should scan as Identifier"
        );
        assert_eq!(tokens[0].token, src);
    }
}

#[test]
fn can_scan_offsets_with_multiple_spaces_and_operators() {
    let scanner = Scanner::new("a  == b");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token, "a");
    assert_eq!(tokens[0].offset, 0);
    assert_eq!(tokens[1].token_type, TokenType::EqualEqual);
    assert_eq!(tokens[1].offset, 3);
    assert_eq!(tokens[2].token, "b");
    assert_eq!(tokens[2].offset, 6);
}

#[test]
fn columns_and_offsets_are_correct_for_non_ascii_input() {
    let scanner = Scanner::new("val s = \"üü\" )");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[4].token_type, TokenType::RightParen);
    assert_eq!(tokens[4].column, 14);
    assert_eq!(tokens[4].offset, 13);
}

#[test]
fn multiline_string_resets_column_for_next_token() {
    let scanner = Scanner::new("val a = \"line1\nline2\" x");
    let tokens = collect_tokens(scanner);

    let x_token = &tokens[4];
    assert_eq!(x_token.token, "x");
    assert_eq!(x_token.line, 2);
    assert_eq!(x_token.column, 8);
    assert_eq!(x_token.offset, 22);
}

#[test]
fn multiline_string_reports_its_own_start_line_and_column() {
    let scanner = Scanner::new("val a = \"line1\nline2\"");
    let tokens = collect_tokens(scanner);

    let string_token = &tokens[3];
    assert_eq!(string_token.token_type, TokenType::String);
    assert_eq!(string_token.line, 1);
    assert_eq!(string_token.column, 9);
}

#[test]
fn unterminated_multiline_string_reports_start_line_and_column() {
    let scanner = Scanner::new("val a = \"line1\nline2");
    let tokens = collect_tokens(scanner);

    let error_token = &tokens[3];
    assert_eq!(
        error_token.token_type,
        TokenType::Error(CompilationErrorKind::UnterminatedString)
    );
    assert_eq!(error_token.line, 1);
    assert_eq!(error_token.column, 9);
}

#[test]
fn valid_escapes_scan_to_their_characters() {
    let cases = [
        ("\"a\\nb\"", "a\nb"),
        ("\"a\\tb\"", "a\tb"),
        ("\"a\\rb\"", "a\rb"),
        ("\"a\\\\b\"", "a\\b"),
        ("\"a\\\"b\"", "a\"b"),
        ("\"cost: \\$5\"", "cost: $5"),
        ("\"\\u{1F600}\"", "\u{1F600}"),
        ("\"\\u{41}\"", "A"),
        ("\"\\u{00004A}\"", "J"),
        ("\"\\${x}\"", "${x}"),
    ];
    for (source, lexeme) in cases {
        assert_first_token(source, TokenType::String, lexeme);
    }
}

#[test]
fn escaped_backslash_before_placeholder() {
    let source = "\"\\\\${x}\"";
    let scanner = Scanner::new(source);
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::StringStart);
    assert_eq!(tokens[0].token, "\\");

    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].token, "x");

    assert_eq!(tokens[2].token_type, TokenType::StringEnd);
    assert_eq!(tokens[2].token, "");
}

#[test]
fn invalid_escape_recovery() {
    let scanner = Scanner::new("\"a\\qb\" x");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidEscapeSequence)
    );
    assert_eq!(tokens[0].token, "Invalid escape sequence");
    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[0].column, 3);
    assert_eq!(tokens[0].offset, 2);

    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].token, "x");
    assert_eq!(tokens[1].column, 8);
    assert_eq!(tokens[1].offset, 7);
}

#[test]
fn invalid_escape_non_ascii_character() {
    let scanner = Scanner::new("\"a\\\u{1234}b\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidEscapeSequence)
    );
    assert_eq!(tokens[0].token, "Invalid escape sequence");
    assert_eq!(tokens[0].column, 3);
    assert_eq!(tokens[0].offset, 2);
}

#[test]
fn invalid_escape_empty_unicode_braces() {
    let scanner = Scanner::new("\"\\u{}\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidEscapeSequence)
    );
    assert_eq!(tokens[0].token, "Invalid escape sequence");
    assert_eq!(tokens[0].column, 2);
    assert_eq!(tokens[0].offset, 1);
}

#[test]
fn invalid_escape_surrogate_codepoint() {
    let scanner = Scanner::new("\"\\u{D800}\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidEscapeSequence)
    );
    assert_eq!(tokens[0].token, "Invalid escape sequence");
    assert_eq!(tokens[0].column, 2);
    assert_eq!(tokens[0].offset, 1);
}

#[test]
fn invalid_escape_codepoint_too_large() {
    let scanner = Scanner::new("\"\\u{110000}\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidEscapeSequence)
    );
    assert_eq!(tokens[0].token, "Invalid escape sequence");
    assert_eq!(tokens[0].column, 2);
    assert_eq!(tokens[0].offset, 1);
}

#[test]
fn invalid_escape_too_many_hex_digits() {
    let scanner = Scanner::new("\"\\u{0000041}\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidEscapeSequence)
    );
    assert_eq!(tokens[0].token, "Invalid escape sequence");
    assert_eq!(tokens[0].column, 2);
    assert_eq!(tokens[0].offset, 1);
}

#[test]
fn invalid_escape_missing_closing_brace() {
    let scanner = Scanner::new("\"\\u{41\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidEscapeSequence)
    );
    assert_eq!(tokens[0].token, "Invalid escape sequence");
    assert_eq!(tokens[0].column, 2);
    assert_eq!(tokens[0].offset, 1);
}

#[test]
fn position_after_escape() {
    let scanner = Scanner::new("\"a\\nb\" y");
    let tokens = collect_tokens(scanner);

    let y_token = &tokens[1];
    assert_eq!(y_token.token_type, TokenType::Identifier);
    assert_eq!(y_token.token, "y");
    assert_eq!(y_token.line, 1);
    assert_eq!(y_token.column, 8);
    assert_eq!(y_token.offset, 7);
}

#[test]
fn multiline_string_with_escape() {
    let scanner = Scanner::new("val a = \"line1\\t\nline2\" x");
    let tokens = collect_tokens(scanner);

    let string_token = &tokens[3];
    assert_eq!(string_token.token_type, TokenType::String);
    assert_eq!(string_token.token, "line1\t\nline2");

    let x_token = &tokens[4];
    assert_eq!(x_token.token, "x");
    assert_eq!(x_token.line, 2);
    assert_eq!(x_token.column, 8);
    assert_eq!(x_token.offset, 24);
}

#[test]
fn invalid_escape_on_second_line() {
    let scanner = Scanner::new("\"line1\n \\q\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidEscapeSequence)
    );
    assert_eq!(tokens[0].token, "Invalid escape sequence");
    assert_eq!(tokens[0].line, 2);
    assert_eq!(tokens[0].column, 2);
}

#[test]
fn invalid_escape_reports_first_backslash() {
    let scanner = Scanner::new("\"\\q\\z\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidEscapeSequence)
    );
    assert_eq!(tokens[0].token, "Invalid escape sequence");
    assert_eq!(tokens[0].column, 2);
    assert_eq!(tokens[0].offset, 1);
}

#[test]
fn backslash_before_eof_is_unterminated_string() {
    let scanner = Scanner::new("\"abc\\");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::UnterminatedString)
    );
    assert_eq!(tokens[0].token, "Unterminated string");
    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[0].column, 1);
    assert_eq!(tokens[0].offset, 0);
}

#[test]
fn invalid_escape_before_raw_newline() {
    let scanner = Scanner::new("\"\\\n\" x");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidEscapeSequence)
    );
    assert_eq!(tokens[0].token, "Invalid escape sequence");
    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[0].column, 2);

    let x_token = &tokens[1];
    assert_eq!(x_token.token, "x");
    assert_eq!(x_token.line, 2);
}

#[test]
fn nested_string_inside_interpolation() {
    let scanner = Scanner::new("\"${\"a\"}\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::StringStart);
    assert_eq!(tokens[0].token, "");

    assert_eq!(tokens[1].token_type, TokenType::String);
    assert_eq!(tokens[1].token, "a");

    assert_eq!(tokens[2].token_type, TokenType::StringEnd);
    assert_eq!(tokens[2].token, "");

    assert_eq!(tokens[3].token_type, TokenType::Eof);
}

#[test]
fn interpolation_eof_in_nested_quote() {
    let scanner = Scanner::new("\"${\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::StringStart);
    assert_eq!(
        tokens[1].token_type,
        TokenType::Error(CompilationErrorKind::ExpectedToken)
    );
    assert_eq!(tokens[1].token, "Expect '}' after interpolated expression.");
    assert_eq!(tokens[1].line, 1);
    assert_eq!(tokens[1].column, 2);
    assert_eq!(tokens[1].offset, 1);

    assert_eq!(tokens[2].token_type, TokenType::Eof);
}

#[test]
fn unterminated_string_after_closed_interpolation() {
    let scanner = Scanner::new("\"${a}");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::StringStart);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);

    assert_eq!(
        tokens[2].token_type,
        TokenType::Error(CompilationErrorKind::UnterminatedString)
    );
    assert_eq!(tokens[2].token, "Unterminated string");
    assert_eq!(tokens[2].line, 1);
    assert_eq!(tokens[2].column, 1);
    assert_eq!(tokens[2].offset, 0);
}

#[test]
fn interpolation_brace_expression() {
    let scanner = Scanner::new("\"${ {} }\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::StringStart);
    assert_eq!(tokens[1].token_type, TokenType::LeftBrace);
    assert_eq!(tokens[2].token_type, TokenType::RightBrace);
    assert_eq!(tokens[3].token_type, TokenType::StringEnd);
    assert_eq!(tokens[4].token_type, TokenType::Eof);
}

#[test]
fn interpolation_line_comment_hides_closing_brace() {
    let scanner = Scanner::new("\"${a // c }\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::StringStart);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);

    assert_eq!(
        tokens[2].token_type,
        TokenType::Error(CompilationErrorKind::ExpectedToken)
    );
    assert_eq!(tokens[2].token, "Expect '}' after interpolated expression.");
    assert_eq!(tokens[2].column, 2);

    assert_eq!(tokens[3].token_type, TokenType::Eof);
}

/// Pulls every `\b(word|word|...)\b` keyword-alternation out of the grammar's raw text.
fn keywords_in_grammar(grammar: &str) -> BTreeSet<String> {
    let mut keywords = BTreeSet::new();
    let mut rest = grammar;
    while let Some(start) = rest.find(r"\\b(") {
        let after = &rest[start + r"\\b(".len()..];
        let Some(close) = after.find(')') else {
            break;
        };
        let words = &after[..close];
        let is_keyword_list = !words.is_empty()
            && words.chars().all(|c| c.is_ascii_lowercase() || c == '|')
            && after[close..].starts_with(r")\\b");
        if is_keyword_list {
            keywords.extend(words.split('|').map(str::to_string));
        }
        rest = &after[close + 1..];
    }
    keywords
}

#[test]
fn grammar_keywords_match_scanner() {
    let grammar_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("editors/vscode/syntaxes/neon.tmLanguage.json");
    let grammar = std::fs::read_to_string(&grammar_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", grammar_path.display()));

    let grammar_keywords = keywords_in_grammar(&grammar);
    assert!(
        !grammar_keywords.is_empty(),
        "found no \\b(...)\\b keyword lists in {}",
        grammar_path.display()
    );

    let scanner_keywords: BTreeSet<String> = KEYWORDS
        .iter()
        .map(|(keyword, _)| keyword.to_string())
        .collect();

    let missing: Vec<&String> = scanner_keywords.difference(&grammar_keywords).collect();
    let extra: Vec<&String> = grammar_keywords.difference(&scanner_keywords).collect();
    assert!(
        missing.is_empty() && extra.is_empty(),
        "neon.tmLanguage.json keywords are out of sync with scanner::KEYWORDS - \
         missing from grammar: {missing:?}, extra in grammar: {extra:?}"
    );
}
