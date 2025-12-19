// =============================================================================
// Math.abs() Error Cases
// =============================================================================

use crate::vm::Result;
use crate::vm::VirtualMachine;

#[test]
fn test_math_abs_with_string() {
    let program = r#"
        print(Math.abs("string"))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_abs_with_boolean() {
    let program = r#"
        print(Math.abs(true))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_abs_with_nil() {
    let program = r#"
        print(Math.abs(nil))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_abs_no_args() {
    let program = r#"
        print(Math.abs())
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_abs_too_many_args() {
    let program = r#"
        print(Math.abs(1, 2))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

// =============================================================================
// Math.floor() Error Cases
// =============================================================================

#[test]
fn test_math_floor_with_string() {
    let program = r#"
        print(Math.floor("hello"))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_floor_with_boolean() {
    let program = r#"
        print(Math.floor(false))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_floor_with_nil() {
    let program = r#"
        print(Math.floor(nil))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_floor_no_args() {
    let program = r#"
        print(Math.floor())
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_floor_too_many_args() {
    let program = r#"
        print(Math.floor(1.5, 2.5))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

// =============================================================================
// Math.ceil() Error Cases
// =============================================================================

#[test]
fn test_math_ceil_with_string() {
    let program = r#"
        print(Math.ceil("world"))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_ceil_with_boolean() {
    let program = r#"
        print(Math.ceil(true))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_ceil_with_nil() {
    let program = r#"
        print(Math.ceil(nil))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_ceil_no_args() {
    let program = r#"
        print(Math.ceil())
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_ceil_too_many_args() {
    let program = r#"
        print(Math.ceil(1.5, 2.5))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

// =============================================================================
// Math.sqrt() Error Cases
// =============================================================================

#[test]
fn test_math_sqrt_with_negative() {
    let program = r#"
        print(Math.sqrt(-1))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_sqrt_with_string() {
    let program = r#"
        print(Math.sqrt("42"))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_sqrt_with_boolean() {
    let program = r#"
        print(Math.sqrt(false))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_sqrt_with_nil() {
    let program = r#"
        print(Math.sqrt(nil))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_sqrt_no_args() {
    let program = r#"
        print(Math.sqrt())
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_sqrt_too_many_args() {
    let program = r#"
        print(Math.sqrt(4, 9))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_sqrt_large_negative() {
    let program = r#"
        print(Math.sqrt(-100))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

// =============================================================================
// Math.min() Error Cases
// =============================================================================

#[test]
fn test_math_min_no_args() {
    let program = r#"
        print(Math.min())
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_min_with_string_first() {
    let program = r#"
        print(Math.min("hello", 2, 3))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_min_with_string_middle() {
    let program = r#"
        print(Math.min(1, "hello", 3))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_min_with_string_last() {
    let program = r#"
        print(Math.min(1, 2, "hello"))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_min_with_boolean() {
    let program = r#"
        print(Math.min(1, true, 3))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_min_with_nil() {
    let program = r#"
        print(Math.min(1, nil, 3))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_min_all_non_numbers() {
    let program = r#"
        print(Math.min("a", "b", "c"))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_min_mixed_types() {
    let program = r#"
        print(Math.min(1, "hello", true, nil, 5))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

// =============================================================================
// Math.max() Error Cases
// =============================================================================

#[test]
fn test_math_max_no_args() {
    let program = r#"
        print(Math.max())
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_max_with_string_first() {
    let program = r#"
        print(Math.max("hello", 2, 3))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_max_with_string_middle() {
    let program = r#"
        print(Math.max(1, "hello", 3))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_max_with_string_last() {
    let program = r#"
        print(Math.max(1, 2, "hello"))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_max_with_boolean() {
    let program = r#"
        print(Math.max(1, false, 3))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_max_with_nil() {
    let program = r#"
        print(Math.max(1, nil, 3))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_max_all_non_numbers() {
    let program = r#"
        print(Math.max(true, false, true))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_max_mixed_types() {
    let program = r#"
        print(Math.max(1, "world", false, nil, 5))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

// =============================================================================
// Edge Cases and Complex Scenarios
// =============================================================================

#[test]
fn test_math_error_in_expression() {
    let program = r#"
        val x = 5 + Math.abs("invalid")
        print(x)
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_error_in_function_call() {
    let program = r#"
        fn test() {
            return Math.sqrt(-10)
        }
        print(test())
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_error_in_if_condition() {
    let program = r#"
        if (Math.min() > 0) {
            print("unreachable")
        }
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_error_in_while_condition() {
    let program = r#"
        while (Math.max() > 0) {
            print("unreachable")
        }
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_error_does_not_crash_vm() {
    let program = r#"
        print("Before error")
        print(Math.abs(nil))
        print("After error (unreachable)")
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
    // VM should print("Before error" but not crash)
    assert_eq!("Before error", vm.get_output());
}

#[test]
fn test_multiple_math_errors() {
    let program = r#"
        print(Math.abs("first error"))
        print(Math.floor(nil))
        print(Math.sqrt(-1))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    // Should fail on the first error
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_error_with_variables() {
    let program = r#"
        val x = "not a number"
        print(Math.floor(x))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_error_with_expression_arg() {
    let program = r#"
        print(Math.abs(5 + "string"))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_nested_error() {
    let program = r#"
        print(Math.abs(Math.sqrt(-5)))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn test_math_min_max_combined_error() {
    let program = r#"
        print(Math.min(Math.max(), 5))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::RuntimeError, result);
}

// =============================================================================
// Valid Cases That Should NOT Error (sanity checks)
// =============================================================================

#[test]
fn test_math_functions_dont_error_on_valid_input() {
    let program = r#"
        print(Math.abs(-5))
        print(Math.floor(3.7))
        print(Math.ceil(2.1))
        print(Math.sqrt(16))
        print(Math.min(1, 2, 3))
        print(Math.max(1, 2, 3))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("5\n3\n3\n4\n1\n3", vm.get_output());
}

#[test]
fn test_math_sqrt_zero_is_valid() {
    let program = r#"
        print(Math.sqrt(0))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("0", vm.get_output());
}

#[test]
fn test_math_single_arg_functions_valid() {
    let program = r#"
        print(Math.min(42))
        print(Math.max(42))
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("42\n42", vm.get_output());
}
