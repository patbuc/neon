use crate::common::opcodes::OpCode;
use crate::common::{Chunk, Value};
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;
use std::rc::Rc;

fn disassemble_program(chunk: &Chunk) -> String {
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
            | OpCode::SetField
            | OpCode::AddConstant
            | OpCode::SubtractConstant
            | OpCode::GreaterConstant
            | OpCode::GreaterEqualConstant
            | OpCode::LessConstant
            | OpCode::LessEqualConstant => {
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
            | OpCode::AddConstant
            | OpCode::SubtractConstant
            | OpCode::GreaterConstant
            | OpCode::GreaterEqualConstant
            | OpCode::LessConstant
            | OpCode::LessEqualConstant => {
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
            | OpCode::AddConstant
            | OpCode::SubtractConstant
            | OpCode::GreaterConstant
            | OpCode::GreaterEqualConstant
            | OpCode::LessConstant
            | OpCode::LessEqualConstant => {
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

#[test]
fn test_function_too_many_locals() {
    // 65,536 distinct val declarations in one function overflow the u16
    // local-slot index.
    let mut body = String::new();
    for i in 0..65536 {
        body.push_str(&format!("val v{i} = 0\n"));
    }
    let program = format!("fn f() {{\n{}\n}}\nf()\n", body);

    let err = compile_program(&program).unwrap_err();
    assert!(err.contains("too many locals"));
    assert!(err.contains("65535"));
}

#[test]
fn test_function_locals_at_limit_compiles() {
    // 65,535 distinct locals is exactly the u16 slot limit.
    let mut body = String::new();
    for i in 0..65535 {
        body.push_str(&format!("val v{i} = 0\n"));
    }
    let program = format!("fn f() {{\n{}\n}}\nf()\n", body);

    assert!(compile_program(&program).is_ok());
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

fn count_ints(chunk: &Chunk, n: i64) -> usize {
    use crate::common::Value;
    chunk
        .constants
        .values
        .iter()
        .filter(|value| matches!(value, Value::Int(v) if *v == n))
        .count()
}

#[test]
fn test_repeated_string_literal_dedups() {
    let program = "print(\"hi\")\n".repeat(10);
    let chunk = compile_program(&program).unwrap();
    assert_eq!(count_strings(&chunk, "hi"), 1);
}

fn assert_no_string_constant_anywhere(chunk: &Chunk, s: &str) {
    assert_eq!(
        count_strings(chunk, s),
        0,
        "expected no {:?} string constant in {:?}",
        s,
        chunk.name
    );
    for constant in &chunk.constants.values {
        if let Value::Function(function) = constant {
            assert_no_string_constant_anywhere(&function.chunk, s);
        }
    }
}

#[test]
fn test_field_and_method_access_do_not_use_the_constant_pool() {
    let program = r#"
    struct P { value }
    impl P {
        fn m(self) { return self.value }
    }
    fn make() {
        val p = P(1)
        p.value = 2
        p.m()
        return p.value
    }
    make()
    "#;
    let chunk = compile_program(program).unwrap();
    assert_no_string_constant_anywhere(&chunk, "value");
    assert_no_string_constant_anywhere(&chunk, "m");
    assert_no_string_constant_anywhere(&chunk, "P");
}

#[test]
fn test_local_field_read_emits_get_local_field() {
    let program = r#"
    struct P { value }
    fn get(p) {
        return p.value
    }
    get(P(1))
    "#;
    let chunk = compile_program(program).unwrap();
    let disassembly = disassemble_program(&chunk);
    assert!(disassembly.contains("GetLocalField"));
    assert!(!disassembly.contains("GetField"));
}

#[test]
fn test_global_field_read_still_emits_get_field() {
    let program = r#"
    struct P { value }
    val p = P(1)
    fn get() {
        return p.value
    }
    get()
    "#;
    let chunk = compile_program(program).unwrap();
    let disassembly = disassemble_program(&chunk);
    assert!(disassembly.contains("GetField"));
    assert!(!disassembly.contains("GetLocalField"));
}

#[test]
fn test_upvalue_field_read_still_emits_get_field() {
    let program = r#"
    struct P { value }
    fn make() {
        val p = P(1)
        return fn() { return p.value }
    }
    make()()
    "#;
    let chunk = compile_program(program).unwrap();
    let disassembly = disassemble_program(&chunk);
    assert!(disassembly.contains("GetField"));
    assert!(!disassembly.contains("GetLocalField"));
}

#[test]
fn test_non_local_object_field_read_still_emits_get_field() {
    let program = r#"
    struct P { value }
    fn make() { return P(1) }
    fn get() {
        return make().value
    }
    get()
    "#;
    let chunk = compile_program(program).unwrap();
    let disassembly = disassemble_program(&chunk);
    assert!(disassembly.contains("GetField"));
    assert!(!disassembly.contains("GetLocalField"));
}

#[test]
fn test_checked_local_field_read_still_emits_get_field() {
    // `a` is read before its declaration runs, so it needs CheckInitialized.
    let program = r#"
    fn get() {
        print(a.value)
        fn a() { return 1 }
    }
    get()
    "#;
    let chunk = compile_program(program).unwrap();
    let disassembly = disassemble_program(&chunk);
    assert!(disassembly.contains("CheckInitialized"));
    assert!(disassembly.contains("GetField"));
    assert!(!disassembly.contains("GetLocalField"));
}

#[test]
fn test_local_assignment_statement_emits_store_local() {
    let program = r#"
    fn f() {
        var x = 1
        x = 2
        return x
    }
    f()
    "#;
    let chunk = compile_program(program).unwrap();
    let disassembly = disassemble_program(&chunk);
    assert!(disassembly.contains("StoreLocal"));
}

#[test]
fn test_local_assignment_expression_still_emits_set_local() {
    let program = r#"
    fn f() {
        var x = 1
        print(x = 2)
    }
    f()
    "#;
    let chunk = compile_program(program).unwrap();
    let disassembly = disassemble_program(&chunk);
    assert!(disassembly.contains("SetLocal"));
    assert!(!disassembly.contains("StoreLocal"));
}

#[test]
fn test_global_assignment_statement_still_emits_set_global_and_pop() {
    let program = r#"
    var x = 1
    fn set() {
        x = 2
    }
    set()
    "#;
    let chunk = compile_program(program).unwrap();
    let disassembly = disassemble_program(&chunk);
    assert!(disassembly.contains("SetGlobal"));
    assert!(!disassembly.contains("StoreLocal"));
}

#[test]
fn test_val_local_field_store_emits_store_local_field() {
    let program = r#"
    struct P { value }
    fn set() {
        val p = P(1)
        p.value = 2
        return p.value
    }
    set()
    "#;
    let chunk = compile_program(program).unwrap();
    let disassembly = disassemble_program(&chunk);
    assert!(disassembly.contains("StoreLocalField"));
    assert!(!disassembly.contains("StoreField"));
}

#[test]
fn test_var_local_field_store_still_emits_store_field() {
    let program = r#"
    struct P { value }
    fn set() {
        var p = P(1)
        p.value = 2
        return p.value
    }
    set()
    "#;
    let chunk = compile_program(program).unwrap();
    let disassembly = disassemble_program(&chunk);
    assert!(disassembly.contains("StoreField"));
    assert!(!disassembly.contains("StoreLocalField"));
}

#[test]
fn test_checked_local_field_store_still_emits_store_field() {
    // `a` is written before its declaration runs, so it needs CheckInitialized
    // and can't be fused into StoreLocalField.
    let program = r#"
    fn get() {
        a.value = 1
        fn a() { return 1 }
    }
    get()
    "#;
    let chunk = compile_program(program).unwrap();
    let disassembly = disassemble_program(&chunk);
    assert!(disassembly.contains("CheckInitialized"));
    assert!(disassembly.contains("StoreField"));
    assert!(!disassembly.contains("StoreLocalField"));
}

#[test]
fn test_field_assignment_expression_still_emits_set_field() {
    let program = r#"
    struct P { value }
    fn set() {
        val p = P(1)
        print(p.value = 2)
    }
    set()
    "#;
    let chunk = compile_program(program).unwrap();
    let disassembly = disassemble_program(&chunk);
    assert!(disassembly.contains("SetField"));
    assert!(!disassembly.contains("StoreField"));
    assert!(!disassembly.contains("StoreLocalField"));
}

#[test]
fn test_repeated_number_literal_dedups() {
    let program = "1\n".repeat(10);
    let chunk = compile_program(&program).unwrap();
    assert_eq!(count_ints(&chunk, 1), 1);
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
            | OpCode::Greater
            | OpCode::GreaterEqual
            | OpCode::LessEqual
            | OpCode::Pop
            | OpCode::Add
            | OpCode::Subtract
            | OpCode::Multiply
            | OpCode::Less
            | OpCode::CloseUpvalue
            | OpCode::Dup
            | OpCode::Dup2
            | OpCode::GetIndex
            | OpCode::SetIndex => 0,
            OpCode::Call => 1,
            OpCode::Invoke => 3,
            OpCode::CreateArray => 2,
            OpCode::Constant
            | OpCode::SetLocal
            | OpCode::StoreLocal
            | OpCode::GetLocal
            | OpCode::GetGlobal
            | OpCode::SetGlobal
            | OpCode::GetBuiltin
            | OpCode::GetField
            | OpCode::SetField
            | OpCode::StoreField
            | OpCode::GetUpvalue
            | OpCode::SetUpvalue
            | OpCode::AddConstant
            | OpCode::SubtractConstant
            | OpCode::GreaterConstant
            | OpCode::GreaterEqualConstant
            | OpCode::LessConstant
            | OpCode::LessEqualConstant => 2,
            OpCode::JumpIfFalse | OpCode::Jump | OpCode::Loop => 4,
            OpCode::GetLocalField | OpCode::StoreLocalField => 4,
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
            OpCode::Constant,
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
fn test_number_literal_right_operand_fuses_into_constant_opcode() {
    let program = "val a = 1\nval b = a - 1\nval c = a <= 1\nval d = a >= 1\nval e = a < 1\nval f = a > 1\nval g = a + 1\n";
    let chunk = compile_program(program).unwrap();

    let ops = op_codes(&chunk);
    let fused = [
        OpCode::SubtractConstant,
        OpCode::LessEqualConstant,
        OpCode::GreaterEqualConstant,
        OpCode::LessConstant,
        OpCode::GreaterConstant,
        OpCode::AddConstant,
    ];
    for op in fused {
        assert!(ops.contains(&op), "expected {:?} in {:?}", op, ops);
    }
    for op in [
        OpCode::Subtract,
        OpCode::LessEqual,
        OpCode::GreaterEqual,
        OpCode::Less,
        OpCode::Greater,
    ] {
        assert!(!ops.contains(&op), "expected no {:?} in {:?}", op, ops);
    }
}

#[test]
fn test_number_literal_left_operand_keeps_generic_opcode() {
    let program = "val a = 1\nval b = 1 - a\n";
    let chunk = compile_program(program).unwrap();

    let ops = op_codes(&chunk);
    assert!(ops.contains(&OpCode::Subtract));
    assert!(!ops.contains(&OpCode::SubtractConstant));
}

#[test]
fn test_non_number_literal_right_operand_keeps_generic_opcode() {
    let program = "val a = \"x\"\nval b = a + \"y\"\n";
    let chunk = compile_program(program).unwrap();

    let ops = op_codes(&chunk);
    assert!(ops.contains(&OpCode::Add));
    assert!(!ops.contains(&OpCode::AddConstant));
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
0003      | Constant 01 '0'
0006      | SetLocal 00
0009      | Pop
000a      3 GetLocal 00
000d      | LessConstant 02 '10'
0010      | JumpIfFalse 0010 -> 0053
0015      | Pop
0016      4 GetLocal 00
0019      | AddConstant 03 '1'
001c      | StoreLocal 00
001f      5 GetLocal 00
0022      | Constant 04 '2'
0025      | Equal
0026      | JumpIfFalse 0026 -> 0031
002b      | Pop
002c      | Jump 002c -> 004e
0031      | Pop
0032      6 GetLocal 00
0035      | Constant 05 '5'
0038      | Equal
0039      | JumpIfFalse 0039 -> 0044
003e      | Pop
003f      | Jump 003f -> 0054
0044      | Pop
0045      7 Constant 06 '<native fn print>'
0048      | GetLocal 00
004b      | Call (args: 1)
004d      6 Pop
004e      3 Loop 004e -> 000a
0053      | Pop
0054      9 Nil
0055      | Return
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
fn test_for_in_bytecode() {
    use crate::vm::VirtualMachine;

    let program = r#"
    for x in [10, 20, 30] {
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
0010      | JumpIfFalse 0010 -> 0028
0015      | Pop
0016      | IteratorNext 00
0019      3 Constant 03 '<native fn print>'
001c      | GetLocal 02
001f      | Call (args: 1)
0021      2 Pop
0022      | Pop
0023      | Loop 0023 -> 000d
0028      | Pop
0029      | Pop
002a      | Pop
002b      5 Nil
002c      | Return
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
0003      3 Constant 01 '<uninitialized>'
0006      2 Nil
0007      | SetLocal 00
000a      | Pop
000b      3 Constant 02 '0'
000e      | SetLocal 01
0011      | Pop
0012      4 GetLocal 01
0015      | LessConstant 03 '3'
0018      | JumpIfFalse 0018 -> 0044
001d      | Pop
001e      5 GetLocal 01
0021      | AddConstant 04 '1'
0024      | StoreLocal 01
0027      7 GetLocal 01
002a      | Constant 05 '10'
002d      | Multiply
002e      8 Closure 06 '<fn anonymous>'
      |                     local 02
0035      | StoreLocal 00
0038      9 CloseUpvalue
0039      | Jump 0039 -> 0045
003e      6 CloseUpvalue
003f      4 Loop 003f -> 0012
0044      | Pop
0045     12 Constant 07 '<native fn print>'
0048      | GetLocal 00
004b      | Call (args: 0)
004d      | Call (args: 1)
004f     11 Pop
0050     13 Nil
0051      | Return
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

#[test]
fn test_if_break_skips_jump() {
    let program = r#"
    while (true) {
        if (true) { break }
        print(1)
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 True
0001      | JumpIfFalse 0001 -> 0022
0006      | Pop
0007      3 True
0008      | JumpIfFalse 0008 -> 0013
000d      | Pop
000e      | Jump 000e -> 0023
0013      | Pop
0014      4 Constant 00 '<native fn print>'
0017      | Constant 01 '1'
001a      | Call (args: 1)
001c      3 Pop
001d      2 Loop 001d -> 0000
0022      | Pop
0023      6 Nil
0024      | Return
=== </main> ===
"#;

    assert_eq!(disassemble_program(&chunk), expected);
}

#[test]
fn test_if_continue_skips_jump() {
    let program = r#"
    while (true) {
        if (true) { continue }
        print(1)
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 True
0001      | JumpIfFalse 0001 -> 0022
0006      | Pop
0007      3 True
0008      | JumpIfFalse 0008 -> 0013
000d      | Pop
000e      | Jump 000e -> 001d
0013      | Pop
0014      4 Constant 00 '<native fn print>'
0017      | Constant 01 '1'
001a      | Call (args: 1)
001c      3 Pop
001d      2 Loop 001d -> 0000
0022      | Pop
0023      6 Nil
0024      | Return
=== </main> ===
"#;

    assert_eq!(disassemble_program(&chunk), expected);
}

#[test]
fn test_if_return_skips_jump() {
    let program = r#"
    fn f() {
        if (true) { return 1 }
        print(1)
        val y = 1
    }
    f()
    "#;
    let chunk = compile_program(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 Nil
0001      | Closure 00 '<fn f>'
0005      | SetLocal 00
0008      | Pop
0009      7 GetLocal 00
000c      | Call (args: 0)
000e      6 Pop
000f      8 Nil
0010      | Return
=== </main> ===
=== <function_f>  ===
0000      3 True
0001      | JumpIfFalse 0001 -> 000b
0006      | Pop
0007      | Constant 00 '1'
000a      | Return
000b      | Pop
000c      4 Constant 01 '<native fn print>'
000f      | Constant 00 '1'
0012      | Call (args: 1)
0014      3 Pop
0015      5 Constant 00 '1'
0018      6 Nil
0019      | Return
=== </function_f> ===
"#;

    assert_eq!(disassemble_program(&chunk), expected);
}

#[test]
fn test_if_fallthrough_emits_jump() {
    let program = r#"
    if (true) {
        print(1)
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 True
0001      | JumpIfFalse 0001 -> 0015
0006      | Pop
0007      3 Constant 00 '<native fn print>'
000a      | Constant 01 '1'
000d      | Call (args: 1)
000f      2 Pop
0010      | Jump 0010 -> 0016
0015      | Pop
0016      5 Nil
0017      | Return
=== </main> ===
"#;

    assert_eq!(disassemble_program(&chunk), expected);
}

#[test]
fn test_if_else_fallthrough_emits_jump() {
    let program = r#"
    if (true) {
        print(1)
    } else {
        print(2)
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 True
0001      | JumpIfFalse 0001 -> 0015
0006      | Pop
0007      3 Constant 00 '<native fn print>'
000a      | Constant 01 '1'
000d      | Call (args: 1)
000f      2 Pop
0010      | Jump 0010 -> 001f
0015      | Pop
0016      5 Constant 02 '<native fn print>'
0019      | Constant 03 '2'
001c      | Call (args: 1)
001e      4 Pop
001f      7 Nil
0020      | Return
=== </main> ===
"#;

    assert_eq!(disassemble_program(&chunk), expected);
}

#[test]
fn test_if_block_ending_in_break_skips_jump() {
    let program = r#"
    while (true) {
        if (true) {
            print(1)
            break
        }
        print(2)
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 True
0001      | JumpIfFalse 0001 -> 002b
0006      | Pop
0007      3 True
0008      | JumpIfFalse 0008 -> 001c
000d      | Pop
000e      4 Constant 00 '<native fn print>'
0011      | Constant 01 '1'
0014      | Call (args: 1)
0016      3 Pop
0017      5 Jump 0017 -> 002c
001c      3 Pop
001d      7 Constant 02 '<native fn print>'
0020      | Constant 03 '2'
0023      | Call (args: 1)
0025      6 Pop
0026      2 Loop 0026 -> 0000
002b      | Pop
002c      9 Nil
002d      | Return
=== </main> ===
"#;

    assert_eq!(disassemble_program(&chunk), expected);
}

#[test]
fn test_if_break_with_else_skips_jump() {
    let program = r#"
    while (true) {
        if (true) { break } else { print(2) }
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 True
0001      | JumpIfFalse 0001 -> 0022
0006      | Pop
0007      3 True
0008      | JumpIfFalse 0008 -> 0013
000d      | Pop
000e      | Jump 000e -> 0023
0013      | Pop
0014      | Constant 00 '<native fn print>'
0017      | Constant 01 '2'
001a      | Call (args: 1)
001c      | Pop
001d      2 Loop 001d -> 0000
0022      | Pop
0023      5 Nil
0024      | Return
=== </main> ===
"#;

    assert_eq!(disassemble_program(&chunk), expected);
}

#[test]
fn test_if_nested_exit_emits_jump() {
    let program = r#"
    while (true) {
        if (true) { if (true) { break } }
    }
    "#;
    let chunk = compile_program(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 True
0001      | JumpIfFalse 0001 -> 0026
0006      | Pop
0007      3 True
0008      | JumpIfFalse 0008 -> 0020
000d      | Pop
000e      | True
000f      | JumpIfFalse 000f -> 001a
0014      | Pop
0015      | Jump 0015 -> 0027
001a      | Pop
001b      | Jump 001b -> 0021
0020      | Pop
0021      2 Loop 0021 -> 0000
0026      | Pop
0027      5 Nil
0028      | Return
=== </main> ===
"#;

    assert_eq!(disassemble_program(&chunk), expected);
}

#[test]
fn nested_function_chunk_shares_symbol_table_with_script() {
    let chunk = compile_program("fn f() { return 1 }\nf()\n").unwrap();
    let function_chunk = chunk
        .constants
        .values
        .iter()
        .find_map(|v| match v {
            Value::Function(function) => Some(function.chunk.clone()),
            _ => None,
        })
        .expect("expected f's Function constant");

    assert!(Rc::ptr_eq(&chunk.symbols, &function_chunk.symbols));
}

#[test]
fn test_val_and_var_locals_skip_self_copy_setlocal() {
    let program = "fn f() {\n    val a = 1\n    var b = 2\n    return a + b\n}\nf()\n";
    let chunk = compile_program(program).unwrap();

    let expected = r#"=== <main>  ===
0000      1 Nil
0001      | Closure 00 '<fn f>'
0005      | SetLocal 00
0008      | Pop
0009      6 GetLocal 00
000c      | Call (args: 0)
000e      5 Pop
000f      7 Nil
0010      | Return
=== </main> ===
=== <function_f>  ===
0000      2 Constant 00 '1'
0003      3 Constant 01 '2'
0006      4 GetLocal 00
0009      | GetLocal 01
000c      | Add
000d      | Return
000e      5 Nil
000f      | Return
=== </function_f> ===
"#;

    assert_eq!(disassemble_program(&chunk), expected);
}

#[test]
fn test_compound_assignment_matches_hand_desugared_bytecode() {
    let compound = r#"
fn f() {
    var a = 10
    a += 2
    var b = 10
    b -= 2
    var c = 10
    c *= 2
    var d = 10
    d /= 2
    var e = 10
    e %= 2
    var g = 10
    g **= 2
    print(a += 2)
}
f()
var x = 10
x += 2
var y = 10
y -= 2
var z = 10
z *= 2
var w = 10
w /= 2
var v = 10
v %= 2
var u = 10
u **= 2
print(x += 2)

fn make_counter() {
    var n = 0
    fn increment() {
        n += 1
    }
    increment()
    increment()
    return n
}
print(make_counter())

var gcount = 0
fn bump() {
    gcount += 3
}
bump()
print(gcount)

var p = 10
var q = 3
p += q
print(p)
"#;

    let desugared = r#"
fn f() {
    var a = 10
    a = a + 2
    var b = 10
    b = b - 2
    var c = 10
    c = c * 2
    var d = 10
    d = d / 2
    var e = 10
    e = e % 2
    var g = 10
    g = g ** 2
    print(a = a + 2)
}
f()
var x = 10
x = x + 2
var y = 10
y = y - 2
var z = 10
z = z * 2
var w = 10
w = w / 2
var v = 10
v = v % 2
var u = 10
u = u ** 2
print(x = x + 2)

fn make_counter() {
    var n = 0
    fn increment() {
        n = n + 1
    }
    increment()
    increment()
    return n
}
print(make_counter())

var gcount = 0
fn bump() {
    gcount = gcount + 3
}
bump()
print(gcount)

var p = 10
var q = 3
p = p + q
print(p)
"#;

    let compound_chunk = compile_program(compound).unwrap();
    let desugared_chunk = compile_program(desugared).unwrap();

    assert_eq!(
        disassemble_program(&compound_chunk),
        disassemble_program(&desugared_chunk)
    );
}

#[test]
fn test_tail_expression_returns_directly() {
    let program = "fn sq(x) {\n    x * x\n}\nprint(sq(3))\n";
    let chunk = compile_program(program).unwrap();

    let Value::Function(function) = &chunk.constants.values[0] else {
        panic!("expected sq's chunk to be the first constant");
    };
    let ops = op_codes(&function.chunk);

    assert_eq!(&ops[ops.len() - 2..], &[OpCode::Multiply, OpCode::Return]);
}

#[test]
fn test_field_compound_assign_emits_dup() {
    let program = r#"
    struct Box {
        n
    }
    fn f() {
        val o = Box(0)
        o.n += 1
    }
    f()
    "#;
    let chunk = compile_program(program).unwrap();
    let function_chunk = chunk
        .constants
        .values
        .iter()
        .find_map(|v| match v {
            Value::Function(function) => Some(function.chunk.clone()),
            _ => None,
        })
        .expect("expected f's Function constant");
    let ops = op_codes(&function_chunk);

    assert!(ops.contains(&OpCode::Dup));
}

#[test]
fn test_index_compound_assign_emits_dup2() {
    let program = r#"
    fn f() {
        val a = [0]
        a[0] += 1
    }
    f()
    "#;
    let chunk = compile_program(program).unwrap();
    let function_chunk = chunk
        .constants
        .values
        .iter()
        .find_map(|v| match v {
            Value::Function(function) => Some(function.chunk.clone()),
            _ => None,
        })
        .expect("expected f's Function constant");
    let ops = op_codes(&function_chunk);

    assert!(ops.contains(&OpCode::Dup2));
}
