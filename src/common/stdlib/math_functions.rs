use crate::common::Value;
use crate::vm::VirtualMachine;
use crate::extract_arg;

/// Native implementation of Math.abs(x)
/// Returns the absolute value of a number
pub fn native_math_abs(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("abs() expects 1 argument, got {}", args.len()));
    }

    let n = extract_arg!(args, 0, Number, "x", "abs")?;
    Ok(Value::Number(n.abs()))
}

/// Native implementation of Math.floor(x)
/// Returns the largest integer less than or equal to a number
pub fn native_math_floor(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("floor() expects 1 argument, got {}", args.len()));
    }

    let n = extract_arg!(args, 0, Number, "x", "floor")?;
    Ok(Value::Number(n.floor()))
}

/// Native implementation of Math.ceil(x)
/// Returns the smallest integer greater than or equal to a number
pub fn native_math_ceil(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("ceil() expects 1 argument, got {}", args.len()));
    }

    let n = extract_arg!(args, 0, Number, "x", "ceil")?;
    Ok(Value::Number(n.ceil()))
}

/// Native implementation of Math.sqrt(x)
/// Returns the square root of a number
pub fn native_math_sqrt(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("sqrt() expects 1 argument, got {}", args.len()));
    }

    let n = extract_arg!(args, 0, Number, "x", "sqrt")?;
    if n < 0.0 {
        return Err("sqrt() requires a non-negative number".to_string());
    }
    Ok(Value::Number(n.sqrt()))
}

/// Native implementation of Math.min(...args)
/// Returns the smallest of the given numbers (variadic)
pub fn native_math_min(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("min() requires at least 1 argument".to_string());
    }

    let mut min_value = extract_arg!(args, 0, Number, "first argument", "min")?;

    for i in 1..args.len() {
        let n = extract_arg!(args, i, Number, &format!("argument {}", i), "min")?;
        if n < min_value {
            min_value = n;
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

    let mut max_value = extract_arg!(args, 0, Number, "first argument", "max")?;

    for i in 1..args.len() {
        let n = extract_arg!(args, i, Number, &format!("argument {}", i), "max")?;
        if n > max_value {
            max_value = n;
        }
    }

    Ok(Value::Number(max_value))
}
