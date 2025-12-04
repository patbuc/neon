use crate::common::{Value, Object, MapKey};
use crate::vm::VirtualMachine;
use std::rc::Rc;
use ordered_float::OrderedFloat;

/// Native implementation of Map.get(key)
/// Returns the value associated with the key, or nil if not found
pub fn native_map_get(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "get() expects 1 argument (key), got {}",
            args.len() - 1
        ));
    }

    // Extract the map
    let map_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Map(m) => m,
            _ => return Err("get() can only be called on maps".to_string()),
        },
        _ => return Err("get() can only be called on maps".to_string()),
    };

    // Convert key to MapKey
    let key = match value_to_map_key(&args[1]) {
        Some(k) => k,
        None => {
            return Err(format!(
                "Invalid map key type: {}. Only strings, numbers, and booleans can be used as map keys.",
                args[1]
            ));
        }
    };

    // Get value from map
    let map = map_ref.borrow();
    Ok(map.get(&key).cloned().unwrap_or(Value::Nil))
}

/// Native implementation of Map.size()
/// Returns the number of entries in the map
pub fn native_map_size(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("size() expects no arguments".to_string());
    }

    // Extract the map
    let map_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Map(m) => m,
            _ => return Err("size() can only be called on maps".to_string()),
        },
        _ => return Err("size() can only be called on maps".to_string()),
    };

    let map = map_ref.borrow();
    Ok(Value::Number(map.len() as f64))
}

/// Native implementation of Map.has(key)
/// Returns true if the map contains the key, false otherwise
pub fn native_map_has(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "has() expects 1 argument (key), got {}",
            args.len() - 1
        ));
    }

    // Extract the map
    let map_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Map(m) => m,
            _ => return Err("has() can only be called on maps".to_string()),
        },
        _ => return Err("has() can only be called on maps".to_string()),
    };

    // Convert key to MapKey
    let key = match value_to_map_key(&args[1]) {
        Some(k) => k,
        None => {
            return Err(format!(
                "Invalid map key type: {}. Only strings, numbers, and booleans can be used as map keys.",
                args[1]
            ));
        }
    };

    // Check if key exists
    let map = map_ref.borrow();
    Ok(Value::Boolean(map.contains_key(&key)))
}

/// Native implementation of Map.remove(key)
/// Removes the entry with the given key and returns its value, or nil if not found
pub fn native_map_remove(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "remove() expects 1 argument (key), got {}",
            args.len() - 1
        ));
    }

    // Extract the map
    let map_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Map(m) => m,
            _ => return Err("remove() can only be called on maps".to_string()),
        },
        _ => return Err("remove() can only be called on maps".to_string()),
    };

    // Convert key to MapKey
    let key = match value_to_map_key(&args[1]) {
        Some(k) => k,
        None => {
            return Err(format!(
                "Invalid map key type: {}. Only strings, numbers, and booleans can be used as map keys.",
                args[1]
            ));
        }
    };

    // Remove entry and return its value
    let mut map = map_ref.borrow_mut();
    Ok(map.remove(&key).unwrap_or(Value::Nil))
}

/// Native implementation of Map.keys()
/// Returns an array of all keys in the map
pub fn native_map_keys(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("keys() expects no arguments".to_string());
    }

    // Extract the map
    let map_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Map(m) => m,
            _ => return Err("keys() can only be called on maps".to_string()),
        },
        _ => return Err("keys() can only be called on maps".to_string()),
    };

    // Collect keys into an array
    let map = map_ref.borrow();
    let keys: Vec<Value> = map.keys().map(|key| map_key_to_value(key)).collect();
    Ok(Value::new_array(keys))
}

/// Native implementation of Map.values()
/// Returns an array of all values in the map
pub fn native_map_values(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("values() expects no arguments".to_string());
    }

    // Extract the map
    let map_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Map(m) => m,
            _ => return Err("values() can only be called on maps".to_string()),
        },
        _ => return Err("values() can only be called on maps".to_string()),
    };

    // Collect values into an array
    let map = map_ref.borrow();
    let values: Vec<Value> = map.values().cloned().collect();
    Ok(Value::new_array(values))
}

/// Native implementation of Map.entries()
/// Returns an array of [key, value] arrays
pub fn native_map_entries(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("entries() expects no arguments".to_string());
    }

    // Extract the map
    let map_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Map(m) => m,
            _ => return Err("entries() can only be called on maps".to_string()),
        },
        _ => return Err("entries() can only be called on maps".to_string()),
    };

    // Collect entries as [key, value] arrays
    let map = map_ref.borrow();
    let entries: Vec<Value> = map
        .iter()
        .map(|(key, value)| {
            Value::new_array(vec![map_key_to_value(key), value.clone()])
        })
        .collect();
    Ok(Value::new_array(entries))
}

/// Helper function to convert a Value to a MapKey
fn value_to_map_key(value: &Value) -> Option<MapKey> {
    match value {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => Some(MapKey::String(Rc::clone(&s.value))),
            _ => None,
        },
        Value::Number(n) => Some(MapKey::Number(OrderedFloat(*n))),
        Value::Boolean(b) => Some(MapKey::Boolean(*b)),
        Value::Nil => None,
    }
}

/// Helper function to convert a MapKey back to a Value
fn map_key_to_value(key: &MapKey) -> Value {
    match key {
        MapKey::String(s) => {
            use crate::common::ObjString;
            Value::Object(Rc::new(Object::String(ObjString {
                value: Rc::clone(s),
            })))
        }
        MapKey::Number(n) => Value::Number(n.into_inner()),
        MapKey::Boolean(b) => Value::Boolean(*b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::string;
    use crate::as_number;
    use std::collections::HashMap;

    #[test]
    fn test_map_get_existing_key() {
        let mut vm = VirtualMachine::new(Vec::new());
        let mut entries = HashMap::new();
        entries.insert(MapKey::String(Rc::from("name")), string!("Alice"));
        entries.insert(MapKey::Number(OrderedFloat(42.0)), Value::Number(100.0));
        let map = Value::new_map(entries);
        let key = string!("name");
        let args = vec![map, key];

        let result = native_map_get(&mut vm, &args).unwrap();
        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::String(s) => assert_eq!(s.value.as_ref(), "Alice"),
                _ => panic!("Expected string value"),
            },
            _ => panic!("Expected object value"),
        }
    }

    #[test]
    fn test_map_get_nonexistent_key() {
        let mut vm = VirtualMachine::new(Vec::new());
        let map = Value::new_map(HashMap::new());
        let key = string!("missing");
        let args = vec![map, key];

        let result = native_map_get(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Nil);
    }

    #[test]
    fn test_map_size_empty() {
        let mut vm = VirtualMachine::new(Vec::new());
        let map = Value::new_map(HashMap::new());
        let args = vec![map];

        let result = native_map_size(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 0.0);
    }

    #[test]
    fn test_map_size_with_entries() {
        let mut vm = VirtualMachine::new(Vec::new());
        let mut entries = HashMap::new();
        entries.insert(MapKey::String(Rc::from("a")), Value::Number(1.0));
        entries.insert(MapKey::String(Rc::from("b")), Value::Number(2.0));
        entries.insert(MapKey::String(Rc::from("c")), Value::Number(3.0));
        let map = Value::new_map(entries);
        let args = vec![map];

        let result = native_map_size(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 3.0);
    }

    #[test]
    fn test_map_has_existing_key() {
        let mut vm = VirtualMachine::new(Vec::new());
        let mut entries = HashMap::new();
        entries.insert(MapKey::String(Rc::from("name")), string!("Alice"));
        let map = Value::new_map(entries);
        let key = string!("name");
        let args = vec![map, key];

        let result = native_map_has(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_map_has_nonexistent_key() {
        let mut vm = VirtualMachine::new(Vec::new());
        let map = Value::new_map(HashMap::new());
        let key = string!("missing");
        let args = vec![map, key];

        let result = native_map_has(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_map_remove_existing_key() {
        let mut vm = VirtualMachine::new(Vec::new());
        let mut entries = HashMap::new();
        entries.insert(MapKey::String(Rc::from("name")), string!("Alice"));
        entries.insert(MapKey::String(Rc::from("age")), Value::Number(30.0));
        let map = Value::new_map(entries);
        let key = string!("name");
        let args = vec![map.clone(), key];

        let result = native_map_remove(&mut vm, &args).unwrap();

        // Check returned value
        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::String(s) => assert_eq!(s.value.as_ref(), "Alice"),
                _ => panic!("Expected string value"),
            },
            _ => panic!("Expected object value"),
        }

        // Verify the key was removed
        match map {
            Value::Object(obj) => match obj.as_ref() {
                Object::Map(m) => {
                    let map_contents = m.borrow();
                    assert!(!map_contents.contains_key(&MapKey::String(Rc::from("name"))));
                    assert_eq!(map_contents.len(), 1);
                }
                _ => panic!("Expected map"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_map_remove_nonexistent_key() {
        let mut vm = VirtualMachine::new(Vec::new());
        let map = Value::new_map(HashMap::new());
        let key = string!("missing");
        let args = vec![map, key];

        let result = native_map_remove(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Nil);
    }

    #[test]
    fn test_map_keys() {
        let mut vm = VirtualMachine::new(Vec::new());
        let mut entries = HashMap::new();
        entries.insert(MapKey::String(Rc::from("a")), Value::Number(1.0));
        entries.insert(MapKey::Number(OrderedFloat(42.0)), Value::Number(2.0));
        entries.insert(MapKey::Boolean(true), Value::Number(3.0));
        let map = Value::new_map(entries);
        let args = vec![map];

        let result = native_map_keys(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(arr) => {
                    let keys = arr.borrow();
                    assert_eq!(keys.len(), 3);
                    // Keys should include all three key types
                    // Order is not guaranteed in HashMap
                }
                _ => panic!("Expected array"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_map_values() {
        let mut vm = VirtualMachine::new(Vec::new());
        let mut entries = HashMap::new();
        entries.insert(MapKey::String(Rc::from("a")), Value::Number(1.0));
        entries.insert(MapKey::String(Rc::from("b")), Value::Number(2.0));
        entries.insert(MapKey::String(Rc::from("c")), Value::Number(3.0));
        let map = Value::new_map(entries);
        let args = vec![map];

        let result = native_map_values(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(arr) => {
                    let values = arr.borrow();
                    assert_eq!(values.len(), 3);
                    // Values should be present (order not guaranteed)
                }
                _ => panic!("Expected array"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_map_entries() {
        let mut vm = VirtualMachine::new(Vec::new());
        let mut entries = HashMap::new();
        entries.insert(MapKey::String(Rc::from("name")), string!("Alice"));
        entries.insert(MapKey::Number(OrderedFloat(42.0)), Value::Number(100.0));
        let map = Value::new_map(entries);
        let args = vec![map];

        let result = native_map_entries(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(arr) => {
                    let entries = arr.borrow();
                    assert_eq!(entries.len(), 2);

                    // Each entry should be an array of [key, value]
                    for entry in entries.iter() {
                        match entry {
                            Value::Object(entry_obj) => match entry_obj.as_ref() {
                                Object::Array(entry_arr) => {
                                    let pair = entry_arr.borrow();
                                    assert_eq!(pair.len(), 2);
                                }
                                _ => panic!("Expected array for entry"),
                            },
                            _ => panic!("Expected object for entry"),
                        }
                    }
                }
                _ => panic!("Expected array"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_map_keys_empty() {
        let mut vm = VirtualMachine::new(Vec::new());
        let map = Value::new_map(HashMap::new());
        let args = vec![map];

        let result = native_map_keys(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(arr) => {
                    let keys = arr.borrow();
                    assert_eq!(keys.len(), 0);
                }
                _ => panic!("Expected array"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_map_values_empty() {
        let mut vm = VirtualMachine::new(Vec::new());
        let map = Value::new_map(HashMap::new());
        let args = vec![map];

        let result = native_map_values(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(arr) => {
                    let values = arr.borrow();
                    assert_eq!(values.len(), 0);
                }
                _ => panic!("Expected array"),
            },
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_map_entries_empty() {
        let mut vm = VirtualMachine::new(Vec::new());
        let map = Value::new_map(HashMap::new());
        let args = vec![map];

        let result = native_map_entries(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => match obj.as_ref() {
                Object::Array(arr) => {
                    let entries = arr.borrow();
                    assert_eq!(entries.len(), 0);
                }
                _ => panic!("Expected array"),
            },
            _ => panic!("Expected object"),
        }
    }
}
