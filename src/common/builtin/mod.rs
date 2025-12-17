use crate::common::builtin::math_functions::*;
use crate::common::constants::VARIADIC_ARITY;
use crate::common::{ObjInstance, ObjString, ObjStruct, Object, Value};
use crate::vm::file_functions::native_file_constructor;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::rc::Rc;

pub mod math_functions;

pub(crate) fn create_builtin_objects(args: Vec<String>) -> IndexMap<String, Value> {
    let mut builtin = IndexMap::new();

    builtin.insert("Math".to_string(), create_math_object());

    builtin.insert(
        "File".to_string(),
        Value::new_native_function("File".to_string(), 1, native_file_constructor),
    );

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

/// The Math object is a struct-like object with function fields
fn create_math_object() -> Value {
    let math_struct = Rc::new(ObjStruct {
        name: "Math".to_string(),
        fields: vec![
            "abs".to_string(),
            "floor".to_string(),
            "ceil".to_string(),
            "sqrt".to_string(),
            "min".to_string(),
            "max".to_string(),
        ],
    });

    let mut fields = HashMap::new();
    fields.insert(
        "abs".to_string(),
        Value::new_native_function("abs".to_string(), 1, native_math_abs),
    );
    fields.insert(
        "floor".to_string(),
        Value::new_native_function("floor".to_string(), 1, native_math_floor),
    );
    fields.insert(
        "ceil".to_string(),
        Value::new_native_function("ceil".to_string(), 1, native_math_ceil),
    );
    fields.insert(
        "sqrt".to_string(),
        Value::new_native_function("sqrt".to_string(), 1, native_math_sqrt),
    );
    fields.insert(
        "min".to_string(),
        Value::new_native_function("min".to_string(), VARIADIC_ARITY, native_math_min),
    );
    fields.insert(
        "max".to_string(),
        Value::new_native_function("max".to_string(), VARIADIC_ARITY, native_math_max),
    );

    let math_instance = ObjInstance {
        r#struct: math_struct,
        fields,
    };

    Value::new_object(math_instance)
}
