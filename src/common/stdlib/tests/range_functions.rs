use crate::vm::{InterpretResult, VirtualMachine};

// ============================================================================
// Range.size()
// ============================================================================

#[test]
fn test_range_size() {
    let program = r#"
        print((1..4).size())
        print((1..=4).size())
        print((5..1).size())
        print((1..1).size())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\n4\n0\n0", vm.get_output());
}

#[test]
fn test_range_size_negative_bounds() {
    let program = r#"
        print((-3..3).size())
        print((-3..=3).size())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("6\n7", vm.get_output());
}

#[test]
fn test_range_is_empty() {
    let program = r#"
        print((1..4).isEmpty())
        print((5..1).isEmpty())
        print((1..1).isEmpty())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("false\ntrue\ntrue", vm.get_output());
}

// ============================================================================
// Range.contains()
// ============================================================================

#[test]
fn test_range_contains() {
    let program = r#"
        print((1..10).contains(10))
        print((1..=10).contains(10))
        print((1..10).contains(1))
        print((1..10).contains(0))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("false\ntrue\ntrue\nfalse", vm.get_output());
}

#[test]
fn test_range_contains_negative_bounds() {
    let program = r#"
        print((-5..-1).contains(-5))
        print((-5..-1).contains(-1))
        print((-5..=-1).contains(-1))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\nfalse\ntrue", vm.get_output());
}

#[test]
fn test_range_contains_non_integer_or_non_number() {
    let program = r#"
        print((1..10).contains(1.5))
        print((1..10).contains("1"))
        print((1..10).contains(nil))
        print((1..10).contains(true))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("false\nfalse\nfalse\nfalse", vm.get_output());
}

#[test]
fn test_range_contains_empty_range() {
    let program = r#"
        print((5..1).contains(3))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("false", vm.get_output());
}

// ============================================================================
// Range.toArray()
// ============================================================================

#[test]
fn test_range_to_array() {
    let program = r#"
        print((1..4).toArray())
        print((1..=4).toArray())
        print((5..1).toArray())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2, 3]\n[1, 2, 3, 4]\n[]", vm.get_output());
}

// ============================================================================
// Range.step()
// ============================================================================

#[test]
fn test_range_step() {
    let program = r#"
        print((0..10).step(3))
        print((0..=9).step(3))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[0, 3, 6, 9]\n[0, 3, 6, 9]", vm.get_output());
}

#[test]
fn test_range_step_zero_is_runtime_error() {
    let program = r#"
        val r = 0..10
        r.step(0)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("step() k must be >= 1, got 0"),
        "{}",
        errors
    );
}

// ============================================================================
// Materializing delegates - one representative case
// ============================================================================

#[test]
fn test_range_sum() {
    let program = r#"
        print((1..4).sum())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("6", vm.get_output());
}

#[test]
fn test_range_sum_keeps_int_variant() {
    let program = r#"
        print(Math.div((1..4).sum(), 1))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("6", vm.get_output());
}

// ============================================================================
// Error Cases
// ============================================================================

#[test]
fn test_range_size_wrong_arg_count() {
    let program = r#"
        val r = 1..4
        r.size(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_range_slice_wrong_arg_count() {
    let program = r#"
        val r = 1..4
        r.slice(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(errors.contains("slice()"), "{}", errors);
}

// ============================================================================
// Range.forEach(fn) / Range.flatMap(fn)
// ============================================================================

#[test]
fn test_range_for_each_prints_each_and_returns_nil() {
    let program = r#"
        print((1..=3).forEach(fn(x) { print(x) }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("1\n2\n3\nnil", vm.get_output());
}

#[test]
fn test_range_flat_map() {
    let program = r#"
        print((1..3).flatMap(fn(x) { return [x, x] }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 1, 2, 2]", vm.get_output());
}

#[test]
fn test_range_flat_map_non_array_return_errors() {
    let program = r#"
        (1..3).flatMap(fn(x) { return 3 })
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("flatMap() callback must return an array, got Int"),
        "{}",
        errors
    );
}

// ============================================================================
// Range.take() / Range.drop()
// ============================================================================

#[test]
fn test_range_take_and_drop() {
    let program = r#"
        print((1..=3).take(2))
        print((1..=3).take(9))
        print((1..=3).drop(1))
        print((1..=3).drop(9))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2]\n[1, 2, 3]\n[2, 3]\n[]", vm.get_output());
}

#[test]
fn test_range_take_negative_errors() {
    let program = r#"
        (1..=3).take(-1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("take() n must be non-negative, got -1"),
        "{}",
        errors
    );
}

#[test]
fn test_range_drop_negative_errors() {
    let program = r#"
        (1..=3).drop(-1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("drop() n must be non-negative, got -1"),
        "{}",
        errors
    );
}

// ============================================================================
// Range.first() / Range.last()
// ============================================================================

#[test]
fn test_range_first_and_last() {
    let program = r#"
        print((1..1).first())
        print((1..1).last())
        print((1..=2).first())
        print((1..=2).last())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("nil\nnil\n1\n2", vm.get_output());
}

#[test]
fn test_range_first_and_last_huge_range_does_not_materialize() {
    let program = r#"
        print((0..2000000000).first())
        print((0..2000000000).last())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("0\n1999999999", vm.get_output());
}

// ============================================================================
// Range.chunked()
// ============================================================================

#[test]
fn test_range_chunked() {
    let program = r#"
        print((1..=5).chunked(2))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[1, 2], [3, 4], [5]]", vm.get_output());
}

#[test]
fn test_range_chunked_zero_errors() {
    let program = r#"
        (1..=3).chunked(0)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("chunked() n must be >= 1, got 0"),
        "{}",
        errors
    );
}

// ============================================================================
// Range.zip() / Range.withIndex()
// ============================================================================

#[test]
fn test_range_zip() {
    let program = r#"
        print((1..3).zip(1..3))
        print((1..=2).zip(["a", "b"]))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[1, 1], [2, 2]]\n[[1, a], [2, b]]", vm.get_output());
}

#[test]
fn test_range_zip_other_wrong_type_errors() {
    let program = r#"
        (1..3).zip(5)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("zip() other must be an array or range, got Int"),
        "{}",
        errors
    );
}

#[test]
fn test_range_with_index() {
    let program = r#"
        print((1..=2).withIndex())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[0, 1], [1, 2]]", vm.get_output());
}
