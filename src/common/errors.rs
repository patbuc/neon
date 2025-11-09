use std::fmt::{Display, Formatter};

use crate::common::SourceLocation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompilationPhase {
    Parse,
    Semantic,
    Codegen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CompilationErrorKind {
    UnexpectedToken,
    DuplicateSymbol,
    UndefinedSymbol,
    ArityExceeded,
    Internal,
    Other,
}

#[derive(Debug, Clone)]
pub(crate) struct CompilationError {
    pub phase: CompilationPhase,
    pub kind: CompilationErrorKind,
    pub message: String,
    pub location: SourceLocation,
}

impl CompilationError {
    pub(crate) fn new(
        phase: CompilationPhase,
        kind: CompilationErrorKind,
        message: impl Into<String>,
        location: SourceLocation,
    ) -> Self {
        CompilationError {
            phase,
            kind,
            message: message.into(),
            location,
        }
    }
}

impl Display for CompilationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:?}] {} at {}",
            self.phase, self.message, self.location
        )
    }
}

impl std::error::Error for CompilationError {}

pub(crate) type CompilationResult<T> = Result<T, Vec<CompilationError>>;
