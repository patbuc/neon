use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct TraceFrame {
    pub function: String,
    pub line: Option<u32>,
}

const TRACE_EDGE_FRAMES: usize = 10;

/// A runtime error produced by the VM: a message, the source line/column
/// where it occurred (when known), and the call chain that led there,
/// innermost frame first.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeError {
    pub message: String,
    pub location: Option<(u32, u32)>,
    pub frames: Vec<TraceFrame>,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.location {
            Some((line, column)) => write!(f, "[{}:{}] {}", line, column, self.message),
            None => write!(f, "[unknown] {}", self.message),
        }
    }
}

impl RuntimeError {
    /// Renders the call trace as one `  at <function> (line <n>)` line per
    /// frame. Past 21 frames, only the innermost and outermost 10 are
    /// shown, with an "N frames omitted" line between them.
    pub fn trace(&self) -> String {
        let render = |frame: &TraceFrame| match frame.line {
            Some(line) => format!("  at {} (line {})", frame.function, line),
            None => format!("  at {} (line ?)", frame.function),
        };

        if self.frames.len() > TRACE_EDGE_FRAMES * 2 + 1 {
            let mut lines: Vec<String> = self.frames[..TRACE_EDGE_FRAMES]
                .iter()
                .map(render)
                .collect();
            lines.push(format!(
                "  ... {} frames omitted",
                self.frames.len() - TRACE_EDGE_FRAMES * 2
            ));
            lines.extend(
                self.frames[self.frames.len() - TRACE_EDGE_FRAMES..]
                    .iter()
                    .map(render),
            );
            lines.join("\n")
        } else {
            self.frames
                .iter()
                .map(render)
                .collect::<Vec<_>>()
                .join("\n")
        }
    }

    /// The full report: the `[line:col] message` line followed by the
    /// call trace.
    pub fn report(&self) -> String {
        format!("{}\n{}", self, self.trace())
    }
}
