use crate::common::*;
use indexmap::IndexMap;
use ordered_float::OrderedFloat;
use std::cell::RefCell;
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

#[test]
fn test_range_display() {
    assert_eq!(format!("{}", Value::new_range(1, 4, false)), "1..4");
    assert_eq!(format!("{}", Value::new_range(1, 4, true)), "1..=4");
    assert_eq!(format!("{}", Value::new_range(5, 1, false)), "5..1");
}

#[test]
fn test_range_equality() {
    assert_eq!(Value::new_range(1, 4, false), Value::new_range(1, 4, false));
    assert_ne!(Value::new_range(1, 4, false), Value::new_range(1, 4, true));
    assert_ne!(Value::new_range(1, 4, false), Value::new_range(1, 5, false));
}

#[test]
fn test_range_len() {
    let Value::Range(exclusive) = Value::new_range(1, 4, false) else {
        panic!("Expected Range value");
    };
    assert_eq!(exclusive.len(), 3);

    let Value::Range(inclusive) = Value::new_range(1, 4, true) else {
        panic!("Expected Range value");
    };
    assert_eq!(inclusive.len(), 4);

    let Value::Range(empty) = Value::new_range(5, 1, false) else {
        panic!("Expected Range value");
    };
    assert_eq!(empty.len(), 0);
}

#[test]
fn test_range_len_extreme_bounds_saturates() {
    let Value::Range(full) = Value::new_range(i64::MIN, i64::MAX, true) else {
        panic!("Expected Range value");
    };
    assert_eq!(full.len(), i64::MAX);
}

#[test]
fn copy_or_clone_scalars() {
    assert_eq!(Value::Number(1.0).copy_or_clone(), Value::Number(1.0));
    assert_eq!(Value::Boolean(true).copy_or_clone(), Value::Boolean(true));
    assert_eq!(Value::Nil.copy_or_clone(), Value::Nil);
}

#[test]
fn copy_or_clone_increments_refcount_for_rc_backed_value() {
    let rc = Rc::new("hello".to_string());
    let value = Value::String(rc.clone());
    assert_eq!(Rc::strong_count(&rc), 2);

    let copy = value.copy_or_clone();
    assert_eq!(Rc::strong_count(&rc), 3);

    drop(value);
    drop(copy);
    assert_eq!(Rc::strong_count(&rc), 1);
}

#[test]
fn discard_drops_rc_backed_value() {
    let rc = Rc::new("hello".to_string());
    let value = Value::String(rc.clone());
    assert_eq!(Rc::strong_count(&rc), 2);

    value.discard();
    assert_eq!(Rc::strong_count(&rc), 1);
}

#[test]
fn discard_drops_array_value() {
    let rc = Rc::new(RefCell::new(vec![Value::Number(1.0)]));
    let value = Value::Array(rc.clone());
    assert_eq!(Rc::strong_count(&rc), 2);

    value.discard();
    assert_eq!(Rc::strong_count(&rc), 1);
}

#[test]
fn struct_field_index_resolves_non_contiguous_symbols() {
    let field_symbols = vec![7, 2, 9];
    let value = Value::new_struct(
        "Sparse".to_string(),
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
        field_symbols,
        0,
    );
    let Value::Struct(s) = value else {
        panic!("expected a struct value");
    };

    assert_eq!(s.field_index(7), Some(0));
    assert_eq!(s.field_index(2), Some(1));
    assert_eq!(s.field_index(9), Some(2));
    assert_eq!(s.field_index(3), None);
}

#[test]
fn enum_variant_display() {
    let value = Value::new_enum_variant("Color".to_string(), "Red".to_string(), 0);
    assert_eq!(format!("{}", value), "Color.Red");
}

#[test]
fn enum_variant_equality_same_enum_and_ordinal() {
    let a = Value::new_enum_variant("Color".to_string(), "Red".to_string(), 0);
    let b = Value::new_enum_variant("Color".to_string(), "Red".to_string(), 0);
    assert_eq!(a, b);
}

#[test]
fn enum_variant_inequality_different_ordinal() {
    let a = Value::new_enum_variant("Color".to_string(), "Red".to_string(), 0);
    let b = Value::new_enum_variant("Color".to_string(), "Green".to_string(), 1);
    assert_ne!(a, b);
}

#[test]
fn enum_variant_inequality_different_enum_same_name() {
    let a = Value::new_enum_variant("Color".to_string(), "Red".to_string(), 0);
    let b = Value::new_enum_variant("Status".to_string(), "Red".to_string(), 0);
    assert_ne!(a, b);
}

#[test]
fn enum_variant_inequality_other_value_kinds() {
    let variant = Value::new_enum_variant("Color".to_string(), "Red".to_string(), 0);
    assert_ne!(variant, Value::Number(0.0));
    assert_ne!(variant, Value::String(Rc::new("Color.Red".to_string())));
}

#[test]
fn int_display_prints_plain_digits() {
    assert_eq!(format!("{}", Value::Int(5)), "5");
    assert_eq!(format!("{}", Value::Int(-5)), "-5");
    assert_eq!(
        format!("{}", Value::Int(9007199254740993)),
        "9007199254740993"
    );
}

#[test]
fn int_debug_is_tagged() {
    assert_eq!(format!("{:?}", Value::Int(5)), "Int(5)");
}

#[test]
fn int_type_name_is_number() {
    assert_eq!(Value::Int(5).type_name(), "number");
}

#[test]
fn int_equality() {
    assert_eq!(Value::Int(1), Value::Int(1));
    assert_ne!(Value::Int(1), Value::Int(2));
    assert_eq!(Value::Int(1), Value::Number(1.0));
    assert_eq!(Value::Number(1.0), Value::Int(1));
    assert_ne!(
        Value::Int(9007199254740993),
        Value::Number(9007199254740992.0)
    );
    assert_ne!(Value::Int(1), Value::Number(f64::NAN));
}

#[test]
fn copy_or_clone_int() {
    assert!(matches!(Value::Int(42).copy_or_clone(), Value::Int(42)));
}

#[test]
fn compare_int_and_float_edges() {
    use std::cmp::Ordering;

    // 2^53 + 1 (exact as an Int) vs 2^53 (the largest float-exact integer).
    assert_eq!(
        compare_int_and_float(9007199254740993, 9007199254740992.0),
        Some(Ordering::Greater)
    );

    // i64::MAX vs 2^63 (one past the largest i64).
    assert_eq!(
        compare_int_and_float(i64::MAX, 9223372036854775808.0),
        Some(Ordering::Less)
    );

    // i64::MIN vs -2^63 (exactly equal).
    assert_eq!(
        compare_int_and_float(i64::MIN, -9223372036854775808.0),
        Some(Ordering::Equal)
    );

    // Negative fractions truncate toward zero.
    assert_eq!(compare_int_and_float(-1, -1.5), Some(Ordering::Greater));
    assert_eq!(compare_int_and_float(-2, -1.5), Some(Ordering::Less));

    // Positive fractions.
    assert_eq!(compare_int_and_float(1, 1.5), Some(Ordering::Less));
    assert_eq!(compare_int_and_float(2, 1.5), Some(Ordering::Greater));

    // Signed zero.
    assert_eq!(compare_int_and_float(0, -0.0), Some(Ordering::Equal));

    // Infinity and NaN.
    assert_eq!(
        compare_int_and_float(0, f64::INFINITY),
        Some(Ordering::Less)
    );
    assert_eq!(
        compare_int_and_float(0, f64::NEG_INFINITY),
        Some(Ordering::Greater)
    );
    assert_eq!(compare_int_and_float(0, f64::NAN), None);
}

#[test]
fn map_key_from_value_normalizes_integral_float_to_int() {
    assert_eq!(
        MapKey::from_value(&Value::Number(1.0), "map key"),
        Ok(MapKey::Int(1))
    );
    assert_eq!(
        MapKey::from_value(&Value::Number(-0.0), "map key"),
        Ok(MapKey::Int(0))
    );
    assert_eq!(
        MapKey::from_value(&Value::Int(1), "map key"),
        Ok(MapKey::Int(1))
    );
    assert_eq!(
        MapKey::from_value(&Value::Number(1.5), "map key"),
        Ok(MapKey::Number(OrderedFloat(1.5)))
    );
}

#[test]
fn map_key_int_and_float_literal_share_an_entry() {
    let mut map: IndexMap<MapKey, Value> = IndexMap::new();
    map.insert(
        MapKey::from_value(&Value::Int(1), "map key").unwrap(),
        Value::String(Rc::new("a".to_string())),
    );
    map.insert(
        MapKey::from_value(&Value::Number(1.0), "map key").unwrap(),
        Value::String(Rc::new("b".to_string())),
    );

    assert_eq!(map.len(), 1);
    assert_eq!(
        map.get(&MapKey::from_value(&Value::Int(1), "map key").unwrap()),
        Some(&Value::String(Rc::new("b".to_string())))
    );
}

#[test]
fn map_key_int_display() {
    assert_eq!(format!("{}", MapKey::Int(42)), "42");
}

#[test]
fn map_key_to_value_int() {
    assert!(matches!(MapKey::Int(7).to_value(), Value::Int(7)));
}

#[test]
fn set_orders_int_and_float_keys_numerically() {
    let mut elements = BTreeSet::new();
    elements.insert(SetKey::Number(OrderedFloat(2.5)));
    elements.insert(SetKey::Int(1));
    elements.insert(SetKey::Int(3));

    let set = Value::new_set(elements);
    assert_eq!(format!("{}", set), "#{1, 2.5, 3}");
}

#[test]
fn map_key_from_value_array_is_frozen_copy() {
    let array = Value::new_array(vec![Value::Int(1), Value::Int(2)]);
    let key = MapKey::from_value(&array, "map key").unwrap();

    if let Value::Array(a) = &array {
        a.borrow_mut().push(Value::Int(9));
    }

    assert_eq!(
        key,
        MapKey::Array(vec![MapKey::Int(1), MapKey::Int(2)].into())
    );
}

#[test]
fn map_key_from_value_nested_array() {
    let inner = Value::new_array(vec![Value::Int(1), Value::Int(2)]);
    let outer = Value::new_array(vec![inner, Value::String(Rc::new("a".to_string()))]);

    let key = MapKey::from_value(&outer, "map key").unwrap();
    assert_eq!(
        key,
        MapKey::Array(
            vec![
                MapKey::Array(vec![MapKey::Int(1), MapKey::Int(2)].into()),
                MapKey::String(Rc::new("a".to_string())),
            ]
            .into()
        )
    );
}

#[test]
fn map_key_from_value_rejects_self_referencing_array() {
    let array = Value::new_array(vec![]);
    if let Value::Array(a) = &array {
        a.borrow_mut().push(array.clone());
    }

    let err = MapKey::from_value(&array, "map key").unwrap_err();
    assert_eq!(err, "Cannot use a self-referencing array as a map key.");
}

#[test]
fn map_key_from_value_rejects_invalid_element_in_array() {
    let array = Value::new_array(vec![Value::Nil]);
    let err = MapKey::from_value(&array, "map key").unwrap_err();
    assert!(err.starts_with("Invalid map key type: nil."));
}

#[test]
fn map_key_from_value_rejects_indirect_cycle() {
    let a = Value::new_array(vec![]);
    let b = Value::new_array(vec![a.clone()]);
    if let Value::Array(a_rc) = &a {
        a_rc.borrow_mut().push(b.clone());
    }

    let err = MapKey::from_value(&a, "map key").unwrap_err();
    assert_eq!(err, "Cannot use a self-referencing array as a map key.");
}

#[test]
fn map_key_from_value_array_diamond() {
    let shared = Value::new_array(vec![Value::Int(1)]);
    let outer = Value::new_array(vec![shared.clone(), shared]);

    let key = MapKey::from_value(&outer, "map key").unwrap();
    assert_eq!(
        key,
        MapKey::Array(
            vec![
                MapKey::Array(vec![MapKey::Int(1)].into()),
                MapKey::Array(vec![MapKey::Int(1)].into()),
            ]
            .into()
        )
    );
}

#[test]
fn map_key_enum_variant_eq() {
    let red1 = Value::new_enum_variant("Color".to_string(), "Red".to_string(), 0);
    let red2 = Value::new_enum_variant("Color".to_string(), "Red".to_string(), 0);
    let green = Value::new_enum_variant("Color".to_string(), "Green".to_string(), 1);

    let key1 = MapKey::from_value(&red1, "map key").unwrap();
    let key2 = MapKey::from_value(&red2, "map key").unwrap();
    let key3 = MapKey::from_value(&green, "map key").unwrap();

    assert_eq!(key1, key2);
    assert_ne!(key1, key3);
}

#[test]
fn map_key_array_display_matches_value_array() {
    let key = MapKey::Array(vec![MapKey::Int(1), MapKey::String(Rc::new("a".to_string()))].into());
    assert_eq!(format!("{}", key), "[1, a]");
}

#[test]
fn map_key_array_ord_lexicographic() {
    let short = MapKey::Array(vec![MapKey::Int(1)].into());
    let long = MapKey::Array(vec![MapKey::Int(1), MapKey::Int(0)].into());
    assert!(short < long);

    let a = MapKey::Array(vec![MapKey::Int(1), MapKey::Int(2)].into());
    let b = MapKey::Array(vec![MapKey::Int(1), MapKey::Int(3)].into());
    assert!(a < b);
}

#[test]
fn map_key_cross_kind_ord() {
    let string_key = MapKey::String(Rc::new("a".to_string()));
    let number_key = MapKey::Int(1);
    let bool_key = MapKey::Boolean(true);
    let red = Value::new_enum_variant("Color".to_string(), "Red".to_string(), 0);
    let enum_key = MapKey::from_value(&red, "map key").unwrap();
    let array_key = MapKey::Array(vec![].into());

    assert!(string_key < number_key);
    assert!(number_key < bool_key);
    assert!(bool_key < enum_key);
    assert!(enum_key < array_key);
}
