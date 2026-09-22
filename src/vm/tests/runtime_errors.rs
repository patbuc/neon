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
