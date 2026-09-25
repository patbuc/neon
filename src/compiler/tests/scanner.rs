use crate::common::errors::CompilationErrorKind;
use crate::compiler::token::TokenType;
use crate::compiler::Scanner;
use crate::compiler::Token;

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
fn can_scan_plusplus_operator() {
    let script = "x++";

    let scanner = Scanner::new(script);
    let x: Vec<Token> = collect_tokens(scanner);

    assert_eq!(x.len(), 3);
    assert_eq!(x[0].token_type, TokenType::Identifier);
    assert_eq!(x[0].token, "x");
    assert_eq!(x[1].token_type, TokenType::PlusPlus);
    assert_eq!(x[1].token, "++");
    assert_eq!(x[2].token_type, TokenType::Eof);
}

#[test]
fn can_scan_minusminus_operator() {
    let script = "x--";

    let scanner = Scanner::new(script);
    let x: Vec<Token> = collect_tokens(scanner);

    assert_eq!(x.len(), 3);
    assert_eq!(x[0].token_type, TokenType::Identifier);
    assert_eq!(x[0].token, "x");
    assert_eq!(x[1].token_type, TokenType::MinusMinus);
    assert_eq!(x[1].token, "--");
    assert_eq!(x[2].token_type, TokenType::Eof);
}

#[test]
fn can_scan_hexadecimal_lowercase() {
    let scanner = Scanner::new("0xff");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "0xff");
}

#[test]
fn can_scan_hexadecimal_uppercase() {
    let scanner = Scanner::new("0XFF");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "0XFF");
}

#[test]
fn can_scan_hexadecimal_mixed_case() {
    let scanner = Scanner::new("0xAbCdEf");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "0xAbCdEf");
}

#[test]
fn can_scan_binary_literal() {
    let scanner = Scanner::new("0b1010");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "0b1010");
}

#[test]
fn can_scan_binary_uppercase() {
    let scanner = Scanner::new("0B11110000");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "0B11110000");
}

#[test]
fn can_scan_octal_literal() {
    let scanner = Scanner::new("0o755");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "0o755");
}

#[test]
fn can_scan_octal_uppercase() {
    let scanner = Scanner::new("0O77");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "0O77");
}

#[test]
fn can_scan_decimal_with_underscores() {
    let scanner = Scanner::new("1_000_000");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "1_000_000");
}

#[test]
fn can_scan_hex_with_underscores() {
    let scanner = Scanner::new("0xFF_FF");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "0xFF_FF");
}

#[test]
fn can_scan_binary_with_underscores() {
    let scanner = Scanner::new("0b1111_0000");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "0b1111_0000");
}

#[test]
fn can_scan_octal_with_underscores() {
    let scanner = Scanner::new("0o7_5_5");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "0o7_5_5");
}

#[test]
fn can_scan_float_with_underscores() {
    let scanner = Scanner::new("1_234.567_89");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].token, "1_234.567_89");
}

#[test]
fn rejects_invalid_binary_digit() {
    let scanner = Scanner::new("0b123");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidNumberLiteral)
    );
    assert!(tokens[0].token.contains("Invalid digit in binary literal"));
}

#[test]
fn rejects_invalid_leading_binary_digit() {
    let scanner = Scanner::new("0b2");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidNumberLiteral)
    );
    assert!(tokens[0]
        .token
        .contains("Invalid digit in binary literal (only 0 and 1 allowed)"));
}

#[test]
fn rejects_invalid_octal_digit() {
    let scanner = Scanner::new("0o89");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidNumberLiteral)
    );
    assert!(tokens[0].token.contains("Invalid digit in octal literal"));
}

#[test]
fn rejects_empty_hex_literal() {
    let scanner = Scanner::new("0x");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidNumberLiteral)
    );
    assert!(tokens[0].token.contains("requires at least one digit"));
}

#[test]
fn rejects_empty_binary_literal() {
    let scanner = Scanner::new("0b");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidNumberLiteral)
    );
    assert!(tokens[0].token.contains("requires at least one digit"));
}

#[test]
fn rejects_trailing_underscore_in_decimal() {
    let scanner = Scanner::new("123_");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[0].token_type,
        TokenType::Error(CompilationErrorKind::InvalidNumberLiteral)
    );
    assert!(tokens[0].token.contains("underscore"));
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
fn rejects_bare_hash() {
    let scanner = Scanner::new("val s = # {1}");
    let tokens = collect_tokens(scanner);

    assert_eq!(
        tokens[3].token_type,
        TokenType::Error(CompilationErrorKind::UnexpectedCharacter)
    );
    assert!(tokens[3].token.contains("Unexpected character"));
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
fn can_scan_fn_keyword() {
    let scanner = Scanner::new("fn");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::Fn);
}

#[test]
fn can_scan_impl_keyword() {
    let scanner = Scanner::new("impl");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::Impl);
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
fn string_escape_newline() {
    let scanner = Scanner::new("\"a\\nb\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(tokens[0].token, "a\nb");
}

#[test]
fn string_escape_tab() {
    let scanner = Scanner::new("\"a\\tb\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(tokens[0].token, "a\tb");
}

#[test]
fn string_escape_carriage_return() {
    let scanner = Scanner::new("\"a\\rb\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(tokens[0].token, "a\rb");
}

#[test]
fn string_escape_backslash() {
    let scanner = Scanner::new("\"a\\\\b\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(tokens[0].token, "a\\b");
}

#[test]
fn string_escape_double_quote() {
    let scanner = Scanner::new("\"a\\\"b\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(tokens[0].token, "a\"b");
}

#[test]
fn string_escape_dollar() {
    let scanner = Scanner::new("\"cost: \\$5\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(tokens[0].token, "cost: $5");
}

#[test]
fn unicode_escape_emoji() {
    let scanner = Scanner::new("\"\\u{1F600}\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(tokens[0].token, "\u{1F600}");
}

#[test]
fn unicode_escape_short() {
    let scanner = Scanner::new("\"\\u{41}\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(tokens[0].token, "A");
}

#[test]
fn unicode_escape_six_hex_digits() {
    let scanner = Scanner::new("\"\\u{00004A}\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(tokens[0].token, "J");
}

#[test]
fn escaped_dollar_brace() {
    let scanner = Scanner::new("\"\\${x}\"");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(tokens[0].token, "${x}");
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
