use crate::common::Value;
use crate::{extract_receiver, string};

/// Native implementation of Number.toString()
/// Converts a number to its string representation
/// Handles edge case: removes trailing ".0" for integer values
pub fn native_number_to_string(args: &[Value]) -> Result<Value, String> {
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
