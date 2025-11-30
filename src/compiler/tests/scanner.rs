use crate::compiler::Token;
use crate::compiler::token::TokenType;
use crate::compiler::Scanner;

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
