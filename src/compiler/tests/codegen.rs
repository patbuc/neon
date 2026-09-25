use crate::common::opcodes::OpCode;
use crate::common::Chunk;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;

fn disassemble_program(chunk: &Chunk) -> String {
    use crate::common::Value;

    let mut out = chunk.disassemble();
    for constant in &chunk.constants.values {
        if let Value::Function(function) = constant {
            out.push_str(&disassemble_program(&function.chunk));
        }
    }
    out
}

fn compile_program(source: &str) -> Result<Chunk, String> {
    // Parse
    let mut parser = Parser::new(source);
    let ast = parser
        .parse()
        .map_err(|e| format!("Parse error: {:?}", e))?;
    let eof_location = parser.eof_location();

    // Semantic analysis
    let mut analyzer = SemanticAnalyzer::new();
    let resolutions = analyzer
        .analyze(&ast)
        .map_err(|e| format!("Semantic error: {:?}", e))?;

    // Code generation
    let mut codegen = CodeGenerator::new(&resolutions, parser.end_locations());
    codegen
        .generate(&ast, eof_location)
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

    assert_eq!(result, crate::vm::InterpretResult::Ok);
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

    assert_eq!(result, crate::vm::InterpretResult::Ok);
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

    assert_eq!(result, crate::vm::InterpretResult::Ok);
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
            | OpCode::SetField => {
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
            | OpCode::SetField => {
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
            | OpCode::SetField => {
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

    assert_eq!(result, crate::vm::InterpretResult::Ok);
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

// =============================================================================
// Constant pool deduplication tests
// =============================================================================

fn count_strings(chunk: &Chunk, s: &str) -> usize {
    use crate::common::Value;
    chunk
        .constants
        .values
        .iter()
        .filter(|value| matches!(value, Value::String(v) if v.as_str() == s))
        .count()
}

fn count_numbers(chunk: &Chunk, n: f64) -> usize {
    use crate::common::Value;
    chunk
        .constants
        .values
        .iter()
        .filter(|value| matches!(value, Value::Number(v) if *v == n))
        .count()
}

#[test]
fn test_repeated_string_literal_dedups() {
    let program = "print(\"hi\")\n".repeat(10);
    let chunk = compile_program(&program).unwrap();
    assert_eq!(count_strings(&chunk, "hi"), 1);
}

#[test]
fn test_repeated_field_name_dedups() {
    let program = r#"
    struct P { x }
    val p = P(1)
    p.x
    p.x
    p.x
    p.x
    p.x
    p.x
    p.x
    p.x
    p.x
    p.x
    "#;
    let chunk = compile_program(program).unwrap();
    assert_eq!(count_strings(&chunk, "x"), 1);
}

#[test]
fn test_repeated_method_name_dedups() {
    let program = r#"
    struct P { }
    impl P {
        fn m(self) { return 1 }
    }
    val p = P()
    p.m()
    p.m()
    p.m()
    p.m()
    p.m()
    p.m()
    p.m()
    p.m()
    p.m()
    p.m()
    "#;
    let chunk = compile_program(program).unwrap();
    assert_eq!(count_strings(&chunk, "m"), 1);
}

#[test]
fn test_repeated_number_literal_dedups() {
    let program = "1\n".repeat(10);
    let chunk = compile_program(&program).unwrap();
    assert_eq!(count_numbers(&chunk, 1.0), 1);
}

#[test]
fn test_zero_and_negative_zero_stay_distinct() {
    use crate::common::Value;

    let mut parser = Parser::new("");
    let ast = parser.parse().unwrap();
    let mut analyzer = SemanticAnalyzer::new();
    let resolutions = analyzer.analyze(&ast).unwrap();
    let mut codegen = CodeGenerator::new(&resolutions, parser.end_locations());

    let positive_zero = codegen.add_constant(Value::Number(0.0));
    let negative_zero = codegen.add_constant(Value::Number(-0.0));

    assert_ne!(positive_zero, negative_zero);
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

    assert_eq!(result, crate::vm::InterpretResult::Ok);
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

    assert_eq!(result, crate::vm::InterpretResult::Ok);
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

    assert_eq!(result, crate::vm::InterpretResult::Ok);
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

    assert_eq!(result, crate::vm::InterpretResult::Ok);
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

    assert_eq!(result, crate::vm::InterpretResult::Ok);
}

#[test]
fn test_native_call_labels() {
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
            Value::NativeFunction(native) => Some(native.name.clone()),
            _ => None,
        })
        .collect();

    assert_eq!(labels, vec!["print", "File.new", "abs"]);
}

#[test]
fn test_method_call_loads_receiver_then_invoke() {
    let program = "val a = [1]\na.size()\n";
    let chunk = compile_program(program).unwrap();

    let ops = op_codes(&chunk);

    let invoke_index = ops
        .iter()
        .position(|op| *op == OpCode::Invoke)
        .expect("expected an Invoke instruction");
    assert_eq!(OpCode::GetLocal, ops[invoke_index - 1]);
}

#[test]
fn test_while_break_continue_bytecode() {
    use crate::vm::VirtualMachine;

    let program = r#"
    var i = 0
    while (i < 10) {
        i = i + 1
        if (i == 2) { continue }
        if (i == 5) { break }
        print(i)
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 Constant 00 '<uninitialized>'
0003      | SetLocal 00
0006      | Constant 01 '0'
0009      | SetLocal 00
000c      | Pop
000d      3 GetLocal 00
0010      | Constant 02 '10'
0013      | Less
0014      | JumpIfFalse 0014 -> 0063
0019      | Pop
001a      4 GetLocal 00
001d      | Constant 03 '1'
0020      | Add
0021      | SetLocal 00
0024      3 Pop
0025      5 GetLocal 00
0028      | Constant 04 '2'
002b      | Equal
002c      | JumpIfFalse 002c -> 003c
0031      | Pop
0032      | Jump 0032 -> 005e
0037      | Jump 0037 -> 003d
003c      | Pop
003d      6 GetLocal 00
0040      | Constant 05 '5'
0043      | Equal
0044      | JumpIfFalse 0044 -> 0054
0049      | Pop
004a      | Jump 004a -> 0064
004f      | Jump 004f -> 0055
0054      | Pop
0055      7 Constant 06 '<native fn print>'
0058      | GetLocal 00
005b      | Call (args: 1)
005d      6 Pop
005e      3 Loop 005e -> 000d
0063      | Pop
0064      9 Nil
0065      | Return
=== </main> ===
"#;

    assert_eq!(disassemble_program(&chunk), expected);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);

    assert_eq!(result, crate::vm::InterpretResult::Ok);
    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "1\n3\n4");
    }
}

#[test]
fn test_c_style_for_bytecode() {
    use crate::vm::VirtualMachine;

    let program = r#"
    for (var i = 0; i < 3; i = i + 1) {
        if (i == 1) { continue }
        print(i)
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 Constant 00 '0'
0003      | SetLocal 00
0006      | Jump 0006 -> 0016
000b      | GetLocal 00
000e      | Constant 01 '1'
0011      | Add
0012      | SetLocal 00
0015      | Pop
0016      | GetLocal 00
0019      | Constant 02 '3'
001c      | Less
001d      | JumpIfFalse 001d -> 0049
0022      | Pop
0023      3 GetLocal 00
0026      | Constant 01 '1'
0029      | Equal
002a      | JumpIfFalse 002a -> 003a
002f      | Pop
0030      | Jump 0030 -> 0044
0035      | Jump 0035 -> 003b
003a      | Pop
003b      4 Constant 03 '<native fn print>'
003e      | GetLocal 00
0041      | Call (args: 1)
0043      3 Pop
0044      2 Loop 0044 -> 000b
0049      | Pop
004a      | Pop
004b      6 Nil
004c      | Return
=== </main> ===
"#;

    assert_eq!(disassemble_program(&chunk), expected);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);

    assert_eq!(result, crate::vm::InterpretResult::Ok);
    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "0\n2");
    }
}

#[test]
fn test_for_in_bytecode() {
    use crate::vm::VirtualMachine;

    let program = r#"
    for (x in [10, 20, 30]) {
        print(x)
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 Constant 00 '10'
0003      | Constant 01 '20'
0006      | Constant 02 '30'
0009      | CreateArray (elements: 3)
000c      | GetIterator
000d      | IteratorDone 00
0010      | JumpIfFalse 0010 -> 002b
0015      | Pop
0016      | IteratorNext 00
0019      | SetLocal 02
001c      3 Constant 03 '<native fn print>'
001f      | GetLocal 02
0022      | Call (args: 1)
0024      2 Pop
0025      | Pop
0026      | Loop 0026 -> 000d
002b      | Pop
002c      | Pop
002d      | Pop
002e      5 Nil
002f      | Return
=== </main> ===
"#;

    assert_eq!(disassemble_program(&chunk), expected);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);

    assert_eq!(result, crate::vm::InterpretResult::Ok);
    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "10\n20\n30");
    }
}

#[test]
fn test_closure_capturing_loop_variable_bytecode() {
    use crate::vm::VirtualMachine;

    let program = r#"
    var fns = []
    for (var i = 0; i < 3; i = i + 1) {
        fns.push(fn() { return i })
    }
    print(fns[0]())
    print(fns[1]())
    print(fns[2]())
    "#;
    let chunk = compile_program(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 Constant 00 '<uninitialized>'
0003      | SetLocal 00
0006      | CreateArray (elements: 0)
0009      | SetLocal 00
000c      | Pop
000d      3 Constant 01 '0'
0010      | SetLocal 01
0013      | Jump 0013 -> 0023
0018      | GetLocal 01
001b      | Constant 02 '1'
001e      | Add
001f      | SetLocal 01
0022      | Pop
0023      | GetLocal 01
0026      | Constant 03 '3'
0029      | Less
002a      | JumpIfFalse 002a -> 0045
002f      | Pop
0030      4 GetLocal 00
0033      | Closure 04 '<fn anonymous>'
      |                     local 01
003a      | Invoke push (args: 1)
003e      3 Pop
003f      | CloseUpvalueInPlace
0040      | Loop 0040 -> 0018
0045      | Pop
0046      | CloseUpvalue
0047      6 Constant 06 '<native fn print>'
004a      | GetLocal 00
004d      | Constant 01 '0'
0050      | GetIndex
0051      | Call (args: 0)
0053      | Call (args: 1)
0055      5 Pop
0056      7 Constant 07 '<native fn print>'
0059      | GetLocal 00
005c      | Constant 02 '1'
005f      | GetIndex
0060      | Call (args: 0)
0062      | Call (args: 1)
0064      6 Pop
0065      8 Constant 08 '<native fn print>'
0068      | GetLocal 00
006b      | Constant 09 '2'
006e      | GetIndex
006f      | Call (args: 0)
0071      | Call (args: 1)
0073      7 Pop
0074      9 Nil
0075      | Return
=== </main> ===
=== <function_anonymous>  ===
0000      4 GetUpvalue 00
0003      | Return
0004      | Nil
0005      | Return
=== </function_anonymous> ===
"#;

    assert_eq!(disassemble_program(&chunk), expected);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);

    assert_eq!(result, crate::vm::InterpretResult::Ok);
    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "0\n1\n2");
    }
}

#[test]
fn test_closure_capturing_block_local_with_break_bytecode() {
    use crate::vm::VirtualMachine;

    let program = r#"
    var captured = nil
    var i = 0
    while (i < 3) {
        i = i + 1
        {
            val local = i * 10
            captured = fn() { return local }
            break
        }
    }
    print(captured())
    "#;
    let chunk = compile_program(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 Constant 00 '<uninitialized>'
0003      | SetLocal 00
0006      3 Constant 01 '<uninitialized>'
0009      | SetLocal 01
000c      2 Nil
000d      | SetLocal 00
0010      | Pop
0011      3 Constant 02 '0'
0014      | SetLocal 01
0017      | Pop
0018      4 GetLocal 01
001b      | Constant 03 '3'
001e      | Less
001f      | JumpIfFalse 001f -> 0051
0024      | Pop
0025      5 GetLocal 01
0028      | Constant 04 '1'
002b      | Add
002c      | SetLocal 01
002f      4 Pop
0030      7 GetLocal 01
0033      | Constant 05 '10'
0036      | Multiply
0037      | SetLocal 02
003a      8 Closure 06 '<fn anonymous>'
      |                     local 02
0041      | SetLocal 00
0044      7 Pop
0045      9 CloseUpvalue
0046      | Jump 0046 -> 0052
004b      6 CloseUpvalue
004c      4 Loop 004c -> 0018
0051      | Pop
0052     12 Constant 07 '<native fn print>'
0055      | GetLocal 00
0058      | Call (args: 0)
005a      | Call (args: 1)
005c     11 Pop
005d     13 Nil
005e      | Return
=== </main> ===
=== <function_anonymous>  ===
0000      8 GetUpvalue 00
0003      | Return
0004      | Nil
0005      | Return
=== </function_anonymous> ===
"#;

    assert_eq!(disassemble_program(&chunk), expected);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);

    assert_eq!(result, crate::vm::InterpretResult::Ok);
    #[cfg(any(test, debug_assertions))]
    {
        assert_eq!(vm.get_output(), "10");
    }
}

#[test]
fn implicit_return_uses_closing_brace_line() {
    use crate::common::Value;

    let program = "fn f(x) {\n    print(x)\n}\n";
    let chunk = compile_program(program).unwrap();

    let function_chunk = chunk
        .constants
        .values
        .iter()
        .find_map(|value| match value {
            Value::Function(function) => Some(&function.chunk),
            _ => None,
        })
        .expect("expected the function constant compiled from `fn f`");

    // The trailing Return has no operand, so it sits at the last offset.
    let return_offset = function_chunk.instruction_count() - 1;
    let line = function_chunk.get_line_info(return_offset).unwrap().line;

    assert_eq!(line, 3);
}
