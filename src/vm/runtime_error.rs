use std::fmt::{Display, Formatter};

/// A runtime error produced by the VM: a message plus the source line/column
/// where it occurred, when known.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeError {
    pub message: String,
    pub location: Option<(u32, u32)>,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.location {
            Some((line, column)) => write!(f, "[{}:{}] {}", line, column, self.message),
            None => write!(f, "[unknown] {}", self.message),
        }
    }
}
