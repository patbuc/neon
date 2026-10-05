use crate::vm::{InterpretResult, VirtualMachine};

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
fn test_range_zip_huge_receiver_does_not_materialize() {
    let program = r#"
        print((0..2000000000).zip([1]))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[0, 1]]", vm.get_output());
}

#[test]
fn test_range_sort_by() {
    let program = r#"
        print((1..5).sortBy(fn(x) { return -x }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[4, 3, 2, 1]", vm.get_output());
}

#[test]
fn test_range_min_by() {
    let program = r#"
        print((1..5).minBy(fn(x) { return x % 3 }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("3", vm.get_output());
}

#[test]
fn test_range_max_by() {
    let program = r#"
        print((1..5).maxBy(fn(x) { return x % 3 }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("2", vm.get_output());
}
