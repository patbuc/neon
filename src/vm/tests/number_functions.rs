use crate::common::Value;
use crate::vm::VirtualMachine;
use crate::vm::number_functions::*;
use crate::as_string;

#[test]
fn test_number_to_string_integer() {
    let mut vm = VirtualMachine::new();
    let num = Value::Number(123.0);
    let args = vec![num];

    let result = native_number_to_string(&mut vm, &args).unwrap();
    assert_eq!("123", as_string!(result).value.as_ref());
}

#[test]
fn test_number_to_string_decimal() {
    let mut vm = VirtualMachine::new();
    let num = Value::Number(45.67);
    let args = vec![num];

    let result = native_number_to_string(&mut vm, &args).unwrap();
    assert_eq!("45.67", as_string!(result).value.as_ref());
}

#[test]
fn test_number_to_string_zero() {
    let mut vm = VirtualMachine::new();
    let num = Value::Number(0.0);
    let args = vec![num];

    let result = native_number_to_string(&mut vm, &args).unwrap();
    assert_eq!("0", as_string!(result).value.as_ref());
}

#[test]
fn test_number_to_string_negative_integer() {
    let mut vm = VirtualMachine::new();
    let num = Value::Number(-42.0);
    let args = vec![num];

    let result = native_number_to_string(&mut vm, &args).unwrap();
    assert_eq!("-42", as_string!(result).value.as_ref());
}

#[test]
fn test_number_to_string_negative_decimal() {
    let mut vm = VirtualMachine::new();
    let num = Value::Number(-3.14);
    let args = vec![num];

    let result = native_number_to_string(&mut vm, &args).unwrap();
    assert_eq!("-3.14", as_string!(result).value.as_ref());
}

#[test]
fn test_number_to_string_large_integer() {
    let mut vm = VirtualMachine::new();
    let num = Value::Number(1000000.0);
    let args = vec![num];

    let result = native_number_to_string(&mut vm, &args).unwrap();
    assert_eq!("1000000", as_string!(result).value.as_ref());
}

#[test]
fn test_number_to_string_small_decimal() {
    let mut vm = VirtualMachine::new();
    let num = Value::Number(0.001);
    let args = vec![num];

    let result = native_number_to_string(&mut vm, &args).unwrap();
    assert_eq!("0.001", as_string!(result).value.as_ref());
}

#[test]
fn test_number_to_string_infinity() {
    let mut vm = VirtualMachine::new();
    let num = Value::Number(f64::INFINITY);
    let args = vec![num];

    let result = native_number_to_string(&mut vm, &args).unwrap();
    assert_eq!("inf", as_string!(result).value.as_ref());
}

#[test]
fn test_number_to_string_negative_infinity() {
    let mut vm = VirtualMachine::new();
    let num = Value::Number(f64::NEG_INFINITY);
    let args = vec![num];

    let result = native_number_to_string(&mut vm, &args).unwrap();
    assert_eq!("-inf", as_string!(result).value.as_ref());
}

#[test]
fn test_number_to_string_nan() {
    let mut vm = VirtualMachine::new();
    let num = Value::Number(f64::NAN);
    let args = vec![num];

    let result = native_number_to_string(&mut vm, &args).unwrap();
    assert_eq!("NaN", as_string!(result).value.as_ref());
}

#[test]
fn test_number_to_string_no_args() {
    let mut vm = VirtualMachine::new();
    let args = vec![];

    let result = native_number_to_string(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(
        "number.toString() requires a number receiver",
        result.unwrap_err()
    );
}

#[test]
fn test_number_to_string_wrong_type() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Boolean(true)];

    let result = native_number_to_string(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(
        "toString() can only be called on numbers",
        result.unwrap_err()
    );
}

#[test]
fn test_number_to_string_scientific_notation() {
    let mut vm = VirtualMachine::new();
    let num = Value::Number(1.23e10);
    let args = vec![num];

    let result = native_number_to_string(&mut vm, &args).unwrap();
    // Rust formats this as "12300000000"
    assert_eq!("12300000000", as_string!(result).value.as_ref());
}

#[test]
fn test_number_to_string_very_small() {
    let mut vm = VirtualMachine::new();
    let num = Value::Number(1.23e-10);
    let args = vec![num];

    let result = native_number_to_string(&mut vm, &args).unwrap();
    // Rust's to_string handles this
    assert!(as_string!(result).value.as_ref().starts_with("0.000000000"));
}
