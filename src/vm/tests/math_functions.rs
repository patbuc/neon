use crate::common::Value;
use crate::vm::VirtualMachine;
use crate::vm::math_functions::*;
use crate::as_number;

#[test]
fn test_abs_positive() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(5.0)];
    let result = native_math_abs(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 5.0);
}

#[test]
fn test_abs_negative() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(-5.0)];
    let result = native_math_abs(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 5.0);
}

#[test]
fn test_abs_zero() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(0.0)];
    let result = native_math_abs(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 0.0);
}

#[test]
fn test_abs_decimal() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(-3.14)];
    let result = native_math_abs(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 3.14);
}

#[test]
fn test_abs_invalid_type() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Boolean(true)];
    let result = native_math_abs(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "abs() requires a number argument");
}

#[test]
fn test_abs_wrong_arg_count() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(5.0), Value::Number(3.0)];
    let result = native_math_abs(&mut vm, &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects 1 argument"));
}

#[test]
fn test_floor_positive() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(5.7)];
    let result = native_math_floor(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 5.0);
}

#[test]
fn test_floor_negative() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(-5.3)];
    let result = native_math_floor(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), -6.0);
}

#[test]
fn test_floor_integer() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(5.0)];
    let result = native_math_floor(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 5.0);
}

#[test]
fn test_floor_invalid_type() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Nil];
    let result = native_math_floor(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "floor() requires a number argument");
}

#[test]
fn test_ceil_positive() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(5.3)];
    let result = native_math_ceil(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 6.0);
}

#[test]
fn test_ceil_negative() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(-5.7)];
    let result = native_math_ceil(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), -5.0);
}

#[test]
fn test_ceil_integer() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(5.0)];
    let result = native_math_ceil(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 5.0);
}

#[test]
fn test_ceil_invalid_type() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Boolean(false)];
    let result = native_math_ceil(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "ceil() requires a number argument");
}

#[test]
fn test_sqrt_positive() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(16.0)];
    let result = native_math_sqrt(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 4.0);
}

#[test]
fn test_sqrt_zero() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(0.0)];
    let result = native_math_sqrt(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 0.0);
}

#[test]
fn test_sqrt_decimal() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(2.0)];
    let result = native_math_sqrt(&mut vm, &args).unwrap();
    assert!((as_number!(result) - 1.4142135623730951).abs() < 1e-10);
}

#[test]
fn test_sqrt_negative() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(-4.0)];
    let result = native_math_sqrt(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "sqrt() requires a non-negative number"
    );
}

#[test]
fn test_sqrt_invalid_type() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Nil];
    let result = native_math_sqrt(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "sqrt() requires a number argument");
}

#[test]
fn test_min_single() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(5.0)];
    let result = native_math_min(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 5.0);
}

#[test]
fn test_min_multiple() {
    let mut vm = VirtualMachine::new();
    let args = vec![
        Value::Number(5.0),
        Value::Number(2.0),
        Value::Number(8.0),
        Value::Number(1.0),
    ];
    let result = native_math_min(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 1.0);
}

#[test]
fn test_min_negative() {
    let mut vm = VirtualMachine::new();
    let args = vec![
        Value::Number(-5.0),
        Value::Number(-2.0),
        Value::Number(-8.0),
    ];
    let result = native_math_min(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), -8.0);
}

#[test]
fn test_min_mixed() {
    let mut vm = VirtualMachine::new();
    let args = vec![
        Value::Number(5.0),
        Value::Number(-2.0),
        Value::Number(8.0),
        Value::Number(-10.0),
    ];
    let result = native_math_min(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), -10.0);
}

#[test]
fn test_min_empty() {
    let mut vm = VirtualMachine::new();
    let args = vec![];
    let result = native_math_min(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "min() requires at least 1 argument"
    );
}

#[test]
fn test_min_invalid_type() {
    let mut vm = VirtualMachine::new();
    let args = vec![
        Value::Number(5.0),
        Value::Boolean(true),
        Value::Number(3.0),
    ];
    let result = native_math_min(&mut vm, &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("requires number arguments"));
}

#[test]
fn test_max_single() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(5.0)];
    let result = native_math_max(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 5.0);
}

#[test]
fn test_max_multiple() {
    let mut vm = VirtualMachine::new();
    let args = vec![
        Value::Number(5.0),
        Value::Number(2.0),
        Value::Number(8.0),
        Value::Number(1.0),
    ];
    let result = native_math_max(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 8.0);
}

#[test]
fn test_max_negative() {
    let mut vm = VirtualMachine::new();
    let args = vec![
        Value::Number(-5.0),
        Value::Number(-2.0),
        Value::Number(-8.0),
    ];
    let result = native_math_max(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), -2.0);
}

#[test]
fn test_max_mixed() {
    let mut vm = VirtualMachine::new();
    let args = vec![
        Value::Number(5.0),
        Value::Number(-2.0),
        Value::Number(8.0),
        Value::Number(-10.0),
    ];
    let result = native_math_max(&mut vm, &args).unwrap();
    assert_eq!(as_number!(result), 8.0);
}

#[test]
fn test_max_empty() {
    let mut vm = VirtualMachine::new();
    let args = vec![];
    let result = native_math_max(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "max() requires at least 1 argument"
    );
}

#[test]
fn test_max_invalid_type() {
    let mut vm = VirtualMachine::new();
    let args = vec![
        Value::Number(5.0),
        Value::Nil,
        Value::Number(3.0),
    ];
    let result = native_math_max(&mut vm, &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("requires number arguments"));
}

#[test]
fn test_min_max_same_value() {
    let mut vm = VirtualMachine::new();
    let args = vec![
        Value::Number(5.0),
        Value::Number(5.0),
        Value::Number(5.0),
    ];

    let min_result = native_math_min(&mut vm, &args).unwrap();
    let max_result = native_math_max(&mut vm, &args).unwrap();

    assert_eq!(as_number!(min_result), 5.0);
    assert_eq!(as_number!(max_result), 5.0);
}

#[test]
fn test_min_max_two_values() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(3.0), Value::Number(7.0)];

    let min_result = native_math_min(&mut vm, &args).unwrap();
    let max_result = native_math_max(&mut vm, &args).unwrap();

    assert_eq!(as_number!(min_result), 3.0);
    assert_eq!(as_number!(max_result), 7.0);
}

#[test]
fn test_sqrt_perfect_squares() {
    let mut vm = VirtualMachine::new();
    let test_cases = vec![(1.0, 1.0), (4.0, 2.0), (9.0, 3.0), (25.0, 5.0), (100.0, 10.0)];

    for (input, expected) in test_cases {
        let args = vec![Value::Number(input)];
        let result = native_math_sqrt(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), expected);
    }
}

#[test]
fn test_floor_ceil_edge_cases() {
    let mut vm = VirtualMachine::new();

    // Test 0.5
    let args = vec![Value::Number(0.5)];
    assert_eq!(as_number!(native_math_floor(&mut vm, &args).unwrap()), 0.0);
    assert_eq!(as_number!(native_math_ceil(&mut vm, &args).unwrap()), 1.0);

    // Test -0.5
    let args = vec![Value::Number(-0.5)];
    assert_eq!(as_number!(native_math_floor(&mut vm, &args).unwrap()), -1.0);
    assert_eq!(as_number!(native_math_ceil(&mut vm, &args).unwrap()), 0.0);

    // Test very small positive number
    let args = vec![Value::Number(0.00001)];
    assert_eq!(as_number!(native_math_floor(&mut vm, &args).unwrap()), 0.0);
    assert_eq!(as_number!(native_math_ceil(&mut vm, &args).unwrap()), 1.0);
}
