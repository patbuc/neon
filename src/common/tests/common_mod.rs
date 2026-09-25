use crate::common::*;
use indexmap::IndexMap;
use ordered_float::OrderedFloat;
use std::collections::BTreeSet;
use std::rc::Rc;

#[test]
fn test_array_creation() {
    let arr = Value::new_array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);

    // Test that the array was created
    match arr {
        Value::Array(_) => {
            // Success - array was created
        }
        _ => panic!("Expected Array value"),
    }
}

#[test]
fn test_array_display() {
    let arr = Value::new_array(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]);

    let display = format!("{}", arr);
    assert_eq!(display, "[1, 2, 3]");
}

#[test]
fn test_empty_array_display() {
    let arr = Value::new_array(vec![]);
    let display = format!("{}", arr);
    assert_eq!(display, "[]");
}

#[test]
fn test_array_equality() {
    let arr1 = Value::new_array(vec![Value::Number(1.0), Value::Number(2.0)]);

    let arr2 = Value::new_array(vec![Value::Number(1.0), Value::Number(2.0)]);

    let arr3 = Value::new_array(vec![Value::Number(1.0), Value::Number(3.0)]);

    // These should be equal
    assert_eq!(arr1, arr2);

    // These should not be equal
    assert_ne!(arr1, arr3);
}

#[test]
fn test_mixed_type_array() {
    let arr = Value::new_array(vec![Value::Number(42.0), Value::Boolean(true), Value::Nil]);

    let display = format!("{}", arr);
    assert_eq!(display, "[42, true, nil]");
}

#[test]
fn test_map_creation() {
    let mut entries = IndexMap::new();
    entries.insert(
        MapKey::String(Rc::new("name".to_string())),
        Value::String(Rc::new("Alice".to_string())),
    );
    entries.insert(MapKey::Number(OrderedFloat(42.0)), Value::Number(100.0));

    let map = Value::new_map(entries);

    // Test that the map was created
    match map {
        Value::Map(_) => {
            // Success - map was created
        }
        _ => panic!("Expected Map value"),
    }
}

#[test]
fn test_map_display() {
    let mut entries = IndexMap::new();
    entries.insert(MapKey::String(Rc::new("a".to_string())), Value::Number(1.0));
    entries.insert(MapKey::String(Rc::new("b".to_string())), Value::Number(2.0));

    let map = Value::new_map(entries);
    let display = format!("{}", map);

    assert_eq!(display, "{a: 1, b: 2}");
}

#[test]
fn test_empty_map_display() {
    let map = Value::new_map(IndexMap::new());
    let display = format!("{}", map);
    assert_eq!(display, "{}");
}

#[test]
fn test_map_equality() {
    let mut entries1 = IndexMap::new();
    entries1.insert(MapKey::String(Rc::new("x".to_string())), Value::Number(1.0));
    entries1.insert(MapKey::String(Rc::new("y".to_string())), Value::Number(2.0));
    let map1 = Value::new_map(entries1);

    let mut entries2 = IndexMap::new();
    entries2.insert(MapKey::String(Rc::new("x".to_string())), Value::Number(1.0));
    entries2.insert(MapKey::String(Rc::new("y".to_string())), Value::Number(2.0));
    let map2 = Value::new_map(entries2);

    let mut entries3 = IndexMap::new();
    entries3.insert(MapKey::String(Rc::new("x".to_string())), Value::Number(1.0));
    entries3.insert(MapKey::String(Rc::new("y".to_string())), Value::Number(3.0));
    let map3 = Value::new_map(entries3);

    // These should be equal
    assert_eq!(map1, map2);

    // These should not be equal
    assert_ne!(map1, map3);
}

#[test]
fn test_map_equality_ignores_insertion_order() {
    let mut entries1 = IndexMap::new();
    entries1.insert(MapKey::String(Rc::new("x".to_string())), Value::Number(1.0));
    entries1.insert(MapKey::String(Rc::new("y".to_string())), Value::Number(2.0));
    let map1 = Value::new_map(entries1);

    let mut entries2 = IndexMap::new();
    entries2.insert(MapKey::String(Rc::new("y".to_string())), Value::Number(2.0));
    entries2.insert(MapKey::String(Rc::new("x".to_string())), Value::Number(1.0));
    let map2 = Value::new_map(entries2);

    assert_eq!(map1, map2);
}

#[test]
fn test_map_with_different_key_types() {
    let mut entries = IndexMap::new();
    entries.insert(
        MapKey::String(Rc::new("name".to_string())),
        Value::String(Rc::new("Alice".to_string())),
    );
    entries.insert(MapKey::Number(OrderedFloat(42.0)), Value::Number(100.0));
    entries.insert(MapKey::Boolean(true), Value::Boolean(false));

    let map = Value::new_map(entries);
    let display = format!("{}", map);

    // Check that all key types are represented
    assert!(display.contains("name:"));
    assert!(display.contains("42:"));
    assert!(display.contains("true:"));
}

#[test]
fn test_map_with_mixed_value_types() {
    let mut entries = IndexMap::new();
    entries.insert(
        MapKey::String(Rc::new("num".to_string())),
        Value::Number(42.0),
    );
    entries.insert(
        MapKey::String(Rc::new("bool".to_string())),
        Value::Boolean(true),
    );
    entries.insert(MapKey::String(Rc::new("nil".to_string())), Value::Nil);

    let map = Value::new_map(entries);
    let display = format!("{}", map);

    // Check that all value types are represented
    assert!(display.contains("num:"));
    assert!(display.contains("bool:"));
    assert!(display.contains("nil:"));
}

#[test]
fn test_set_creation() {
    let mut elements = BTreeSet::new();
    elements.insert(SetKey::Number(OrderedFloat(1.0)));
    elements.insert(SetKey::Number(OrderedFloat(2.0)));
    elements.insert(SetKey::Number(OrderedFloat(3.0)));

    let set = Value::new_set(elements);

    // Test that the set was created
    match set {
        Value::Set(_) => {
            // Success - set was created
        }
        _ => panic!("Expected Set value"),
    }
}

#[test]
fn test_set_display() {
    let mut elements = BTreeSet::new();
    elements.insert(SetKey::Number(OrderedFloat(1.0)));
    elements.insert(SetKey::Number(OrderedFloat(2.0)));
    elements.insert(SetKey::Number(OrderedFloat(3.0)));

    let set = Value::new_set(elements);
    let display = format!("{}", set);

    // HashSet order is not guaranteed, so we check for the format and presence of elements
    assert!(display.starts_with("#{"));
    assert!(display.ends_with("}"));
    assert!(display.contains("1"));
    assert!(display.contains("2"));
    assert!(display.contains("3"));
}

#[test]
fn test_empty_set_display() {
    let set = Value::new_set(BTreeSet::new());
    let display = format!("{}", set);
    assert_eq!(display, "#{}");
}

#[test]
fn test_set_equality() {
    let mut elements1 = BTreeSet::new();
    elements1.insert(SetKey::String(Rc::new("a".to_string())));
    elements1.insert(SetKey::String(Rc::new("b".to_string())));
    let set1 = Value::new_set(elements1);

    let mut elements2 = BTreeSet::new();
    elements2.insert(SetKey::String(Rc::new("a".to_string())));
    elements2.insert(SetKey::String(Rc::new("b".to_string())));
    let set2 = Value::new_set(elements2);

    let mut elements3 = BTreeSet::new();
    elements3.insert(SetKey::String(Rc::new("a".to_string())));
    elements3.insert(SetKey::String(Rc::new("c".to_string())));
    let set3 = Value::new_set(elements3);

    // These should be equal
    assert_eq!(set1, set2);

    // These should not be equal
    assert_ne!(set1, set3);
}

#[test]
fn test_set_with_different_key_types() {
    let mut elements = BTreeSet::new();
    elements.insert(SetKey::String(Rc::new("hello".to_string())));
    elements.insert(SetKey::Number(OrderedFloat(42.0)));
    elements.insert(SetKey::Boolean(true));

    let set = Value::new_set(elements);
    let display = format!("{}", set);

    // Check that all key types are represented
    assert!(display.contains("hello"));
    assert!(display.contains("42"));
    assert!(display.contains("true"));
}

#[test]
fn test_set_uniqueness() {
    let mut elements = BTreeSet::new();
    elements.insert(SetKey::Number(OrderedFloat(1.0)));
    elements.insert(SetKey::Number(OrderedFloat(1.0))); // Duplicate
    elements.insert(SetKey::Number(OrderedFloat(2.0)));

    let set = Value::new_set(elements);

    // Verify that the set contains only unique elements
    if let Value::Set(set_ref) = &set {
        assert_eq!(set_ref.borrow().len(), 2); // Should only contain 2 unique elements
    } else {
        panic!("Expected Set value");
    }
}

#[test]
fn value_is_16_bytes() {
    assert_eq!(std::mem::size_of::<Value>(), 16);
}

#[test]
fn value_debug_is_tagged() {
    assert_eq!(format!("{:?}", Value::Number(1.0)), "Number(1.0)");
    assert_eq!(
        format!("{:?}", Value::String(Rc::new("x".to_string()))),
        "String(\"x\")"
    );
    assert_eq!(format!("{:?}", Value::Nil), "Nil");

    let arr = Value::new_array(vec![Value::Number(1.0), Value::Number(2.0)]);
    assert_eq!(format!("{:?}", arr), format!("{}", arr));
}
