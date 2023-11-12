use crate::vm::scanner::Scanner;

pub(crate) struct Compiler {}

impl Compiler {
    pub(super) fn compile(&self, source: String) {
        let scanner = Scanner::new(source);
        for token in scanner {
            println!("{:?}", token);
        }
    }
}
