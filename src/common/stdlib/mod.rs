use crate::common::static_type::StaticType;
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

/// Name of the command-line arguments builtin, shared with `BUILTIN_VALUES`
/// so the VM and the semantic analyzer never drift apart on its name.
const ARGS: &str = "args";

/// Runtime builtin values and their static type, used by both
/// `create_builtin_objects` (to build the VM's values) and
/// `SemanticAnalyzer::new` (to predefine them for method validation). Their
/// order here is also the `GetBuiltin` operand a resolution's `Res::Builtin`
/// carries, so the VM and the semantic analyzer must agree on it. Math and
/// File are namespaces, not values, and come from the method registry
/// instead (see `method_registry::namespaces`).
pub const BUILTIN_VALUES: &[(&str, StaticType)] = &[(ARGS, StaticType::Array)];

/// Create stdlib objects for the VM, in `BUILTIN_VALUES` order.
pub fn create_builtin_objects(args: Vec<String>) -> IndexMap<String, Value> {
    BUILTIN_VALUES
        .iter()
        .map(|(name, _type_name)| {
            let value = match *name {
                ARGS => create_args_array(&args),
                other => panic!("no constructor for builtin value '{}'", other),
            };
            (name.to_string(), value)
        })
        .collect()
}

fn create_args_array(args: &[String]) -> Value {
    let elements: Vec<Value> = args
        .iter()
        .map(|arg| {
            Value::Object(Rc::new(Object::String(ObjString {
                value: Rc::from(arg.as_str()),
            })))
        })
        .collect();
    Value::new_array(elements)
}
