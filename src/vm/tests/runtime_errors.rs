use crate::common::opcodes::OpCode;
use crate::common::Chunk;
use crate::number;
use crate::vm::{Result, VirtualMachine};

#[test]
fn array_index_read_out_of_bounds_halts() {
    let program = r#"
        print([1, 2][5])
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn array_index_read_out_of_bounds_reports_exactly_one_error() {
    let program = r#"
        print([1, 2][5])
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    assert_eq!(1, vm.get_runtime_errors().lines().count());
    assert_eq!("", vm.get_output());
}

#[test]
fn array_index_write_out_of_bounds_halts() {
    let program = r#"
        val a = [1, 2]
        a[5] = 3
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn get_field_missing_halts() {
    let program = r#"
        struct P { x }
        val p = P(1)
        print(p.y)
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn set_field_missing_halts() {
    let program = r#"
        struct P { x }
        val p = P(1)
        p.y = 2
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn create_map_invalid_key_reports_exactly_one_error() {
    let program = r#"
        print({[1]: 2})
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    assert_eq!(1, vm.get_runtime_errors().lines().count());
    assert_eq!("", vm.get_output());
}

#[test]
fn create_set_invalid_element_reports_exactly_one_error() {
    let program = r#"
        print({[1]})
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    assert_eq!(1, vm.get_runtime_errors().lines().count());
    assert_eq!("", vm.get_output());
}

#[test]
fn nothing_after_a_runtime_error_executes() {
    let program = r#"
        struct P { x }
        val p = P(1)
        print(p.y)
        print("after")
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    assert!(!vm.get_output().contains("after"));
}

#[test]
fn get_local_out_of_range_slot_halts() {
    // Stack is empty, so slot 0 is exactly one past the last valid slot.
    let mut chunk = Chunk::new("get_local_oob");
    chunk.write_op_code_variant(OpCode::GetLocal, 0, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn set_local_out_of_range_slot_halts() {
    // One value on the stack, so slot 1 is exactly one past the last valid slot.
    let mut chunk = Chunk::new("set_local_oob");
    chunk.write_constant(number!(42.0), 0, 0);
    chunk.write_op_code_variant(OpCode::SetLocal, 1, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn get_global_unknown_slot_halts() {
    // Stack is empty, so slot 0 is exactly one past the last valid slot.
    let mut chunk = Chunk::new("get_global_unknown");
    chunk.write_op_code_variant(OpCode::GetGlobal, 0, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn set_global_out_of_range_slot_halts() {
    // One value on the stack, so slot 1 is exactly one past the last valid slot.
    let mut chunk = Chunk::new("set_global_oob");
    chunk.write_constant(number!(42.0), 0, 0);
    chunk.write_op_code_variant(OpCode::SetGlobal, 1, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn get_builtin_unknown_index_halts() {
    let mut chunk = Chunk::new("get_builtin_unknown");
    chunk.write_op_code_variant(OpCode::GetBuiltin, 9999, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn iterator_done_without_iterator_halts() {
    let mut chunk = Chunk::new("iterator_done_without_iterator");
    chunk.write_op_code(OpCode::IteratorDone, 0, 0);

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn invalid_opcode_byte_halts() {
    let mut chunk = Chunk::new("invalid_opcode");
    chunk.write_op_code(OpCode::Nil, 1, 1);
    chunk.instructions[0] = 0xFF;

    let mut vm = VirtualMachine::new();
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn subtract_with_non_number_operand_reports_operator_location() {
    let program = r#"
        val a = 1
        val b = a - "x"
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    assert!(vm.get_runtime_errors().starts_with("[3:19]"));
}

#[test]
fn bad_operand_type_errors() {
    let cases: &[(&str, &str)] = &[
        (
            r#"print(1 - "a")"#,
            "Operands of '-' must be numbers, got number and string",
        ),
        (
            r#"print("a" - 1)"#,
            "Operands of '-' must be numbers, got string and number",
        ),
        (
            "print(true - 1)",
            "Operands of '-' must be numbers, got boolean and number",
        ),
        (
            r#"print(5 * "x")"#,
            "Operands of '*' must be numbers, got number and string",
        ),
        (
            "print(nil * 5)",
            "Operands of '*' must be numbers, got nil and number",
        ),
        (
            r#"print(5 / "x")"#,
            "Operands of '/' must be numbers, got number and string",
        ),
        (
            "print([1] / 5)",
            "Operands of '/' must be numbers, got array and number",
        ),
        (
            r#"print(5 % "x")"#,
            "Operands of '%' must be numbers, got number and string",
        ),
        (
            "print(5 % true)",
            "Operands of '%' must be numbers, got number and boolean",
        ),
        (
            r#"print(5 ** "x")"#,
            "Operands of '**' must be numbers, got number and string",
        ),
        (
            "print(5 ** nil)",
            "Operands of '**' must be numbers, got number and nil",
        ),
        (
            r#"print(5 & "x")"#,
            "Operands of '&' must be numbers, got number and string",
        ),
        (
            "print(5 & [1])",
            "Operands of '&' must be numbers, got number and array",
        ),
        (
            r#"print(5 | "x")"#,
            "Operands of '|' must be numbers, got number and string",
        ),
        (
            "print(5 | {1: 2})",
            "Operands of '|' must be numbers, got number and map",
        ),
        (
            r#"print(5 ^ "x")"#,
            "Operands of '^' must be numbers, got number and string",
        ),
        (
            "print(5 ^ true)",
            "Operands of '^' must be numbers, got number and boolean",
        ),
        (
            r#"print(5 << "x")"#,
            "Operands of '<<' must be numbers, got number and string",
        ),
        (
            "print(5 << nil)",
            "Operands of '<<' must be numbers, got number and nil",
        ),
        (
            r#"print(5 >> "x")"#,
            "Operands of '>>' must be numbers, got number and string",
        ),
        (
            "print(5 >> [1])",
            "Operands of '>>' must be numbers, got number and array",
        ),
        (
            r#"print(1 < "a")"#,
            "Operands of a comparison must be two numbers or two strings, got number and string",
        ),
        (
            r#"print("a" < 1)"#,
            "Operands of a comparison must be two numbers or two strings, got string and number",
        ),
        (
            "print(nil < 1)",
            "Operands of a comparison must be two numbers or two strings, got nil and number",
        ),
        (
            "print(true < false)",
            "Operands of a comparison must be two numbers or two strings, got boolean and boolean",
        ),
        (
            r#"print(1 > "a")"#,
            "Operands of a comparison must be two numbers or two strings, got number and string",
        ),
        (
            "print([1] > [2])",
            "Operands of a comparison must be two numbers or two strings, got array and array",
        ),
        (
            r#"print(1 <= "a")"#,
            "Operands of a comparison must be two numbers or two strings, got number and string",
        ),
        (
            r#"print(1 >= "a")"#,
            "Operands of a comparison must be two numbers or two strings, got number and string",
        ),
    ];

    for (program, expected_message) in cases {
        let mut vm = VirtualMachine::new();
        let result = vm.interpret(program.to_string());
        assert_eq!(
            Result::RuntimeError,
            result,
            "expected runtime error for: {}",
            program
        );
        let errors = vm.get_runtime_errors();
        assert!(
            errors.contains(expected_message),
            "program {:?}: expected {:?} in {:?}",
            program,
            expected_message,
            errors
        );
    }
}
