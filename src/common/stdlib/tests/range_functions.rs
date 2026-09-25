use crate::vm::{Result, VirtualMachine};

// ============================================================================
// Range.size() / Range.length()
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
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\n4\n0\n0", vm.get_output());
}

#[test]
fn test_range_length_matches_size() {
    let program = r#"
        print((-3..3).length())
        print((-3..=3).length())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("6\n7", vm.get_output());
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
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
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
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
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
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("false\nfalse\nfalse\nfalse", vm.get_output());
}

#[test]
fn test_range_contains_empty_range() {
    let program = r#"
        print((5..1).contains(3))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
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
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2, 3]\n[1, 2, 3, 4]\n[]", vm.get_output());
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
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("6", vm.get_output());
}

// ============================================================================
// Immutability
// ============================================================================

#[test]
fn test_range_push_reports_immutable() {
    let program = r#"
        val r = 1..4
        r.push(5)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
    let errors = vm.get_runtime_errors();
    assert!(errors.contains("immutable"), "{}", errors);
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
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_range_slice_wrong_arg_count() {
    let program = r#"
        val r = 1..4
        r.slice(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
    let errors = vm.get_runtime_errors();
    assert!(errors.contains("slice()"), "{}", errors);
}
