use crate::vm::{InterpretResult, VirtualMachine};

// Basic push/pop/ties/peek/size ordering and the print format are pinned by
// tests/scripts/priority_queue_basic.n. These unit tests cover what that
// script can't: variants, error paths, map-key rejection, NaN, and extreme
// priority values.

#[test]
fn test_priority_queue_empty() {
    let program = r#"
        use "std/pq"
        val queue = pq.new()
        print(queue.pop())
        print(queue.peek())
        print(queue.size())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("nil\nnil\n0", vm.get_output());
}

#[test]
fn test_priority_queue_is_empty() {
    let program = r#"
        use "std/pq"
        val queue = pq.new()
        print(queue.isEmpty())
        queue.push(1, "a")
        print(queue.isEmpty())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\nfalse", vm.get_output());
}

#[test]
fn test_priority_queue_mixed_int_float_priorities() {
    let program = r#"
        use "std/pq"
        val queue = pq.new()
        queue.push(2, "int")
        queue.push(1.5, "float")
        print(queue.pop())
        print(queue.pop())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("float\nint", vm.get_output());
}

#[test]
fn test_priority_queue_extreme_priorities() {
    // Ints near +-2^63 and the float infinities must still compare exactly
    // against each other, not round-trip through a lossy f64 conversion.
    let program = r#"
        use "std/pq"
        val queue = pq.new()
        queue.push(9223372036854775807, "int max")
        queue.push(-9223372036854775807 - 1, "int min")
        queue.push(1.0 / 0.0, "+inf")
        queue.push(-1.0 / 0.0, "-inf")
        queue.push(0, "zero")
        print(queue.pop())
        print(queue.pop())
        print(queue.pop())
        print(queue.pop())
        print(queue.pop())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("-inf\nint min\nzero\nint max\n+inf", vm.get_output());
}

#[test]
fn test_priority_queue_nan_priority() {
    let program = r#"
        use "std/pq"
        val queue = pq.new()
        queue.push(0.0 / 0.0, "x")
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_priority_queue_non_number_priority() {
    let program = r#"
        use "std/pq"
        val queue = pq.new()
        queue.push("not a number", "x")
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_priority_queue_push_wrong_arg_count() {
    let program = r#"
        use "std/pq"
        val queue = pq.new()
        queue.push(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_priority_queue_constructor_wrong_arg_count() {
    let program = r#"
        use "std/pq"
        val queue = pq.new(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::CompileError,
        vm.interpret(program.to_string())
    );

    let program = r#"
        use "std/pq"
        val make = pq.new
        make(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    assert!(vm
        .get_runtime_errors()
        .contains("pq.new() expects 0 arguments, got 1"));
}

#[test]
fn test_priority_queue_not_a_map_key() {
    let program = r#"
        use "std/pq"
        val queue = pq.new()
        val m = {}
        m[queue] = 1
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}
