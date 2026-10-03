use crate::common::{compare_numeric, f64_fits_i64, Numeric, Value};
use crate::extract_arg;

fn extract_numeric(
    args: &[Value],
    idx: usize,
    arg_name: &str,
    method: &str,
) -> Result<Numeric, String> {
    match args.get(idx) {
        Some(value) => Numeric::from_value(value)
            .ok_or_else(|| format!("{}() {} must be a number", method, arg_name)),
        None => Err(format!(
            "{}() missing required argument: {}",
            method, arg_name
        )),
    }
}

/// Native implementation of Math.abs(x)
/// Returns the absolute value of a number, keeping an int an int.
pub fn native_math_abs(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("abs() expects 1 argument, got {}", args.len()));
    }

    match extract_numeric(args, 0, "x", "abs")? {
        Numeric::Int(n) => n
            .checked_abs()
            .map(Value::Int)
            .ok_or_else(|| "integer overflow in abs()".to_string()),
        Numeric::Float(n) => Ok(Value::Number(n.abs())),
    }
}

/// Converts an integral `f64` into `Value::Int`, or an error naming `method`
/// if it is NaN, infinite, or doesn't fit in `i64`.
fn integral_to_int(f: f64, method: &str) -> Result<Value, String> {
    if !f64_fits_i64(f) {
        return Err(format!("{}() result is out of range", method));
    }
    Ok(Value::Int(f as i64))
}

/// Native implementation of Math.floor(x)
/// Returns the largest integer less than or equal to a number.
pub fn native_math_floor(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("floor() expects 1 argument, got {}", args.len()));
    }

    match extract_numeric(args, 0, "x", "floor")? {
        Numeric::Int(n) => Ok(Value::Int(n)),
        Numeric::Float(n) => integral_to_int(n.floor(), "floor"),
    }
}

/// Native implementation of Math.ceil(x)
/// Returns the smallest integer greater than or equal to a number.
pub fn native_math_ceil(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("ceil() expects 1 argument, got {}", args.len()));
    }

    match extract_numeric(args, 0, "x", "ceil")? {
        Numeric::Int(n) => Ok(Value::Int(n)),
        Numeric::Float(n) => integral_to_int(n.ceil(), "ceil"),
    }
}

/// Native implementation of Math.sqrt(x)
/// Returns the square root of a number
pub fn native_math_sqrt(args: &[Value]) -> Result<Value, String> {
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
pub fn native_math_min(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("min() requires at least 1 argument".to_string());
    }

    let mut min_value = extract_numeric(args, 0, "first argument", "min")?;

    for i in 1..args.len() {
        let candidate = extract_numeric(args, i, &format!("argument {}", i), "min")?;
        if compare_numeric(candidate, min_value) == Some(std::cmp::Ordering::Less) {
            min_value = candidate;
        }
    }

    Ok(min_value.into_value())
}

/// Native implementation of Math.max(...args)
/// Returns the largest of the given numbers (variadic)
pub fn native_math_max(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("max() requires at least 1 argument".to_string());
    }

    let mut max_value = extract_numeric(args, 0, "first argument", "max")?;

    for i in 1..args.len() {
        let candidate = extract_numeric(args, i, &format!("argument {}", i), "max")?;
        if compare_numeric(candidate, max_value) == Some(std::cmp::Ordering::Greater) {
            max_value = candidate;
        }
    }

    Ok(max_value.into_value())
}

fn extract_div_operand(args: &[Value], idx: usize, name: &str) -> Result<i64, String> {
    match args.get(idx) {
        Some(Value::Int(i)) => Ok(*i),
        Some(Value::Number(_)) => Err("div() expects two integers, got float".to_string()),
        Some(_) => Err(format!("div() {} must be a number", name)),
        None => Err(format!("div() missing required argument: {}", name)),
    }
}

/// Native implementation of Math.div(a, b)
/// Floor division on two ints: rounds the quotient toward negative infinity.
pub fn native_math_div(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("div() expects 2 arguments, got {}", args.len()));
    }

    let a = extract_div_operand(args, 0, "a")?;
    let b = extract_div_operand(args, 1, "b")?;

    if b == 0 {
        return Err("div() division by zero".to_string());
    }

    let quotient = a
        .checked_div(b)
        .ok_or_else(|| "integer overflow in div()".to_string())?;
    let remainder = a % b;
    let floored = if remainder != 0 && (remainder < 0) != (b < 0) {
        quotient - 1
    } else {
        quotient
    };

    Ok(Value::Int(floored))
}
