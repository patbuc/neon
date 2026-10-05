use crate::common::errors::CompilationError;
use crate::common::{Chunk, SourceLocation, Value};
use crate::compiler::Compiler;
use crate::vm::{InterpretResult, VirtualMachine};

pub fn dummy_location() -> SourceLocation {
    SourceLocation {
        offset: 0,
        line: 1,
        column: 1,
    }
}

pub fn compile(source: &str) -> Result<Chunk, Vec<CompilationError>> {
    let mut compiler = Compiler::new();
    compiler
        .compile(source)
        .ok_or_else(|| compiler.get_structured_errors())
}

/// Disassembles `chunk` and every function nested in its constant pool, so
/// an assertion can look for an opcode anywhere in the compiled program.
pub fn disassemble(chunk: &Chunk) -> String {
    let mut out = chunk.disassemble();
    for constant in &chunk.constants.values {
        if let Value::Function(function) = constant {
            out.push_str(&disassemble(&function.chunk));
        }
    }
    out
}

pub fn compile_errors(source: &str) -> Vec<CompilationError> {
    match compile(source) {
        Ok(_) => panic!("expected a compile error, but this compiled:\n{source}"),
        Err(errors) => errors,
    }
}

pub fn assert_compile_error(source: &str, expected: &str) -> Vec<CompilationError> {
    let errors = compile_errors(source);
    assert!(
        errors
            .first()
            .is_some_and(|error| error.message.contains(expected)),
        "expected the first error to contain {expected:?} for:\n{source}\ngot errors: {errors:#?}"
    );
    errors
}

pub fn run(source: &str) -> String {
    let chunk = match compile(source) {
        Ok(chunk) => chunk,
        Err(errors) => panic!("expected this to compile:\n{source}\ngot errors: {errors:#?}"),
    };
    let mut vm = VirtualMachine::new();
    assert_eq!(vm.run_chunk(chunk), InterpretResult::Ok, "for:\n{source}");
    vm.get_output()
}
