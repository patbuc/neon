use crate::common::Value;
use crate::vm::VirtualMachine;
use crate::string;

/// Native implementation of Boolean.toString()
/// Converts a boolean to its string representation
/// Returns "true" for true values and "false" for false values
pub fn native_boolean_to_string(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("boolean.toString() requires a boolean receiver".to_string());
    }

    match &args[0] {
        Value::Boolean(b) => {
            let bool_str = b.to_string();
            Ok(string!(bool_str))
        }
        _ => Err("toString() can only be called on booleans".to_string()),
    }
}
