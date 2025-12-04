use crate::common::{Value, Object, SetKey};
use crate::vm::VirtualMachine;
use std::rc::Rc;
use std::collections::BTreeSet;
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

/// Native implementation of Set.union(other)
/// Returns a new set with all elements from both sets
pub fn native_set_union(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "union() expects 1 argument (other set), got {}",
            args.len() - 1
        ));
    }

    // Extract the first set (receiver)
    let set1_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Set(s) => s,
            _ => return Err("union() can only be called on sets".to_string()),
        },
        _ => return Err("union() can only be called on sets".to_string()),
    };

    // Extract the second set (argument)
    let set2_ref = match &args[1] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Set(s) => s,
            _ => return Err("union() requires a set as argument".to_string()),
        },
        _ => return Err("union() requires a set as argument".to_string()),
    };

    // Create union of both sets
    let set1 = set1_ref.borrow();
    let set2 = set2_ref.borrow();
    let union_set: BTreeSet<SetKey> = set1.union(&*set2).cloned().collect();

    Ok(Value::new_set(union_set))
}

/// Native implementation of Set.intersection(other)
/// Returns a new set with only elements common to both sets
pub fn native_set_intersection(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "intersection() expects 1 argument (other set), got {}",
            args.len() - 1
        ));
    }

    // Extract the first set (receiver)
    let set1_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Set(s) => s,
            _ => return Err("intersection() can only be called on sets".to_string()),
        },
        _ => return Err("intersection() can only be called on sets".to_string()),
    };

    // Extract the second set (argument)
    let set2_ref = match &args[1] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Set(s) => s,
            _ => return Err("intersection() requires a set as argument".to_string()),
        },
        _ => return Err("intersection() requires a set as argument".to_string()),
    };

    // Create intersection of both sets
    let set1 = set1_ref.borrow();
    let set2 = set2_ref.borrow();
    let intersection_set: BTreeSet<SetKey> = set1.intersection(&*set2).cloned().collect();

    Ok(Value::new_set(intersection_set))
}

/// Native implementation of Set.difference(other)
/// Returns a new set with elements in the first set but not in the second
pub fn native_set_difference(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "difference() expects 1 argument (other set), got {}",
            args.len() - 1
        ));
    }

    // Extract the first set (receiver)
    let set1_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Set(s) => s,
            _ => return Err("difference() can only be called on sets".to_string()),
        },
        _ => return Err("difference() can only be called on sets".to_string()),
    };

    // Extract the second set (argument)
    let set2_ref = match &args[1] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Set(s) => s,
            _ => return Err("difference() requires a set as argument".to_string()),
        },
        _ => return Err("difference() requires a set as argument".to_string()),
    };

    // Create difference of both sets
    let set1 = set1_ref.borrow();
    let set2 = set2_ref.borrow();
    let difference_set: BTreeSet<SetKey> = set1.difference(&*set2).cloned().collect();

    Ok(Value::new_set(difference_set))
}

/// Native implementation of Set.isSubset(other)
/// Returns true if all elements of the first set are in the second set
pub fn native_set_is_subset(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "isSubset() expects 1 argument (other set), got {}",
            args.len() - 1
        ));
    }

    // Extract the first set (receiver)
    let set1_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Set(s) => s,
            _ => return Err("isSubset() can only be called on sets".to_string()),
        },
        _ => return Err("isSubset() can only be called on sets".to_string()),
    };

    // Extract the second set (argument)
    let set2_ref = match &args[1] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Set(s) => s,
            _ => return Err("isSubset() requires a set as argument".to_string()),
        },
        _ => return Err("isSubset() requires a set as argument".to_string()),
    };

    // Check if first set is a subset of second set
    let set1 = set1_ref.borrow();
    let set2 = set2_ref.borrow();
    let is_subset = set1.is_subset(&*set2);

    Ok(Value::Boolean(is_subset))
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

/// Helper function to convert a SetKey back to a Value
fn set_key_to_value(key: &SetKey) -> Value {
    match key {
        SetKey::String(s) => {
            Value::Object(Rc::new(Object::String(crate::common::ObjString {
                value: Rc::clone(s),
            })))
        }
        SetKey::Number(n) => Value::Number(n.0),
        SetKey::Boolean(b) => Value::Boolean(*b),
    }
}

/// Native implementation of Set.toArray()
/// Returns a new array containing all elements from the set
pub fn native_set_to_array(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("toArray() expects no arguments".to_string());
    }

    // Extract the set
    let set_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Set(s) => s,
            _ => return Err("toArray() can only be called on sets".to_string()),
        },
        _ => return Err("toArray() can only be called on sets".to_string()),
    };

    // Convert set elements to array
    let set = set_ref.borrow();
    let array_elements: Vec<Value> = set.iter().map(set_key_to_value).collect();

    Ok(Value::new_array(array_elements))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::string;
    use crate::as_number;
    use std::collections::BTreeSet;

    #[test]
    fn test_set_add_new_element() {
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
        let mut elements = BTreeSet::new();
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
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let element = Value::Nil;
        let args = vec![set, element];

        let result = native_set_add(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid set element type"));
    }

    #[test]
    fn test_set_remove_existing_element() {
        let mut vm = VirtualMachine::new(Vec::new());
        let mut elements = BTreeSet::new();
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
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let element = Value::Number(42.0);
        let args = vec![set, element];

        let result = native_set_remove(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_set_has_existing_element() {
        let mut vm = VirtualMachine::new(Vec::new());
        let mut elements = BTreeSet::new();
        elements.insert(SetKey::String(Rc::from("hello")));
        let set = Value::new_set(elements);
        let element = string!("hello");
        let args = vec![set, element];

        let result = native_set_has(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_set_has_nonexistent_element() {
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let element = string!("hello");
        let args = vec![set, element];

        let result = native_set_has(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_set_size_empty() {
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let args = vec![set];

        let result = native_set_size(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 0.0);
    }

    #[test]
    fn test_set_size_with_elements() {
        let mut vm = VirtualMachine::new(Vec::new());
        let mut elements = BTreeSet::new();
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
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
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
        let mut vm = VirtualMachine::new(Vec::new());
        let mut elements = BTreeSet::new();
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
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let args = vec![set];

        let result = native_set_add(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 1 argument"));
    }

    #[test]
    fn test_set_remove_wrong_arg_count() {
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let args = vec![set];

        let result = native_set_remove(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 1 argument"));
    }

    #[test]
    fn test_set_has_wrong_arg_count() {
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let args = vec![set];

        let result = native_set_has(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 1 argument"));
    }

    #[test]
    fn test_set_size_wrong_arg_count() {
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let element = Value::Number(42.0);
        let args = vec![set, element];

        let result = native_set_size(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects no arguments"));
    }

    #[test]
    fn test_set_clear_wrong_arg_count() {
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let element = Value::Number(42.0);
        let args = vec![set, element];

        let result = native_set_clear(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects no arguments"));
    }

    #[test]
    fn test_set_methods_on_non_set() {
        let mut vm = VirtualMachine::new(Vec::new());
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

    // Tests for set algebra methods

    #[test]
    fn test_set_union_basic() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create set1 = {1, 2, 3}
        let mut elements1 = BTreeSet::new();
        elements1.insert(SetKey::Number(OrderedFloat(1.0)));
        elements1.insert(SetKey::Number(OrderedFloat(2.0)));
        elements1.insert(SetKey::Number(OrderedFloat(3.0)));
        let set1 = Value::new_set(elements1);

        // Create set2 = {3, 4, 5}
        let mut elements2 = BTreeSet::new();
        elements2.insert(SetKey::Number(OrderedFloat(3.0)));
        elements2.insert(SetKey::Number(OrderedFloat(4.0)));
        elements2.insert(SetKey::Number(OrderedFloat(5.0)));
        let set2 = Value::new_set(elements2);

        let args = vec![set1.clone(), set2.clone()];
        let result = native_set_union(&mut vm, &args).unwrap();

        // Result should be {1, 2, 3, 4, 5}
        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    let set_contents = s.borrow();
                    assert_eq!(set_contents.len(), 5);
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(1.0))));
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(2.0))));
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(3.0))));
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(4.0))));
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(5.0))));
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }

        // Verify original sets unchanged
        match set1 {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    assert_eq!(s.borrow().len(), 3);
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
        match set2 {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    assert_eq!(s.borrow().len(), 3);
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_union_empty_sets() {
        let mut vm = VirtualMachine::new(Vec::new());

        let set1 = Value::new_set(BTreeSet::new());
        let set2 = Value::new_set(BTreeSet::new());

        let args = vec![set1, set2];
        let result = native_set_union(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    assert_eq!(s.borrow().len(), 0);
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_union_one_empty() {
        let mut vm = VirtualMachine::new(Vec::new());

        let mut elements = BTreeSet::new();
        elements.insert(SetKey::Number(OrderedFloat(1.0)));
        elements.insert(SetKey::Number(OrderedFloat(2.0)));
        let set1 = Value::new_set(elements);
        let set2 = Value::new_set(BTreeSet::new());

        let args = vec![set1, set2];
        let result = native_set_union(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    let set_contents = s.borrow();
                    assert_eq!(set_contents.len(), 2);
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(1.0))));
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(2.0))));
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_union_mixed_types() {
        let mut vm = VirtualMachine::new(Vec::new());

        let mut elements1 = BTreeSet::new();
        elements1.insert(SetKey::Number(OrderedFloat(1.0)));
        elements1.insert(SetKey::String(Rc::from("hello")));
        let set1 = Value::new_set(elements1);

        let mut elements2 = BTreeSet::new();
        elements2.insert(SetKey::Boolean(true));
        elements2.insert(SetKey::String(Rc::from("world")));
        let set2 = Value::new_set(elements2);

        let args = vec![set1, set2];
        let result = native_set_union(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    let set_contents = s.borrow();
                    assert_eq!(set_contents.len(), 4);
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(1.0))));
                    assert!(set_contents.contains(&SetKey::String(Rc::from("hello"))));
                    assert!(set_contents.contains(&SetKey::Boolean(true)));
                    assert!(set_contents.contains(&SetKey::String(Rc::from("world"))));
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_intersection_basic() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create set1 = {1, 2, 3, 4}
        let mut elements1 = BTreeSet::new();
        elements1.insert(SetKey::Number(OrderedFloat(1.0)));
        elements1.insert(SetKey::Number(OrderedFloat(2.0)));
        elements1.insert(SetKey::Number(OrderedFloat(3.0)));
        elements1.insert(SetKey::Number(OrderedFloat(4.0)));
        let set1 = Value::new_set(elements1);

        // Create set2 = {3, 4, 5, 6}
        let mut elements2 = BTreeSet::new();
        elements2.insert(SetKey::Number(OrderedFloat(3.0)));
        elements2.insert(SetKey::Number(OrderedFloat(4.0)));
        elements2.insert(SetKey::Number(OrderedFloat(5.0)));
        elements2.insert(SetKey::Number(OrderedFloat(6.0)));
        let set2 = Value::new_set(elements2);

        let args = vec![set1.clone(), set2.clone()];
        let result = native_set_intersection(&mut vm, &args).unwrap();

        // Result should be {3, 4}
        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    let set_contents = s.borrow();
                    assert_eq!(set_contents.len(), 2);
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(3.0))));
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(4.0))));
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }

        // Verify original sets unchanged
        match set1 {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    assert_eq!(s.borrow().len(), 4);
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_intersection_no_common_elements() {
        let mut vm = VirtualMachine::new(Vec::new());

        let mut elements1 = BTreeSet::new();
        elements1.insert(SetKey::Number(OrderedFloat(1.0)));
        elements1.insert(SetKey::Number(OrderedFloat(2.0)));
        let set1 = Value::new_set(elements1);

        let mut elements2 = BTreeSet::new();
        elements2.insert(SetKey::Number(OrderedFloat(3.0)));
        elements2.insert(SetKey::Number(OrderedFloat(4.0)));
        let set2 = Value::new_set(elements2);

        let args = vec![set1, set2];
        let result = native_set_intersection(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    assert_eq!(s.borrow().len(), 0);
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_intersection_empty_sets() {
        let mut vm = VirtualMachine::new(Vec::new());

        let set1 = Value::new_set(BTreeSet::new());
        let set2 = Value::new_set(BTreeSet::new());

        let args = vec![set1, set2];
        let result = native_set_intersection(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    assert_eq!(s.borrow().len(), 0);
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_difference_basic() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create set1 = {1, 2, 3, 4}
        let mut elements1 = BTreeSet::new();
        elements1.insert(SetKey::Number(OrderedFloat(1.0)));
        elements1.insert(SetKey::Number(OrderedFloat(2.0)));
        elements1.insert(SetKey::Number(OrderedFloat(3.0)));
        elements1.insert(SetKey::Number(OrderedFloat(4.0)));
        let set1 = Value::new_set(elements1);

        // Create set2 = {3, 4, 5, 6}
        let mut elements2 = BTreeSet::new();
        elements2.insert(SetKey::Number(OrderedFloat(3.0)));
        elements2.insert(SetKey::Number(OrderedFloat(4.0)));
        elements2.insert(SetKey::Number(OrderedFloat(5.0)));
        elements2.insert(SetKey::Number(OrderedFloat(6.0)));
        let set2 = Value::new_set(elements2);

        let args = vec![set1.clone(), set2.clone()];
        let result = native_set_difference(&mut vm, &args).unwrap();

        // Result should be {1, 2} (in set1 but not in set2)
        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    let set_contents = s.borrow();
                    assert_eq!(set_contents.len(), 2);
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(1.0))));
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(2.0))));
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }

        // Verify original sets unchanged
        match set1 {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    assert_eq!(s.borrow().len(), 4);
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_difference_no_overlap() {
        let mut vm = VirtualMachine::new(Vec::new());

        let mut elements1 = BTreeSet::new();
        elements1.insert(SetKey::Number(OrderedFloat(1.0)));
        elements1.insert(SetKey::Number(OrderedFloat(2.0)));
        let set1 = Value::new_set(elements1);

        let mut elements2 = BTreeSet::new();
        elements2.insert(SetKey::Number(OrderedFloat(3.0)));
        elements2.insert(SetKey::Number(OrderedFloat(4.0)));
        let set2 = Value::new_set(elements2);

        let args = vec![set1, set2];
        let result = native_set_difference(&mut vm, &args).unwrap();

        // Result should be {1, 2} (all of set1)
        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    let set_contents = s.borrow();
                    assert_eq!(set_contents.len(), 2);
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(1.0))));
                    assert!(set_contents.contains(&SetKey::Number(OrderedFloat(2.0))));
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_difference_empty_result() {
        let mut vm = VirtualMachine::new(Vec::new());

        let mut elements1 = BTreeSet::new();
        elements1.insert(SetKey::Number(OrderedFloat(1.0)));
        elements1.insert(SetKey::Number(OrderedFloat(2.0)));
        let set1 = Value::new_set(elements1.clone());
        let set2 = Value::new_set(elements1);

        let args = vec![set1, set2];
        let result = native_set_difference(&mut vm, &args).unwrap();

        // Result should be empty (same elements)
        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    assert_eq!(s.borrow().len(), 0);
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_difference_empty_sets() {
        let mut vm = VirtualMachine::new(Vec::new());

        let set1 = Value::new_set(BTreeSet::new());
        let set2 = Value::new_set(BTreeSet::new());

        let args = vec![set1, set2];
        let result = native_set_difference(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Set(s) => {
                    assert_eq!(s.borrow().len(), 0);
                }
                _ => panic!("Expected set"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_is_subset_true() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create set1 = {1, 2}
        let mut elements1 = BTreeSet::new();
        elements1.insert(SetKey::Number(OrderedFloat(1.0)));
        elements1.insert(SetKey::Number(OrderedFloat(2.0)));
        let set1 = Value::new_set(elements1);

        // Create set2 = {1, 2, 3, 4}
        let mut elements2 = BTreeSet::new();
        elements2.insert(SetKey::Number(OrderedFloat(1.0)));
        elements2.insert(SetKey::Number(OrderedFloat(2.0)));
        elements2.insert(SetKey::Number(OrderedFloat(3.0)));
        elements2.insert(SetKey::Number(OrderedFloat(4.0)));
        let set2 = Value::new_set(elements2);

        let args = vec![set1, set2];
        let result = native_set_is_subset(&mut vm, &args).unwrap();

        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_set_is_subset_false() {
        let mut vm = VirtualMachine::new(Vec::new());

        // Create set1 = {1, 2, 5}
        let mut elements1 = BTreeSet::new();
        elements1.insert(SetKey::Number(OrderedFloat(1.0)));
        elements1.insert(SetKey::Number(OrderedFloat(2.0)));
        elements1.insert(SetKey::Number(OrderedFloat(5.0)));
        let set1 = Value::new_set(elements1);

        // Create set2 = {1, 2, 3, 4}
        let mut elements2 = BTreeSet::new();
        elements2.insert(SetKey::Number(OrderedFloat(1.0)));
        elements2.insert(SetKey::Number(OrderedFloat(2.0)));
        elements2.insert(SetKey::Number(OrderedFloat(3.0)));
        elements2.insert(SetKey::Number(OrderedFloat(4.0)));
        let set2 = Value::new_set(elements2);

        let args = vec![set1, set2];
        let result = native_set_is_subset(&mut vm, &args).unwrap();

        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_set_is_subset_equal_sets() {
        let mut vm = VirtualMachine::new(Vec::new());

        let mut elements = BTreeSet::new();
        elements.insert(SetKey::Number(OrderedFloat(1.0)));
        elements.insert(SetKey::Number(OrderedFloat(2.0)));
        let set1 = Value::new_set(elements.clone());
        let set2 = Value::new_set(elements);

        let args = vec![set1, set2];
        let result = native_set_is_subset(&mut vm, &args).unwrap();

        // A set is a subset of itself
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_set_is_subset_empty_set() {
        let mut vm = VirtualMachine::new(Vec::new());

        let set1 = Value::new_set(BTreeSet::new());

        let mut elements = BTreeSet::new();
        elements.insert(SetKey::Number(OrderedFloat(1.0)));
        let set2 = Value::new_set(elements);

        let args = vec![set1, set2];
        let result = native_set_is_subset(&mut vm, &args).unwrap();

        // Empty set is a subset of any set
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_set_is_subset_both_empty() {
        let mut vm = VirtualMachine::new(Vec::new());

        let set1 = Value::new_set(BTreeSet::new());
        let set2 = Value::new_set(BTreeSet::new());

        let args = vec![set1, set2];
        let result = native_set_is_subset(&mut vm, &args).unwrap();

        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_set_union_wrong_arg_count() {
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let args = vec![set];

        let result = native_set_union(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 1 argument"));
    }

    #[test]
    fn test_set_intersection_wrong_arg_count() {
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let args = vec![set];

        let result = native_set_intersection(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 1 argument"));
    }

    #[test]
    fn test_set_difference_wrong_arg_count() {
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let args = vec![set];

        let result = native_set_difference(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 1 argument"));
    }

    #[test]
    fn test_set_is_subset_wrong_arg_count() {
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let args = vec![set];

        let result = native_set_is_subset(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects 1 argument"));
    }

    #[test]
    fn test_set_algebra_on_non_set() {
        let mut vm = VirtualMachine::new(Vec::new());
        let not_a_set = Value::Number(42.0);
        let set = Value::new_set(BTreeSet::new());

        let union_result = native_set_union(&mut vm, &[not_a_set.clone(), set.clone()]);
        assert!(union_result.is_err());
        assert!(union_result.unwrap_err().contains("can only be called on sets"));

        let intersection_result = native_set_intersection(&mut vm, &[not_a_set.clone(), set.clone()]);
        assert!(intersection_result.is_err());
        assert!(intersection_result.unwrap_err().contains("can only be called on sets"));

        let difference_result = native_set_difference(&mut vm, &[not_a_set.clone(), set.clone()]);
        assert!(difference_result.is_err());
        assert!(difference_result.unwrap_err().contains("can only be called on sets"));

        let subset_result = native_set_is_subset(&mut vm, &[not_a_set, set]);
        assert!(subset_result.is_err());
        assert!(subset_result.unwrap_err().contains("can only be called on sets"));
    }

    #[test]
    fn test_set_algebra_with_non_set_argument() {
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let not_a_set = Value::Number(42.0);

        let union_result = native_set_union(&mut vm, &[set.clone(), not_a_set.clone()]);
        assert!(union_result.is_err());
        assert!(union_result.unwrap_err().contains("requires a set as argument"));

        let intersection_result = native_set_intersection(&mut vm, &[set.clone(), not_a_set.clone()]);
        assert!(intersection_result.is_err());
        assert!(intersection_result.unwrap_err().contains("requires a set as argument"));

        let difference_result = native_set_difference(&mut vm, &[set.clone(), not_a_set.clone()]);
        assert!(difference_result.is_err());
        assert!(difference_result.unwrap_err().contains("requires a set as argument"));

        let subset_result = native_set_is_subset(&mut vm, &[set, not_a_set]);
        assert!(subset_result.is_err());
        assert!(subset_result.unwrap_err().contains("requires a set as argument"));
    }

    // Tests for toArray method

    #[test]
    fn test_set_to_array_empty() {
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let args = vec![set];

        let result = native_set_to_array(&mut vm, &args).unwrap();

        // Verify it's an array
        match result {
            Value::Object(obj) => match obj.as_ref() {
                crate::common::Object::Array(arr) => {
                    let elements = arr.borrow();
                    assert_eq!(elements.len(), 0);
                }
                _ => panic!("Expected array"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_to_array_with_numbers() {
        let mut vm = VirtualMachine::new(Vec::new());
        let mut elements = BTreeSet::new();
        elements.insert(SetKey::Number(OrderedFloat(1.0)));
        elements.insert(SetKey::Number(OrderedFloat(2.0)));
        elements.insert(SetKey::Number(OrderedFloat(3.0)));
        let set = Value::new_set(elements);
        let args = vec![set];

        let result = native_set_to_array(&mut vm, &args).unwrap();

        // Verify it's an array with the correct elements
        match result {
            Value::Object(obj) => match obj.as_ref() {
                crate::common::Object::Array(arr) => {
                    let array_elements = arr.borrow();
                    assert_eq!(array_elements.len(), 3);

                    // Check that all elements are present (order not guaranteed)
                    let contains_1 = array_elements.iter().any(|v| matches!(v, Value::Number(n) if *n == 1.0));
                    let contains_2 = array_elements.iter().any(|v| matches!(v, Value::Number(n) if *n == 2.0));
                    let contains_3 = array_elements.iter().any(|v| matches!(v, Value::Number(n) if *n == 3.0));
                    assert!(contains_1);
                    assert!(contains_2);
                    assert!(contains_3);
                }
                _ => panic!("Expected array"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_to_array_with_strings() {
        let mut vm = VirtualMachine::new(Vec::new());
        let mut elements = BTreeSet::new();
        elements.insert(SetKey::String(Rc::from("apple")));
        elements.insert(SetKey::String(Rc::from("banana")));
        let set = Value::new_set(elements);
        let args = vec![set];

        let result = native_set_to_array(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                crate::common::Object::Array(arr) => {
                    let array_elements = arr.borrow();
                    assert_eq!(array_elements.len(), 2);

                    // Check that both strings are present
                    let mut has_apple = false;
                    let mut has_banana = false;
                    for element in array_elements.iter() {
                        if let Value::Object(obj) = element {
                            if let crate::common::Object::String(s) = obj.as_ref() {
                                if s.value.as_ref() == "apple" {
                                    has_apple = true;
                                } else if s.value.as_ref() == "banana" {
                                    has_banana = true;
                                }
                            }
                        }
                    }
                    assert!(has_apple);
                    assert!(has_banana);
                }
                _ => panic!("Expected array"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_to_array_with_booleans() {
        let mut vm = VirtualMachine::new(Vec::new());
        let mut elements = BTreeSet::new();
        elements.insert(SetKey::Boolean(true));
        elements.insert(SetKey::Boolean(false));
        let set = Value::new_set(elements);
        let args = vec![set];

        let result = native_set_to_array(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                crate::common::Object::Array(arr) => {
                    let array_elements = arr.borrow();
                    assert_eq!(array_elements.len(), 2);

                    // Check that both booleans are present
                    let has_true = array_elements.iter().any(|v| matches!(v, Value::Boolean(true)));
                    let has_false = array_elements.iter().any(|v| matches!(v, Value::Boolean(false)));
                    assert!(has_true);
                    assert!(has_false);
                }
                _ => panic!("Expected array"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_to_array_mixed_types() {
        let mut vm = VirtualMachine::new(Vec::new());
        let mut elements = BTreeSet::new();
        elements.insert(SetKey::Number(OrderedFloat(42.0)));
        elements.insert(SetKey::String(Rc::from("hello")));
        elements.insert(SetKey::Boolean(true));
        let set = Value::new_set(elements);
        let args = vec![set];

        let result = native_set_to_array(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                crate::common::Object::Array(arr) => {
                    let array_elements = arr.borrow();
                    assert_eq!(array_elements.len(), 3);

                    // Check that all types are present
                    let has_number = array_elements.iter().any(|v| matches!(v, Value::Number(n) if *n == 42.0));
                    let has_string = array_elements.iter().any(|v| {
                        if let Value::Object(obj) = v {
                            if let crate::common::Object::String(s) = obj.as_ref() {
                                return s.value.as_ref() == "hello";
                            }
                        }
                        false
                    });
                    let has_boolean = array_elements.iter().any(|v| matches!(v, Value::Boolean(true)));
                    assert!(has_number);
                    assert!(has_string);
                    assert!(has_boolean);
                }
                _ => panic!("Expected array"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_set_to_array_wrong_arg_count() {
        let mut vm = VirtualMachine::new(Vec::new());
        let set = Value::new_set(BTreeSet::new());
        let extra_arg = Value::Number(42.0);
        let args = vec![set, extra_arg];

        let result = native_set_to_array(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects no arguments"));
    }

    #[test]
    fn test_set_to_array_on_non_set() {
        let mut vm = VirtualMachine::new(Vec::new());
        let not_a_set = Value::Number(42.0);
        let args = vec![not_a_set];

        let result = native_set_to_array(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("can only be called on sets"));
    }
}
