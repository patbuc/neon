use crate::common::{ObjInstance, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Heap pointers already copied, so shared and cyclic structure comes out
/// shared and cyclic in the copy rather than duplicated or recursed forever.
/// Reusing one `Seen` across several copies keeps structure shared between
/// them too (see `fiber::TaskSnapshot`).
pub(crate) type Seen = HashMap<*const (), Value>;

impl Value {
    /// A copy sharing no mutable state with `self`, for a value crossing a
    /// task boundary. Immutable values (scalars, strings, ranges, function
    /// templates, struct definitions) are shared as they are. A closure that
    /// captures variables, or a fiber, cannot be copied: both are handles on
    /// live state, so the copy would still alias it.
    pub(crate) fn deep_copy(&self) -> Result<Value, String> {
        let mut seen = Seen::new();
        self.deep_copy_with(&mut seen)
    }

    /// `deep_copy`, reusing the copies already recorded in `seen`.
    pub(crate) fn deep_copy_with(&self, seen: &mut Seen) -> Result<Value, String> {
        match self {
            Value::Number(_)
            | Value::Boolean(_)
            | Value::Nil
            | Value::Uninitialized(_)
            | Value::String(_)
            | Value::Function(_)
            | Value::NativeFunction(_)
            | Value::Struct(_)
            | Value::File(_)
            | Value::Range(_) => Ok(self.clone()),
            Value::Closure(closure) => {
                if closure.upvalues.is_empty() {
                    Ok(self.clone())
                } else {
                    Err(format!(
                        "Cannot copy function '{}' across a task boundary: it captures variables",
                        closure.function.name
                    ))
                }
            }
            Value::Fiber(_) => Err(format!(
                "Cannot copy a {} across a task boundary",
                self.type_name()
            )),
            Value::Array(array) => {
                let key = Rc::as_ptr(array) as *const ();
                if let Some(copy) = seen.get(&key) {
                    return Ok(copy.clone());
                }
                let copy = Value::new_array(Vec::with_capacity(array.borrow().len()));
                seen.insert(key, copy.clone());
                let Value::Array(target) = &copy else {
                    unreachable!("new_array returns an array")
                };
                for element in array.borrow().iter() {
                    let element = element.deep_copy_with(seen)?;
                    target.borrow_mut().push(element);
                }
                Ok(copy)
            }
            Value::Map(map) => {
                let key = Rc::as_ptr(map) as *const ();
                if let Some(copy) = seen.get(&key) {
                    return Ok(copy.clone());
                }
                let copy = Value::new_map(Default::default());
                seen.insert(key, copy.clone());
                let Value::Map(target) = &copy else {
                    unreachable!("new_map returns a map")
                };
                for (entry_key, value) in map.borrow().iter() {
                    let value = value.deep_copy_with(seen)?;
                    target.borrow_mut().insert(entry_key.clone(), value);
                }
                Ok(copy)
            }
            // Set elements are immutable keys, so a shallow clone of the
            // set's contents into a fresh cell is already a deep copy.
            Value::Set(set) => Ok(Value::new_set(set.borrow().clone())),
            Value::Instance(instance) => {
                let key = Rc::as_ptr(instance) as *const ();
                if let Some(copy) = seen.get(&key) {
                    return Ok(copy.clone());
                }
                let r#struct = Rc::clone(&instance.borrow().r#struct);
                let target = Rc::new(RefCell::new(ObjInstance {
                    r#struct,
                    fields: Vec::new(),
                }));
                let copy = Value::Instance(Rc::clone(&target));
                seen.insert(key, copy.clone());
                for field in instance.borrow().fields.iter() {
                    let field = field.deep_copy_with(seen)?;
                    target.borrow_mut().fields.push(field);
                }
                Ok(copy)
            }
        }
    }
}
