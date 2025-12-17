use crate::vm::{Result, VirtualMachine};

// ============================================================================
// Math.abs() - Success Cases
// ============================================================================

#[test]
fn test_math_abs() {
    let program = r#"
        print Math.abs(5)
        print Math.abs(-5)
        print Math.abs(0)
        print Math.abs(-3.15)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("5\n5\n0\n3.15", vm.get_output());
}

// ============================================================================
// Math.floor() - Success Cases
// ============================================================================

#[test]
fn test_math_floor() {
    let program = r#"
        print Math.floor(5.7)
        print Math.floor(-5.3)
        print Math.floor(5)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("5\n-6\n5", vm.get_output());
}

// ============================================================================
// Math.ceil() - Success Cases
// ============================================================================

#[test]
fn test_math_ceil() {
    let program = r#"
        print Math.ceil(5.3)
        print Math.ceil(-5.7)
        print Math.ceil(5)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("6\n-5\n5", vm.get_output());
}

// ============================================================================
// Math.sqrt() - Success Cases
// ============================================================================

#[test]
fn test_math_sqrt() {
    let program = r#"
        print Math.sqrt(16)
        print Math.sqrt(0)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("4\n0", vm.get_output());
}

#[test]
fn test_math_sqrt_perfect_squares() {
    let program = r#"
        print Math.sqrt(1)
        print Math.sqrt(4)
        print Math.sqrt(9)
        print Math.sqrt(25)
        print Math.sqrt(100)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("1\n2\n3\n5\n10", vm.get_output());
}

// ============================================================================
// Math.min() - Success Cases
// ============================================================================

#[test]
fn test_math_min() {
    let program = r#"
        print Math.min(5)
        print Math.min(5, 2, 8, 1)
        print Math.min(-5, -2, -8)
        print Math.min(5, -2, 8, -10)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("5\n1\n-8\n-10", vm.get_output());
}

// ============================================================================
// Math.max() - Success Cases
// ============================================================================

#[test]
fn test_math_max() {
    let program = r#"
        print Math.max(5)
        print Math.max(5, 2, 8, 1)
        print Math.max(-5, -2, -8)
        print Math.max(5, -2, 8, -10)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("5\n8\n-2\n8", vm.get_output());
}

// ============================================================================
// Math.min() and Math.max() - Combined Tests
// ============================================================================

#[test]
fn test_math_min_max_combined() {
    let program = r#"
        print Math.min(5, 5, 5)
        print Math.max(5, 5, 5)
        print Math.min(3, 7)
        print Math.max(3, 7)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("5\n5\n3\n7", vm.get_output());
}

// ============================================================================
// Math.floor() and Math.ceil() - Edge Cases
// ============================================================================

#[test]
fn test_math_floor_ceil_edge_cases() {
    let program = r#"
        print Math.floor(0.5)
        print Math.ceil(0.5)
        print Math.floor(-0.5)
        print Math.ceil(-0.5)
        print Math.floor(0.00001)
        print Math.ceil(0.00001)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("0\n1\n-1\n-0\n0\n1", vm.get_output());
}
