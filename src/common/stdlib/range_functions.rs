use crate::common::stdlib::array_functions;
use crate::common::{NativeCallError, ObjRange, Value};
use crate::extract_receiver;
use crate::vm::VirtualMachine;

/// The range's elements as `Number` values, in order.
fn elements(range: &ObjRange) -> Vec<Value> {
    (0..range.len())
        .map(|i| Value::Number(range.get(i) as f64))
        .collect()
}

/// Rebuilds `args` as `[array, rest...]`, so a materializing method can
/// delegate to the existing Array implementation instead of duplicating it.
fn materialize(args: &[Value], method: &str) -> Result<Vec<Value>, String> {
    let range = extract_receiver!(args, Range, method)?;
    let mut new_args = Vec::with_capacity(args.len());
    new_args.push(Value::new_array(elements(range)));
    new_args.extend_from_slice(&args[1..]);
    Ok(new_args)
}

/// Native implementation of Range.size()
/// Returns the number of integers the range covers
pub fn native_range_size(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "size() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let range = extract_receiver!(args, Range, "size")?;
    Ok(Value::Number(range.len() as f64))
}

/// Native implementation of Range.length()
/// Returns the number of integers the range covers
pub fn native_range_length(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "length() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let range = extract_receiver!(args, Range, "length")?;
    Ok(Value::Number(range.len() as f64))
}

/// Native implementation of Range.contains(element)
/// True iff element is a Number that is an integer within the range
pub fn native_range_contains(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "contains() expects 1 argument (element), got {}",
            args.len() - 1
        ));
    }

    let range = extract_receiver!(args, Range, "contains")?;
    let contains = match args[1] {
        Value::Number(n) if n.fract() == 0.0 => {
            let n = n as i64;
            n >= range.start && n < range.start + range.len()
        }
        _ => false,
    };

    Ok(Value::Boolean(contains))
}

/// Native implementation of Range.toArray()
/// Returns a new array of the range's elements
pub fn native_range_to_array(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "toArray() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let range = extract_receiver!(args, Range, "toArray")?;
    Ok(Value::new_array(elements(range)))
}

/// Native implementation of Range.slice(start, end)
pub fn native_range_slice(args: &[Value]) -> Result<Value, String> {
    array_functions::native_array_slice(&materialize(args, "slice")?)
}

/// Native implementation of Range.join(delimiter)
pub fn native_range_join(args: &[Value]) -> Result<Value, String> {
    array_functions::native_array_join(&materialize(args, "join")?)
}

/// Native implementation of Range.indexOf(element)
pub fn native_range_index_of(args: &[Value]) -> Result<Value, String> {
    array_functions::native_array_index_of(&materialize(args, "indexOf")?)
}

/// Native implementation of Range.sum()
pub fn native_range_sum(args: &[Value]) -> Result<Value, String> {
    array_functions::native_array_sum(&materialize(args, "sum")?)
}

/// Native implementation of Range.min()
pub fn native_range_min(args: &[Value]) -> Result<Value, String> {
    array_functions::native_array_min(&materialize(args, "min")?)
}

/// Native implementation of Range.max()
pub fn native_range_max(args: &[Value]) -> Result<Value, String> {
    array_functions::native_array_max(&materialize(args, "max")?)
}

/// Native implementation of Range.map(fn)
pub fn native_range_map(vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, NativeCallError> {
    array_functions::native_array_map(vm, &materialize(args, "map")?)
}

/// Native implementation of Range.filter(fn)
pub fn native_range_filter(
    vm: &mut VirtualMachine,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    array_functions::native_array_filter(vm, &materialize(args, "filter")?)
}

/// Native implementation of Range.reduce(fn, initial)
pub fn native_range_reduce(
    vm: &mut VirtualMachine,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    array_functions::native_array_reduce(vm, &materialize(args, "reduce")?)
}

fn immutable_error(method: &str) -> String {
    format!(
        "{}() cannot be called on a range: ranges are immutable",
        method
    )
}

/// Native implementation of Range.push(value) - always errors
pub fn native_range_push(_args: &[Value]) -> Result<Value, String> {
    Err(immutable_error("push"))
}

/// Native implementation of Range.pop() - always errors
pub fn native_range_pop(_args: &[Value]) -> Result<Value, String> {
    Err(immutable_error("pop"))
}

/// Native implementation of Range.sort() - always errors
pub fn native_range_sort(_args: &[Value]) -> Result<Value, String> {
    Err(immutable_error("sort"))
}

/// Native implementation of Range.reverse() - always errors
pub fn native_range_reverse(_args: &[Value]) -> Result<Value, String> {
    Err(immutable_error("reverse"))
}
