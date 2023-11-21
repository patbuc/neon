use crate::compiler::Token;

#[derive(Debug, PartialEq)]
pub(in crate::compiler) enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    InterpolatedString,
    Number,

    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

impl Token {
    pub(in crate::compiler) const INVALID: Token = Token {
        token_type: TokenType::Eof,
        start: 0,
        length: 0,
        line: 0,
    };

    pub(in crate::compiler) fn new(
        token_type: TokenType,
        start: usize,
        length: usize,
        line: u32,
    ) -> Token {
        Token {
            token_type,
            start,
            length,
            line,
        }
    }
}
