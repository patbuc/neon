use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct TraceFrame {
    pub function: String,
    pub line: Option<u32>,
}

pub(crate) const TRACE_EDGE_FRAMES: usize = 10;

/// A runtime error produced by the VM: a message, the source line/column
/// where it occurred (when known), and the call chain that led there,
/// innermost frame first. Past `TRACE_EDGE_FRAMES * 2 + 1` frames, only
/// the innermost and outermost `TRACE_EDGE_FRAMES` are kept in `frames`, and
/// `omitted_frames` records how many were left out.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeError {
    pub message: String,
    pub location: Option<(u32, u32)>,
    pub frames: Vec<TraceFrame>,
    pub omitted_frames: usize,
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
    /// frame. When `omitted_frames` is non-zero, `frames` holds only the
    /// innermost and outermost `TRACE_EDGE_FRAMES`, with an "N frames
    /// omitted" line rendered between them.
    pub fn trace(&self) -> String {
        let render = |frame: &TraceFrame| match frame.line {
            Some(line) => format!("  at {} (line {})", frame.function, line),
            None => format!("  at {} (line ?)", frame.function),
        };

        if self.omitted_frames > 0 {
            let mut lines: Vec<String> = self.frames[..TRACE_EDGE_FRAMES]
                .iter()
                .map(render)
                .collect();
            lines.push(format!("  ... {} frames omitted", self.omitted_frames));
            lines.extend(self.frames[TRACE_EDGE_FRAMES..].iter().map(render));
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
