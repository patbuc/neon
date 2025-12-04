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

/// Native implementation of Array.sort()
/// Sorts array in place (numbers ascending, strings alphabetically)
pub fn native_array_sort(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sort() expects no arguments".to_string());
    }

    // Extract the array
    let array_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => arr,
            _ => return Err("sort() can only be called on arrays".to_string()),
        },
        _ => return Err("sort() can only be called on arrays".to_string()),
    };

    // Sort the array
    let mut array = array_ref.borrow_mut();

    // Sort with custom comparison that handles mixed types
    array.sort_by(|a, b| {
        match (a, b) {
            (Value::Number(n1), Value::Number(n2)) => {
                n1.partial_cmp(n2).unwrap_or(std::cmp::Ordering::Equal)
            }
            (Value::Object(o1), Value::Object(o2)) => {
                match (o1.as_ref(), o2.as_ref()) {
                    (Object::String(s1), Object::String(s2)) => {
                        s1.value.cmp(&s2.value)
                    }
                    _ => std::cmp::Ordering::Equal
                }
            }
            (Value::Number(_), _) => std::cmp::Ordering::Less,
            (_, Value::Number(_)) => std::cmp::Ordering::Greater,
            (Value::Object(_), _) => std::cmp::Ordering::Less,
            (_, Value::Object(_)) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal
        }
    });

    Ok(Value::Nil)
}

/// Native implementation of Array.reverse()
/// Reverses array in place
pub fn native_array_reverse(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("reverse() expects no arguments".to_string());
    }

    // Extract the array
    let array_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => arr,
            _ => return Err("reverse() can only be called on arrays".to_string()),
        },
        _ => return Err("reverse() can only be called on arrays".to_string()),
    };

    // Reverse the array
    let mut array = array_ref.borrow_mut();
    array.reverse();

    Ok(Value::Nil)
}

/// Native implementation of Array.slice(start, end)
/// Extracts a subarray (supports negative indices)
pub fn native_array_slice(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "slice() expects 2 arguments (start, end), got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => arr,
            _ => return Err("slice() can only be called on arrays".to_string()),
        },
        _ => return Err("slice() can only be called on arrays".to_string()),
    };

    // Extract start index
    let start = match &args[1] {
        Value::Number(n) => *n as i64,
        _ => return Err("slice() start index must be a number".to_string()),
    };

    // Extract end index
    let end = match &args[2] {
        Value::Number(n) => *n as i64,
        _ => return Err("slice() end index must be a number".to_string()),
    };

    let array = array_ref.borrow();
    let len = array.len() as i64;

    // Handle negative indices
    let start_idx = if start < 0 {
        (len + start).max(0) as usize
    } else {
        start.min(len) as usize
    };

    let end_idx = if end < 0 {
        (len + end).max(0) as usize
    } else {
        end.min(len) as usize
    };

    // Extract the slice
    let sliced = if start_idx < end_idx {
        array[start_idx..end_idx].to_vec()
    } else {
        vec![]
    };

    Ok(Value::new_array(sliced))
}

/// Native implementation of Array.join(delimiter)
/// Joins array elements into string
pub fn native_array_join(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "join() expects 1 argument (delimiter), got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => arr,
            _ => return Err("join() can only be called on arrays".to_string()),
        },
        _ => return Err("join() can only be called on arrays".to_string()),
    };

    // Extract delimiter
    let delimiter = match &args[1] {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => s.value.as_ref(),
            _ => return Err("join() delimiter must be a string".to_string()),
        },
        _ => return Err("join() delimiter must be a string".to_string()),
    };

    let array = array_ref.borrow();
    let parts: Vec<String> = array.iter().map(|v| format!("{}", v)).collect();
    let result = parts.join(delimiter);

    Ok(Value::Object(std::rc::Rc::new(Object::String(crate::common::ObjString {
        value: std::rc::Rc::from(result),
    }))))
}

/// Native implementation of Array.indexOf(element)
/// Finds first occurrence index (-1 if not found)
pub fn native_array_index_of(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "indexOf() expects 1 argument (element), got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => arr,
            _ => return Err("indexOf() can only be called on arrays".to_string()),
        },
        _ => return Err("indexOf() can only be called on arrays".to_string()),
    };

    let element = &args[1];
    let array = array_ref.borrow();

    // Find the element
    let index = array.iter().position(|e| e == element);

    match index {
        Some(idx) => Ok(Value::Number(idx as f64)),
        None => Ok(Value::Number(-1.0)),
    }
}

/// Native implementation of Array.sum()
/// Sums numeric array (error if non-numeric)
pub fn native_array_sum(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("sum() expects no arguments".to_string());
    }

    // Extract the array
    let array_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => arr,
            _ => return Err("sum() can only be called on arrays".to_string()),
        },
        _ => return Err("sum() can only be called on arrays".to_string()),
    };

    let array = array_ref.borrow();
    let mut sum = 0.0;

    for (i, value) in array.iter().enumerate() {
        match value {
            Value::Number(n) => sum += n,
            _ => return Err(format!("sum() requires all elements to be numbers, but element at index {} is not", i)),
        }
    }

    Ok(Value::Number(sum))
}

/// Native implementation of Array.min()
/// Finds minimum value in array
pub fn native_array_min(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("min() expects no arguments".to_string());
    }

    // Extract the array
    let array_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => arr,
            _ => return Err("min() can only be called on arrays".to_string()),
        },
        _ => return Err("min() can only be called on arrays".to_string()),
    };

    let array = array_ref.borrow();

    if array.is_empty() {
        return Err("min() cannot be called on an empty array".to_string());
    }

    // Find minimum - handle both numbers and strings
    let mut min = &array[0];

    for value in array.iter().skip(1) {
        let is_less = match (value, min) {
            (Value::Number(n1), Value::Number(n2)) => n1 < n2,
            (Value::Object(o1), Value::Object(o2)) => {
                match (o1.as_ref(), o2.as_ref()) {
                    (Object::String(s1), Object::String(s2)) => s1.value < s2.value,
                    _ => return Err("min() can only compare numbers or strings".to_string()),
                }
            }
            _ => return Err("min() can only compare numbers or strings".to_string()),
        };

        if is_less {
            min = value;
        }
    }

    Ok(min.clone())
}

/// Native implementation of Array.max()
/// Finds maximum value in array
pub fn native_array_max(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("max() expects no arguments".to_string());
    }

    // Extract the array
    let array_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Array(arr) => arr,
            _ => return Err("max() can only be called on arrays".to_string()),
        },
        _ => return Err("max() can only be called on arrays".to_string()),
    };

    let array = array_ref.borrow();

    if array.is_empty() {
        return Err("max() cannot be called on an empty array".to_string());
    }

    // Find maximum - handle both numbers and strings
    let mut max = &array[0];

    for value in array.iter().skip(1) {
        let is_greater = match (value, max) {
            (Value::Number(n1), Value::Number(n2)) => n1 > n2,
            (Value::Object(o1), Value::Object(o2)) => {
                match (o1.as_ref(), o2.as_ref()) {
                    (Object::String(s1), Object::String(s2)) => s1.value > s2.value,
                    _ => return Err("max() can only compare numbers or strings".to_string()),
                }
            }
            _ => return Err("max() can only compare numbers or strings".to_string()),
        };

        if is_greater {
            max = value;
        }
    }

    Ok(max.clone())
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
        use crate::common::ObjString;
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
        use crate::common::ObjString;
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
        use crate::common::ObjString;
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
}
