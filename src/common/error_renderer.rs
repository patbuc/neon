use crate::common::errors::CompilationError;
use crate::common::SourceLocation;
use colored::Colorize;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct ErrorRenderer {
    use_color: bool,
    module_sources: HashMap<PathBuf, String>,
}

impl ErrorRenderer {
    pub fn new(use_color: bool) -> Self {
        ErrorRenderer {
            use_color,
            module_sources: HashMap::new(),
        }
    }

    /// Sources of the imported modules errors may be attributed to.
    pub fn with_module_sources(mut self, module_sources: HashMap<PathBuf, String>) -> Self {
        self.module_sources = module_sources;
        self
    }

    pub fn render_errors(
        &self,
        errors: &[CompilationError],
        source: &str,
        filename: &str,
    ) -> String {
        let mut output = String::new();

        for (i, error) in errors.iter().enumerate() {
            if i > 0 {
                output.push('\n');
            }
            output.push_str(&self.render_error(error, source, filename));
        }

        // Add summary
        if !errors.is_empty() {
            output.push('\n');
            let error_word = if errors.len() == 1 { "error" } else { "errors" };
            let summary = format!(
                "\nerror: aborting due to {} previous {}",
                errors.len(),
                error_word
            );
            output.push_str(&self.colorize(&summary, "red", true));
            output.push('\n');
        }

        output
    }

    #[allow(clippy::expect_used)]
    fn render_error(&self, error: &CompilationError, source: &str, filename: &str) -> String {
        let mut output = String::new();
        let (source, filename) = match &error.file {
            Some(file) => (
                self.module_sources
                    .get(file)
                    .expect("an error attributed to a module has that module's source")
                    .as_str(),
                file.display().to_string(),
            ),
            None => (source, filename.to_string()),
        };
        let filename = filename.as_str();

        let error_label = self.colorize(&format!("error[{}]", error.kind.code()), "red", true);
        let message = format!(": {}", self.lowercase_first(&error.message));
        output.push_str(&format!("{}{}\n", error_label, message));

        // Location line: --> filename:line:column
        let location_line = format!(
            "  {} {}:{}:{}",
            self.colorize("-->", "blue", true),
            filename,
            error.location.line,
            error.location.column
        );
        output.push_str(&location_line);
        output.push('\n');

        // Extract source context
        let snippet = self.extract_source_snippet(source, error.location);

        // Render source lines
        output.push_str(&self.render_snippet(&snippet, error.location.column));

        if let Some(help) = &error.help {
            output.push_str(&format!(
                "{}{} {}: {}\n",
                " ".repeat(snippet.line_num_width() + 2),
                self.colorize("=", "blue", true),
                self.colorize("help", "cyan", true),
                help
            ));
        }

        output
    }

    fn extract_source_snippet(&self, source: &str, location: SourceLocation) -> SourceSnippet {
        let lines: Vec<&str> = source.lines().collect();
        let target_line = (location.line as usize).saturating_sub(1);

        // Show 1 line before and the error line
        let start_line = target_line.saturating_sub(1);
        let end_line = (target_line + 1).min(lines.len());

        let mut snippet_lines = Vec::new();
        for i in start_line..end_line {
            if i < lines.len() {
                snippet_lines.push(SourceLine {
                    line_number: (i + 1) as u32,
                    content: lines[i].to_string(),
                    is_error_line: i == target_line,
                });
            }
        }

        SourceSnippet {
            lines: snippet_lines,
        }
    }

    fn render_snippet(&self, snippet: &SourceSnippet, error_column: u32) -> String {
        let mut output = String::new();

        let line_num_width = snippet.line_num_width();

        for line in &snippet.lines {
            // Line number and content
            let line_num = format!("{:>width$}", line.line_number, width = line_num_width);
            let line_num_colored = self.colorize(&line_num, "blue", true);
            let separator = self.colorize("|", "blue", true);

            // If this is the error line, add the caret on the same line
            if line.is_error_line {
                // Calculate spaces before the caret (error_column is 1-based)
                let spaces_before = " ".repeat((error_column as usize).saturating_sub(1));
                let indicator = self.colorize("^", "red", true);

                output.push_str(&format!(
                    " {} {} {}\n{}{}\n",
                    line_num_colored,
                    separator,
                    line.content,
                    " ".repeat(line_num_width + 4 + spaces_before.len()),
                    indicator
                ));
            } else {
                output.push_str(&format!(
                    " {} {} {}\n",
                    line_num_colored, separator, line.content
                ));
            }
        }

        output
    }

    fn lowercase_first(&self, s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => {
                let lowercase_first = first.to_lowercase().collect::<String>();
                lowercase_first + chars.as_str()
            }
        }
    }

    fn colorize(&self, text: &str, color: &str, bold: bool) -> String {
        if !self.use_color {
            return text.to_string();
        }

        let colored_text = match color {
            "red" => text.red(),
            "blue" => text.blue(),
            "yellow" => text.yellow(),
            "cyan" => text.cyan(),
            _ => text.normal(),
        };

        if bold {
            colored_text.bold().to_string()
        } else {
            colored_text.to_string()
        }
    }
}

impl Default for ErrorRenderer {
    fn default() -> Self {
        // Disable colors for wasm target, or check NO_COLOR environment variable
        #[cfg(target_arch = "wasm32")]
        let use_color = false;

        #[cfg(not(target_arch = "wasm32"))]
        let use_color = std::env::var("NO_COLOR").is_err();

        ErrorRenderer::new(use_color)
    }
}

struct SourceSnippet {
    lines: Vec<SourceLine>,
}

impl SourceSnippet {
    fn line_num_width(&self) -> usize {
        self.lines
            .iter()
            .map(|l| l.line_number)
            .max()
            .unwrap_or(0)
            .to_string()
            .len()
    }
}

struct SourceLine {
    line_number: u32,
    content: String,
    is_error_line: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::errors::{CompilationErrorKind, CompilationPhase};

    #[test]
    fn render_error_includes_kind_code_in_header() {
        let renderer = ErrorRenderer::new(false);
        let error = CompilationError::new(
            CompilationPhase::Semantic,
            CompilationErrorKind::UndefinedVariable,
            "Undefined variable 'x'",
            SourceLocation {
                offset: 0,
                line: 1,
                column: 1,
            },
        );

        let output = renderer.render_errors(&[error], "x\n", "test.n");

        assert!(
            output.starts_with("error[E0013]: undefined variable 'x'"),
            "{}",
            output
        );
    }

    #[test]
    fn renders_help() {
        let renderer = ErrorRenderer::new(false);
        let error = CompilationError::new(
            CompilationPhase::Semantic,
            CompilationErrorKind::UndefinedVariable,
            "Undefined variable 'x'",
            SourceLocation {
                offset: 2,
                line: 2,
                column: 1,
            },
        )
        .with_help("available methods: push, pop");

        let output = renderer.render_errors(&[error], "a\nx\n", "test.n");

        assert!(
            output.contains("^\n   = help: available methods: push, pop\n"),
            "{}",
            output
        );
    }

    #[test]
    fn omits_absent_help() {
        let renderer = ErrorRenderer::new(false);
        let error = CompilationError::new(
            CompilationPhase::Semantic,
            CompilationErrorKind::UndefinedVariable,
            "Undefined variable 'x'",
            SourceLocation {
                offset: 2,
                line: 2,
                column: 1,
            },
        );

        let output = renderer.render_errors(&[error], "a\nx\n", "test.n");

        assert!(!output.contains("= help"), "{}", output);
    }

    #[test]
    fn aligns_help_with_a_wide_gutter() {
        let renderer = ErrorRenderer::new(false);
        let error = CompilationError::new(
            CompilationPhase::Semantic,
            CompilationErrorKind::UndefinedVariable,
            "Undefined variable 'x'",
            SourceLocation {
                offset: 18,
                line: 10,
                column: 1,
            },
        )
        .with_help("available methods: push, pop");
        let source = "a\n".repeat(9) + "x\n";

        let output = renderer.render_errors(&[error], &source, "test.n");

        assert!(
            output.contains("^\n    = help: available methods: push, pop\n"),
            "{}",
            output
        );
    }
}
