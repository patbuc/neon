use super::helpers::{assert_compile_error, compile, disassemble, run};
use crate::common::opcodes::OpCode;
use crate::common::{Chunk, Value};
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::parser::Parser;
use crate::compiler::semantic::SemanticAnalyzer;
use std::rc::Rc;

#[test]
fn test_end_to_end_execution() {
    let program = r#"
    val x = 10
    val y = 20
    val sum = x + y
    print(sum)
    "#;
    assert_eq!(run(program), "30");
}

#[test]
fn test_end_to_end_function() {
    let program = r#"
    fn add(a, b) {
        return a + b
    }
    val result = add(15, 27)
    print(result)
    "#;
    assert_eq!(run(program), "42");
}

#[test]
fn test_end_to_end_forward_reference() {
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
    assert_eq!(run(program), "99");
}

#[test]
fn test_else_if_bytecode_simple() {
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
    let chunk = compile(program).unwrap();

    // Verify that bytecode contains the expected jump instructions
    // Pattern should be:
    // 1. First condition (x == 1)
    // 2. PopJumpIfFalse (skip first then-branch)
    // 3. First then-branch code
    // 4. Jump (skip else-if and else)
    // 5. Second condition (x == 2) - this is the else-if
    // 6. PopJumpIfFalse (skip second then-branch)
    // 7. Second then-branch code
    // 8. Jump (skip else)
    // 9. Else-branch code

    let ops = op_codes(&chunk);
    let jump_if_false_count = ops
        .iter()
        .filter(|op| **op == OpCode::PopJumpIfFalse)
        .count();
    let jump_count = ops.iter().filter(|op| **op == OpCode::Jump).count();

    // We should have 2 PopJumpIfFalse (one for each condition)
    assert_eq!(
        jump_if_false_count, 2,
        "Expected 2 PopJumpIfFalse instructions for if and else-if conditions"
    );

    // We should have 2 Jump instructions (one after each then-branch)
    assert_eq!(
        jump_count, 2,
        "Expected 2 Jump instructions to skip remaining branches"
    );
}

#[test]
fn test_else_if_bytecode_multiple_branches() {
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
    let chunk = compile(program).unwrap();

    let ops = op_codes(&chunk);
    let less_jump_count = ops
        .iter()
        .filter(|op| **op == OpCode::LessConstantJumpIfFalse)
        .count();
    let jump_count = ops.iter().filter(|op| **op == OpCode::Jump).count();

    // We should have 4 LessConstantJumpIfFalse (one for each condition)
    assert_eq!(
        less_jump_count, 4,
        "Expected 4 LessConstantJumpIfFalse instructions for all conditions"
    );

    // We should have 4 Jump instructions (one after each then-branch)
    assert_eq!(
        jump_count, 4,
        "Expected 4 Jump instructions to skip remaining branches"
    );
}

#[test]
fn test_else_if_bytecode_without_final_else() {
    // Test else-if chain without final else
    let program = r#"
    val x = 7
    if (x == 5) {
        print(5)
    } else if (x == 7) {
        print(7)
    }
    "#;
    let chunk = compile(program).unwrap();

    let ops = op_codes(&chunk);
    let jump_if_false_count = ops
        .iter()
        .filter(|op| **op == OpCode::PopJumpIfFalse)
        .count();
    let jump_count = ops.iter().filter(|op| **op == OpCode::Jump).count();

    // We should have 2 PopJumpIfFalse (one for each condition)
    assert_eq!(
        jump_if_false_count, 2,
        "Expected 2 PopJumpIfFalse instructions"
    );

    // Only the first then-branch needs a Jump: the else-if has no else to skip
    assert_eq!(jump_count, 1, "Expected 1 Jump instruction");
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
    let chunk = compile(program).unwrap();

    let jumps: Vec<(usize, usize)> = instructions(&chunk)
        .into_iter()
        .filter(|(_, op)| matches!(op, OpCode::PopJumpIfFalse | OpCode::Jump))
        .map(|(pos, _)| (pos, pos + 5 + chunk.read_u32(pos + 1) as usize))
        .collect();
    assert_eq!(jumps.len(), 4);

    // Every jump must land on an instruction start or the end of the chunk
    let starts: Vec<usize> = instructions(&chunk)
        .into_iter()
        .map(|(pos, _)| pos)
        .collect();
    for (pos, target) in &jumps {
        assert!(
            starts.contains(target) || *target == chunk.instruction_count(),
            "Jump at position {} targets {}, not an instruction start",
            pos,
            target
        );
    }
}

#[test]
fn test_else_if_end_to_end_execution() {
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
    assert_eq!(run(program), "20");
}

// =============================================================================
// Array Literal Tests
// =============================================================================

#[test]
fn test_array_literal_too_large() {
    // Generate an array literal with more than 65535 elements
    let mut elements = Vec::new();
    for i in 0..70000 {
        elements.push(i.to_string());
    }
    let array_literal = format!("[{}]", elements.join(", "));
    let program = format!("val arr = {}", array_literal);

    let errors = assert_compile_error(&program, "array literal too large");
    assert!(errors[0].message.contains("70000"));
    assert!(errors[0].message.contains("65535"));
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

    let errors = assert_compile_error(&program, "constants");
    assert!(errors[0].message.contains("65535"));
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

    assert!(compile(&program).is_ok());
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

    let errors = compile(&program).unwrap_err();
    let count = errors
        .iter()
        .filter(|e| e.message.contains("too many constants"))
        .count();
    assert_eq!(count, 1, "{:#?}", errors);
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

    let errors = compile(&program).unwrap_err();
    let count = errors
        .iter()
        .filter(|e| e.message.contains("too many constants"))
        .count();
    assert_eq!(count, 2, "{:#?}", errors);
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

    let errors = assert_compile_error(&program, "too many locals");
    assert!(errors[0].message.contains("65535"));
}

#[test]
fn test_function_locals_at_limit_compiles() {
    // 65,535 distinct locals is exactly the u16 slot limit.
    let mut body = String::new();
    for i in 0..65535 {
        body.push_str(&format!("val v{i} = 0\n"));
    }
    let program = format!("fn f() {{\n{}\n}}\nf()\n", body);

    assert!(compile(&program).is_ok());
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
    let chunk = compile(&program).unwrap();
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
    let chunk = compile(program).unwrap();
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
    let chunk = compile(program).unwrap();
    let disassembly = disassemble(&chunk);
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
    let chunk = compile(program).unwrap();
    let disassembly = disassemble(&chunk);
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
    let chunk = compile(program).unwrap();
    let disassembly = disassemble(&chunk);
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
    let chunk = compile(program).unwrap();
    let disassembly = disassemble(&chunk);
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
    let chunk = compile(program).unwrap();
    let disassembly = disassemble(&chunk);
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
    let chunk = compile(program).unwrap();
    let disassembly = disassemble(&chunk);
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
    let chunk = compile(program).unwrap();
    let disassembly = disassemble(&chunk);
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
    let chunk = compile(program).unwrap();
    let disassembly = disassemble(&chunk);
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
    let chunk = compile(program).unwrap();
    let disassembly = disassemble(&chunk);
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
    let chunk = compile(program).unwrap();
    let disassembly = disassemble(&chunk);
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
    let chunk = compile(program).unwrap();
    let disassembly = disassemble(&chunk);
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
    let chunk = compile(program).unwrap();
    let disassembly = disassemble(&chunk);
    assert!(disassembly.contains("SetField"));
    assert!(!disassembly.contains("StoreField"));
    assert!(!disassembly.contains("StoreLocalField"));
}

#[test]
fn test_repeated_number_literal_dedups() {
    let program = "1\n".repeat(10);
    let chunk = compile(&program).unwrap();
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

    let errors = assert_compile_error(&program, "map literal too large");
    assert!(errors[0].message.contains("70000"));
    assert!(errors[0].message.contains("65535"));
}

#[test]
fn test_set_literal_too_large() {
    // Generate a set literal with more than 65535 elements
    let elements: Vec<String> = (0..70000).map(|i| i.to_string()).collect();
    let program = format!("val s = #{{{}}}", elements.join(", "));

    let errors = assert_compile_error(&program, "set literal too large");
    assert!(errors[0].message.contains("70000"));
    assert!(errors[0].message.contains("65535"));
}

/// Walks a chunk's bytecode, stepping over each instruction's operand bytes,
/// and returns each instruction's offset and opcode in order. Panics by name
/// on an opcode whose operand width it does not know rather than guessing.
fn instructions(chunk: &Chunk) -> Vec<(usize, OpCode)> {
    let mut ops = Vec::new();
    let mut offset = 0;
    while offset < chunk.instruction_count() {
        let op = OpCode::from_u8(chunk.read_u8(offset)).unwrap();
        let operand_bytes = match op {
            OpCode::Return
            | OpCode::Nil
            | OpCode::Equal
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
            OpCode::JumpIfFalse
            | OpCode::PopJumpIfFalse
            | OpCode::GreaterJumpIfFalse
            | OpCode::GreaterEqualJumpIfFalse
            | OpCode::LessJumpIfFalse
            | OpCode::LessEqualJumpIfFalse
            | OpCode::JumpIfNotNil
            | OpCode::JumpIfNil
            | OpCode::Jump
            | OpCode::Loop => 4,
            OpCode::GetLocalField | OpCode::StoreLocalField => 4,
            OpCode::GreaterConstantJumpIfFalse
            | OpCode::GreaterEqualConstantJumpIfFalse
            | OpCode::LessConstantJumpIfFalse
            | OpCode::LessEqualConstantJumpIfFalse => 6,
            OpCode::Closure => {
                // 2-byte constant index, then a 1-byte upvalue count and
                // that many (is_local, index) pairs (1 + 2 bytes each).
                let upvalue_count = chunk.read_u8(offset + 3) as usize;
                3 + upvalue_count * 3
            }
            _ => panic!("instructions: unhandled opcode {op:?}, add its operand width"),
        };
        ops.push((offset, op));
        offset += 1 + operand_bytes;
    }
    ops
}

fn op_codes(chunk: &Chunk) -> Vec<OpCode> {
    instructions(chunk).into_iter().map(|(_, op)| op).collect()
}

#[test]
fn and_or_keep_jump_if_false() {
    let program = "val a = 1\nval b = 2\nval c = a && b\nval d = a || b\n";
    let ops = op_codes(&compile(program).unwrap());

    assert_eq!(
        2,
        ops.iter().filter(|op| **op == OpCode::JumpIfFalse).count()
    );
    assert!(!ops.contains(&OpCode::PopJumpIfFalse), "{ops:?}");
}

#[test]
fn test_two_operand_fused_jump_bytecode() {
    let program = "val a = 1\nval b = 2\nif a < b { print(1) }\n";
    let chunk = compile(program).unwrap();

    let expected = r#"=== <main>  ===
0000      1 Constant 00 '<uninitialized>'
0003      2 Constant 01 '<uninitialized>'
0006      1 Constant 02 '1'
0009      | SetLocal 00
000c      | Pop
000d      2 Constant 03 '2'
0010      | SetLocal 01
0013      | Pop
0014      3 GetLocal 00
0017      | GetLocal 01
001a      | LessJumpIfFalse 001a -> 0028
001f      | Constant 04 '<native fn print>'
0022      | Constant 02 '1'
0025      | Call (args: 1)
0027      | Pop
0028      4 Nil
0029      | Return
=== </main> ===
"#;

    assert_eq!(disassemble(&chunk), expected);
}

#[test]
fn test_comparison_conditions_fuse_into_jumps() {
    let program = "val a = 1\nval b = 2\nif a < b { print(1) }\nwhile a >= 2.5 { print(2) }\n";
    let ops = op_codes(&compile(program).unwrap());

    assert!(ops.contains(&OpCode::LessJumpIfFalse));
    assert!(ops.contains(&OpCode::GreaterEqualConstantJumpIfFalse));
    for op in [
        OpCode::Less,
        OpCode::GreaterEqualConstant,
        OpCode::PopJumpIfFalse,
    ] {
        assert!(!ops.contains(&op), "unexpected {op:?} in {ops:?}");
    }
}

#[test]
fn test_greater_equal_less_equal_opcodes() {
    let program = "val a = 1\nval b = 2\nprint(a >= b)\nprint(a <= b)\n";
    let chunk = compile(program).unwrap();

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
    let chunk = compile(program).unwrap();

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
    let chunk = compile(program).unwrap();

    let ops = op_codes(&chunk);
    assert!(ops.contains(&OpCode::Subtract));
    assert!(!ops.contains(&OpCode::SubtractConstant));
}

#[test]
fn test_non_number_literal_right_operand_keeps_generic_opcode() {
    let program = "val a = \"x\"\nval b = a + \"y\"\n";
    let chunk = compile(program).unwrap();

    let ops = op_codes(&chunk);
    assert!(ops.contains(&OpCode::Add));
    assert!(!ops.contains(&OpCode::AddConstant));
}

// =============================================================================
// Per-Function Loop State Tests
// =============================================================================

#[test]
fn test_top_level_fn_named_print_shadows_native() {
    let program = r#"
    fn print(x) {}
    print("native")
    "#;
    assert_eq!(run(program), "");
}

#[test]
fn test_native_call_labels() {
    use crate::common::Value;

    let program = r#"
    import "std/file"
    import "std/math"
    print(1)
    file.open("x")
    math.abs(1)
    "#;
    let chunk = compile(program).unwrap();

    let labels: Vec<String> = chunk
        .constants
        .values
        .iter()
        .filter_map(|value| match value {
            Value::NativeFunction(native) => Some(native.name.clone()),
            _ => None,
        })
        .collect();

    assert_eq!(labels, vec!["print", "open", "abs"]);
}

#[test]
fn test_method_call_loads_receiver_then_invoke() {
    let program = "val a = [1]\na.size()\n";
    let chunk = compile(program).unwrap();

    let ops = op_codes(&chunk);

    let invoke_index = ops
        .iter()
        .position(|op| *op == OpCode::Invoke)
        .expect("expected an Invoke instruction");
    assert_eq!(OpCode::GetLocal, ops[invoke_index - 1]);
}

#[test]
fn test_while_break_continue_bytecode() {
    let program = r#"
    var i = 0
    while (i < 10) {
        i = i + 1
        if (i == 2) { continue }
        if (i == 5) { break }
        print(i)
    }
    "#;
    let chunk = compile(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 Constant 00 '<uninitialized>'
0003      | Constant 01 '0'
0006      | SetLocal 00
0009      | Pop
000a      3 GetLocal 00
000d      | LessConstantJumpIfFalse 02 '10' 000d -> 004d
0014      4 GetLocal 00
0017      | AddConstant 03 '1'
001a      | StoreLocal 00
001d      5 GetLocal 00
0020      | Constant 04 '2'
0023      | Equal
0024      | PopJumpIfFalse 0024 -> 002e
0029      | Jump 0029 -> 0048
002e      6 GetLocal 00
0031      | Constant 05 '5'
0034      | Equal
0035      | PopJumpIfFalse 0035 -> 003f
003a      | Jump 003a -> 004d
003f      7 Constant 06 '<native fn print>'
0042      | GetLocal 00
0045      | Call (args: 1)
0047      6 Pop
0048      3 Loop 0048 -> 000a
004d      9 Nil
004e      | Return
=== </main> ===
"#;

    assert_eq!(disassemble(&chunk), expected);

    assert_eq!(run(program), "1\n3\n4");
}

#[test]
fn test_for_in_bytecode() {
    let program = r#"
    for x in [10, 20, 30] {
        print(x)
    }
    "#;
    let chunk = compile(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 Constant 00 '10'
0003      | Constant 01 '20'
0006      | Constant 02 '30'
0009      | CreateArray (elements: 3)
000c      | GetIterator (pairs: false)
000e      | IteratorDone 00
0011      | PopJumpIfFalse 0011 -> 0028
0016      | IteratorNext 00
0019      3 Constant 03 '<native fn print>'
001c      | GetLocal 02
001f      | Call (args: 1)
0021      2 Pop
0022      | Pop
0023      | Loop 0023 -> 000e
0028      | Pop
0029      | Pop
002a      5 Nil
002b      | Return
=== </main> ===
"#;

    assert_eq!(disassemble(&chunk), expected);

    assert_eq!(run(program), "10\n20\n30");
}

#[test]
fn test_closure_capturing_block_local_with_break_bytecode() {
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
    let chunk = compile(program).unwrap();

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
0015      | LessConstantJumpIfFalse 03 '3' 0015 -> 0042
001c      5 GetLocal 01
001f      | AddConstant 04 '1'
0022      | StoreLocal 01
0025      7 GetLocal 01
0028      | Constant 05 '10'
002b      | Multiply
002c      8 Closure 06 '<fn anonymous>'
      |                     local 02
0033      | StoreLocal 00
0036      9 CloseUpvalue
0037      | Jump 0037 -> 0042
003c      6 CloseUpvalue
003d      4 Loop 003d -> 0012
0042     12 Constant 07 '<native fn print>'
0045      | GetLocal 00
0048      | Call (args: 0)
004a      | Call (args: 1)
004c     11 Pop
004d     13 Nil
004e      | Return
=== </main> ===
=== <function_anonymous>  ===
0000      8 GetUpvalue 00
0003      | Return
0004      | Nil
0005      | Return
=== </function_anonymous> ===
"#;

    assert_eq!(disassemble(&chunk), expected);

    assert_eq!(run(program), "10");
}

#[test]
fn implicit_return_uses_closing_brace_line() {
    use crate::common::Value;

    let program = "fn f(x) {\n    print(x)\n}\n";
    let chunk = compile(program).unwrap();

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
    let chunk = compile(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 True
0001      | PopJumpIfFalse 0001 -> 001f
0006      3 True
0007      | PopJumpIfFalse 0007 -> 0011
000c      | Jump 000c -> 001f
0011      4 Constant 00 '<native fn print>'
0014      | Constant 01 '1'
0017      | Call (args: 1)
0019      3 Pop
001a      2 Loop 001a -> 0000
001f      6 Nil
0020      | Return
=== </main> ===
"#;

    assert_eq!(disassemble(&chunk), expected);
}

#[test]
fn test_if_continue_skips_jump() {
    let program = r#"
    while (true) {
        if (true) { continue }
        print(1)
    }
    "#;
    let chunk = compile(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 True
0001      | PopJumpIfFalse 0001 -> 001f
0006      3 True
0007      | PopJumpIfFalse 0007 -> 0011
000c      | Jump 000c -> 001a
0011      4 Constant 00 '<native fn print>'
0014      | Constant 01 '1'
0017      | Call (args: 1)
0019      3 Pop
001a      2 Loop 001a -> 0000
001f      6 Nil
0020      | Return
=== </main> ===
"#;

    assert_eq!(disassemble(&chunk), expected);
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
    let chunk = compile(program).unwrap();

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
0001      | PopJumpIfFalse 0001 -> 000a
0006      | Constant 00 '1'
0009      | Return
000a      4 Constant 01 '<native fn print>'
000d      | Constant 00 '1'
0010      | Call (args: 1)
0012      3 Pop
0013      5 Constant 00 '1'
0016      6 Nil
0017      | Return
=== </function_f> ===
"#;

    assert_eq!(disassemble(&chunk), expected);
}

#[test]
fn test_if_without_else_skips_jump() {
    let program = r#"
    if (true) {
        print(1)
    }
    "#;
    let chunk = compile(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 True
0001      | PopJumpIfFalse 0001 -> 000f
0006      3 Constant 00 '<native fn print>'
0009      | Constant 01 '1'
000c      | Call (args: 1)
000e      2 Pop
000f      5 Nil
0010      | Return
=== </main> ===
"#;

    assert_eq!(disassemble(&chunk), expected);
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
    let chunk = compile(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 True
0001      | PopJumpIfFalse 0001 -> 0014
0006      3 Constant 00 '<native fn print>'
0009      | Constant 01 '1'
000c      | Call (args: 1)
000e      2 Pop
000f      | Jump 000f -> 001d
0014      5 Constant 02 '<native fn print>'
0017      | Constant 03 '2'
001a      | Call (args: 1)
001c      4 Pop
001d      7 Nil
001e      | Return
=== </main> ===
"#;

    assert_eq!(disassemble(&chunk), expected);
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
    let chunk = compile(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 True
0001      | PopJumpIfFalse 0001 -> 0028
0006      3 True
0007      | PopJumpIfFalse 0007 -> 001a
000c      4 Constant 00 '<native fn print>'
000f      | Constant 01 '1'
0012      | Call (args: 1)
0014      3 Pop
0015      5 Jump 0015 -> 0028
001a      7 Constant 02 '<native fn print>'
001d      | Constant 03 '2'
0020      | Call (args: 1)
0022      6 Pop
0023      2 Loop 0023 -> 0000
0028      9 Nil
0029      | Return
=== </main> ===
"#;

    assert_eq!(disassemble(&chunk), expected);
}

#[test]
fn test_if_break_with_else_skips_jump() {
    let program = r#"
    while (true) {
        if (true) { break } else { print(2) }
    }
    "#;
    let chunk = compile(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 True
0001      | PopJumpIfFalse 0001 -> 001f
0006      3 True
0007      | PopJumpIfFalse 0007 -> 0011
000c      | Jump 000c -> 001f
0011      | Constant 00 '<native fn print>'
0014      | Constant 01 '2'
0017      | Call (args: 1)
0019      | Pop
001a      2 Loop 001a -> 0000
001f      5 Nil
0020      | Return
=== </main> ===
"#;

    assert_eq!(disassemble(&chunk), expected);
}

#[test]
fn test_if_nested_exit_emits_jump() {
    let program = r#"
    while (true) {
        if (true) { if (true) { break } } else { print(2) }
    }
    "#;
    let chunk = compile(program).unwrap();

    let expected = r#"=== <main>  ===
0000      2 True
0001      | PopJumpIfFalse 0001 -> 002a
0006      3 True
0007      | PopJumpIfFalse 0007 -> 001c
000c      | True
000d      | PopJumpIfFalse 000d -> 0017
0012      | Jump 0012 -> 002a
0017      | Jump 0017 -> 0025
001c      | Constant 00 '<native fn print>'
001f      | Constant 01 '2'
0022      | Call (args: 1)
0024      | Pop
0025      2 Loop 0025 -> 0000
002a      5 Nil
002b      | Return
=== </main> ===
"#;

    assert_eq!(disassemble(&chunk), expected);
}

#[test]
fn nested_function_chunk_shares_symbol_table_with_script() {
    let chunk = compile("fn f() { return 1 }\nf()\n").unwrap();
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
    let chunk = compile(program).unwrap();

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

    assert_eq!(disassemble(&chunk), expected);
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

    let compound_chunk = compile(compound).unwrap();
    let desugared_chunk = compile(desugared).unwrap();

    assert_eq!(disassemble(&compound_chunk), disassemble(&desugared_chunk));
}

#[test]
fn test_tail_expression_returns_directly() {
    let program = "fn sq(x) {\n    x * x\n}\nprint(sq(3))\n";
    let chunk = compile(program).unwrap();

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
    let chunk = compile(program).unwrap();
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
    let chunk = compile(program).unwrap();
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

#[test]
fn test_nil_coalesce_emits_jump_if_not_nil() {
    let program = r#"
    val a = nil
    val b = 1
    a ?? b
    "#;
    let chunk = compile(program).unwrap();
    let ops = op_codes(&chunk);

    let jump_index = ops
        .iter()
        .position(|op| *op == OpCode::JumpIfNotNil)
        .expect("expected a JumpIfNotNil instruction");
    assert_eq!(
        &[OpCode::JumpIfNotNil, OpCode::Pop, OpCode::GetLocal],
        &ops[jump_index..jump_index + 3]
    );
}

#[test]
fn exported_declarations_are_usable_after_declaration() {
    let program = r#"
export val x = 1
export var y = 2
y = y + 5
export fn double(n) {
    return n * 2
}
export struct Point { a }
export enum Color { Red }
print(x)
print(y)
print(double(4))
print(Point(3).a)
print(Color.Red)
"#;
    assert_eq!(run(program), "1\n7\n8\n3\nColor.Red");
}

#[test]
fn an_exported_fn_is_callable_before_its_declaration() {
    let program = r#"
print(f())
export fn f() {
    return 5
}
"#;
    assert_eq!(run(program), "5");
}
