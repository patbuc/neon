use crate::common::{MapKey, Object, Value};
use crate::extract_receiver;
use crate::vm::VirtualMachine;
use ordered_float::OrderedFloat;
use std::rc::Rc;

pub fn native_map_get(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "get() expects 1 argument (key), got {}",
            args.len() - 1
        ));
    }

    // Extract the map
    let map_ref = extract_receiver!(args, Map, "get")?;

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

pub fn native_map_size(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("size() expects no arguments".to_string());
    }

    // Extract the map
    let map_ref = extract_receiver!(args, Map, "size")?;

    let map = map_ref.borrow();
    Ok(Value::Number(map.len() as f64))
}

pub fn native_map_has(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "has() expects 1 argument (key), got {}",
            args.len() - 1
        ));
    }

    // Extract the map
    let map_ref = extract_receiver!(args, Map, "has")?;

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

pub fn native_map_remove(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "remove() expects 1 argument (key), got {}",
            args.len() - 1
        ));
    }

    // Extract the map
    let map_ref = extract_receiver!(args, Map, "remove")?;

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

pub fn native_map_keys(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("keys() expects no arguments".to_string());
    }

    // Extract the map
    let map_ref = extract_receiver!(args, Map, "keys")?;

    // Collect keys into an array
    let map = map_ref.borrow();
    let keys: Vec<Value> = map.keys().map(map_key_to_value).collect();
    Ok(Value::new_array(keys))
}

pub fn native_map_values(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("values() expects no arguments".to_string());
    }

    // Extract the map
    let map_ref = extract_receiver!(args, Map, "values")?;

    // Collect values into an array
    let map = map_ref.borrow();
    let values: Vec<Value> = map.values().cloned().collect();
    Ok(Value::new_array(values))
}

pub fn native_map_entries(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("entries() expects no arguments".to_string());
    }

    // Extract the map
    let map_ref = extract_receiver!(args, Map, "entries")?;

    // Collect entries as [key, value] arrays
    let map = map_ref.borrow();
    let entries: Vec<Value> = map
        .iter()
        .map(|(key, value)| Value::new_array(vec![map_key_to_value(key), value.clone()]))
        .collect();
    Ok(Value::new_array(entries))
}

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
