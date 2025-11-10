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
    ImmutableAssignment,
    ArityExceeded,
    Internal,
    #[allow(dead_code)]
    Other,
}

impl Display for CompilationErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilationErrorKind::UnexpectedToken => write!(f, "Unexpected Token"),
            CompilationErrorKind::DuplicateSymbol => write!(f, "Duplicate Symbol"),
            CompilationErrorKind::UndefinedSymbol => write!(f, "Undefined Symbol"),
            CompilationErrorKind::ImmutableAssignment => write!(f, "Immutable Assignment"),
            CompilationErrorKind::ArityExceeded => write!(f, "Arity Exceeded"),
            CompilationErrorKind::Internal => write!(f, "Internal Error"),
            CompilationErrorKind::Other => write!(f, "Error"),
        }
    }
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
            "[{:?}] {}: {} at {}",
            self.phase, self.kind, self.message, self.location
        )
    }
}

impl std::error::Error for CompilationError {}

pub(crate) type CompilationResult<T> = Result<T, Vec<CompilationError>>;
