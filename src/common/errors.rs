use std::fmt::{Display, Formatter};

use crate::common::SourceLocation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilationPhase {
    Parse,
    Semantic,
    Codegen,
}

/// Declares `CompilationErrorKind`, its `code()`, and its `ALL` listing.
macro_rules! compilation_error_kinds {
    ($($variant:ident => $code:literal),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum CompilationErrorKind {
            $($variant),+
        }

        impl CompilationErrorKind {
            pub const ALL: &'static [CompilationErrorKind] = &[
                $(CompilationErrorKind::$variant),+
            ];

            pub fn code(self) -> &'static str {
                match self {
                    $(CompilationErrorKind::$variant => $code),+
                }
            }
        }
    };
}

compilation_error_kinds! {
    UnexpectedCharacter => "E0001",
    UnterminatedString => "E0002",
    InvalidEscapeSequence => "E0003",
    InvalidNumberLiteral => "E0004",
    ExpectedToken => "E0005",
    ExpectedExpression => "E0006",
    InvalidAssignmentTarget => "E0007",
    NumberLiteralTooLarge => "E0008",
    TooManyParameters => "E0009",
    TooManyCallArguments => "E0010",
    NestingTooDeep => "E0011",
    DuplicateSymbol => "E0012",
    UndefinedVariable => "E0013",
    UndefinedType => "E0014",
    ImmutableAssignment => "E0015",
    ReadInOwnInitializer => "E0016",
    UseBeforeDeclaration => "E0017",
    ReservedStructName => "E0018",
    StructNotTopLevel => "E0019",
    ImplNotTopLevel => "E0020",
    DuplicateMethod => "E0021",
    NativeMethodConflict => "E0022",
    MethodFieldConflict => "E0023",
    StaticMethodOnBuiltinType => "E0024",
    StaticCallOnBuiltinType => "E0025",
    LoopControlOutsideLoop => "E0026",
    NamespaceAsValue => "E0027",
    InvalidIncrementTarget => "E0028",
    UnknownNamespaceMethod => "E0029",
    UnknownMethod => "E0030",
    UnknownField => "E0031",
    MethodNeedsInstance => "E0032",
    MethodIsStatic => "E0033",
    NotCallable => "E0034",
    TooFewArguments => "E0035",
    TooManyArguments => "E0036",
    LimitExceeded => "E0037",
}

#[derive(Debug, Clone)]
pub struct CompilationError {
    pub phase: CompilationPhase,
    pub kind: CompilationErrorKind,
    pub message: String,
    pub location: SourceLocation,
}

impl CompilationError {
    pub fn new(
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
            self.phase,
            self.kind.code(),
            self.message,
            self.location
        )
    }
}

impl std::error::Error for CompilationError {}

pub(crate) type CompilationResult<T> = Result<T, Vec<CompilationError>>;
