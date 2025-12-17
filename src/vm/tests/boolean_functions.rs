use crate::as_string;
use crate::common::Value;
use crate::string;
use crate::vm::boolean_functions::*;
use crate::vm::VirtualMachine;

#[test]
fn test_boolean_to_string_true() {
    let vm = VirtualMachine::new();
    let bool_val = Value::Boolean(true);
    let args = vec![bool_val];

    let result = native_boolean_to_string(&args).unwrap();
    assert_eq!("true", as_string!(result).value.as_ref());
}

#[test]
fn test_boolean_to_string_false() {
    let vm = VirtualMachine::new();
    let bool_val = Value::Boolean(false);
    let args = vec![bool_val];

    let result = native_boolean_to_string(&args).unwrap();
    assert_eq!("false", as_string!(result).value.as_ref());
}

#[test]
fn test_boolean_to_string_no_args() {
    let vm = VirtualMachine::new();
    let args = vec![];

    let result = native_boolean_to_string(&args);
    assert!(result.is_err());
    assert_eq!(
        "boolean.toString() requires a boolean receiver",
        result.unwrap_err()
    );
}

#[test]
fn test_boolean_to_string_wrong_type_number() {
    let vm = VirtualMachine::new();
    let args = vec![Value::Number(42.0)];

    let result = native_boolean_to_string(&args);
    assert!(result.is_err());
    assert_eq!(
        "toString() can only be called on booleans",
        result.unwrap_err()
    );
}

#[test]
fn test_boolean_to_string_wrong_type_string() {
    let vm = VirtualMachine::new();
    let args = vec![string!("test")];

    let result = native_boolean_to_string(&args);
    assert!(result.is_err());
    assert_eq!(
        "toString() can only be called on booleans",
        result.unwrap_err()
    );
}

#[test]
fn test_boolean_to_string_multiple_true_calls() {
    let vm = VirtualMachine::new();

    for _ in 0..3 {
        let bool_val = Value::Boolean(true);
        let args = vec![bool_val];
        let result = native_boolean_to_string(&args).unwrap();
        assert_eq!("true", as_string!(result).value.as_ref());
    }
}

#[test]
fn test_boolean_to_string_multiple_false_calls() {
    let vm = VirtualMachine::new();

    for _ in 0..3 {
        let bool_val = Value::Boolean(false);
        let args = vec![bool_val];
        let result = native_boolean_to_string(&args).unwrap();
        assert_eq!("false", as_string!(result).value.as_ref());
    }
}

#[test]
fn test_boolean_to_string_alternating() {
    let vm = VirtualMachine::new();

    let test_cases = vec![
        (true, "true"),
        (false, "false"),
        (true, "true"),
        (false, "false"),
    ];

    for (bool_val, expected) in test_cases {
        let args = vec![Value::Boolean(bool_val)];
        let result = native_boolean_to_string(&args).unwrap();
        assert_eq!(expected, as_string!(result).value.as_ref());
    }
}
