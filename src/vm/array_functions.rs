use crate::common::{Value, Object};
use crate::vm::VirtualMachine;

/// Native implementation of Array.push(value)
/// Adds an element to the end of the array and returns nil
pub fn native_array_push(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "push() expects 1 argument (value), got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => arr,
            _ => return Err("push() can only be called on arrays".to_string()),
        },
        _ => return Err("push() can only be called on arrays".to_string()),
    };

    // Push the value onto the array
    let mut array = array_ref.borrow_mut();
    array.push(args[1].clone());

    Ok(Value::Nil)
}

/// Native implementation of Array.pop()
/// Removes and returns the last element of the array, or nil if the array is empty
pub fn native_array_pop(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("pop() expects no arguments".to_string());
    }

    // Extract the array
    let array_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => arr,
            _ => return Err("pop() can only be called on arrays".to_string()),
        },
        _ => return Err("pop() can only be called on arrays".to_string()),
    };

    // Pop the last element
    let mut array = array_ref.borrow_mut();
    Ok(array.pop().unwrap_or(Value::Nil))
}

/// Native implementation of Array.length()
/// Returns the number of elements in the array
pub fn native_array_length(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("length() expects no arguments".to_string());
    }

    // Extract the array
    let array_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => arr,
            _ => return Err("length() can only be called on arrays".to_string()),
        },
        _ => return Err("length() can only be called on arrays".to_string()),
    };

    let array = array_ref.borrow();
    Ok(Value::Number(array.len() as f64))
}

/// Native implementation of Array.size()
/// Returns the number of elements in the array
pub fn native_array_size(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("array.size() requires an array receiver".to_string());
    }

    match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(array) => {
                let elements = array.borrow();
                Ok(Value::Number(elements.len() as f64))
            }
            _ => Err("size() can only be called on arrays".to_string()),
        },
        _ => Err("size() can only be called on arrays".to_string()),
    }
}

/// Native implementation of Array.contains(element)
/// Returns true if the array contains the specified element
pub fn native_array_contains(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "contains() expects 1 argument (element), got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => arr,
            _ => return Err("contains() can only be called on arrays".to_string()),
        },
        _ => return Err("contains() can only be called on arrays".to_string()),
    };

    // Get the element to search for
    let element = &args[1];

    // Check if the array contains the element
    let elements = array.borrow();
    let contains = elements.iter().any(|e| e == element);

    Ok(Value::Boolean(contains))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Object;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_array_push() {
        let mut vm = VirtualMachine::new(Vec::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
        let not_array = Value::Number(42.0);
        let value = Value::Number(1.0);
        let args = vec![not_array, value];

        let result = native_array_push(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "push() can only be called on arrays");
    }

    #[test]
    fn test_array_pop() {
        let mut vm = VirtualMachine::new(Vec::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
        let array = Value::new_array(vec![]);
        let args = vec![array];

        let result = native_array_pop(&mut vm, &args).unwrap();

        // pop on empty array returns nil
        assert_eq!(result, Value::Nil);
    }

    #[test]
    fn test_array_pop_wrong_arg_count() {
        let mut vm = VirtualMachine::new(Vec::new());
        let array = Value::new_array(vec![Value::Number(1.0)]);

        // Too many arguments
        let args = vec![array, Value::Number(1.0)];
        let result = native_array_pop(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "pop() expects no arguments");
    }

    #[test]
    fn test_array_pop_on_non_array() {
        let mut vm = VirtualMachine::new(Vec::new());
        let not_array = Value::Number(42.0);
        let args = vec![not_array];

        let result = native_array_pop(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "pop() can only be called on arrays");
    }

    #[test]
    fn test_array_length() {
        let mut vm = VirtualMachine::new(Vec::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
        let array = Value::new_array(vec![]);
        let args = vec![array];

        let result = native_array_length(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_array_size_empty() {
        let mut vm = VirtualMachine::new(Vec::new());
        let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![])))));
        let result = native_array_size(&mut vm, &[array]).unwrap();
        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn test_array_length_wrong_arg_count() {
        let mut vm = VirtualMachine::new(Vec::new());
        let array = Value::new_array(vec![]);

        // Too many arguments
        let args = vec![array, Value::Number(1.0)];
        let result = native_array_length(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "length() expects no arguments");
    }

    #[test]
    fn test_array_length_on_non_array() {
        let mut vm = VirtualMachine::new(Vec::new());
        let not_array = Value::Number(42.0);
        let args = vec![not_array];

        let result = native_array_length(&mut vm, &args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "length() can only be called on arrays");
    }

    #[test]
    fn test_array_push_different_types() {
        let mut vm = VirtualMachine::new(Vec::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
        let result = native_array_size(&mut vm, &[]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "array.size() requires an array receiver");
    }

    #[test]
    fn test_array_size_wrong_type() {
        let mut vm = VirtualMachine::new(Vec::new());
        let result = native_array_size(&mut vm, &[Value::Number(42.0)]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "size() can only be called on arrays");
    }

    #[test]
    fn test_array_contains_found() {
        let mut vm = VirtualMachine::new(Vec::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
        let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![])))));
        let result = native_array_contains(&mut vm, &[array, Value::Number(1.0)]).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_array_contains_string() {
        use crate::common::ObjString;
        let mut vm = VirtualMachine::new(Vec::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
        let array = Value::Object(Rc::new(Object::Array(Rc::new(RefCell::new(vec![])))));
        let result = native_array_contains(&mut vm, &[array]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 1 argument"));
    }

    #[test]
    fn test_array_contains_wrong_type() {
        let mut vm = VirtualMachine::new(Vec::new());
        let result = native_array_contains(&mut vm, &[Value::Number(42.0), Value::Number(1.0)]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "contains() can only be called on arrays");
    }
}
