use crate::common::Value;
use crate::extract_arg;

/// Native implementation of Error(message)
pub fn native_error_constructor(args: &[Value]) -> Result<Value, String> {
    let message = extract_arg!(args, 0, String, "message", "Error")?;
    Ok(Value::new_error(message.to_string()))
}
