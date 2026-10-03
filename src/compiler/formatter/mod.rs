mod printer;
mod source_map;

use crate::common::errors::CompilationError;
use crate::compiler::parser::Parser;
use printer::Printer;
pub(crate) use source_map::SourceMap;

/// Parses `source` and prints it back out in the formatter's canonical
/// style. Fails exactly when `--check` would: a parse error returns the
/// same errors.
pub fn format(source: &str) -> Result<String, Vec<CompilationError>> {
    let mut parser = Parser::new(source);
    let stmts = parser.parse()?;
    let map = SourceMap::new(source);
    Ok(Printer::new(&map).program(&stmts))
}
