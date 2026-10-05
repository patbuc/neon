use crate::common::errors::CompilationErrorKind;
use crate::compiler::Token;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Default)]
pub(crate) enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    HashLeftBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    QuestionDot,
    DotDot,
    DotDotEqual,
    Minus,
    MinusEqual,
    Arrow,
    Plus,
    PlusEqual,
    Percent,
    PercentEqual,
    Semicolon,
    Colon,
    Question,
    QuestionQuestion,
    NewLine,
    Slash,
    SlashEqual,
    Star,
    StarStar,
    StarEqual,
    StarStarEqual,

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

    Identifier,
    String,
    StringStart,
    StringMiddle,
    StringEnd,
    Number,

    Break,
    Continue,
    Else,
    Enum,
    False,
    For,
    Fn,
    If,
    Impl,
    Match,
    Nil,
    Return,
    Struct,
    True,
    Val,
    Var,
    While,
    In,

    Error(CompilationErrorKind),

    #[default]
    Eof,
}

impl Token {
    pub(in crate::compiler) fn new(
        token_type: TokenType,
        token: String,
        raw: String,
        line: u32,
        column: u32,
        offset: usize,
    ) -> Token {
        Token {
            token_type,
            token,
            raw,
            line,
            column,
            offset,
        }
    }
}
