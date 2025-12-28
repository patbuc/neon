use crate::common::Value;
use crate::{extract_receiver};
use crate::vm::VirtualMachine;

/// Native implementation of Boolean.toString()
/// Converts a boolean to its string representation
/// Returns "true" for true values and "false" for false values
pub fn native_boolean_to_string(vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    let b = extract_receiver!(args, Boolean, "toString")?;
    let bool_str = b.to_string();
    Ok(vm.intern_string(&bool_str))
}
