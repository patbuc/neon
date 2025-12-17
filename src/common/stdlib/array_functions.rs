use crate::common::{Object, Value};

/// Native implementation of Array.push(value)
/// Adds an element to the end of the array and returns nil
pub fn native_array_push(args: &[Value]) -> Result<Value, String> {
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
pub fn native_array_pop(args: &[Value]) -> Result<Value, String> {
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
pub fn native_array_length(args: &[Value]) -> Result<Value, String> {
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
pub fn native_array_size(args: &[Value]) -> Result<Value, String> {
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
pub fn native_array_contains(args: &[Value]) -> Result<Value, String> {
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
pub fn native_array_sort(args: &[Value]) -> Result<Value, String> {
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
    array.sort_by(|a, b| match (a, b) {
        (Value::Number(n1), Value::Number(n2)) => {
            n1.partial_cmp(n2).unwrap_or(std::cmp::Ordering::Equal)
        }
        (Value::Object(o1), Value::Object(o2)) => match (o1.as_ref(), o2.as_ref()) {
            (Object::String(s1), Object::String(s2)) => s1.value.cmp(&s2.value),
            _ => std::cmp::Ordering::Equal,
        },
        (Value::Number(_), _) => std::cmp::Ordering::Less,
        (_, Value::Number(_)) => std::cmp::Ordering::Greater,
        (Value::Object(_), _) => std::cmp::Ordering::Less,
        (_, Value::Object(_)) => std::cmp::Ordering::Greater,
        _ => std::cmp::Ordering::Equal,
    });

    Ok(Value::Nil)
}

/// Native implementation of Array.reverse()
/// Reverses array in place
pub fn native_array_reverse(args: &[Value]) -> Result<Value, String> {
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
pub fn native_array_slice(args: &[Value]) -> Result<Value, String> {
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
pub fn native_array_join(args: &[Value]) -> Result<Value, String> {
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

    Ok(Value::Object(std::rc::Rc::new(Object::String(
        crate::common::ObjString {
            value: std::rc::Rc::from(result),
        },
    ))))
}

/// Native implementation of Array.indexOf(element)
/// Finds first occurrence index (-1 if not found)
pub fn native_array_index_of(args: &[Value]) -> Result<Value, String> {
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
pub fn native_array_sum(args: &[Value]) -> Result<Value, String> {
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
            _ => {
                return Err(format!(
                    "sum() requires all elements to be numbers, but element at index {} is not",
                    i
                ))
            }
        }
    }

    Ok(Value::Number(sum))
}

/// Native implementation of Array.min()
/// Finds minimum value in array
pub fn native_array_min(args: &[Value]) -> Result<Value, String> {
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
            (Value::Object(o1), Value::Object(o2)) => match (o1.as_ref(), o2.as_ref()) {
                (Object::String(s1), Object::String(s2)) => s1.value < s2.value,
                _ => return Err("min() can only compare numbers or strings".to_string()),
            },
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
pub fn native_array_max(args: &[Value]) -> Result<Value, String> {
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
            (Value::Object(o1), Value::Object(o2)) => match (o1.as_ref(), o2.as_ref()) {
                (Object::String(s1), Object::String(s2)) => s1.value > s2.value,
                _ => return Err("max() can only compare numbers or strings".to_string()),
            },
            _ => return Err("max() can only compare numbers or strings".to_string()),
        };

        if is_greater {
            max = value;
        }
    }

    Ok(max.clone())
}
