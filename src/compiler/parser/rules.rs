use crate::compiler::token::TokenType;
use crate::compiler::Parser;
use lazy_static::lazy_static;
use std::intrinsics::transmute;

#[derive(Debug, Clone, Copy)]
pub(super) enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    #[inline(always)]
    pub fn from_u8(value: u8) -> Precedence {
        unsafe { transmute(value) }
    }
}

type ParserOp = fn(&mut Parser);

#[derive(Debug, Clone)]
pub(super) struct ParseRule {
    pub(super) prefix: Option<ParserOp>,
    pub(super) infix: Option<ParserOp>,
    pub(super) precedence: Precedence,
}

impl ParseRule {
    fn new(prefix: Option<ParserOp>, infix: Option<ParserOp>, precedence: Precedence) -> ParseRule {
        ParseRule {
            prefix,
            infix,
            precedence,
        }
    }
}

#[rustfmt::skip]
lazy_static! {
    pub(super) static ref PARSE_RULES: Vec<(TokenType, ParseRule)> = vec![
        (TokenType::LeftParen, ParseRule::new(Some(Parser::grouping), None, Precedence::None)),
        (TokenType::RightParen, ParseRule::new(None, None, Precedence::None)),
        (TokenType::LeftBrace, ParseRule::new(None, None, Precedence::None)),
        (TokenType::RightBrace, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Comma, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Dot, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Minus, ParseRule::new(Some(Parser::unary), Some(Parser::binary), Precedence::Term)),
        (TokenType::Plus, ParseRule::new(None, Some(Parser::binary), Precedence::Term)),
        (TokenType::Semicolon, ParseRule::new(None, None, Precedence::None)),
        (TokenType::NewLine, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Slash, ParseRule::new(None, Some(Parser::binary), Precedence::Factor)),
        (TokenType::Star, ParseRule::new(None, Some(Parser::binary), Precedence::Factor)),
        (TokenType::Bang, ParseRule::new(Some(Parser::unary), None, Precedence::None)),
        (TokenType::BangEqual, ParseRule::new(None, Some(Parser::binary), Precedence::Equality)),
        (TokenType::Equal, ParseRule::new(None, None, Precedence::None)),
        (TokenType::EqualEqual, ParseRule::new(None, Some(Parser::binary), Precedence::Equality)),
        (TokenType::Greater, ParseRule::new(None, Some(Parser::binary), Precedence::Comparison)),
        (TokenType::GreaterEqual, ParseRule::new(None, Some(Parser::binary), Precedence::Comparison)),
        (TokenType::Less, ParseRule::new(None, Some(Parser::binary), Precedence::Comparison)),
        (TokenType::LessEqual, ParseRule::new(None, Some(Parser::binary), Precedence::Comparison)),
        (TokenType::Identifier, ParseRule::new(None, None, Precedence::None)),
        (TokenType::String, ParseRule::new(Some(Parser::string), None, Precedence::None)),
        (TokenType::InterpolatedString, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Number, ParseRule::new(Some(Parser::number), None, Precedence::None)),
        (TokenType::And, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Class, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Else, ParseRule::new(None, None, Precedence::None)),
        (TokenType::False, ParseRule::new(Some(Parser::literal), None, Precedence::None)),
        (TokenType::For, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Fn, ParseRule::new(None, None, Precedence::None)),
        (TokenType::If, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Nil, ParseRule::new(Some(Parser::literal), None, Precedence::None)),
        (TokenType::Or, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Print, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Return, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Super, ParseRule::new(None, None, Precedence::None)),
        (TokenType::This, ParseRule::new(None, None, Precedence::None)),
        (TokenType::True, ParseRule::new(Some(Parser::literal), None, Precedence::None)),
        (TokenType::Val, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Var, ParseRule::new(None, None, Precedence::None)),
        (TokenType::While, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Error, ParseRule::new(None, None, Precedence::None)),
        (TokenType::Eof, ParseRule::new(None, None, Precedence::None)),
    ];
}
