//! Semantic token generation for Neon language
//!
//! This module provides functionality to convert Neon source code into LSP semantic tokens
//! for syntax highlighting. It scans the source using Neon's existing Scanner and maps
//! TokenType to LSP semantic token types with delta encoding.

use tower_lsp::lsp_types::{SemanticToken, SemanticTokenType, SemanticTokensLegend};
use crate::compiler::{Scanner, Token};
use crate::compiler::token::TokenType;

/// Maps Neon's TokenType to LSP semantic token type index
///
/// Returns None for token types that should not be highlighted (e.g., whitespace, EOF, errors)
pub fn token_type_to_semantic_index(token_type: &TokenType) -> Option<u32> {
    match token_type {
        // Keywords
        TokenType::And
        | TokenType::Class
        | TokenType::Else
        | TokenType::False
        | TokenType::For
        | TokenType::Fn
        | TokenType::If
        | TokenType::Nil
        | TokenType::Or
        | TokenType::Print
        | TokenType::Return
        | TokenType::Struct
        | TokenType::Super
        | TokenType::This
        | TokenType::True
        | TokenType::Val
        | TokenType::Var
        | TokenType::While => Some(0), // KEYWORD

        // Operators
        TokenType::Plus
        | TokenType::Minus
        | TokenType::Star
        | TokenType::Slash
        | TokenType::Percent
        | TokenType::Bang
        | TokenType::BangEqual
        | TokenType::Equal
        | TokenType::EqualEqual
        | TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::Less
        | TokenType::LessEqual => Some(1), // OPERATOR

        // Literals
        TokenType::String | TokenType::InterpolatedString => Some(2), // STRING
        TokenType::Number => Some(3),                                  // NUMBER

        // Identifiers (default to variable, context-aware highlighting can refine this)
        TokenType::Identifier => Some(4), // VARIABLE

        // Delimiters and other tokens - no semantic highlighting
        TokenType::LeftParen
        | TokenType::RightParen
        | TokenType::LeftBrace
        | TokenType::RightBrace
        | TokenType::Comma
        | TokenType::Dot
        | TokenType::Semicolon
        | TokenType::NewLine
        | TokenType::Error
        | TokenType::Eof => None,
    }
}

/// Creates the semantic tokens legend for the LSP server
///
/// This defines the token types and modifiers that will be used
pub fn create_legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: vec![
            SemanticTokenType::KEYWORD,
            SemanticTokenType::OPERATOR,
            SemanticTokenType::STRING,
            SemanticTokenType::NUMBER,
            SemanticTokenType::VARIABLE,
        ],
        token_modifiers: vec![],
    }
}

/// Generates LSP semantic tokens from Neon source code
///
/// Tokens are delta-encoded as per LSP specification:
/// - First token: absolute position (line, char)
/// - Subsequent tokens: relative to previous token
///
/// # Arguments
/// * `source` - The Neon source code to tokenize
///
/// # Returns
/// A vector of SemanticToken in LSP delta-encoded format
pub fn generate_semantic_tokens(source: &str) -> Vec<SemanticToken> {
    let mut scanner = Scanner::new(source);
    let mut semantic_tokens = Vec::new();

    // Track previous token position for delta encoding
    let mut prev_line = 0u32;
    let mut prev_start = 0u32;

    loop {
        let token = scanner.scan_token();

        // Stop at EOF
        if token.token_type == TokenType::Eof {
            break;
        }

        // Skip tokens that don't need semantic highlighting
        let token_type_index = match token_type_to_semantic_index(&token.token_type) {
            Some(idx) => idx,
            None => continue,
        };

        // Convert to LSP semantic token with delta encoding
        let semantic_token = encode_semantic_token(
            &token,
            token_type_index,
            prev_line,
            prev_start,
        );

        // Update previous position for next delta
        prev_line = token.line.saturating_sub(1); // LSP uses 0-based line numbers
        prev_start = token.column.saturating_sub(1); // LSP uses 0-based character positions

        semantic_tokens.push(semantic_token);
    }

    semantic_tokens
}

/// Encodes a token into LSP delta-encoded format
///
/// # Arguments
/// * `token` - The Neon token to encode
/// * `token_type` - The semantic token type index
/// * `prev_line` - Previous token's line number (0-based)
/// * `prev_start` - Previous token's start character (0-based)
///
/// # Returns
/// A SemanticToken with delta-encoded position
fn encode_semantic_token(
    token: &Token,
    token_type: u32,
    prev_line: u32,
    prev_start: u32,
) -> SemanticToken {
    // Neon uses 1-based line/column, LSP uses 0-based
    let line = token.line.saturating_sub(1);
    let start = token.column.saturating_sub(1);
    let length = token.token.len() as u32;

    // Calculate deltas
    let delta_line = line.saturating_sub(prev_line);
    let delta_start = if delta_line == 0 {
        // Same line: delta from previous token
        start.saturating_sub(prev_start)
    } else {
        // New line: absolute position on new line
        start
    };

    SemanticToken {
        delta_line,
        delta_start,
        length,
        token_type,
        token_modifiers_bitset: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_type_mapping_completeness() {
        // Verify all TokenType variants are handled (no panics)
        let all_token_types = vec![
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Comma,
            TokenType::Dot,
            TokenType::Minus,
            TokenType::Plus,
            TokenType::Percent,
            TokenType::Semicolon,
            TokenType::NewLine,
            TokenType::Slash,
            TokenType::Star,
            TokenType::Bang,
            TokenType::BangEqual,
            TokenType::Equal,
            TokenType::EqualEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Identifier,
            TokenType::String,
            TokenType::InterpolatedString,
            TokenType::Number,
            TokenType::And,
            TokenType::Class,
            TokenType::Else,
            TokenType::False,
            TokenType::For,
            TokenType::Fn,
            TokenType::If,
            TokenType::Nil,
            TokenType::Or,
            TokenType::Print,
            TokenType::Return,
            TokenType::Struct,
            TokenType::Super,
            TokenType::This,
            TokenType::True,
            TokenType::Val,
            TokenType::Var,
            TokenType::While,
            TokenType::Error,
            TokenType::Eof,
        ];

        for token_type in all_token_types {
            // Should not panic
            let _ = token_type_to_semantic_index(&token_type);
        }
    }

    #[test]
    fn test_keywords_mapped_correctly() {
        assert_eq!(token_type_to_semantic_index(&TokenType::Var), Some(0));
        assert_eq!(token_type_to_semantic_index(&TokenType::Val), Some(0));
        assert_eq!(token_type_to_semantic_index(&TokenType::Fn), Some(0));
        assert_eq!(token_type_to_semantic_index(&TokenType::If), Some(0));
        assert_eq!(token_type_to_semantic_index(&TokenType::Else), Some(0));
        assert_eq!(token_type_to_semantic_index(&TokenType::While), Some(0));
        assert_eq!(token_type_to_semantic_index(&TokenType::For), Some(0));
        assert_eq!(token_type_to_semantic_index(&TokenType::Return), Some(0));
    }

    #[test]
    fn test_operators_mapped_correctly() {
        assert_eq!(token_type_to_semantic_index(&TokenType::Plus), Some(1));
        assert_eq!(token_type_to_semantic_index(&TokenType::Minus), Some(1));
        assert_eq!(token_type_to_semantic_index(&TokenType::Star), Some(1));
        assert_eq!(token_type_to_semantic_index(&TokenType::Slash), Some(1));
        assert_eq!(token_type_to_semantic_index(&TokenType::EqualEqual), Some(1));
        assert_eq!(token_type_to_semantic_index(&TokenType::BangEqual), Some(1));
    }

    #[test]
    fn test_literals_mapped_correctly() {
        assert_eq!(token_type_to_semantic_index(&TokenType::String), Some(2));
        assert_eq!(token_type_to_semantic_index(&TokenType::InterpolatedString), Some(2));
        assert_eq!(token_type_to_semantic_index(&TokenType::Number), Some(3));
    }

    #[test]
    fn test_delimiters_not_highlighted() {
        assert_eq!(token_type_to_semantic_index(&TokenType::LeftParen), None);
        assert_eq!(token_type_to_semantic_index(&TokenType::RightParen), None);
        assert_eq!(token_type_to_semantic_index(&TokenType::Semicolon), None);
        assert_eq!(token_type_to_semantic_index(&TokenType::Eof), None);
    }

    #[test]
    fn test_simple_script_tokenization() {
        let source = "var x = 42";
        let tokens = generate_semantic_tokens(source);

        // Should produce: var (keyword), x (variable), = (operator), 42 (number)
        // But operators like = are highlighted, check token count
        assert_eq!(tokens.len(), 4);

        // First token: "var" at line 0, col 0
        assert_eq!(tokens[0].delta_line, 0);
        assert_eq!(tokens[0].delta_start, 0);
        assert_eq!(tokens[0].length, 3);
        assert_eq!(tokens[0].token_type, 0); // KEYWORD

        // Second token: "x" - delta encoding
        assert_eq!(tokens[1].delta_line, 0); // Same line
        assert_eq!(tokens[1].length, 1);
        assert_eq!(tokens[1].token_type, 4); // VARIABLE
    }

    #[test]
    fn test_multiline_tokenization() {
        let source = "var x = 1\nvar y = 2";
        let tokens = generate_semantic_tokens(source);

        // Should have tokens from both lines
        assert!(tokens.len() >= 6); // At least var, x, 1, var, y, 2

        // Find first token on second line (should have delta_line > 0)
        let second_line_token = tokens.iter().find(|t| t.delta_line > 0);
        assert!(second_line_token.is_some());
    }

    #[test]
    fn test_string_tokenization() {
        let source = r#"var s = "hello""#;
        let tokens = generate_semantic_tokens(source);

        // Should have: var, s, =, "hello"
        assert_eq!(tokens.len(), 4);

        // String token should be type 2 (STRING)
        let string_token = &tokens[3];
        assert_eq!(string_token.token_type, 2);
        assert_eq!(string_token.length, 7); // "hello" including quotes
    }

    #[test]
    fn test_interpolated_string_tokenization() {
        let source = r#"var s = "hello ${name}""#;
        let tokens = generate_semantic_tokens(source);

        // Should have: var, s, =, interpolated string
        assert_eq!(tokens.len(), 4);

        // Interpolated string token should be type 2 (STRING)
        let string_token = &tokens[3];
        assert_eq!(string_token.token_type, 2);
    }

    #[test]
    fn test_delta_encoding_same_line() {
        let source = "var x = 1";
        let tokens = generate_semantic_tokens(source);

        // All tokens on same line should have delta_line = 0
        for token in &tokens {
            assert_eq!(token.delta_line, 0);
        }

        // Deltas should be cumulative on same line
        assert_eq!(tokens[0].delta_start, 0); // "var" at column 0
        assert!(tokens[1].delta_start > 0); // "x" after "var "
    }

    #[test]
    fn test_delta_encoding_different_lines() {
        let source = "var x\nvar y";
        let tokens = generate_semantic_tokens(source);

        // First line tokens
        assert_eq!(tokens[0].delta_line, 0);
        assert_eq!(tokens[1].delta_line, 0);

        // Second line tokens - should have delta_line = 1
        assert_eq!(tokens[2].delta_line, 1);
        assert_eq!(tokens[2].delta_start, 0); // Reset to start of new line
    }

    #[test]
    fn test_legend_creation() {
        let legend = create_legend();

        // Should have 5 token types
        assert_eq!(legend.token_types.len(), 5);
        assert_eq!(legend.token_types[0], SemanticTokenType::KEYWORD);
        assert_eq!(legend.token_types[1], SemanticTokenType::OPERATOR);
        assert_eq!(legend.token_types[2], SemanticTokenType::STRING);
        assert_eq!(legend.token_types[3], SemanticTokenType::NUMBER);
        assert_eq!(legend.token_types[4], SemanticTokenType::VARIABLE);

        // No modifiers for now
        assert_eq!(legend.token_modifiers.len(), 0);
    }
}
