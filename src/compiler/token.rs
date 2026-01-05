use crate::compiler::Token;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Default)]
pub(crate) enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    DotDot,
    DotDotEqual,
    Minus,
    MinusMinus,
    Plus,
    PlusPlus,
    Percent,
    Semicolon,
    Colon,
    Question,
    NewLine,
    Slash,
    SlashSlash,
    SlashEqual,
    Star,
    StarStar,
    StarEqual,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    AndAnd,
    OrOr,
    Ampersand,      // &
    Pipe,           // |
    Caret,          // ^
    Tilde,          // ~
    LessLess,       // <<
    GreaterGreater, // >>

    PlusEqual,
    MinusEqual,
    PercentEqual,

    Identifier,
    String,
    InterpolatedString,
    Number,

    And,
    Break,
    Continue,
    Else,
    False,
    For,
    Fn,
    If,
    Nil,
    Or,
    Return,
    Struct,
    Super,
    This,
    True,
    Val,
    Var,
    While,
    In,

    Error,

    #[default]
    Eof,
}

impl Token {
    pub(in crate::compiler) fn new(
        token_type: TokenType,
        token: String,
        line: u32,
        column: u32,
        offset: usize,
    ) -> Token {
        Token {
            token_type,
            token,
            line,
            column,
            offset,
        }
    }
}
