use crate::common::{ObjString, Object, Value};
use indexmap::IndexMap;
use std::rc::Rc;

pub(crate) mod array_functions;
pub(crate) mod boolean_functions;
pub(crate) mod file_functions;
pub(crate) mod map_functions;
pub(crate) mod math_functions;
pub(crate) mod number_functions;
pub(crate) mod set_functions;
pub(crate) mod string_functions;
pub(crate) mod system_functions;

#[macro_use]
pub(crate) mod extraction_macros;

#[cfg(test)]
mod tests;

/// Create stdlib objects for the VM.
/// Math and File are now handled through the unified registry system.
/// Only runtime values like args remain as builtins.
pub fn create_builtin_objects(args: Vec<String>) -> IndexMap<String, Value> {
    let mut builtin = IndexMap::new();

    // args is a runtime value, so it remains a stdlib
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
