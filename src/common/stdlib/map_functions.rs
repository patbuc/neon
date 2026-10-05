use crate::common::NativeContext;
use crate::common::{MapKey, NativeCallError, Value};
use crate::{extract_receiver, is_false_like};
use indexmap::IndexMap;

pub fn native_map_get(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "get() expects 1 argument (key), got {}",
            args.len() - 1
        ));
    }

    // Extract the map
    let map_ref = extract_receiver!(args, Map, "get")?;

    let key = MapKey::from_value(&args[1], "map key")?;

    // Get value from map
    let map = map_ref.borrow();
    Ok(map.get(&key).cloned().unwrap_or(Value::Nil))
}

pub fn native_map_size(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("size() expects no arguments".to_string());
    }

    // Extract the map
    let map_ref = extract_receiver!(args, Map, "size")?;

    let map = map_ref.borrow();
    Ok(Value::Int(map.len() as i64))
}

pub fn native_map_is_empty(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("isEmpty() expects no arguments".to_string());
    }

    // Extract the map
    let map_ref = extract_receiver!(args, Map, "isEmpty")?;

    let map = map_ref.borrow();
    Ok(Value::Boolean(map.is_empty()))
}

pub fn native_map_contains(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "contains() expects 1 argument (key), got {}",
            args.len() - 1
        ));
    }

    // Extract the map
    let map_ref = extract_receiver!(args, Map, "contains")?;

    let key = MapKey::from_value(&args[1], "map key")?;

    // Check if key exists
    let map = map_ref.borrow();
    Ok(Value::Boolean(map.contains_key(&key)))
}

pub fn native_map_remove(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "remove() expects 1 argument (key), got {}",
            args.len() - 1
        ));
    }

    // Extract the map
    let map_ref = extract_receiver!(args, Map, "remove")?;

    let key = MapKey::from_value(&args[1], "map key")?;

    // shift_remove, not swap_remove, so the remaining entries keep their order.
    let mut map = map_ref.borrow_mut();
    Ok(map.shift_remove(&key).unwrap_or(Value::Nil))
}

pub fn native_map_keys(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("keys() expects no arguments".to_string());
    }

    // Extract the map
    let map_ref = extract_receiver!(args, Map, "keys")?;

    // Collect keys into an array
    let map = map_ref.borrow();
    let keys: Vec<Value> = map.keys().map(MapKey::to_value).collect();
    Ok(Value::new_array(keys))
}

pub fn native_map_values(args: &[Value]) -> Result<Value, String> {
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

pub fn native_map_entries(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("entries() expects no arguments".to_string());
    }

    // Extract the map
    let map_ref = extract_receiver!(args, Map, "entries")?;

    // Collect entries as [key, value] arrays
    let map = map_ref.borrow();
    let entries: Vec<Value> = map
        .iter()
        .map(|(key, value)| Value::new_array(vec![key.to_value(), value.clone()]))
        .collect();
    Ok(Value::new_array(entries))
}

fn snapshot_entries(
    args: &[Value],
    name: &str,
) -> Result<(Vec<(MapKey, Value)>, Value), NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "{}() expects 1 argument (function), got {}",
            name,
            args.len() - 1
        )
        .into());
    }

    let map_ref = extract_receiver!(args, Map, name)?;
    let entries = map_ref
        .borrow()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    Ok((entries, args[1].clone()))
}

/// Native implementation of Map.forEach(fn)
/// Calls fn with (key, value) for each entry in insertion order. Returns nil.
pub fn native_map_for_each(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    let (entries, callback) = snapshot_entries(args, "forEach")?;
    for (key, value) in entries {
        vm.call_value(callback.clone(), &[key.to_value(), value])?;
    }
    Ok(Value::Nil)
}

/// Native implementation of Map.map(fn)
/// Returns an array of fn(key, value) for each entry in insertion order.
pub fn native_map_map(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    let (entries, callback) = snapshot_entries(args, "map")?;
    let mut mapped = Vec::with_capacity(entries.len());
    for (key, value) in entries {
        mapped.push(vm.call_value(callback.clone(), &[key.to_value(), value])?);
    }
    Ok(Value::new_array(mapped))
}

/// Native implementation of Map.filter(fn)
/// Returns a new map of the entries for which fn(key, value) is truthy.
pub fn native_map_filter(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    let (entries, callback) = snapshot_entries(args, "filter")?;
    let mut kept = IndexMap::new();
    for (key, value) in entries {
        let result = vm.call_value(callback.clone(), &[key.to_value(), value.clone()])?;
        if !is_false_like!(result) {
            kept.insert(key, value);
        }
    }
    Ok(Value::new_map(kept))
}

/// Native implementation of Map.mapValues(fn)
/// Returns a new map with the same keys and fn(key, value) as each value.
pub fn native_map_map_values(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    let (entries, callback) = snapshot_entries(args, "mapValues")?;
    let mut mapped = IndexMap::with_capacity(entries.len());
    for (key, value) in entries {
        let new_value = vm.call_value(callback.clone(), &[key.to_value(), value])?;
        mapped.insert(key, new_value);
    }
    Ok(Value::new_map(mapped))
}

/// Native implementation of Map.some(fn)
/// True if fn(key, value) is truthy for any entry, stopping at the first.
pub fn native_map_some(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    let (entries, callback) = snapshot_entries(args, "some")?;
    for (key, value) in entries {
        let result = vm.call_value(callback.clone(), &[key.to_value(), value])?;
        if !is_false_like!(result) {
            return Ok(Value::Boolean(true));
        }
    }
    Ok(Value::Boolean(false))
}

/// Native implementation of Map.every(fn)
/// True if fn(key, value) is truthy for every entry, stopping at the first
/// that isn't. True on an empty map.
pub fn native_map_every(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    let (entries, callback) = snapshot_entries(args, "every")?;
    for (key, value) in entries {
        let result = vm.call_value(callback.clone(), &[key.to_value(), value])?;
        if is_false_like!(result) {
            return Ok(Value::Boolean(false));
        }
    }
    Ok(Value::Boolean(true))
}
