use crate::common::errors::CompilationError;
use crate::common::SourceLocation;
use colored::Colorize;

pub struct ErrorRenderer {
    use_color: bool,
}

enum Color { Red, Blue }

impl ErrorRenderer {
    pub fn new(use_color: bool) -> Self {
        ErrorRenderer { use_color }
    }

    pub(crate) fn render_errors(
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
            output.push_str(&self.colorize(&summary, Color::Red, true));
            output.push('\n');
        }

        output
    }

    fn render_error(&self, error: &CompilationError, source: &str, filename: &str) -> String {
        let mut output = String::new();

        // Error header: error: <message>
        let error_label = self.colorize("error", Color::Red, true);
        let message = format!(": {}", &error.message);
        output.push_str(&format!("{}{}\n", error_label, message));

        // Location line: --> filename:line:column
        let location_line = format!(
            "  {} {}:{}:{}",
            self.colorize("-->", Color::Blue, true),
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

        // Calculate max line number width for alignment
        let max_line_num = snippet
            .lines
            .iter()
            .map(|l| l.line_number)
            .max()
            .unwrap_or(0);
        let line_num_width = max_line_num.to_string().len();

        for line in &snippet.lines {
            // Line number and content
            let line_num = format!("{:>width$}", line.line_number, width = line_num_width);
            let line_num_colored = self.colorize(&line_num, Color::Blue, true);
            let separator = self.colorize("|", Color::Blue, true);

            // If this is the error line, add the caret on the same line
            if line.is_error_line {
                // Calculate spaces before the caret (error_column is 1-based)
                let spaces_before = " ".repeat((error_column as usize).saturating_sub(1));
                let indicator = self.colorize("^", Color::Red, true);

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



    fn colorize(&self, text: &str, color: Color, bold: bool) -> String {
        if !self.use_color {
            return text.to_string();
        }

        let colored_text = match color {
            Color::Red => text.red(),
            Color::Blue => text.blue(),
            
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

struct SourceLine {
    line_number: u32,
    content: String,
    is_error_line: bool,
}
