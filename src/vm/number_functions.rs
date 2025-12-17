use crate::common::Value;
use crate::string;

/// Native implementation of Number.toString()
/// Converts a number to its string representation
/// Handles edge case: removes trailing ".0" for integer values
pub fn native_number_to_string(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("number.toString() requires a number receiver".to_string());
    }

    match &args[0] {
        Value::Number(num) => {
            let num_str = if num.fract() == 0.0 && num.is_finite() {
                // Integer value: format without decimal point
                format!("{:.0}", num)
            } else {
                // Decimal value: use standard formatting
                num.to_string()
            };
            Ok(string!(num_str))
        }
        _ => Err("toString() can only be called on numbers".to_string()),
    }
}
