use crate::common::Value;
use crate::vm::VirtualMachine;

/// Native implementation of Math.abs(x)
/// Returns the absolute value of a number
pub fn native_math_abs(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("abs() expects 1 argument, got {}", args.len()));
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.abs())),
        _ => Err("abs() requires a number argument".to_string()),
    }
}

/// Native implementation of Math.floor(x)
/// Returns the largest integer less than or equal to a number
pub fn native_math_floor(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("floor() expects 1 argument, got {}", args.len()));
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.floor())),
        _ => Err("floor() requires a number argument".to_string()),
    }
}

/// Native implementation of Math.ceil(x)
/// Returns the smallest integer greater than or equal to a number
pub fn native_math_ceil(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("ceil() expects 1 argument, got {}", args.len()));
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.ceil())),
        _ => Err("ceil() requires a number argument".to_string()),
    }
}

/// Native implementation of Math.sqrt(x)
/// Returns the square root of a number
pub fn native_math_sqrt(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("sqrt() expects 1 argument, got {}", args.len()));
    }

    match &args[0] {
        Value::Number(n) => {
            if *n < 0.0 {
                return Err("sqrt() requires a non-negative number".to_string());
            }
            Ok(Value::Number(n.sqrt()))
        }
        _ => Err("sqrt() requires a number argument".to_string()),
    }
}

/// Native implementation of Math.min(...args)
/// Returns the smallest of the given numbers (variadic)
pub fn native_math_min(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("min() requires at least 1 argument".to_string());
    }

    let mut min_value = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err("min() requires number arguments".to_string()),
    };

    for (i, arg) in args.iter().enumerate().skip(1) {
        match arg {
            Value::Number(n) => {
                if *n < min_value {
                    min_value = *n;
                }
            }
            _ => {
                return Err(format!(
                    "min() requires number arguments, got non-number at position {}",
                    i
                ))
            }
        }
    }

    Ok(Value::Number(min_value))
}

/// Native implementation of Math.max(...args)
/// Returns the largest of the given numbers (variadic)
pub fn native_math_max(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("max() requires at least 1 argument".to_string());
    }

    let mut max_value = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err("max() requires number arguments".to_string()),
    };

    for (i, arg) in args.iter().enumerate().skip(1) {
        match arg {
            Value::Number(n) => {
                if *n > max_value {
                    max_value = *n;
                }
            }
            _ => {
                return Err(format!(
                    "max() requires number arguments, got non-number at position {}",
                    i
                ))
            }
        }
    }

    Ok(Value::Number(max_value))
}
