use crate::common::Object;
use crate::vm::array_functions::*;
use crate::vm::VirtualMachine;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_array_push() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![crate::common::Value::Number(1.0), crate::common::Value::Number(2.0)]);
    let value = crate::common::Value::Number(3.0);
    let args = vec![array.clone(), value];

    let result = native_array_push(&mut vm, &args).unwrap();

    // push returns nil
    assert_eq!(result, crate::common::Value::Nil);

    // Verify the array was modified
    match array {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 3);
                assert_eq!(contents[0], crate::common::Value::Number(1.0));
                assert_eq!(contents[1], crate::common::Value::Number(2.0));
                assert_eq!(contents[2], crate::common::Value::Number(3.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_push_to_empty() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![]);
    let value = crate::common::Value::Number(42.0);
    let args = vec![array.clone(), value];

    let result = native_array_push(&mut vm, &args).unwrap();
    assert_eq!(result, crate::common::Value::Nil);

    // Verify the array was modified
    match array {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 1);
                assert_eq!(contents[0], crate::common::Value::Number(42.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_push_wrong_arg_count() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![]);

    // Too few arguments
    let args = vec![array.clone()];
    let result = native_array_push(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "push() expects 1 argument (value), got 0");

    // Too many arguments
    let args = vec![array, crate::common::Value::Number(1.0), crate::common::Value::Number(2.0)];
    let result = native_array_push(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "push() expects 1 argument (value), got 2");
}

#[test]
fn test_array_push_on_non_array() {
    let mut vm = VirtualMachine::new();
    let not_array = crate::common::Value::Number(42.0);
    let value = crate::common::Value::Number(1.0);
    let args = vec![not_array, value];

    let result = native_array_push(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "push() can only be called on arrays");
}

#[test]
fn test_array_pop() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(2.0),
        crate::common::Value::Number(3.0),
    ]);
    let args = vec![array.clone()];

    let result = native_array_pop(&mut vm, &args).unwrap();

    // pop returns the last element
    assert_eq!(result, crate::common::Value::Number(3.0));

    // Verify the array was modified
    match array {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 2);
                assert_eq!(contents[0], crate::common::Value::Number(1.0));
                assert_eq!(contents[1], crate::common::Value::Number(2.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_pop_empty() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![]);
    let args = vec![array];

    let result = native_array_pop(&mut vm, &args).unwrap();

    // pop on empty array returns nil
    assert_eq!(result, crate::common::Value::Nil);
}

#[test]
fn test_array_pop_wrong_arg_count() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![crate::common::Value::Number(1.0)]);

    // Too many arguments
    let args = vec![array, crate::common::Value::Number(1.0)];
    let result = native_array_pop(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "pop() expects no arguments");
}

#[test]
fn test_array_pop_on_non_array() {
    let mut vm = VirtualMachine::new();
    let not_array = crate::common::Value::Number(42.0);
    let args = vec![not_array];

    let result = native_array_pop(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "pop() can only be called on arrays");
}

#[test]
fn test_array_length() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(2.0),
        crate::common::Value::Number(3.0),
    ]);
    let args = vec![array];

    let result = native_array_length(&mut vm, &args).unwrap();
    assert_eq!(result, crate::common::Value::Number(3.0));
}

#[test]
fn test_array_length_empty() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![]);
    let args = vec![array];

    let result = native_array_length(&mut vm, &args).unwrap();
    assert_eq!(result, crate::common::Value::Number(0.0));
}

#[test]
fn test_array_size_empty() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![])))));
    let result = native_array_size(&mut vm, &[array]).unwrap();
    assert_eq!(result, crate::common::Value::Number(0.0));
}

#[test]
fn test_array_length_wrong_arg_count() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![]);

    // Too many arguments
    let args = vec![array, crate::common::Value::Number(1.0)];
    let result = native_array_length(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "length() expects no arguments");
}

#[test]
fn test_array_length_on_non_array() {
    let mut vm = VirtualMachine::new();
    let not_array = crate::common::Value::Number(42.0);
    let args = vec![not_array];

    let result = native_array_length(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "length() can only be called on arrays");
}

#[test]
fn test_array_push_different_types() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![crate::common::Value::Number(1.0)]);

    // Push boolean
    let args = vec![array.clone(), crate::common::Value::Boolean(true)];
    native_array_push(&mut vm, &args).unwrap();

    // Push nil
    let args = vec![array.clone(), crate::common::Value::Nil];
    native_array_push(&mut vm, &args).unwrap();

    // Verify array contents
    match array {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 3);
                assert_eq!(contents[0], crate::common::Value::Number(1.0));
                assert_eq!(contents[1], crate::common::Value::Boolean(true));
                assert_eq!(contents[2], crate::common::Value::Nil);
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_operations_sequence() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![]);

    // Start with empty array
    let args = vec![array.clone()];
    let len = native_array_length(&mut vm, &args).unwrap();
    assert_eq!(len, crate::common::Value::Number(0.0));

    // Push three elements
    let args = vec![array.clone(), crate::common::Value::Number(1.0)];
    native_array_push(&mut vm, &args).unwrap();

    let args = vec![array.clone(), crate::common::Value::Number(2.0)];
    native_array_push(&mut vm, &args).unwrap();

    let args = vec![array.clone(), crate::common::Value::Number(3.0)];
    native_array_push(&mut vm, &args).unwrap();

    // Check length
    let args = vec![array.clone()];
    let len = native_array_length(&mut vm, &args).unwrap();
    assert_eq!(len, crate::common::Value::Number(3.0));

    // Pop one element
    let args = vec![array.clone()];
    let popped = native_array_pop(&mut vm, &args).unwrap();
    assert_eq!(popped, crate::common::Value::Number(3.0));

    // Check length again
    let args = vec![array.clone()];
    let len = native_array_length(&mut vm, &args).unwrap();
    assert_eq!(len, crate::common::Value::Number(2.0));

    // Verify final contents
    match array {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 2);
                assert_eq!(contents[0], crate::common::Value::Number(1.0));
                assert_eq!(contents[1], crate::common::Value::Number(2.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_size_with_elements() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(2.0),
        crate::common::Value::Number(3.0),
    ])))));
    let result = native_array_size(&mut vm, &[array]).unwrap();
    assert_eq!(result, crate::common::Value::Number(3.0));
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
    let result = native_array_size(&mut vm, &[crate::common::Value::Number(42.0)]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "size() can only be called on arrays");
}

#[test]
fn test_array_contains_found() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(2.0),
        crate::common::Value::Number(3.0),
    ])))));
    let result = native_array_contains(&mut vm, &[array, crate::common::Value::Number(2.0)]).unwrap();
    assert_eq!(result, crate::common::Value::Boolean(true));
}

#[test]
fn test_array_contains_not_found() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(2.0),
        crate::common::Value::Number(3.0),
    ])))));
    let result = native_array_contains(&mut vm, &[array, crate::common::Value::Number(5.0)]).unwrap();
    assert_eq!(result, crate::common::Value::Boolean(false));
}

#[test]
fn test_array_contains_empty_array() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![])))));
    let result = native_array_contains(&mut vm, &[array, crate::common::Value::Number(1.0)]).unwrap();
    assert_eq!(result, crate::common::Value::Boolean(false));
}

#[test]
fn test_array_contains_string() {
    use crate::common::ObjString;
    let mut vm = VirtualMachine::new();
    let string_val = crate::common::Value::Object(Rc::new(Object::String(ObjString {
        value: "hello".into(),
    })));
    let array = crate::common::Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![
        string_val.clone(),
        crate::common::Value::Number(2.0),
    ])))));
    let result = native_array_contains(&mut vm, &[array, string_val]).unwrap();
    assert_eq!(result, crate::common::Value::Boolean(true));
}

#[test]
fn test_array_contains_wrong_arg_count() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![])))));
    let result = native_array_contains(&mut vm, &[array]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects 1 argument"));
}

#[test]
fn test_array_contains_wrong_type() {
    let mut vm = VirtualMachine::new();
    let result = native_array_contains(&mut vm, &[crate::common::Value::Number(42.0), crate::common::Value::Number(1.0)]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "contains() can only be called on arrays");
}

#[test]
fn test_array_sort_numbers() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Number(3.0),
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(4.0),
        crate::common::Value::Number(1.5),
        crate::common::Value::Number(9.0),
    ]);
    let args = vec![array.clone()];

    let result = native_array_sort(&mut vm, &args).unwrap();
    assert_eq!(result, crate::common::Value::Nil);

    match array {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 5);
                assert_eq!(contents[0], crate::common::Value::Number(1.0));
                assert_eq!(contents[1], crate::common::Value::Number(1.5));
                assert_eq!(contents[2], crate::common::Value::Number(3.0));
                assert_eq!(contents[3], crate::common::Value::Number(4.0));
                assert_eq!(contents[4], crate::common::Value::Number(9.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_sort_strings() {
    use crate::common::ObjString;
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Object(Rc::new(Object::String(ObjString { value: "cherry".into() }))),
        crate::common::Value::Object(Rc::new(Object::String(ObjString { value: "apple".into() }))),
        crate::common::Value::Object(Rc::new(Object::String(ObjString { value: "banana".into() }))),
    ]);
    let args = vec![array.clone()];

    let result = native_array_sort(&mut vm, &args).unwrap();
    assert_eq!(result, crate::common::Value::Nil);

    match array {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 3);
                if let crate::common::Value::Object(obj) = &contents[0] {
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
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(2.0),
        crate::common::Value::Number(3.0),
    ]);
    let args = vec![array.clone()];

    let result = native_array_reverse(&mut vm, &args).unwrap();
    assert_eq!(result, crate::common::Value::Nil);

    match array {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 3);
                assert_eq!(contents[0], crate::common::Value::Number(3.0));
                assert_eq!(contents[1], crate::common::Value::Number(2.0));
                assert_eq!(contents[2], crate::common::Value::Number(1.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_slice() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(2.0),
        crate::common::Value::Number(3.0),
        crate::common::Value::Number(4.0),
        crate::common::Value::Number(5.0),
    ]);
    let args = vec![array, crate::common::Value::Number(1.0), crate::common::Value::Number(4.0)];

    let result = native_array_slice(&mut vm, &args).unwrap();

    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 3);
                assert_eq!(contents[0], crate::common::Value::Number(2.0));
                assert_eq!(contents[1], crate::common::Value::Number(3.0));
                assert_eq!(contents[2], crate::common::Value::Number(4.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_slice_negative_indices() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(2.0),
        crate::common::Value::Number(3.0),
        crate::common::Value::Number(4.0),
        crate::common::Value::Number(5.0),
    ]);
    let args = vec![array, crate::common::Value::Number(-3.0), crate::common::Value::Number(-1.0)];

    let result = native_array_slice(&mut vm, &args).unwrap();

    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => {
                let contents = arr.borrow();
                assert_eq!(contents.len(), 2);
                assert_eq!(contents[0], crate::common::Value::Number(3.0));
                assert_eq!(contents[1], crate::common::Value::Number(4.0));
            }
            _ => panic!("Expected array"),
        },
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_array_join() {
    use crate::common::ObjString;
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(2.0),
        crate::common::Value::Number(3.0),
    ]);
    let delimiter = crate::common::Value::Object(Rc::new(Object::String(ObjString { value: ", ".into() })));
    let args = vec![array, delimiter];

    let result = native_array_join(&mut vm, &args).unwrap();

    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
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
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(2.0),
        crate::common::Value::Number(3.0),
    ]);
    let args = vec![array, crate::common::Value::Number(2.0)];

    let result = native_array_index_of(&mut vm, &args).unwrap();
    assert_eq!(result, crate::common::Value::Number(1.0));
}

#[test]
fn test_array_index_of_not_found() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(2.0),
        crate::common::Value::Number(3.0),
    ]);
    let args = vec![array, crate::common::Value::Number(5.0)];

    let result = native_array_index_of(&mut vm, &args).unwrap();
    assert_eq!(result, crate::common::Value::Number(-1.0));
}

#[test]
fn test_array_sum() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(2.0),
        crate::common::Value::Number(3.0),
    ]);
    let args = vec![array];

    let result = native_array_sum(&mut vm, &args).unwrap();
    assert_eq!(result, crate::common::Value::Number(6.0));
}

#[test]
fn test_array_sum_empty() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![]);
    let args = vec![array];

    let result = native_array_sum(&mut vm, &args).unwrap();
    assert_eq!(result, crate::common::Value::Number(0.0));
}

#[test]
fn test_array_sum_non_numeric() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Number(1.0),
        crate::common::Value::Boolean(true),
        crate::common::Value::Number(3.0),
    ]);
    let args = vec![array];

    let result = native_array_sum(&mut vm, &args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("requires all elements to be numbers"));
}

#[test]
fn test_array_min_numbers() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Number(3.0),
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(4.0),
    ]);
    let args = vec![array];

    let result = native_array_min(&mut vm, &args).unwrap();
    assert_eq!(result, crate::common::Value::Number(1.0));
}

#[test]
fn test_array_min_empty() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![]);
    let args = vec![array];

    let result = native_array_min(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "min() cannot be called on an empty array");
}

#[test]
fn test_array_max_numbers() {
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Number(3.0),
        crate::common::Value::Number(1.0),
        crate::common::Value::Number(4.0),
    ]);
    let args = vec![array];

    let result = native_array_max(&mut vm, &args).unwrap();
    assert_eq!(result, crate::common::Value::Number(4.0));
}

#[test]
fn test_array_max_strings() {
    use crate::common::ObjString;
    let mut vm = VirtualMachine::new();
    let array = crate::common::Value::new_array(vec![
        crate::common::Value::Object(Rc::new(Object::String(ObjString { value: "apple".into() }))),
        crate::common::Value::Object(Rc::new(Object::String(ObjString { value: "banana".into() }))),
        crate::common::Value::Object(Rc::new(Object::String(ObjString { value: "cherry".into() }))),
    ]);
    let args = vec![array];

    let result = native_array_max(&mut vm, &args).unwrap();
    match result {
        crate::common::Value::Object(obj) => match obj.as_ref() {
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
    let array = crate::common::Value::new_array(vec![]);
    let args = vec![array];

    let result = native_array_max(&mut vm, &args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "max() cannot be called on an empty array");
}
