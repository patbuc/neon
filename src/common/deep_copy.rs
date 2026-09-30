use crate::common::{MapKey, ObjInstance, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Heap pointers already copied, so shared and cyclic structure comes out
/// shared and cyclic in the copy rather than duplicated or recursed forever.
type Seen = HashMap<*const (), Value>;

/// Where a value being copied belongs once its copy is ready. The container
/// itself is pre-sized with placeholders up front, so a slot writes by
/// position/key rather than by appending — the worklist below is a stack,
/// so children are not necessarily finished in source order.
enum Slot {
    ArrayElement {
        target: Rc<RefCell<Vec<Value>>>,
        index: usize,
    },
    MapEntry {
        target: Rc<RefCell<indexmap::IndexMap<MapKey, Value>>>,
        key: MapKey,
    },
    InstanceField {
        target: Rc<RefCell<ObjInstance>>,
        index: usize,
    },
}

impl Value {
    /// A copy sharing no mutable state with `self`, for a value crossing a
    /// task boundary. Immutable values (scalars, strings, ranges, function
    /// templates, struct definitions) are shared as they are. A closure that
    /// captures variables, or a fiber, cannot be copied: both are handles on
    /// live state, so the copy would still alias it.
    ///
    /// Iterative rather than recursive, so an array (or map/instance) nested
    /// arbitrarily deep does not overflow the stack: an explicit worklist
    /// stands in for the call stack a recursive version would use.
    pub(crate) fn deep_copy(&self) -> Result<Value, String> {
        let mut seen = Seen::new();
        let mut work: Vec<(Value, Option<Slot>)> = vec![(self.clone(), None)];
        let mut result = None;

        while let Some((value, slot)) = work.pop() {
            let copy = match &value {
                Value::Number(_)
                | Value::Boolean(_)
                | Value::Nil
                | Value::Uninitialized(_)
                | Value::String(_)
                | Value::Function(_)
                | Value::NativeFunction(_)
                | Value::Struct(_)
                | Value::File(_)
                | Value::Range(_) => value.clone(),
                Value::Closure(closure) => {
                    if closure.upvalues.is_empty() {
                        value.clone()
                    } else {
                        return Err(format!(
                            "Cannot copy function '{}' across a task boundary: it captures variables",
                            closure.function.name
                        ));
                    }
                }
                Value::Fiber(_) => {
                    return Err(format!(
                        "Cannot copy a {} across a task boundary",
                        value.type_name()
                    ));
                }
                Value::Array(array) => {
                    let key = Rc::as_ptr(array) as *const ();
                    if let Some(copy) = seen.get(&key) {
                        copy.clone()
                    } else {
                        let len = array.borrow().len();
                        let copy = Value::new_array(vec![Value::Nil; len]);
                        seen.insert(key, copy.clone());
                        let Value::Array(target) = &copy else {
                            unreachable!("new_array returns an array")
                        };
                        for (index, element) in array.borrow().iter().enumerate() {
                            work.push((
                                element.clone(),
                                Some(Slot::ArrayElement {
                                    target: Rc::clone(target),
                                    index,
                                }),
                            ));
                        }
                        copy
                    }
                }
                Value::Map(map) => {
                    let key = Rc::as_ptr(map) as *const ();
                    if let Some(copy) = seen.get(&key) {
                        copy.clone()
                    } else {
                        let mut entries = indexmap::IndexMap::with_capacity(map.borrow().len());
                        for entry_key in map.borrow().keys() {
                            entries.insert(entry_key.clone(), Value::Nil);
                        }
                        let copy = Value::new_map(entries);
                        seen.insert(key, copy.clone());
                        let Value::Map(target) = &copy else {
                            unreachable!("new_map returns a map")
                        };
                        for (entry_key, entry_value) in map.borrow().iter() {
                            work.push((
                                entry_value.clone(),
                                Some(Slot::MapEntry {
                                    target: Rc::clone(target),
                                    key: entry_key.clone(),
                                }),
                            ));
                        }
                        copy
                    }
                }
                // Set elements are immutable keys, so a shallow clone of the
                // set's contents is already a deep copy; only the pointer
                // needs deduplicating against `seen`.
                Value::Set(set) => {
                    let key = Rc::as_ptr(set) as *const ();
                    if let Some(copy) = seen.get(&key) {
                        copy.clone()
                    } else {
                        let copy = Value::new_set(set.borrow().clone());
                        seen.insert(key, copy.clone());
                        copy
                    }
                }
                Value::Instance(instance) => {
                    let key = Rc::as_ptr(instance) as *const ();
                    if let Some(copy) = seen.get(&key) {
                        copy.clone()
                    } else {
                        let r#struct = Rc::clone(&instance.borrow().r#struct);
                        let len = instance.borrow().fields.len();
                        let target = Rc::new(RefCell::new(ObjInstance {
                            r#struct,
                            fields: vec![Value::Nil; len],
                        }));
                        let copy = Value::Instance(Rc::clone(&target));
                        seen.insert(key, copy.clone());
                        for (index, field) in instance.borrow().fields.iter().enumerate() {
                            work.push((
                                field.clone(),
                                Some(Slot::InstanceField {
                                    target: Rc::clone(&target),
                                    index,
                                }),
                            ));
                        }
                        copy
                    }
                }
            };

            match slot {
                None => result = Some(copy),
                Some(Slot::ArrayElement { target, index }) => target.borrow_mut()[index] = copy,
                Some(Slot::MapEntry { target, key }) => {
                    target.borrow_mut().insert(key, copy);
                }
                Some(Slot::InstanceField { target, index }) => {
                    target.borrow_mut().fields[index] = copy
                }
            }
        }

        Ok(result.expect("the root value is always copied"))
    }
}
