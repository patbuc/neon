use crate::common::{ObjString, Object, Value};
use indexmap::IndexMap;
use std::rc::Rc;

pub mod math_functions;

/// Create builtin objects for the VM.
/// Math and File are now handled through the unified registry system.
/// Only runtime values like args remain as builtins.
pub(crate) fn create_builtin_objects(args: Vec<String>) -> IndexMap<String, Value> {
    let mut builtin = IndexMap::new();

    // args is a runtime value, so it remains a builtin
    builtin.insert("args".to_string(), create_args_array(args));

    builtin
}

fn create_args_array(args: Vec<String>) -> Value {
    let elements: Vec<Value> = args
        .into_iter()
        .map(|arg| {
            Value::Object(Rc::new(Object::String(ObjString {
                value: Rc::from(arg.as_str()),
            })))
        })
        .collect();
    Value::new_array(elements)
}
