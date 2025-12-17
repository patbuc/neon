use crate::as_number;
use crate::common::{MapKey, Object, Value};
use crate::string;
use crate::vm::map_functions::*;
use crate::vm::VirtualMachine;
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::rc::Rc;

#[test]
fn test_map_get_existing_key() {
    let vm = VirtualMachine::new();
    let mut entries = HashMap::new();
    entries.insert(MapKey::String(Rc::from("name")), string!("Alice"));
    entries.insert(MapKey::Number(OrderedFloat(42.0)), Value::Number(100.0));
    let map = Value::new_map(entries);
    let key = string!("name");
    let args = vec![map, key];

    let result = native_map_get(&args).unwrap();
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
    let vm = VirtualMachine::new();
    let map = Value::new_map(HashMap::new());
    let key = string!("missing");
    let args = vec![map, key];

    let result = native_map_get(&args).unwrap();
    assert_eq!(result, Value::Nil);
}

#[test]
fn test_map_size_empty() {
    let vm = VirtualMachine::new();
    let map = Value::new_map(HashMap::new());
    let args = vec![map];

    let result = native_map_size(&args).unwrap();
    assert_eq!(as_number!(result), 0.0);
}

#[test]
fn test_map_size_with_entries() {
    let vm = VirtualMachine::new();
    let mut entries = HashMap::new();
    entries.insert(MapKey::String(Rc::from("a")), Value::Number(1.0));
    entries.insert(MapKey::String(Rc::from("b")), Value::Number(2.0));
    entries.insert(MapKey::String(Rc::from("c")), Value::Number(3.0));
    let map = Value::new_map(entries);
    let args = vec![map];

    let result = native_map_size(&args).unwrap();
    assert_eq!(as_number!(result), 3.0);
}

#[test]
fn test_map_has_existing_key() {
    let vm = VirtualMachine::new();
    let mut entries = HashMap::new();
    entries.insert(MapKey::String(Rc::from("name")), string!("Alice"));
    let map = Value::new_map(entries);
    let key = string!("name");
    let args = vec![map, key];

    let result = native_map_has(&args).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_map_has_nonexistent_key() {
    let vm = VirtualMachine::new();
    let map = Value::new_map(HashMap::new());
    let key = string!("missing");
    let args = vec![map, key];

    let result = native_map_has(&args).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_map_remove_existing_key() {
    let vm = VirtualMachine::new();
    let mut entries = HashMap::new();
    entries.insert(MapKey::String(Rc::from("name")), string!("Alice"));
    entries.insert(MapKey::String(Rc::from("age")), Value::Number(30.0));
    let map = Value::new_map(entries);
    let key = string!("name");
    let args = vec![map.clone(), key];

    let result = native_map_remove(&args).unwrap();

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
    let vm = VirtualMachine::new();
    let map = Value::new_map(HashMap::new());
    let key = string!("missing");
    let args = vec![map, key];

    let result = native_map_remove(&args).unwrap();
    assert_eq!(result, Value::Nil);
}

#[test]
fn test_map_keys() {
    let vm = VirtualMachine::new();
    let mut entries = HashMap::new();
    entries.insert(MapKey::String(Rc::from("a")), Value::Number(1.0));
    entries.insert(MapKey::Number(OrderedFloat(42.0)), Value::Number(2.0));
    entries.insert(MapKey::Boolean(true), Value::Number(3.0));
    let map = Value::new_map(entries);
    let args = vec![map];

    let result = native_map_keys(&args).unwrap();

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
    let vm = VirtualMachine::new();
    let mut entries = HashMap::new();
    entries.insert(MapKey::String(Rc::from("a")), Value::Number(1.0));
    entries.insert(MapKey::String(Rc::from("b")), Value::Number(2.0));
    entries.insert(MapKey::String(Rc::from("c")), Value::Number(3.0));
    let map = Value::new_map(entries);
    let args = vec![map];

    let result = native_map_values(&args).unwrap();

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
    let vm = VirtualMachine::new();
    let mut entries = HashMap::new();
    entries.insert(MapKey::String(Rc::from("name")), string!("Alice"));
    entries.insert(MapKey::Number(OrderedFloat(42.0)), Value::Number(100.0));
    let map = Value::new_map(entries);
    let args = vec![map];

    let result = native_map_entries(&args).unwrap();

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
    let vm = VirtualMachine::new();
    let map = Value::new_map(HashMap::new());
    let args = vec![map];

    let result = native_map_keys(&args).unwrap();

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
    let vm = VirtualMachine::new();
    let map = Value::new_map(HashMap::new());
    let args = vec![map];

    let result = native_map_values(&args).unwrap();

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
    let vm = VirtualMachine::new();
    let map = Value::new_map(HashMap::new());
    let args = vec![map];

    let result = native_map_entries(&args).unwrap();

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
