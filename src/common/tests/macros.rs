use crate::{number, as_number, boolean, as_bool, string, as_string, nil, as_nil};

#[test]
fn value_can_be_created_from_number() {
    let value = number!(123.45);
    assert_eq!(123.45, as_number!(value));
}

#[test]
fn value_can_be_created_from_bool() {
    let value = boolean!(true);
    assert!(as_bool!(value))
}

#[test]
fn value_can_be_created_from_string() {
    let value = string!("Hello, World!");
    assert_eq!("Hello, World!", as_string!(value));
}

#[test]
fn value_can_be_created_from_nil() {
    let value = nil!();
    assert_eq!(crate::common::Value::Nil, as_nil!(value));
}
