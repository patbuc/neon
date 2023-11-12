use crate::vm::compiler::scanner::Scanner;
use crate::vm::compiler::Compiler;

impl Compiler {
    pub(in crate::vm) fn compile(&self, source: String) {
        let scanner = Scanner::new(source);
        for token in scanner {
            println!("{:?}", token);
        }
    }
}
