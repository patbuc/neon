use crate::common::opcodes::OpCode;
use crate::common::Chunk;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;

fn compile_program(source: &str) -> Result<Chunk, String> {
    // Parse
    let mut parser = Parser::new(source);
    let ast = parser
        .parse()
        .map_err(|e| format!("Parse error: {:?}", e))?;

    // Semantic analysis
    let mut analyzer = SemanticAnalyzer::new();
    let resolutions = analyzer
        .analyze(&ast)
        .map_err(|e| format!("Semantic error: {:?}", e))?;

    // Code generation
    let mut codegen = CodeGenerator::new(&resolutions);
    codegen
        .generate(&ast)
        .map_err(|e| format!("Codegen error: {:?}", e))
}

#[test]
fn test_simple_number() {
    let chunk = compile_program("42\n").unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_val_declaration() {
    let chunk = compile_program("val x = 5\n").unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_binary_expression() {
    let chunk = compile_program("1 + 2\n").unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_variable_reference() {
    let chunk = compile_program("val x = 5\nprint(x)\n").unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_function() {
    let program = r#"
    fn add(a, b) {
        return a + b
    }
    val result = add(1, 2)
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_if_statement() {
    let program = r#"
    val x = 10
    if (x > 5) {
        print(x)
    }
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_while_loop() {
    let program = r#"
    var i = 0
    while (i < 10) {
        i = i + 1
    }
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_end_to_end_execution() {
    use crate::vm::VirtualMachine;

    let program = r#"
    val x = 10
    val y = 20
    val sum = x + y
    print(sum)
    "#;
    let chunk = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);

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
    print(result)
    "#;
    let chunk = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);

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

    print(foo())
    "#;
    let chunk = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);

    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "99");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}

#[test]
fn test_else_if_bytecode_simple() {
    use crate::common::opcodes::OpCode;

    // Test simple else-if chain bytecode generation
    let program = r#"
    val x = 5
    if (x == 1) {
        print(1)
    } else if (x == 2) {
        print(2)
    } else {
        print(3)
    }
    "#;
    let chunk = compile_program(program).unwrap();

    // Verify that bytecode contains the expected jump instructions
    // Pattern should be:
    // 1. First condition (x == 1)
    // 2. JumpIfFalse (skip first then-branch)
    // 3. First then-branch code
    // 4. Jump (skip else-if and else)
    // 5. Second condition (x == 2) - this is the else-if
    // 6. JumpIfFalse (skip second then-branch)
    // 7. Second then-branch code
    // 8. Jump (skip else)
    // 9. Else-branch code

    let mut jump_if_false_count = 0;
    let mut jump_count = 0;

    let mut offset = 0;
    while offset < chunk.instruction_count() {
        let op = OpCode::from_u8(chunk.read_u8(offset)).unwrap();
        match op {
            OpCode::JumpIfFalse => {
                jump_if_false_count += 1;
                offset += 5; // OpCode (1 byte) + offset (4 bytes)
            }
            OpCode::Jump => {
                jump_count += 1;
                offset += 5; // OpCode (1 byte) + offset (4 bytes)
            }
            OpCode::Constant
            | OpCode::SetLocal
            | OpCode::GetLocal
            | OpCode::GetGlobal
            | OpCode::SetGlobal
            | OpCode::GetField
            | OpCode::SetField
            | OpCode::String => {
                offset += 3; // OpCode (1 byte) + u16 operand
            }
            OpCode::Call => {
                offset += 2; // OpCode (1 byte) + 1-byte argument count
            }
            _ => {
                offset += 1; // Simple instructions
            }
        }
    }

    // We should have 2 JumpIfFalse (one for each condition)
    assert_eq!(
        jump_if_false_count, 2,
        "Expected 2 JumpIfFalse instructions for if and else-if conditions"
    );

    // We should have 2 Jump instructions (one after each then-branch)
    assert_eq!(
        jump_count, 2,
        "Expected 2 Jump instructions to skip remaining branches"
    );
}

#[test]
fn test_else_if_bytecode_multiple_branches() {
    use crate::common::opcodes::OpCode;

    // Test multiple else-if branches
    let program = r#"
    val x = 10
    if (x < 5) {
        print(1)
    } else if (x < 10) {
        print(2)
    } else if (x < 15) {
        print(3)
    } else if (x < 20) {
        print(4)
    } else {
        print(5)
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let mut jump_if_false_count = 0;
    let mut jump_count = 0;

    let mut offset = 0;
    while offset < chunk.instruction_count() {
        let op = OpCode::from_u8(chunk.read_u8(offset)).unwrap();
        match op {
            OpCode::JumpIfFalse => {
                jump_if_false_count += 1;
                offset += 5; // OpCode (1 byte) + offset (4 bytes)
            }
            OpCode::Jump => {
                jump_count += 1;
                offset += 5; // OpCode (1 byte) + offset (4 bytes)
            }
            OpCode::Constant
            | OpCode::SetLocal
            | OpCode::GetLocal
            | OpCode::GetGlobal
            | OpCode::SetGlobal
            | OpCode::GetField
            | OpCode::SetField
            | OpCode::String => {
                offset += 3; // OpCode (1 byte) + u16 operand
            }
            OpCode::Call => {
                offset += 2; // OpCode (1 byte) + 1-byte argument count
            }
            _ => {
                offset += 1; // Simple instructions
            }
        }
    }

    // We should have 4 JumpIfFalse (one for each condition)
    assert_eq!(
        jump_if_false_count, 4,
        "Expected 4 JumpIfFalse instructions for all conditions"
    );

    // We should have 4 Jump instructions (one after each then-branch)
    assert_eq!(
        jump_count, 4,
        "Expected 4 Jump instructions to skip remaining branches"
    );
}

#[test]
fn test_else_if_bytecode_without_final_else() {
    use crate::common::opcodes::OpCode;

    // Test else-if chain without final else
    let program = r#"
    val x = 7
    if (x == 5) {
        print(5)
    } else if (x == 7) {
        print(7)
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let mut jump_if_false_count = 0;
    let mut jump_count = 0;

    let mut offset = 0;
    while offset < chunk.instruction_count() {
        let op = OpCode::from_u8(chunk.read_u8(offset)).unwrap();
        match op {
            OpCode::JumpIfFalse => {
                jump_if_false_count += 1;
                offset += 5; // OpCode (1 byte) + offset (4 bytes)
            }
            OpCode::Jump => {
                jump_count += 1;
                offset += 5; // OpCode (1 byte) + offset (4 bytes)
            }
            OpCode::Constant
            | OpCode::SetLocal
            | OpCode::GetLocal
            | OpCode::GetGlobal
            | OpCode::SetGlobal
            | OpCode::GetField
            | OpCode::SetField
            | OpCode::String => {
                offset += 3; // OpCode (1 byte) + u16 operand
            }
            OpCode::Call => {
                offset += 2; // OpCode (1 byte) + 1-byte argument count
            }
            _ => {
                offset += 1; // Simple instructions
            }
        }
    }

    // We should have 2 JumpIfFalse (one for each condition)
    assert_eq!(
        jump_if_false_count, 2,
        "Expected 2 JumpIfFalse instructions"
    );

    // We should have 2 Jump instructions (one after each then-branch)
    assert_eq!(jump_count, 2, "Expected 2 Jump instructions");
}

#[test]
fn test_else_if_bytecode_jump_offsets() {
    // Test that jump offsets are correctly calculated
    let program = r#"
    val x = 5
    if (x == 1) {
        print(1)
    } else if (x == 2) {
        print(2)
    } else {
        print(3)
    }
    "#;
    let chunk = compile_program(program).unwrap();

    // Verify the bytecode compiles and has instructions
    assert!(
        chunk.instruction_count() > 0,
        "Bytecode should not be empty"
    );

    // Walk through bytecode to find and verify jump instructions
    let mut i = 0;
    let mut jumps = Vec::new();

    while i < chunk.instruction_count() {
        let op = crate::common::opcodes::OpCode::from_u8(chunk.read_u8(i)).unwrap();
        match op {
            crate::common::opcodes::OpCode::JumpIfFalse | crate::common::opcodes::OpCode::Jump => {
                // Read the 4-byte offset
                let offset = chunk.read_u32(i + 1);
                let target = i + 5 + offset as usize;
                jumps.push((i, op, target));
                i += 5; // OpCode (1 byte) + offset (4 bytes)
            }
            _ => i += 1,
        }
    }

    // Verify jumps are pointing to valid locations within bytecode
    for (pos, _op, target) in &jumps {
        assert!(
            *target <= chunk.instruction_count(),
            "Jump at position {} targets invalid offset {}",
            pos,
            target
        );
    }
}

#[test]
fn test_else_if_end_to_end_execution() {
    use crate::vm::VirtualMachine;

    // Test that else-if chains execute correctly
    let program = r#"
    val x = 15
    if (x < 10) {
        print(10)
    } else if (x < 20) {
        print(20)
    } else {
        print(30)
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);

    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "20");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}

#[test]
fn test_map_literal_empty() {
    let program = r#"
    val m = {}
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_map_literal_single_entry() {
    let program = r#"
    val m = {"name": "Alice"}
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
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
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_map_index_access() {
    let program = r#"
    val m = {"key": "value"}
    val result = m["key"]
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_map_index_assignment() {
    let program = r#"
    var m = {"x": 10}
    m["x"] = 20
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_map_dynamic_key_access() {
    let program = r#"
    val m = {"a": 1, "b": 2}
    val key = "a"
    val value = m[key]
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_map_nested_operations() {
    let program = r#"
    val outer = {"inner": {"value": 42}}
    val result = outer["inner"]
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_map_with_expressions_as_keys() {
    let program = r#"
    val key1 = "first"
    val key2 = "second"
    val m = {key1: 100, key2: 200}
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_map_with_number_keys() {
    let program = r#"
    val m = {1: "one", 2: "two", 3: "three"}
    val value = m[2]
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

// =============================================================================
// Array Literal Tests
// =============================================================================

#[test]
fn test_array_literal_empty() {
    let program = r#"
    val arr = []
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_array_literal_single_element() {
    let program = r#"
    val arr = [42]
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_array_literal_multiple_elements() {
    let program = r#"
    val arr = [1, 2, 3, 4, 5]
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_array_literal_mixed_types() {
    let program = r#"
    val arr = [1, "hello", true, nil]
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_array_index_access() {
    let program = r#"
    val arr = [1, 2, 3]
    val result = arr[0]
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_array_index_assignment() {
    let program = r#"
    var arr = [1, 2, 3]
    arr[0] = 99
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_array_negative_indexing() {
    let program = r#"
    val arr = [1, 2, 3]
    val last = arr[-1]
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_array_nested() {
    let program = r#"
    val arr = [[1, 2], [3, 4]]
    val inner = arr[0]
    val value = inner[1]
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_array_with_expressions() {
    let program = r#"
    val arr = [1 + 1, 2 * 3, 10 - 5]
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_array_dynamic_index() {
    let program = r#"
    val arr = [10, 20, 30]
    val i = 1
    val value = arr[i]
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_array_method_push() {
    let program = r#"
    var arr = [1, 2, 3]
    arr.push(4)
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_array_method_pop() {
    let program = r#"
    var arr = [1, 2, 3]
    val last = arr.pop()
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_array_method_length() {
    let program = r#"
    val arr = [1, 2, 3]
    val len = arr.length()
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_array_in_map() {
    let program = r#"
    val m = {
        "numbers": [1, 2, 3],
        "data": [4, 5, 6]
    }
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
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

#[test]
fn test_function_constant_pool_too_large() {
    // 65,536 distinct number-literal statements in one function overflow the
    // u16 constant-pool index.
    let mut body = String::new();
    for i in 0..65536 {
        body.push_str(&i.to_string());
        body.push('\n');
    }
    let program = format!("fn f() {{\n{}\n}}\nf()\n", body);

    let result = compile_program(&program);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("constants"));
    assert!(err.contains("65535"));
}

#[test]
fn test_function_constant_pool_at_limit_compiles() {
    // 65,535 distinct constants is exactly the u16 index limit.
    let mut body = String::new();
    for i in 0..65535 {
        body.push_str(&i.to_string());
        body.push('\n');
    }
    let program = format!("fn f() {{\n{}\n}}\nf()\n", body);

    assert!(compile_program(&program).is_ok());
}

#[test]
fn test_function_constant_pool_overflow_reports_once() {
    // Overflowing further than the minimum still reports a single error.
    let mut body = String::new();
    for i in 0..65540 {
        body.push_str(&i.to_string());
        body.push('\n');
    }
    let program = format!("fn f() {{\n{}\n}}\nf()\n", body);

    let err = compile_program(&program).unwrap_err();
    assert_eq!(err.matches("too many constants").count(), 1, "{}", err);
}

#[test]
fn test_nested_functions_each_report_their_own_constant_overflow() {
    // Overflowing constant pools in two different functions are two
    // independent errors, not deduplicated across functions.
    let mut body = String::new();
    for i in 0..65536 {
        body.push_str(&i.to_string());
        body.push('\n');
    }
    let program = format!(
        "fn outer() {{\n{body}\n    fn inner() {{\n{body}\n    }}\n    inner()\n}}\nouter()\n"
    );

    let err = compile_program(&program).unwrap_err();
    assert_eq!(err.matches("too many constants").count(), 2, "{}", err);
}

#[test]
fn test_map_literal_too_large() {
    // Generate a map literal with more than 65535 entries
    let entries: Vec<String> = (0..70000).map(|i| format!("{}: {}", i, i)).collect();
    let program = format!("val m = {{{}}}", entries.join(", "));

    let result = compile_program(&program);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("map literal too large"));
    assert!(err.contains("70000"));
    assert!(err.contains("65535"));
}

#[test]
fn test_set_literal_too_large() {
    // Generate a set literal with more than 65535 elements
    let elements: Vec<String> = (0..70000).map(|i| i.to_string()).collect();
    let program = format!("val s = #{{{}}}", elements.join(", "));

    let result = compile_program(&program);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("set literal too large"));
    assert!(err.contains("70000"));
    assert!(err.contains("65535"));
}

// =============================================================================
// Postfix Increment/Decrement Tests
// =============================================================================

#[test]
fn test_postfix_increment_compiles() {
    let program = r#"
    var x = 5
    x++
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_postfix_decrement_compiles() {
    let program = r#"
    var x = 10
    x--
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_postfix_increment_in_expression() {
    let program = r#"
    var x = 5
    val y = x++
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_postfix_decrement_in_expression() {
    let program = r#"
    var x = 10
    val y = x--
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_postfix_increment_in_print() {
    let program = r#"
    var x = 5
    print(x++)
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_postfix_decrement_in_print() {
    let program = r#"
    var x = 10
    print(x--)
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_multiple_postfix_operations() {
    let program = r#"
    var x = 5
    var y = 10
    x++
    y--
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_postfix_increment_in_loop() {
    let program = r#"
    var i = 0
    while (i < 5) {
        i++
    }
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_postfix_operations_with_arithmetic() {
    let program = r#"
    var x = 5
    val y = x++ + 10
    "#;
    let chunk = compile_program(program).unwrap();
    assert!(chunk.instruction_count() > 0);
}

#[test]
fn test_postfix_increment_end_to_end() {
    use crate::vm::VirtualMachine;

    let program = r#"
    var x = 5
    val old = x++
    print(old)
    print(x)
    "#;
    let chunk = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);

    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "5\n6");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}

#[test]
fn test_postfix_decrement_end_to_end() {
    use crate::vm::VirtualMachine;

    let program = r#"
    var x = 10
    val old = x--
    print(old)
    print(x)
    "#;
    let chunk = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);

    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "10\n9");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}

#[test]
fn test_postfix_increment_multiple_times() {
    use crate::vm::VirtualMachine;

    let program = r#"
    var x = 0
    x++
    x++
    x++
    print(x)
    "#;
    let chunk = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);

    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "3");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}

#[test]
fn test_postfix_operations_in_function() {
    use crate::vm::VirtualMachine;

    let program = r#"
    fn increment_and_return(value) {
        var x = value
        val old = x++
        return old
    }

    val result = increment_and_return(5)
    print(result)
    "#;
    let chunk = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);

    #[cfg(any(test, debug_assertions))]
    {
        // The function creates a local mutable variable and increments it
        // Should return the old value (5)
        assert_eq!(vm.get_output(), "5");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}

/// Walks a chunk's bytecode, stepping over each instruction's operand bytes,
/// and returns just the opcodes in order. Only knows the operand width of
/// the opcodes the `>=`/`<=` fixture below emits; panics by name on any
/// other opcode rather than guessing its width.
fn op_codes(chunk: &Chunk) -> Vec<OpCode> {
    let mut ops = Vec::new();
    let mut offset = 0;
    while offset < chunk.instruction_count() {
        let op = OpCode::from_u8(chunk.read_u8(offset)).unwrap();
        let operand_bytes = match op {
            OpCode::Return
            | OpCode::Nil
            | OpCode::GreaterEqual
            | OpCode::LessEqual
            | OpCode::Pop
            | OpCode::Add
            | OpCode::Less
            | OpCode::CloseUpvalue
            | OpCode::CloseUpvalueInPlace => 0,
            OpCode::Call => 1,
            OpCode::Invoke => 3,
            OpCode::CreateArray => 2,
            OpCode::Constant
            | OpCode::String
            | OpCode::SetLocal
            | OpCode::GetLocal
            | OpCode::GetGlobal
            | OpCode::SetGlobal
            | OpCode::GetBuiltin
            | OpCode::GetField
            | OpCode::SetField
            | OpCode::GetUpvalue
            | OpCode::SetUpvalue => 2,
            OpCode::JumpIfFalse | OpCode::Jump | OpCode::Loop => 4,
            OpCode::Closure => {
                // 2-byte constant index, then a 1-byte upvalue count and
                // that many (is_local, index) pairs (1 + 2 bytes each).
                let upvalue_count = chunk.read_u8(offset + 3) as usize;
                3 + upvalue_count * 3
            }
            _ => panic!("op_codes: unhandled opcode {op:?}, add its operand width"),
        };
        offset += 1 + operand_bytes;
        ops.push(op);
    }
    ops
}

#[test]
fn test_greater_equal_less_equal_opcodes() {
    let program = "val a = 1\nval b = 2\nprint(a >= b)\nprint(a <= b)\n";
    let chunk = compile_program(program).unwrap();

    let ops = op_codes(&chunk);

    assert_eq!(
        ops,
        vec![
            OpCode::Constant,
            OpCode::SetLocal,
            OpCode::Constant,
            OpCode::SetLocal,
            OpCode::Constant,
            OpCode::SetLocal,
            OpCode::Pop,
            OpCode::Constant,
            OpCode::SetLocal,
            OpCode::Pop,
            OpCode::Constant,
            OpCode::GetLocal,
            OpCode::GetLocal,
            OpCode::GreaterEqual,
            OpCode::Call,
            OpCode::Pop,
            OpCode::Constant,
            OpCode::GetLocal,
            OpCode::GetLocal,
            OpCode::LessEqual,
            OpCode::Call,
            OpCode::Pop,
            OpCode::Nil,
            OpCode::Return,
        ]
    );
}

#[test]
fn test_uncaptured_for_loop_emits_no_close_upvalue_in_place() {
    let program = r#"
    for (var i = 0; i < 3; i = i + 1) {
        print(i)
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let close_upvalue_in_place_count = op_codes(&chunk)
        .into_iter()
        .filter(|op| *op == OpCode::CloseUpvalueInPlace)
        .count();

    assert_eq!(close_upvalue_in_place_count, 0);
}

#[test]
fn test_captured_for_loop_emits_one_close_upvalue_in_place() {
    let program = r#"
    var last = nil
    for (var i = 0; i < 3; i = i + 1) {
        last = fn() { return i }
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let close_upvalue_in_place_count = op_codes(&chunk)
        .into_iter()
        .filter(|op| *op == OpCode::CloseUpvalueInPlace)
        .count();

    assert_eq!(close_upvalue_in_place_count, 1);
}

// =============================================================================
// Per-Function Loop State Tests
// =============================================================================

#[test]
fn test_top_level_fn_named_print_shadows_native() {
    use crate::vm::VirtualMachine;

    let program = r#"
    fn print(x) {}
    print("native")
    "#;
    let chunk = compile_program(program).unwrap();

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);

    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "");
    }

    assert_eq!(result, crate::vm::Result::Ok);
}

#[test]
fn test_native_call_labels() {
    use crate::common::Object;
    use crate::common::Value;

    let program = r#"
    print(1)
    File("x")
    Math.abs(1)
    "#;
    let chunk = compile_program(program).unwrap();

    let labels: Vec<String> = chunk
        .constants
        .values
        .iter()
        .filter_map(|value| match value {
            Value::Object(object) => match object.as_ref() {
                Object::NativeFunction(native) => Some(native.name.clone()),
                _ => None,
            },
            _ => None,
        })
        .collect();

    assert_eq!(labels, vec!["print", "File.new", "abs"]);
}

#[test]
fn method_call_loads_receiver_then_invoke() {
    let program = "val a = [1]\na.size()\n";
    let chunk = compile_program(program).unwrap();

    let ops = op_codes(&chunk);

    let invoke_index = ops
        .iter()
        .position(|op| *op == OpCode::Invoke)
        .expect("expected an Invoke instruction");
    assert_eq!(OpCode::GetLocal, ops[invoke_index - 1]);
}
