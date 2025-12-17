use crate::as_number;
use crate::common::{Object, SetKey, Value};
use crate::string;
use crate::vm::set_functions::*;
use crate::vm::VirtualMachine;
use ordered_float::OrderedFloat;
use std::collections::BTreeSet;
use std::rc::Rc;

#[test]
fn test_set_add_new_element() {
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let element = Value::Number(42.0);
    let args = vec![set.clone(), element];

    let result = native_set_add(&args).unwrap();
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
    let vm = VirtualMachine::new();
    let mut elements = BTreeSet::new();
    elements.insert(SetKey::Number(OrderedFloat(42.0)));
    let set = Value::new_set(elements);
    let element = Value::Number(42.0);
    let args = vec![set.clone(), element];

    let result = native_set_add(&args).unwrap();
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
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let element = string!("hello");
    let args = vec![set.clone(), element];

    let result = native_set_add(&args).unwrap();
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
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let element = Value::Boolean(true);
    let args = vec![set.clone(), element];

    let result = native_set_add(&args).unwrap();
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
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let element = Value::Nil;
    let args = vec![set, element];

    let result = native_set_add(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid set element type"));
}

#[test]
fn test_set_remove_existing_element() {
    let vm = VirtualMachine::new();
    let mut elements = BTreeSet::new();
    elements.insert(SetKey::Number(OrderedFloat(42.0)));
    elements.insert(SetKey::Number(OrderedFloat(100.0)));
    let set = Value::new_set(elements);
    let element = Value::Number(42.0);
    let args = vec![set.clone(), element];

    let result = native_set_remove(&args).unwrap();
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
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let element = Value::Number(42.0);
    let args = vec![set, element];

    let result = native_set_remove(&args).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_set_has_existing_element() {
    let vm = VirtualMachine::new();
    let mut elements = BTreeSet::new();
    elements.insert(SetKey::String(Rc::from("hello")));
    let set = Value::new_set(elements);
    let element = string!("hello");
    let args = vec![set, element];

    let result = native_set_has(&args).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_set_has_nonexistent_element() {
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let element = string!("hello");
    let args = vec![set, element];

    let result = native_set_has(&args).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_set_size_empty() {
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let args = vec![set];

    let result = native_set_size(&args).unwrap();
    assert_eq!(as_number!(result), 0.0);
}

#[test]
fn test_set_size_with_elements() {
    let vm = VirtualMachine::new();
    let mut elements = BTreeSet::new();
    elements.insert(SetKey::Number(OrderedFloat(1.0)));
    elements.insert(SetKey::Number(OrderedFloat(2.0)));
    elements.insert(SetKey::Number(OrderedFloat(3.0)));
    let set = Value::new_set(elements);
    let args = vec![set];

    let result = native_set_size(&args).unwrap();
    assert_eq!(as_number!(result), 3.0);
}

#[test]
fn test_set_clear_empty() {
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let args = vec![set.clone()];

    let result = native_set_clear(&args).unwrap();
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
    let vm = VirtualMachine::new();
    let mut elements = BTreeSet::new();
    elements.insert(SetKey::Number(OrderedFloat(1.0)));
    elements.insert(SetKey::Number(OrderedFloat(2.0)));
    elements.insert(SetKey::Number(OrderedFloat(3.0)));
    let set = Value::new_set(elements);
    let args = vec![set.clone()];

    let result = native_set_clear(&args).unwrap();
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
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let args = vec![set];

    let result = native_set_add(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects 1 argument"));
}

#[test]
fn test_set_remove_wrong_arg_count() {
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let args = vec![set];

    let result = native_set_remove(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects 1 argument"));
}

#[test]
fn test_set_has_wrong_arg_count() {
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let args = vec![set];

    let result = native_set_has(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects 1 argument"));
}

#[test]
fn test_set_size_wrong_arg_count() {
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let element = Value::Number(42.0);
    let args = vec![set, element];

    let result = native_set_size(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects no arguments"));
}

#[test]
fn test_set_clear_wrong_arg_count() {
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let element = Value::Number(42.0);
    let args = vec![set, element];

    let result = native_set_clear(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects no arguments"));
}

#[test]
fn test_set_methods_on_non_set() {
    let vm = VirtualMachine::new();
    let not_a_set = Value::Number(42.0);
    let element = Value::Number(1.0);

    let add_result = native_set_add(&[not_a_set.clone(), element.clone()]);
    assert!(add_result.is_err());
    assert!(add_result
        .unwrap_err()
        .contains("can only be called on sets"));

    let remove_result = native_set_remove(&[not_a_set.clone(), element.clone()]);
    assert!(remove_result.is_err());
    assert!(remove_result
        .unwrap_err()
        .contains("can only be called on sets"));

    let has_result = native_set_has(&[not_a_set.clone(), element]);
    assert!(has_result.is_err());
    assert!(has_result
        .unwrap_err()
        .contains("can only be called on sets"));

    let size_result = native_set_size(&[not_a_set.clone()]);
    assert!(size_result.is_err());
    assert!(size_result
        .unwrap_err()
        .contains("can only be called on sets"));

    let clear_result = native_set_clear(&[not_a_set]);
    assert!(clear_result.is_err());
    assert!(clear_result
        .unwrap_err()
        .contains("can only be called on sets"));
}

// Tests for set algebra methods

#[test]
fn test_set_union_basic() {
    let vm = VirtualMachine::new();

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
    let result = native_set_union(&args).unwrap();

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
    let vm = VirtualMachine::new();

    let set1 = Value::new_set(BTreeSet::new());
    let set2 = Value::new_set(BTreeSet::new());

    let args = vec![set1, set2];
    let result = native_set_union(&args).unwrap();

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
    let vm = VirtualMachine::new();

    let mut elements = BTreeSet::new();
    elements.insert(SetKey::Number(OrderedFloat(1.0)));
    elements.insert(SetKey::Number(OrderedFloat(2.0)));
    let set1 = Value::new_set(elements);
    let set2 = Value::new_set(BTreeSet::new());

    let args = vec![set1, set2];
    let result = native_set_union(&args).unwrap();

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
    let vm = VirtualMachine::new();

    let mut elements1 = BTreeSet::new();
    elements1.insert(SetKey::Number(OrderedFloat(1.0)));
    elements1.insert(SetKey::String(Rc::from("hello")));
    let set1 = Value::new_set(elements1);

    let mut elements2 = BTreeSet::new();
    elements2.insert(SetKey::Boolean(true));
    elements2.insert(SetKey::String(Rc::from("world")));
    let set2 = Value::new_set(elements2);

    let args = vec![set1, set2];
    let result = native_set_union(&args).unwrap();

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
    let vm = VirtualMachine::new();

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
    let result = native_set_intersection(&args).unwrap();

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
    let vm = VirtualMachine::new();

    let mut elements1 = BTreeSet::new();
    elements1.insert(SetKey::Number(OrderedFloat(1.0)));
    elements1.insert(SetKey::Number(OrderedFloat(2.0)));
    let set1 = Value::new_set(elements1);

    let mut elements2 = BTreeSet::new();
    elements2.insert(SetKey::Number(OrderedFloat(3.0)));
    elements2.insert(SetKey::Number(OrderedFloat(4.0)));
    let set2 = Value::new_set(elements2);

    let args = vec![set1, set2];
    let result = native_set_intersection(&args).unwrap();

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
    let vm = VirtualMachine::new();

    let set1 = Value::new_set(BTreeSet::new());
    let set2 = Value::new_set(BTreeSet::new());

    let args = vec![set1, set2];
    let result = native_set_intersection(&args).unwrap();

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
    let vm = VirtualMachine::new();

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
    let result = native_set_difference(&args).unwrap();

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
    let vm = VirtualMachine::new();

    let mut elements1 = BTreeSet::new();
    elements1.insert(SetKey::Number(OrderedFloat(1.0)));
    elements1.insert(SetKey::Number(OrderedFloat(2.0)));
    let set1 = Value::new_set(elements1);

    let mut elements2 = BTreeSet::new();
    elements2.insert(SetKey::Number(OrderedFloat(3.0)));
    elements2.insert(SetKey::Number(OrderedFloat(4.0)));
    let set2 = Value::new_set(elements2);

    let args = vec![set1, set2];
    let result = native_set_difference(&args).unwrap();

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
    let vm = VirtualMachine::new();

    let mut elements1 = BTreeSet::new();
    elements1.insert(SetKey::Number(OrderedFloat(1.0)));
    elements1.insert(SetKey::Number(OrderedFloat(2.0)));
    let set1 = Value::new_set(elements1.clone());
    let set2 = Value::new_set(elements1);

    let args = vec![set1, set2];
    let result = native_set_difference(&args).unwrap();

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
    let vm = VirtualMachine::new();

    let set1 = Value::new_set(BTreeSet::new());
    let set2 = Value::new_set(BTreeSet::new());

    let args = vec![set1, set2];
    let result = native_set_difference(&args).unwrap();

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
    let vm = VirtualMachine::new();

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
    let result = native_set_is_subset(&args).unwrap();

    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_set_is_subset_false() {
    let vm = VirtualMachine::new();

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
    let result = native_set_is_subset(&args).unwrap();

    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_set_is_subset_equal_sets() {
    let vm = VirtualMachine::new();

    let mut elements = BTreeSet::new();
    elements.insert(SetKey::Number(OrderedFloat(1.0)));
    elements.insert(SetKey::Number(OrderedFloat(2.0)));
    let set1 = Value::new_set(elements.clone());
    let set2 = Value::new_set(elements);

    let args = vec![set1, set2];
    let result = native_set_is_subset(&args).unwrap();

    // A set is a subset of itself
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_set_is_subset_empty_set() {
    let vm = VirtualMachine::new();

    let set1 = Value::new_set(BTreeSet::new());

    let mut elements = BTreeSet::new();
    elements.insert(SetKey::Number(OrderedFloat(1.0)));
    let set2 = Value::new_set(elements);

    let args = vec![set1, set2];
    let result = native_set_is_subset(&args).unwrap();

    // Empty set is a subset of any set
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_set_is_subset_both_empty() {
    let vm = VirtualMachine::new();

    let set1 = Value::new_set(BTreeSet::new());
    let set2 = Value::new_set(BTreeSet::new());

    let args = vec![set1, set2];
    let result = native_set_is_subset(&args).unwrap();

    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_set_union_wrong_arg_count() {
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let args = vec![set];

    let result = native_set_union(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects 1 argument"));
}

#[test]
fn test_set_intersection_wrong_arg_count() {
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let args = vec![set];

    let result = native_set_intersection(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects 1 argument"));
}

#[test]
fn test_set_difference_wrong_arg_count() {
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let args = vec![set];

    let result = native_set_difference(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects 1 argument"));
}

#[test]
fn test_set_is_subset_wrong_arg_count() {
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let args = vec![set];

    let result = native_set_is_subset(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects 1 argument"));
}

#[test]
fn test_set_algebra_on_non_set() {
    let vm = VirtualMachine::new();
    let not_a_set = Value::Number(42.0);
    let set = Value::new_set(BTreeSet::new());

    let union_result = native_set_union(&[not_a_set.clone(), set.clone()]);
    assert!(union_result.is_err());
    assert!(union_result
        .unwrap_err()
        .contains("can only be called on sets"));

    let intersection_result = native_set_intersection(&[not_a_set.clone(), set.clone()]);
    assert!(intersection_result.is_err());
    assert!(intersection_result
        .unwrap_err()
        .contains("can only be called on sets"));

    let difference_result = native_set_difference(&[not_a_set.clone(), set.clone()]);
    assert!(difference_result.is_err());
    assert!(difference_result
        .unwrap_err()
        .contains("can only be called on sets"));

    let subset_result = native_set_is_subset(&[not_a_set, set]);
    assert!(subset_result.is_err());
    assert!(subset_result
        .unwrap_err()
        .contains("can only be called on sets"));
}

#[test]
fn test_set_algebra_with_non_set_argument() {
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let not_a_set = Value::Number(42.0);

    let union_result = native_set_union(&[set.clone(), not_a_set.clone()]);
    assert!(union_result.is_err());
    assert!(union_result
        .unwrap_err()
        .contains("requires a set as argument"));

    let intersection_result = native_set_intersection(&[set.clone(), not_a_set.clone()]);
    assert!(intersection_result.is_err());
    assert!(intersection_result
        .unwrap_err()
        .contains("requires a set as argument"));

    let difference_result = native_set_difference(&[set.clone(), not_a_set.clone()]);
    assert!(difference_result.is_err());
    assert!(difference_result
        .unwrap_err()
        .contains("requires a set as argument"));

    let subset_result = native_set_is_subset(&[set, not_a_set]);
    assert!(subset_result.is_err());
    assert!(subset_result
        .unwrap_err()
        .contains("requires a set as argument"));
}

// Tests for toArray method

#[test]
fn test_set_to_array_empty() {
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let args = vec![set];

    let result = native_set_to_array(&args).unwrap();

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
    let vm = VirtualMachine::new();
    let mut elements = BTreeSet::new();
    elements.insert(SetKey::Number(OrderedFloat(1.0)));
    elements.insert(SetKey::Number(OrderedFloat(2.0)));
    elements.insert(SetKey::Number(OrderedFloat(3.0)));
    let set = Value::new_set(elements);
    let args = vec![set];

    let result = native_set_to_array(&args).unwrap();

    // Verify it's an array with the correct elements
    match result {
        Value::Object(obj) => match obj.as_ref() {
            crate::common::Object::Array(arr) => {
                let array_elements = arr.borrow();
                assert_eq!(array_elements.len(), 3);

                // Check that all elements are present (order not guaranteed)
                let contains_1 = array_elements
                    .iter()
                    .any(|v| matches!(v, Value::Number(n) if *n == 1.0));
                let contains_2 = array_elements
                    .iter()
                    .any(|v| matches!(v, Value::Number(n) if *n == 2.0));
                let contains_3 = array_elements
                    .iter()
                    .any(|v| matches!(v, Value::Number(n) if *n == 3.0));
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
    let vm = VirtualMachine::new();
    let mut elements = BTreeSet::new();
    elements.insert(SetKey::String(Rc::from("apple")));
    elements.insert(SetKey::String(Rc::from("banana")));
    let set = Value::new_set(elements);
    let args = vec![set];

    let result = native_set_to_array(&args).unwrap();

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
    let vm = VirtualMachine::new();
    let mut elements = BTreeSet::new();
    elements.insert(SetKey::Boolean(true));
    elements.insert(SetKey::Boolean(false));
    let set = Value::new_set(elements);
    let args = vec![set];

    let result = native_set_to_array(&args).unwrap();

    match result {
        Value::Object(obj) => match obj.as_ref() {
            crate::common::Object::Array(arr) => {
                let array_elements = arr.borrow();
                assert_eq!(array_elements.len(), 2);

                // Check that both booleans are present
                let has_true = array_elements
                    .iter()
                    .any(|v| matches!(v, Value::Boolean(true)));
                let has_false = array_elements
                    .iter()
                    .any(|v| matches!(v, Value::Boolean(false)));
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
    let vm = VirtualMachine::new();
    let mut elements = BTreeSet::new();
    elements.insert(SetKey::Number(OrderedFloat(42.0)));
    elements.insert(SetKey::String(Rc::from("hello")));
    elements.insert(SetKey::Boolean(true));
    let set = Value::new_set(elements);
    let args = vec![set];

    let result = native_set_to_array(&args).unwrap();

    match result {
        Value::Object(obj) => match obj.as_ref() {
            crate::common::Object::Array(arr) => {
                let array_elements = arr.borrow();
                assert_eq!(array_elements.len(), 3);

                // Check that all types are present
                let has_number = array_elements
                    .iter()
                    .any(|v| matches!(v, Value::Number(n) if *n == 42.0));
                let has_string = array_elements.iter().any(|v| {
                    if let Value::Object(obj) = v {
                        if let crate::common::Object::String(s) = obj.as_ref() {
                            return s.value.as_ref() == "hello";
                        }
                    }
                    false
                });
                let has_boolean = array_elements
                    .iter()
                    .any(|v| matches!(v, Value::Boolean(true)));
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
    let vm = VirtualMachine::new();
    let set = Value::new_set(BTreeSet::new());
    let extra_arg = Value::Number(42.0);
    let args = vec![set, extra_arg];

    let result = native_set_to_array(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expects no arguments"));
}

#[test]
fn test_set_to_array_on_non_set() {
    let vm = VirtualMachine::new();
    let not_a_set = Value::Number(42.0);
    let args = vec![not_a_set];

    let result = native_set_to_array(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("can only be called on sets"));
}
