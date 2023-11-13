use crate::compiler::{Compiler, Scanner};

impl Compiler {
    pub(crate) fn compile(&self, source: String) {
        let mut scanner = Scanner::new(source);
        for token in scanner.tokens() {
            println!("{:?}", token);
        }
    }
}
