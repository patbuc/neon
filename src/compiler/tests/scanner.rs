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
    assert_eq!(x[3].token_type, TokenType::InterpolatedString);
    assert_eq!(x[4].token_type, TokenType::Semicolon);
    assert_eq!(x[5].token_type, TokenType::Eof);
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

    assert_eq!(tokens[0].token_type, TokenType::Error);
    assert!(tokens[0].token.contains("Invalid digit in binary literal"));
}

#[test]
fn rejects_invalid_octal_digit() {
    let scanner = Scanner::new("0o89");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::Error);
    assert!(tokens[0].token.contains("Invalid digit in octal literal"));
}

#[test]
fn rejects_empty_hex_literal() {
    let scanner = Scanner::new("0x");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::Error);
    assert!(tokens[0].token.contains("requires at least one digit"));
}

#[test]
fn rejects_empty_binary_literal() {
    let scanner = Scanner::new("0b");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::Error);
    assert!(tokens[0].token.contains("requires at least one digit"));
}

#[test]
fn rejects_trailing_underscore_in_decimal() {
    let scanner = Scanner::new("123_");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens[0].token_type, TokenType::Error);
    assert!(tokens[0].token.contains("underscore"));
}

#[test]
fn can_scan_impl_keyword() {
    let scanner = Scanner::new("impl");
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Impl);
    assert_eq!(tokens[0].token, "impl");
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn can_scan_impl_block_structure() {
    let script = "impl Point { }";

    let scanner = Scanner::new(script);
    let tokens = collect_tokens(scanner);

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].token_type, TokenType::Impl);
    assert_eq!(tokens[0].token, "impl");
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].token, "Point");
    assert_eq!(tokens[2].token_type, TokenType::LeftBrace);
    assert_eq!(tokens[3].token_type, TokenType::RightBrace);
    assert_eq!(tokens[4].token_type, TokenType::Eof);
}
