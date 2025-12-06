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
