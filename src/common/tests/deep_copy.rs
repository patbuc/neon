use crate::common::fiber::FiberKind;
use crate::common::*;
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

fn closure(upvalues: Vec<Rc<RefCell<Upvalue>>>) -> Rc<ObjClosure> {
    Rc::new(ObjClosure {
        function: Rc::new(ObjFunction {
            name: "f".to_string(),
            arity: 0,
            chunk: Rc::new(Chunk::new("f")),
        }),
        upvalues,
    })
}

#[test]
fn deep_copy_of_an_array_does_not_alias_the_original() {
    let original = Value::new_array(vec![Value::new_array(vec![Value::Number(1.0)])]);
    let copy = original.deep_copy().unwrap();
    assert_eq!(original, copy);

    let (Value::Array(original_outer), Value::Array(copy_outer)) = (&original, &copy) else {
        panic!("expected arrays");
    };
    assert!(!Rc::ptr_eq(original_outer, copy_outer));
    let (Value::Array(original_inner), Value::Array(copy_inner)) =
        (&original_outer.borrow()[0], &copy_outer.borrow()[0])
    else {
        panic!("expected nested arrays");
    };
    assert!(!Rc::ptr_eq(original_inner, copy_inner));
}

#[test]
fn deep_copy_keeps_shared_references_shared() {
    let shared = Value::new_array(vec![]);
    let original = Value::new_array(vec![shared.clone(), shared]);
    let copy = original.deep_copy().unwrap();

    let Value::Array(copy) = copy else {
        panic!("expected an array");
    };
    let copy = copy.borrow();
    let (Value::Array(first), Value::Array(second)) = (&copy[0], &copy[1]) else {
        panic!("expected nested arrays");
    };
    assert!(Rc::ptr_eq(first, second));
}

#[test]
fn deep_copy_of_a_cyclic_array_is_cyclic() {
    let original = Value::new_array(vec![]);
    let Value::Array(cell) = &original else {
        unreachable!()
    };
    cell.borrow_mut().push(original.clone());

    let copy = original.deep_copy().unwrap();
    let Value::Array(copy_cell) = &copy else {
        panic!("expected an array");
    };
    let inner = copy_cell.borrow()[0].clone();
    let Value::Array(inner) = &inner else {
        panic!("expected a nested array");
    };
    assert!(Rc::ptr_eq(copy_cell, inner));
    assert!(!Rc::ptr_eq(copy_cell, cell));

    // Break the cycles so the test doesn't leak.
    cell.borrow_mut().clear();
    copy_cell.borrow_mut().clear();
}

#[test]
fn deep_copy_of_an_array_referencing_a_set_twice_shares_the_copy() {
    let set = Value::new_set(BTreeSet::from([SetKey::Number(1.0.into())]));
    let original = Value::new_array(vec![set.clone(), set.clone()]);
    let copy = original.deep_copy().unwrap();

    let Value::Array(copy) = copy else {
        panic!("expected an array");
    };
    let copy = copy.borrow();
    let (Value::Set(first), Value::Set(second)) = (&copy[0], &copy[1]) else {
        panic!("expected sets");
    };
    assert!(Rc::ptr_eq(first, second));

    let Value::Set(original_set) = &set else {
        panic!("expected a set");
    };
    assert!(!Rc::ptr_eq(original_set, first));
}

fn instance(r#struct: &Rc<ObjStruct>, fields: Vec<Value>) -> Value {
    Value::Instance(Rc::new(RefCell::new(ObjInstance {
        r#struct: Rc::clone(r#struct),
        fields,
    })))
}

fn empty_struct(name: &str) -> Rc<ObjStruct> {
    Rc::new(ObjStruct {
        name: name.to_string(),
        fields: vec!["field".to_string()],
        field_symbols: vec![0],
        name_symbol: 0,
    })
}

#[test]
fn deep_copy_of_an_array_referencing_a_map_twice_shares_the_copy() {
    let mut entries = IndexMap::new();
    entries.insert(MapKey::Number(1.0.into()), Value::Number(1.0));
    let map = Value::new_map(entries);
    let original = Value::new_array(vec![map.clone(), map.clone()]);
    let copy = original.deep_copy().unwrap();

    let Value::Array(copy) = copy else {
        panic!("expected an array");
    };
    let copy = copy.borrow();
    let (Value::Map(first), Value::Map(second)) = (&copy[0], &copy[1]) else {
        panic!("expected maps");
    };
    assert!(Rc::ptr_eq(first, second));

    let Value::Map(original_map) = &map else {
        panic!("expected a map");
    };
    assert!(!Rc::ptr_eq(original_map, first));
}

#[test]
fn deep_copy_of_a_map_keeps_entry_order_and_copies_container_values() {
    let mut entries = IndexMap::new();
    entries.insert(
        MapKey::String(Rc::new("a".to_string())),
        Value::new_array(vec![Value::Number(1.0)]),
    );
    entries.insert(
        MapKey::String(Rc::new("b".to_string())),
        Value::new_array(vec![Value::Number(2.0)]),
    );
    entries.insert(
        MapKey::String(Rc::new("c".to_string())),
        Value::new_array(vec![Value::Number(3.0)]),
    );
    let original = Value::new_map(entries);
    let copy = original.deep_copy().unwrap();

    let (Value::Map(original), Value::Map(copy)) = (&original, &copy) else {
        panic!("expected maps");
    };
    let original = original.borrow();
    let copy = copy.borrow();
    assert_eq!(
        original.keys().collect::<Vec<_>>(),
        copy.keys().collect::<Vec<_>>()
    );
    for (original_value, copy_value) in original.values().zip(copy.values()) {
        assert_eq!(original_value, copy_value);
        let (Value::Array(original_value), Value::Array(copy_value)) = (original_value, copy_value)
        else {
            panic!("expected arrays");
        };
        assert!(!Rc::ptr_eq(original_value, copy_value));
    }
}

#[test]
fn deep_copy_of_a_cyclic_instance_is_cyclic() {
    let r#struct = empty_struct("S");
    let original = instance(&r#struct, vec![Value::Nil]);
    let Value::Instance(cell) = &original else {
        unreachable!()
    };
    cell.borrow_mut().fields[0] = original.clone();

    let copy = original.deep_copy().unwrap();
    let Value::Instance(copy_cell) = &copy else {
        panic!("expected an instance");
    };
    let field = copy_cell.borrow().fields[0].clone();
    let Value::Instance(field) = &field else {
        panic!("expected an instance field");
    };
    assert!(Rc::ptr_eq(copy_cell, field));
    assert!(!Rc::ptr_eq(copy_cell, cell));
}

#[test]
fn deep_copy_of_a_closure_without_captures_shares_it() {
    let value = Value::Closure(closure(vec![]));
    assert_eq!(value, value.deep_copy().unwrap());
}

#[test]
fn deep_copy_of_a_capturing_closure_is_an_error() {
    let upvalue = Rc::new(RefCell::new(Upvalue::Closed(Value::Nil)));
    let value = Value::Closure(closure(vec![upvalue]));
    assert_eq!(
        "Cannot copy function 'f' across a task boundary: it captures variables",
        value.deep_copy().unwrap_err()
    );
}

#[test]
fn deep_copy_of_a_fiber_is_an_error() {
    let fiber = Value::new_fiber(FiberKind::Fiber, closure(vec![]));
    assert_eq!(
        "Cannot copy a fiber across a task boundary",
        fiber.deep_copy().unwrap_err()
    );
}

#[test]
fn deep_copy_of_a_deeply_nested_array_does_not_overflow_the_stack() {
    const DEPTH: usize = 100_000;

    let mut original = Value::new_array(vec![]);
    for _ in 0..DEPTH {
        original = Value::new_array(vec![original]);
    }

    let copy = original.deep_copy().unwrap();

    let mut depth = 0;
    let mut current = copy.clone();
    loop {
        let next = {
            let Value::Array(array) = &current else {
                panic!("expected an array");
            };
            let array = array.borrow();
            if array.is_empty() {
                break;
            }
            array[0].clone()
        };
        depth += 1;
        current = next;
    }
    assert_eq!(DEPTH, depth);

    // Dropping a 100,000-deep Rc chain recursively overflows the stack in a
    // debug build, so tear both chains down from the outside in first.
    unnest(original);
    unnest(copy);
}

/// Pops the inner array out of each level of a deeply nested array before
/// the outer `Value` is dropped, so `Drop` never has to recurse.
fn unnest(mut value: Value) {
    loop {
        let inner = {
            let Value::Array(array) = &value else {
                panic!("expected an array");
            };
            let mut array = array.borrow_mut();
            if array.is_empty() {
                break;
            }
            array.pop()
        };
        value = inner.expect("checked non-empty above");
    }
}
