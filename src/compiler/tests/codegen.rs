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

// ===== Array Bytecode Generation Tests =====

#[test]
fn test_array_literal_empty() {
    let bloq = compile_program("[]\n").unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_literal_single_element() {
    let bloq = compile_program("[42]\n").unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_literal_multiple_elements() {
    let bloq = compile_program("[1, 2, 3]\n").unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_literal_mixed_types() {
    let bloq = compile_program(r#"[1, "hello", true, nil]"#).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_literal_with_expressions() {
    let program = r#"
    val x = 10
    val y = 20
    [x + y, x * 2, y - x]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_literal_nested() {
    let bloq = compile_program("[[1, 2], [3, 4]]\n").unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_indexing_constant() {
    let program = r#"
    val arr = [1, 2, 3]
    arr[0]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_indexing_variable() {
    let program = r#"
    val arr = [1, 2, 3]
    val i = 1
    arr[i]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_indexing_expression() {
    let program = r#"
    val arr = [10, 20, 30]
    val i = 1
    arr[i + 1]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_indexing_nested() {
    let program = r#"
    val matrix = [[1, 2], [3, 4]]
    matrix[0][1]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_set_index_constant() {
    let program = r#"
    var arr = [1, 2, 3]
    arr[0] = 10
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_set_index_variable() {
    let program = r#"
    var arr = [1, 2, 3]
    val i = 1
    arr[i] = 20
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_set_index_expression_index() {
    let program = r#"
    var arr = [1, 2, 3]
    val i = 0
    arr[i + 1] = 99
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_set_index_expression_value() {
    let program = r#"
    var arr = [1, 2, 3]
    val x = 5
    arr[0] = x * 2
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_set_index_nested() {
    let program = r#"
    var matrix = [[1, 2], [3, 4]]
    matrix[0][1] = 99
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_with_val_assignment() {
    let program = r#"
    val arr = [1, 2, 3, 4, 5]
    print arr
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_with_var_assignment() {
    let program = r#"
    var arr = [10, 20]
    arr[0] = 30
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_in_function() {
    let program = r#"
    fn get_array() {
        return [1, 2, 3]
    }
    val result = get_array()
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_as_function_parameter() {
    let program = r#"
    fn sum_array(arr) {
        return arr[0] + arr[1]
    }
    val result = sum_array([10, 20])
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_complex_expression() {
    let program = r#"
    val arr = [1, 2, 3]
    val i = 0
    val j = 1
    val result = arr[i] + arr[j] * arr[i + j]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_in_if_statement() {
    let program = r#"
    val arr = [1, 2, 3]
    if (arr[0] > 0) {
        print arr[1]
    }
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_in_while_loop() {
    let program = r#"
    var arr = [1, 2, 3, 4, 5]
    var i = 0
    while (i < 5) {
        print arr[i]
        i = i + 1
    }
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}
