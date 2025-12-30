// Diagnostic generation from compiler errors

use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};
use crate::common::errors::CompilationError;
use crate::compiler::Compiler;

/// Convert a Neon compilation error to an LSP diagnostic
///
/// Maps CompilationError with 1-indexed line/column to LSP Diagnostic with 0-indexed positions.
/// Attempts to calculate end position by finding token length in source, defaulting to single character.
pub fn compilation_error_to_diagnostic(error: &CompilationError, source: &str) -> Diagnostic {
    // LSP uses 0-indexed positions, Neon uses 1-indexed
    let line = error.location.line.saturating_sub(1);
    let column = error.location.column.saturating_sub(1);

    // Calculate end position by finding token length
    let end_column = calculate_end_column(source, error.location.offset, column);

    Diagnostic {
        range: Range {
            start: Position {
                line,
                character: column,
            },
            end: Position {
                line,
                character: end_column,
            },
        },
        severity: Some(DiagnosticSeverity::ERROR),
        code: None,
        code_description: None,
        source: Some("neon".to_string()),
        message: error.message.clone(),
        related_information: None,
        tags: None,
        data: None,
    }
}

/// Calculate the end column for a diagnostic by finding token length in source
///
/// Uses the offset to find the token in source and measure its length.
/// Falls back to column + 1 (single character) if offset is invalid or source is empty.
fn calculate_end_column(source: &str, offset: usize, start_column: u32) -> u32 {
    // If offset is beyond source length, default to single character
    if offset >= source.len() {
        return start_column + 1;
    }

    // Find the end of the current token
    // Tokens end at whitespace, punctuation, or end of string
    let chars: Vec<char> = source.chars().collect();
    if offset >= chars.len() {
        return start_column + 1;
    }

    let mut length = 0;
    for c in chars.iter().skip(offset) {
        if c.is_whitespace() || is_punctuation(*c) {
            break;
        }
        length += 1;
    }

    // If we found no token (e.g., offset points to whitespace), default to 1
    if length == 0 {
        length = 1;
    }

    start_column + length
}

/// Check if a character is punctuation/operator that terminates a token
fn is_punctuation(c: char) -> bool {
    matches!(c,
        '(' | ')' | '{' | '}' | '[' | ']' |
        ',' | ';' | ':' | '.' |
        '+' | '-' | '*' | '/' | '%' |
        '=' | '!' | '<' | '>' | '&' | '|'
    )
}

/// Generate LSP diagnostics from Neon source code
///
/// Compiles the source and converts any compilation errors to LSP diagnostics.
/// Returns an empty vector if there are no errors.
pub fn generate_diagnostics(source: &str) -> Vec<Diagnostic> {
    let mut compiler = Compiler::new();

    // Compile the source (errors will be captured internally)
    let _ = compiler.compile(source);

    // Extract structured errors and convert to diagnostics
    compiler
        .get_structured_errors()
        .iter()
        .map(|error| compilation_error_to_diagnostic(error, source))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::errors::{CompilationError, CompilationPhase, CompilationErrorKind};
    use crate::common::SourceLocation;

    #[test]
    fn test_compilation_error_to_diagnostic_converts_location() {
        let error = CompilationError::new(
            CompilationPhase::Parse,
            CompilationErrorKind::UnexpectedToken,
            "Unexpected token '}'",
            SourceLocation {
                offset: 10,
                line: 5,
                column: 12,
            },
        );

        let source = "fn test() { }";
        let diagnostic = compilation_error_to_diagnostic(&error, source);

        // Verify 1-indexed to 0-indexed conversion
        assert_eq!(diagnostic.range.start.line, 4); // line 5 -> 4
        assert_eq!(diagnostic.range.start.character, 11); // column 12 -> 11
        assert_eq!(diagnostic.severity, Some(DiagnosticSeverity::ERROR));
        assert_eq!(diagnostic.message, "Unexpected token '}'");
        assert_eq!(diagnostic.source.as_deref(), Some("neon"));
    }

    #[test]
    fn test_compilation_error_to_diagnostic_calculates_end_position() {
        let error = CompilationError::new(
            CompilationPhase::Semantic,
            CompilationErrorKind::UndefinedSymbol,
            "Undefined variable 'foo'",
            SourceLocation {
                offset: 0,
                line: 1,
                column: 1,
            },
        );

        let source = "foo + bar";
        let diagnostic = compilation_error_to_diagnostic(&error, source);

        // Should calculate token length ('foo' = 3 chars)
        assert_eq!(diagnostic.range.start.character, 0);
        assert_eq!(diagnostic.range.end.character, 3);
    }

    #[test]
    fn test_compilation_error_to_diagnostic_defaults_to_single_char() {
        let error = CompilationError::new(
            CompilationPhase::Parse,
            CompilationErrorKind::UnexpectedToken,
            "Unexpected token",
            SourceLocation {
                offset: 100, // Beyond source length
                line: 1,
                column: 1,
            },
        );

        let source = "x";
        let diagnostic = compilation_error_to_diagnostic(&error, source);

        // Should default to single character when offset is invalid
        assert_eq!(diagnostic.range.start.character, 0);
        assert_eq!(diagnostic.range.end.character, 1);
    }

    #[test]
    fn test_generate_diagnostics_with_parse_error() {
        let source = "var x = ";
        let diagnostics = generate_diagnostics(source);

        // Should produce at least one diagnostic
        assert!(!diagnostics.is_empty());
        assert_eq!(diagnostics[0].severity, Some(DiagnosticSeverity::ERROR));
        assert_eq!(diagnostics[0].source.as_deref(), Some("neon"));
    }

    #[test]
    fn test_generate_diagnostics_with_semantic_error() {
        let source = "print(undefined_var)";
        let diagnostics = generate_diagnostics(source);

        // Should produce diagnostic for undefined variable
        assert!(!diagnostics.is_empty());
        assert_eq!(diagnostics[0].severity, Some(DiagnosticSeverity::ERROR));
    }

    #[test]
    fn test_generate_diagnostics_with_valid_code() {
        let source = r#"
            var x = 42
            print(x)
        "#;
        let diagnostics = generate_diagnostics(source);

        // Valid code should produce no diagnostics
        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn test_generate_diagnostics_with_multiple_errors() {
        let source = "var x = \nvar y = ";
        let diagnostics = generate_diagnostics(source);

        // Should produce multiple diagnostics (at least one for each incomplete statement)
        // The exact number depends on how the parser reports errors
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_generate_diagnostics_with_empty_source() {
        let source = "";
        let diagnostics = generate_diagnostics(source);

        // Empty source should not produce diagnostics
        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn test_calculate_end_column_with_identifier() {
        // "hello world" with offset at 'h'
        let source = "hello world";
        let end_col = calculate_end_column(source, 0, 0);
        assert_eq!(end_col, 5); // "hello" = 5 chars
    }

    #[test]
    fn test_calculate_end_column_with_punctuation() {
        // "foo(bar)" with offset at 'f'
        let source = "foo(bar)";
        let end_col = calculate_end_column(source, 0, 0);
        assert_eq!(end_col, 3); // "foo" stops at '('
    }

    #[test]
    fn test_calculate_end_column_with_whitespace() {
        // "  foo" with offset at whitespace
        let source = "  foo";
        let end_col = calculate_end_column(source, 0, 0);
        assert_eq!(end_col, 1); // Default to single char for whitespace
    }

    #[test]
    fn test_calculate_end_column_with_invalid_offset() {
        let source = "foo";
        let end_col = calculate_end_column(source, 100, 0);
        assert_eq!(end_col, 1); // Default to single char
    }
}
