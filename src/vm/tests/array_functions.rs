use crate::vm::array_functions::*;
use crate::vm::VirtualMachine;
use crate::common::{Value, Object, ObjString};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_array_push() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![Value::Number(1.0), Value::Number(2.0)]);
    let value = Value::Number(3.0);
    let args = vec![array.clone(), value];

    let result = native_array_push(&mut vm, &args).unwrap();

    // push returns nil
    assert_eq!(result, Value::Nil);

    // Verify the array was modified
    match array {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 3);
                assert_eq!(contents[0], Value::Number(1.0));
                assert_eq!(contents[1], Value::Number(2.0));
                assert_eq!(contents[2], Value::Number(3.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_push_to_empty() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![]);
    let value = Value::Number(42.0);
    let args = vec![array.clone(), value];

    let result = native_array_push(&mut vm, &args).unwrap();
    assert_eq!(result, Value::Nil);

    // Verify the array was modified
    match array {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 1);
                assert_eq!(contents[0], Value::Number(42.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_push_wrong_arg_count() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![]);

    // Too few arguments
    let args = vec![array.clone()];
    let result = native_array_push(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "push() expects 1 argument (value), got 0");

    // Too many arguments
    let args = vec![array, Value::Number(1.0), Value::Number(2.0)];
    let result = native_array_push(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "push() expects 1 argument (value), got 2");
}

#[test]
fn test_array_push_on_non_array() {
    let mut vm = VirtualMachine::new();
    let not_array = Value::Number(42.0);
    let value = Value::Number(1.0);
    let args = vec![not_array, value];

    let result = native_array_push(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "push() can only be called on arrays");
}

#[test]
fn test_array_pop() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);
    let args = vec![array.clone()];

    let result = native_array_pop(&mut vm, &args).unwrap();

    // pop returns the last element
    assert_eq!(result, Value::Number(3.0));

    // Verify the array was modified
    match array {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 2);
                assert_eq!(contents[0], Value::Number(1.0));
                assert_eq!(contents[1], Value::Number(2.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_pop_empty() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![]);
    let args = vec![array];

    let result = native_array_pop(&mut vm, &args).unwrap();

    // pop on empty array returns nil
    assert_eq!(result, Value::Nil);
}

#[test]
fn test_array_pop_wrong_arg_count() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![Value::Number(1.0)]);

    // Too many arguments
    let args = vec![array, Value::Number(1.0)];
    let result = native_array_pop(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "pop() expects no arguments");
}

#[test]
fn test_array_pop_on_non_array() {
    let mut vm = VirtualMachine::new();
    let not_array = Value::Number(42.0);
    let args = vec![not_array];

    let result = native_array_pop(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "pop() can only be called on arrays");
}

#[test]
fn test_array_length() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);
    let args = vec![array];

    let result = native_array_length(&mut vm, &args).unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_array_length_empty() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![]);
    let args = vec![array];

    let result = native_array_length(&mut vm, &args).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_array_size_empty() {
    let mut vm = VirtualMachine::new();
    let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![])))));
    let result = native_array_size(&mut vm, &[array]).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_array_length_wrong_arg_count() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![]);

    // Too many arguments
    let args = vec![array, Value::Number(1.0)];
    let result = native_array_length(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "length() expects no arguments");
}

#[test]
fn test_array_length_on_non_array() {
    let mut vm = VirtualMachine::new();
    let not_array = Value::Number(42.0);
    let args = vec![not_array];

    let result = native_array_length(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "length() can only be called on arrays");
}

#[test]
fn test_array_push_different_types() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![Value::Number(1.0)]);

    // Push boolean
    let args = vec![array.clone(), Value::Boolean(true)];
    native_array_push(&mut vm, &args).unwrap();

    // Push nil
    let args = vec![array.clone(), Value::Nil];
    native_array_push(&mut vm, &args).unwrap();

    // Verify array contents
    match array {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 3);
                assert_eq!(contents[0], Value::Number(1.0));
                assert_eq!(contents[1], Value::Boolean(true));
                assert_eq!(contents[2], Value::Nil);
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_operations_sequence() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![]);

    // Start with empty array
    let args = vec![array.clone()];
    let len = native_array_length(&mut vm, &args).unwrap();
    assert_eq!(len, Value::Number(0.0));

    // Push three elements
    let args = vec![array.clone(), Value::Number(1.0)];
    native_array_push(&mut vm, &args).unwrap();

    let args = vec![array.clone(), Value::Number(2.0)];
    native_array_push(&mut vm, &args).unwrap();

    let args = vec![array.clone(), Value::Number(3.0)];
    native_array_push(&mut vm, &args).unwrap();

    // Check length
    let args = vec![array.clone()];
    let len = native_array_length(&mut vm, &args).unwrap();
    assert_eq!(len, Value::Number(3.0));

    // Pop one element
    let args = vec![array.clone()];
    let popped = native_array_pop(&mut vm, &args).unwrap();
    assert_eq!(popped, Value::Number(3.0));

    // Check length again
    let args = vec![array.clone()];
    let len = native_array_length(&mut vm, &args).unwrap();
    assert_eq!(len, Value::Number(2.0));

    // Verify final contents
    match array {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 2);
                assert_eq!(contents[0], Value::Number(1.0));
                assert_eq!(contents[1], Value::Number(2.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_size_with_elements() {
    let mut vm = VirtualMachine::new();
    let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ])))));
    let result = native_array_size(&mut vm, &[array]).unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_array_size_no_args() {
    let mut vm = VirtualMachine::new();
    let result = native_array_size(&mut vm, &[]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "array.size() requires an array receiver");
}

#[test]
fn test_array_size_wrong_type() {
    let mut vm = VirtualMachine::new();
    let result = native_array_size(&mut vm, &[Value::Number(42.0)]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "size() can only be called on arrays");
}

#[test]
fn test_array_contains_found() {
    let mut vm = VirtualMachine::new();
    let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ])))));
    let result = native_array_contains(&mut vm, &[array, Value::Number(2.0)]).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_array_contains_not_found() {
    let mut vm = VirtualMachine::new();
    let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ])))));
    let result = native_array_contains(&mut vm, &[array, Value::Number(5.0)]).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_array_contains_empty_array() {
    let mut vm = VirtualMachine::new();
    let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![])))));
    let result = native_array_contains(&mut vm, &[array, Value::Number(1.0)]).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_array_contains_string() {
    let mut vm = VirtualMachine::new();
    let string_val = Value::Object(Rc::new(Object::String(ObjString {
        value: "hello".into(),
    })));
    let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![
        string_val.clone(),
        Value::Number(2.0),
    ])))));
    let result = native_array_contains(&mut vm, &[array, string_val]).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_array_contains_wrong_arg_count() {
    let mut vm = VirtualMachine::new();
    let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![])))));
    let result = native_array_contains(&mut vm, &[array]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects 1 argument"));
}

#[test]
fn test_array_contains_wrong_type() {
    let mut vm = VirtualMachine::new();
    let result = native_array_contains(&mut vm, &[Value::Number(42.0), Value::Number(1.0)]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "contains() can only be called on arrays");
}

#[test]
fn test_array_sort_numbers() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Number(3.0),
        Value::Number(1.0),
        Value::Number(4.0),
        Value::Number(1.5),
        Value::Number(9.0),
    ]);
    let args = vec![array.clone()];

    let result = native_array_sort(&mut vm, &args).unwrap();
    assert_eq!(result, Value::Nil);

    match array {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 5);
                assert_eq!(contents[0], Value::Number(1.0));
                assert_eq!(contents[1], Value::Number(1.5));
                assert_eq!(contents[2], Value::Number(3.0));
                assert_eq!(contents[3], Value::Number(4.0));
                assert_eq!(contents[4], Value::Number(9.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_sort_strings() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Object(Rc::new(Object::String(ObjString { value: "cherry".into() }))),
        Value::Object(Rc::new(Object::String(ObjString { value: "apple".into() }))),
        Value::Object(Rc::new(Object::String(ObjString { value: "banana".into() }))),
    ]);
    let args = vec![array.clone()];

    let result = native_array_sort(&mut vm, &args).unwrap();
    assert_eq!(result, Value::Nil);

    match array {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 3);
                if let Value::Object(obj) = &contents[0] {
                    if let Object::String(s) = obj.as_ref() {
                        assert_eq!(s.value.as_ref(), "apple");
                    }
                }
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_reverse() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);
    let args = vec![array.clone()];

    let result = native_array_reverse(&mut vm, &args).unwrap();
    assert_eq!(result, Value::Nil);

    match array {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 3);
                assert_eq!(contents[0], Value::Number(3.0));
                assert_eq!(contents[1], Value::Number(2.0));
                assert_eq!(contents[2], Value::Number(1.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_slice() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
        Value::Number(5.0),
    ]);
    let args = vec![array, Value::Number(1.0), Value::Number(4.0)];

    let result = native_array_slice(&mut vm, &args).unwrap();

    match result {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 3);
                assert_eq!(contents[0], Value::Number(2.0));
                assert_eq!(contents[1], Value::Number(3.0));
                assert_eq!(contents[2], Value::Number(4.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_slice_negative_indices() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
        Value::Number(4.0),
        Value::Number(5.0),
    ]);
    let args = vec![array, Value::Number(-3.0), Value::Number(-1.0)];

    let result = native_array_slice(&mut vm, &args).unwrap();

    match result {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 2);
                assert_eq!(contents[0], Value::Number(3.0));
                assert_eq!(contents[1], Value::Number(4.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_join() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);
    let delimiter = Value::Object(Rc::new(Object::String(ObjString { value: ", ".into() })));
    let args = vec![array, delimiter];

    let result = native_array_join(&mut vm, &args).unwrap();

    match result {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => {
                assert_eq!(s.value.as_ref(), "1, 2, 3");
            }
            _ => panic!("Expected string"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_index_of_found() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);
    let args = vec![array, Value::Number(2.0)];

    let result = native_array_index_of(&mut vm, &args).unwrap();
    assert_eq!(result, Value::Number(1.0));
}

#[test]
fn test_array_index_of_not_found() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);
    let args = vec![array, Value::Number(5.0)];

    let result = native_array_index_of(&mut vm, &args).unwrap();
    assert_eq!(result, Value::Number(-1.0));
}

#[test]
fn test_array_sum() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);
    let args = vec![array];

    let result = native_array_sum(&mut vm, &args).unwrap();
    assert_eq!(result, Value::Number(6.0));
}

#[test]
fn test_array_sum_empty() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![]);
    let args = vec![array];

    let result = native_array_sum(&mut vm, &args).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_array_sum_non_numeric() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Number(1.0),
        Value::Boolean(true),
        Value::Number(3.0),
    ]);
    let args = vec![array];

    let result = native_array_sum(&mut vm, &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("requires all elements to be numbers"));
}

#[test]
fn test_array_min_numbers() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Number(3.0),
        Value::Number(1.0),
        Value::Number(4.0),
    ]);
    let args = vec![array];

    let result = native_array_min(&mut vm, &args).unwrap();
    assert_eq!(result, Value::Number(1.0));
}

#[test]
fn test_array_min_empty() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![]);
    let args = vec![array];

    let result = native_array_min(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "min() cannot be called on an empty array");
}

#[test]
fn test_array_max_numbers() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Number(3.0),
        Value::Number(1.0),
        Value::Number(4.0),
    ]);
    let args = vec![array];

    let result = native_array_max(&mut vm, &args).unwrap();
    assert_eq!(result, Value::Number(4.0));
}

#[test]
fn test_array_max_strings() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![
        Value::Object(Rc::new(Object::String(ObjString { value: "apple".into() }))),
        Value::Object(Rc::new(Object::String(ObjString { value: "banana".into() }))),
        Value::Object(Rc::new(Object::String(ObjString { value: "cherry".into() }))),
    ]);
    let args = vec![array];

    let result = native_array_max(&mut vm, &args).unwrap();
    match result {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => {
                assert_eq!(s.value.as_ref(), "cherry");
            }
            _ => panic!("Expected string"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_max_empty() {
    let mut vm = VirtualMachine::new();
    let array = Value::new_array(vec![]);
    let args = vec![array];

    let result = native_array_max(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "max() cannot be called on an empty array");
}
