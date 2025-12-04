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

    let mut vm = VirtualMachine::new(Vec::new());
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

    let mut vm = VirtualMachine::new(Vec::new());
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

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.run_bloq(bloq);

    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "99");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}

#[test]
fn test_map_literal_empty() {
    let program = r#"
    val m = {}
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_map_literal_single_entry() {
    let program = r#"
    val m = {"name": "Alice"}
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_map_literal_multiple_entries() {
    let program = r#"
    val person = {
        "name": "Bob",
        "age": 30,
        "city": "New York"
    }
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_map_index_access() {
    let program = r#"
    val m = {"key": "value"}
    val result = m["key"]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_map_index_assignment() {
    let program = r#"
    var m = {"x": 10}
    m["x"] = 20
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_map_dynamic_key_access() {
    let program = r#"
    val m = {"a": 1, "b": 2}
    val key = "a"
    val value = m[key]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_map_nested_operations() {
    let program = r#"
    val outer = {"inner": {"value": 42}}
    val result = outer["inner"]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_map_with_expressions_as_keys() {
    let program = r#"
    val key1 = "first"
    val key2 = "second"
    val m = {key1: 100, key2: 200}
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_map_with_number_keys() {
    let program = r#"
    val m = {1: "one", 2: "two", 3: "three"}
    val value = m[2]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

// =============================================================================
// Array Literal Tests
// =============================================================================

#[test]
fn test_array_literal_empty() {
    let program = r#"
    val arr = []
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_literal_single_element() {
    let program = r#"
    val arr = [42]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_literal_multiple_elements() {
    let program = r#"
    val arr = [1, 2, 3, 4, 5]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_literal_mixed_types() {
    let program = r#"
    val arr = [1, "hello", true, nil]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_index_access() {
    let program = r#"
    val arr = [1, 2, 3]
    val result = arr[0]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_index_assignment() {
    let program = r#"
    var arr = [1, 2, 3]
    arr[0] = 99
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_negative_indexing() {
    let program = r#"
    val arr = [1, 2, 3]
    val last = arr[-1]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_nested() {
    let program = r#"
    val arr = [[1, 2], [3, 4]]
    val inner = arr[0]
    val value = inner[1]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_with_expressions() {
    let program = r#"
    val arr = [1 + 1, 2 * 3, 10 - 5]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_dynamic_index() {
    let program = r#"
    val arr = [10, 20, 30]
    val i = 1
    val value = arr[i]
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_method_push() {
    let program = r#"
    var arr = [1, 2, 3]
    arr.push(4)
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_method_pop() {
    let program = r#"
    var arr = [1, 2, 3]
    val last = arr.pop()
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_method_length() {
    let program = r#"
    val arr = [1, 2, 3]
    val len = arr.length()
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_in_map() {
    let program = r#"
    val m = {
        "numbers": [1, 2, 3],
        "data": [4, 5, 6]
    }
    "#;
    let bloq = compile_program(program).unwrap();
    assert!(bloq.instruction_count() > 0);
}

#[test]
fn test_array_literal_too_large() {
    // Generate an array literal with more than 65535 elements
    let mut elements = Vec::new();
    for i in 0..70000 {
        elements.push(i.to_string());
    }
    let array_literal = format!("[{}]", elements.join(", "));
    let program = format!("val arr = {}", array_literal);

    let result = compile_program(&program);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("array literal too large"));
    assert!(err.contains("70000"));
    assert!(err.contains("65535"));
}
