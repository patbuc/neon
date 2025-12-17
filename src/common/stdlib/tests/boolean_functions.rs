use crate::common::stdlib::boolean_functions::*;
use crate::common::Value;
use crate::{as_string, string};

#[test]
fn test_boolean_to_string_true() {
    let bool_val = Value::Boolean(true);
    let args = vec![bool_val];

    let result = native_boolean_to_string(&args).unwrap();
    assert_eq!("true", as_string!(result).value.as_ref());
}

#[test]
fn test_boolean_to_string_false() {
    let bool_val = Value::Boolean(false);
    let args = vec![bool_val];

    let result = native_boolean_to_string(&args).unwrap();
    assert_eq!("false", as_string!(result).value.as_ref());
}

#[test]
fn test_boolean_to_string_no_args() {
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
    for _ in 0..3 {
        let bool_val = Value::Boolean(true);
        let args = vec![bool_val];
        let result = native_boolean_to_string(&args).unwrap();
        assert_eq!("true", as_string!(result).value.as_ref());
    }
}

#[test]
fn test_boolean_to_string_multiple_false_calls() {
    for _ in 0..3 {
        let bool_val = Value::Boolean(false);
        let args = vec![bool_val];
        let result = native_boolean_to_string(&args).unwrap();
        assert_eq!("false", as_string!(result).value.as_ref());
    }
}

#[test]
fn test_boolean_to_string_alternating() {
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
