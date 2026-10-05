use crate::common::{MapKey, Value};
use crate::extract_receiver;

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
