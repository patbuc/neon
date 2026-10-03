use crate::common::{f64_fits_i64, Value};
use crate::{extract_receiver, string};

/// Native implementation of Number.toString()
/// Converts a number to its string representation
/// Handles edge case: removes trailing ".0" for integer values
pub fn native_number_to_string(args: &[Value]) -> Result<Value, String> {
    if let Some(Value::Int(n)) = args.first() {
        return Ok(string!(n.to_string()));
    }
    let num = extract_receiver!(args, Number, "toString")?;
    let num_str = if num.fract() == 0.0 && num.is_finite() {
        // Integer value: format without decimal point
        format!("{:.0}", num)
    } else {
        // Decimal value: use standard formatting
        num.to_string()
    };
    Ok(string!(num_str))
}

/// Native implementation of Number.toInt()
/// An int receiver is returned as is; a float truncates toward zero and
/// errors if the result doesn't fit in `i64` or is NaN/infinite.
pub fn native_number_to_int(args: &[Value]) -> Result<Value, String> {
    match args.first() {
        Some(Value::Int(n)) => Ok(Value::Int(*n)),
        Some(Value::Number(n)) => {
            if !f64_fits_i64(n.trunc()) {
                return Err("toInt() result is out of range".to_string());
            }
            Ok(Value::Int(n.trunc() as i64))
        }
        _ => Err("toInt() can only be called on numbers".to_string()),
    }
}

/// Native implementation of Number.toFloat()
/// Returns the value as a float, converting an int exactly where possible.
pub fn native_number_to_float(args: &[Value]) -> Result<Value, String> {
    match args.first() {
        Some(Value::Int(n)) => Ok(Value::Number(*n as f64)),
        Some(Value::Number(n)) => Ok(Value::Number(*n)),
        _ => Err("toFloat() can only be called on numbers".to_string()),
    }
}
