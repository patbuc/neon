use crate::common::Value;
use crate::vm::VirtualMachine;
use crate::vm::boolean_functions::*;
use crate::as_string;
use crate::string;

#[test]
fn test_boolean_to_string_true() {
    let mut vm = VirtualMachine::new();
    let bool_val = Value::Boolean(true);
    let args = vec![bool_val];

    let result = native_boolean_to_string(&mut vm, &args).unwrap();
    assert_eq!("true", as_string!(result).value.as_ref());
}

#[test]
fn test_boolean_to_string_false() {
    let mut vm = VirtualMachine::new();
    let bool_val = Value::Boolean(false);
    let args = vec![bool_val];

    let result = native_boolean_to_string(&mut vm, &args).unwrap();
    assert_eq!("false", as_string!(result).value.as_ref());
}

#[test]
fn test_boolean_to_string_no_args() {
    let mut vm = VirtualMachine::new();
    let args = vec![];

    let result = native_boolean_to_string(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(
        "boolean.toString() requires a boolean receiver",
        result.unwrap_err()
    );
}

#[test]
fn test_boolean_to_string_wrong_type_number() {
    let mut vm = VirtualMachine::new();
    let args = vec![Value::Number(42.0)];

    let result = native_boolean_to_string(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(
        "toString() can only be called on booleans",
        result.unwrap_err()
    );
}

#[test]
fn test_boolean_to_string_wrong_type_string() {
    let mut vm = VirtualMachine::new();
    let args = vec![string!("test")];

    let result = native_boolean_to_string(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(
        "toString() can only be called on booleans",
        result.unwrap_err()
    );
}

#[test]
fn test_boolean_to_string_multiple_true_calls() {
    let mut vm = VirtualMachine::new();

    for _ in 0..3 {
        let bool_val = Value::Boolean(true);
        let args = vec![bool_val];
        let result = native_boolean_to_string(&mut vm, &args).unwrap();
        assert_eq!("true", as_string!(result).value.as_ref());
    }
}

#[test]
fn test_boolean_to_string_multiple_false_calls() {
    let mut vm = VirtualMachine::new();

    for _ in 0..3 {
        let bool_val = Value::Boolean(false);
        let args = vec![bool_val];
        let result = native_boolean_to_string(&mut vm, &args).unwrap();
        assert_eq!("false", as_string!(result).value.as_ref());
    }
}

#[test]
fn test_boolean_to_string_alternating() {
    let mut vm = VirtualMachine::new();

    let test_cases = vec![
        (true, "true"),
        (false, "false"),
        (true, "true"),
        (false, "false"),
    ];

    for (bool_val, expected) in test_cases {
        let args = vec![Value::Boolean(bool_val)];
        let result = native_boolean_to_string(&mut vm, &args).unwrap();
        assert_eq!(expected, as_string!(result).value.as_ref());
    }
}
