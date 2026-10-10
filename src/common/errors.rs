use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use crate::common::SourceLocation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilationPhase {
    Parse,
    Semantic,
    Codegen,
    Format,
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
    UnknownMethod => "E0030",
    UnknownField => "E0031",
    MethodNeedsInstance => "E0032",
    MethodIsStatic => "E0033",
    NotCallable => "E0034",
    TooFewArguments => "E0035",
    TooManyArguments => "E0036",
    LimitExceeded => "E0037",
    DuplicateField => "E0038",
    TooManySymbols => "E0039",
    DuplicateEnumVariant => "E0040",
    UnknownEnumVariant => "E0041",
    EnumAsValue => "E0042",
    EnumNotTopLevel => "E0043",
    ImplOnEnum => "E0044",
    UnplaceableComment => "E0045",
    OptionalDotOnType => "E0046",
    NonExhaustiveMatch => "E0047",
    PatternNotInEnum => "E0048",
    UnreachablePattern => "E0049",
    InvalidMatchPattern => "E0050",
    ImportNotTopLevel => "E0051",
    ExportNotTopLevel => "E0052",
    ImportCycle => "E0054",
    UnknownModule => "E0055",
    FileImportUnavailable => "E0056",
    UnknownExport => "E0057",
    InvalidImportName => "E0058",
    ImplOutsideOwnModule => "E0059",
    YieldOutsideFunction => "E0060",
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompilationError {
    pub phase: CompilationPhase,
    pub kind: CompilationErrorKind,
    pub message: String,
    pub location: SourceLocation,
    pub file: Option<PathBuf>,
    pub help: Option<String>,
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
            file: None,
            help: None,
        }
    }

    pub fn with_file(mut self, file: &Path) -> Self {
        self.file = Some(file.to_path_buf());
        self
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
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
