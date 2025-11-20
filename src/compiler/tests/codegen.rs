use crate::common::Bloq;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;

fn compile_program(source: &str) -> Result<Bloq, String> {
    // Parse
    let mut parser = Parser::new(source);
    let ast = parser
        .parse()
        .map_err(|e| format!("Parse error: {:?}", e))?;

    // Semantic analysis
    let mut analyzer = SemanticAnalyzer::new();
    let _ = analyzer
        .analyze(&ast)
        .map_err(|e| format!("Semantic error: {:?}", e))?;

    // Code generation
    let mut codegen = CodeGenerator::new();
    codegen
        .generate(&ast)
        .map_err(|e| format!("Codegen error: {:?}", e))
}

#[test]
fn test_simple_number() {
    let bloq = compile_program("42\n").unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_val_declaration() {
    let bloq = compile_program("val x = 5\n").unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_binary_expression() {
    let bloq = compile_program("1 + 2\n").unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_variable_reference() {
    let bloq = compile_program("val x = 5\nprint x\n").unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_function() {
    let program = r#"
    fn add(a, b) {
        return a + b
    }
    val result = add(1, 2)
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_if_statement() {
    let program = r#"
    val x = 10
    if (x > 5) {
        print x
    }
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_while_loop() {
    let program = r#"
    var i = 0
    while (i < 10) {
        i = i + 1
    }
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_end_to_end_execution() {
    use crate::vm::VirtualMachine;

    let program = r#"
    val x = 10
    val y = 20
    val sum = x + y
    print sum
    "#;
    let bloq = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_bloq(bloq);

    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "30");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}

#[test]
fn test_end_to_end_function() {
    use crate::vm::VirtualMachine;

    let program = r#"
    fn add(a, b) {
        return a + b
    }
    val result = add(15, 27)
    print result
    "#;
    let bloq = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_bloq(bloq);

    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "42");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}

#[test]
fn test_end_to_end_forward_reference() {
    use crate::vm::VirtualMachine;

    // This tests that forward function references work!
    let program = r#"
    fn foo() {
        return bar()
    }

    fn bar() {
        return 99
    }

    print foo()
    "#;
    let bloq = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_bloq(bloq);

    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "99");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}
