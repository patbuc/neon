mod compiler;
mod scanner;
mod tokens;

pub(crate) struct Compiler {}

struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: u32,
}
