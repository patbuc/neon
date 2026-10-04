mod printer;
mod source_map;

use crate::common::errors::CompilationError;
use crate::compiler::parser::Parser;
use printer::Printer;
pub(crate) use source_map::SourceMap;

/// Parses `source` and prints it back out in the formatter's canonical
/// style. Fails on a parse error with the same errors `--check` would
/// report for it.
pub fn format(source: &str) -> Result<String, Vec<CompilationError>> {
    let mut parser = Parser::new(source);
    let stmts = parser.parse()?;
    let map = SourceMap::new(source);
    Printer::new(&map, parser.trivia()).program(&stmts)
}
