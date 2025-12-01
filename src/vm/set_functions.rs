use crate::common::{Value, Object, SetKey};
use crate::vm::VirtualMachine;
use std::rc::Rc;
use ordered_float::OrderedFloat;

/// Native implementation of Set.add(element)
/// Adds an element to the set, returns true if added (was not present), false otherwise
pub fn native_set_add(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "add() expects 1 argument (element), got {}",
            args.len() - 1
        ));
    }

    // Extract the set
    let set_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Set(s) => s,
            _ => return Err("add() can only be called on sets".to_string()),
        },
        _ => return Err("add() can only be called on sets".to_string()),
    };

    // Convert element to SetKey
    let key = match value_to_set_key(&args[1]) {
        Some(k) => k,
        None => {
            return Err(format!(
                "Invalid set element type: {}. Only strings, numbers, and booleans can be used as set elements.",
                args[1]
            ));
        }
    };

    // Add element to set
    let mut set = set_ref.borrow_mut();
    let was_added = set.insert(key);
    Ok(Value::Boolean(was_added))
}

/// Native implementation of Set.remove(element)
/// Removes an element from the set, returns true if removed (was present), false otherwise
pub fn native_set_remove(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "remove() expects 1 argument (element), got {}",
            args.len() - 1
        ));
    }

    // Extract the set
    let set_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Set(s) => s,
            _ => return Err("remove() can only be called on sets".to_string()),
        },
        _ => return Err("remove() can only be called on sets".to_string()),
    };

    // Convert element to SetKey
    let key = match value_to_set_key(&args[1]) {
        Some(k) => k,
        None => {
            return Err(format!(
                "Invalid set element type: {}. Only strings, numbers, and booleans can be used as set elements.",
                args[1]
            ));
        }
    };

    // Remove element from set
    let mut set = set_ref.borrow_mut();
    let was_removed = set.remove(&key);
    Ok(Value::Boolean(was_removed))
}

/// Native implementation of Set.has(element)
/// Returns true if the set contains the element, false otherwise
pub fn native_set_has(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "has() expects 1 argument (element), got {}",
            args.len() - 1
        ));
    }

    // Extract the set
    let set_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Set(s) => s,
            _ => return Err("has() can only be called on sets".to_string()),
        },
        _ => return Err("has() can only be called on sets".to_string()),
    };

    // Convert element to SetKey
    let key = match value_to_set_key(&args[1]) {
        Some(k) => k,
        None => {
            return Err(format!(
                "Invalid set element type: {}. Only strings, numbers, and booleans can be used as set elements.",
                args[1]
            ));
        }
    };

    // Check if element exists
    let set = set_ref.borrow();
    Ok(Value::Boolean(set.contains(&key)))
}

/// Native implementation of Set.size()
/// Returns the number of elements in the set as a number
pub fn native_set_size(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("size() expects no arguments".to_string());
    }

    // Extract the set
    let set_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Set(s) => s,
            _ => return Err("size() can only be called on sets".to_string()),
        },
        _ => return Err("size() can only be called on sets".to_string()),
    };

    let set = set_ref.borrow();
    Ok(Value::Number(set.len() as f64))
}

/// Native implementation of Set.clear()
/// Removes all elements from the set, returns nil
pub fn native_set_clear(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("clear() expects no arguments".to_string());
    }

    // Extract the set
    let set_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Set(s) => s,
            _ => return Err("clear() can only be called on sets".to_string()),
        },
        _ => return Err("clear() can only be called on sets".to_string()),
    };

    // Clear the set
    let mut set = set_ref.borrow_mut();
    set.clear();
    Ok(Value::Nil)
}

/// Helper function to convert a Value to a SetKey
fn value_to_set_key(value: &Value) -> Option<SetKey> {
    match value {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => Some(SetKey::String(Rc::clone(&s.value))),
            _ => None,
        },
        Value::Number(n) => Some(SetKey::Number(OrderedFloat(*n))),
        Value::Boolean(b) => Some(SetKey::Boolean(*b)),
        Value::Nil => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::string;
    use crate::as_number;
    use std::collections::HashSet;

    #[test]
    fn test_set_add_new_element() {
        let mut vm = VirtualMachine::new();
        let set = Value::new_set(HashSet::new());
        let element = Value::Number(42.0);
        let args = vec![set.clone(), element];

        let result = native_set_add(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(true));

        // Verify element was added
        match set {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    let set_contents = s.borrow();
                    assert_eq!(set_contents.len(), 1);
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(42.0))));
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_add_duplicate_element() {
        let mut vm = VirtualMachine::new();
        let mut elements = HashSet::new();
        elements.insert(SetKey::Number(OrderedFloat(42.0)));
        let set = Value::new_set(elements);
        let element = Value::Number(42.0);
        let args = vec![set.clone(), element];

        let result = native_set_add(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(false));

        // Verify size is still 1
        match set {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    let set_contents = s.borrow();
                    assert_eq!(set_contents.len(), 1);
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_add_string() {
        let mut vm = VirtualMachine::new();
        let set = Value::new_set(HashSet::new());
        let element = string!("hello");
        let args = vec![set.clone(), element];

        let result = native_set_add(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(true));

        // Verify element was added
        match set {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    let set_contents = s.borrow();
                    assert_eq!(set_contents.len(), 1);
                    assert!(set_contents.contains(&SetKey::String(Rc::from("hello"))));
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_add_boolean() {
        let mut vm = VirtualMachine::new();
        let set = Value::new_set(HashSet::new());
        let element = Value::Boolean(true);
        let args = vec![set.clone(), element];

        let result = native_set_add(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(true));

        // Verify element was added
        match set {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    let set_contents = s.borrow();
                    assert_eq!(set_contents.len(), 1);
                    assert!(set_contents.contains(&SetKey::Boolean(true)));
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_add_invalid_type() {
        let mut vm = VirtualMachine::new();
        let set = Value::new_set(HashSet::new());
        let element = Value::Nil;
        let args = vec![set, element];

        let result = native_set_add(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid set element type"));
    }

    #[test]
    fn test_set_remove_existing_element() {
        let mut vm = VirtualMachine::new();
        let mut elements = HashSet::new();
        elements.insert(SetKey::Number(OrderedFloat(42.0)));
        elements.insert(SetKey::Number(OrderedFloat(100.0)));
        let set = Value::new_set(elements);
        let element = Value::Number(42.0);
        let args = vec![set.clone(), element];

        let result = native_set_remove(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(true));

        // Verify element was removed
        match set {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    let set_contents = s.borrow();
                    assert_eq!(set_contents.len(), 1);
                    assert!(!set_contents.contains(&SetKey::Number(OrderedFloat(42.0))));
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(100.0))));
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_remove_nonexistent_element() {
        let mut vm = VirtualMachine::new();
        let set = Value::new_set(HashSet::new());
        let element = Value::Number(42.0);
        let args = vec![set, element];

        let result = native_set_remove(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_set_has_existing_element() {
        let mut vm = VirtualMachine::new();
        let mut elements = HashSet::new();
        elements.insert(SetKey::String(Rc::from("hello")));
        let set = Value::new_set(elements);
        let element = string!("hello");
        let args = vec![set, element];

        let result = native_set_has(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_set_has_nonexistent_element() {
        let mut vm = VirtualMachine::new();
        let set = Value::new_set(HashSet::new());
        let element = string!("hello");
        let args = vec![set, element];

        let result = native_set_has(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_set_size_empty() {
        let mut vm = VirtualMachine::new();
        let set = Value::new_set(HashSet::new());
        let args = vec![set];

        let result = native_set_size(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 0.0);
    }

    #[test]
    fn test_set_size_with_elements() {
        let mut vm = VirtualMachine::new();
        let mut elements = HashSet::new();
        elements.insert(SetKey::Number(OrderedFloat(1.0)));
        elements.insert(SetKey::Number(OrderedFloat(2.0)));
        elements.insert(SetKey::Number(OrderedFloat(3.0)));
        let set = Value::new_set(elements);
        let args = vec![set];

        let result = native_set_size(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 3.0);
    }

    #[test]
    fn test_set_clear_empty() {
        let mut vm = VirtualMachine::new();
        let set = Value::new_set(HashSet::new());
        let args = vec![set.clone()];

        let result = native_set_clear(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Nil);

        // Verify set is still empty
        match set {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    let set_contents = s.borrow();
                    assert_eq!(set_contents.len(), 0);
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_clear_with_elements() {
        let mut vm = VirtualMachine::new();
        let mut elements = HashSet::new();
        elements.insert(SetKey::Number(OrderedFloat(1.0)));
        elements.insert(SetKey::Number(OrderedFloat(2.0)));
        elements.insert(SetKey::Number(OrderedFloat(3.0)));
        let set = Value::new_set(elements);
        let args = vec![set.clone()];

        let result = native_set_clear(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Nil);

        // Verify set is empty
        match set {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    let set_contents = s.borrow();
                    assert_eq!(set_contents.len(), 0);
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_add_wrong_arg_count() {
        let mut vm = VirtualMachine::new();
        let set = Value::new_set(HashSet::new());
        let args = vec![set];

        let result = native_set_add(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 1 argument"));
    }

    #[test]
    fn test_set_remove_wrong_arg_count() {
        let mut vm = VirtualMachine::new();
        let set = Value::new_set(HashSet::new());
        let args = vec![set];

        let result = native_set_remove(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 1 argument"));
    }

    #[test]
    fn test_set_has_wrong_arg_count() {
        let mut vm = VirtualMachine::new();
        let set = Value::new_set(HashSet::new());
        let args = vec![set];

        let result = native_set_has(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 1 argument"));
    }

    #[test]
    fn test_set_size_wrong_arg_count() {
        let mut vm = VirtualMachine::new();
        let set = Value::new_set(HashSet::new());
        let element = Value::Number(42.0);
        let args = vec![set, element];

        let result = native_set_size(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects no arguments"));
    }

    #[test]
    fn test_set_clear_wrong_arg_count() {
        let mut vm = VirtualMachine::new();
        let set = Value::new_set(HashSet::new());
        let element = Value::Number(42.0);
        let args = vec![set, element];

        let result = native_set_clear(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects no arguments"));
    }

    #[test]
    fn test_set_methods_on_non_set() {
        let mut vm = VirtualMachine::new();
        let not_a_set = Value::Number(42.0);
        let element = Value::Number(1.0);

        let add_result = native_set_add(&mut vm, &[not_a_set.clone(), element.clone()]);
        assert!(add_result.is_err());
        assert!(add_result.unwrap_err().contains("can only be called on sets"));

        let remove_result = native_set_remove(&mut vm, &[not_a_set.clone(), element.clone()]);
        assert!(remove_result.is_err());
        assert!(remove_result.unwrap_err().contains("can only be called on sets"));

        let has_result = native_set_has(&mut vm, &[not_a_set.clone(), element]);
        assert!(has_result.is_err());
        assert!(has_result.unwrap_err().contains("can only be called on sets"));

        let size_result = native_set_size(&mut vm, &[not_a_set.clone()]);
        assert!(size_result.is_err());
        assert!(size_result.unwrap_err().contains("can only be called on sets"));

        let clear_result = native_set_clear(&mut vm, &[not_a_set]);
        assert!(clear_result.is_err());
        assert!(clear_result.unwrap_err().contains("can only be called on sets"));
    }
}
