use crate::common::Value;
use crate::{extract_arg, extract_receiver};
use std::rc::Rc;

/// Native implementation of Error(message)
pub fn native_error_constructor(args: &[Value]) -> Result<Value, String> {
    let message = extract_arg!(args, 0, String, "message", "Error")?;
    Ok(Value::new_error(message.to_string()))
}

/// Native implementation of Error.trace(). Returns the call trace captured
/// when the error was first thrown, or "" if it never was.
pub fn native_error_trace(args: &[Value]) -> Result<Value, String> {
    let error = extract_receiver!(args, Error, "trace")?;
    let trace = error
        .thrown_at
        .get()
        .map(|at| at.trace())
        .unwrap_or_default();
    Ok(Value::String(Rc::new(trace)))
}
