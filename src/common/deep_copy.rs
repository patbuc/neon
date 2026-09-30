use crate::common::{ObjInstance, Value};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Value {
    /// A copy sharing no mutable state with `self`, for a value crossing a
    /// task boundary. Iterative so deeply nested containers can't overflow
    /// the stack.
    pub(crate) fn deep_copy(&self) -> Result<Value, String> {
        let mut seen: HashMap<*const (), Value> = HashMap::new();
        let mut work: Vec<(Value, Value)> = Vec::new();
        let result = Self::copy_child(self, &mut seen, &mut work)?;

        while let Some((source, copy)) = work.pop() {
            match (&source, &copy) {
                (Value::Array(source), Value::Array(copy)) => {
                    for element in source.borrow().iter() {
                        let element = Self::copy_child(element, &mut seen, &mut work)?;
                        copy.borrow_mut().push(element);
                    }
                }
                (Value::Map(source), Value::Map(copy)) => {
                    for (map_key, entry) in source.borrow().iter() {
                        let entry = Self::copy_child(entry, &mut seen, &mut work)?;
                        copy.borrow_mut().insert(map_key.clone(), entry);
                    }
                }
                (Value::Instance(source), Value::Instance(copy)) => {
                    for field in source.borrow().fields.iter() {
                        let field = Self::copy_child(field, &mut seen, &mut work)?;
                        copy.borrow_mut().fields.push(field);
                    }
                }
                _ => unreachable!("only arrays, maps, and instances are queued"),
            }
        }

        Ok(result)
    }

    /// The copy of one child value: a clone for a shareable value, an error
    /// for a fiber or capturing closure, the `seen` copy for a container
    /// already visited, or else a fresh empty copy queued for filling in.
    fn copy_child(
        value: &Value,
        seen: &mut HashMap<*const (), Value>,
        work: &mut Vec<(Value, Value)>,
    ) -> Result<Value, String> {
        match value {
            Value::Number(_)
            | Value::Boolean(_)
            | Value::Nil
            | Value::Uninitialized(_)
            | Value::String(_)
            | Value::Function(_)
            | Value::NativeFunction(_)
            | Value::Struct(_)
            | Value::File(_)
            | Value::Range(_) => Ok(value.clone()),
            Value::Closure(closure) if closure.upvalues.is_empty() => Ok(value.clone()),
            Value::Closure(closure) => Err(format!(
                "Cannot copy function '{}' across a task boundary: it captures variables",
                closure.function.name
            )),
            Value::Fiber(_) => Err(format!(
                "Cannot copy a {} across a task boundary",
                value.type_name()
            )),
            Value::Set(set) => {
                let key = Rc::as_ptr(set) as *const ();
                if let Some(copy) = seen.get(&key) {
                    return Ok(copy.clone());
                }
                let copy = Value::new_set(set.borrow().clone());
                seen.insert(key, copy.clone());
                Ok(copy)
            }
            Value::Array(array) => {
                let key = Rc::as_ptr(array) as *const ();
                if let Some(copy) = seen.get(&key) {
                    return Ok(copy.clone());
                }
                let copy = Value::new_array(Vec::with_capacity(array.borrow().len()));
                seen.insert(key, copy.clone());
                work.push((value.clone(), copy.clone()));
                Ok(copy)
            }
            Value::Map(map) => {
                let key = Rc::as_ptr(map) as *const ();
                if let Some(copy) = seen.get(&key) {
                    return Ok(copy.clone());
                }
                let copy = Value::new_map(IndexMap::with_capacity(map.borrow().len()));
                seen.insert(key, copy.clone());
                work.push((value.clone(), copy.clone()));
                Ok(copy)
            }
            Value::Instance(instance) => {
                let key = Rc::as_ptr(instance) as *const ();
                if let Some(copy) = seen.get(&key) {
                    return Ok(copy.clone());
                }
                let r#struct = Rc::clone(&instance.borrow().r#struct);
                let copy = Value::Instance(Rc::new(RefCell::new(ObjInstance {
                    r#struct,
                    fields: Vec::new(),
                })));
                seen.insert(key, copy.clone());
                work.push((value.clone(), copy.clone()));
                Ok(copy)
            }
        }
    }
}
