use crate::common::stdlib::array_functions;
use crate::common::stdlib::extraction_macros::extract_integer_arg;
use crate::common::NativeContext;
use crate::common::{f64_fits_i64, NativeCallError, ObjRange, Value};
use crate::extract_receiver;

/// The range's elements as `Int` values, in order.
fn elements(range: &ObjRange) -> Vec<Value> {
    (0..range.len()).map(|i| Value::Int(range.get(i))).collect()
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
    Ok(Value::Int(range.len()))
}

/// Native implementation of Range.isEmpty()
/// Returns true if the range covers no integers
pub fn native_range_is_empty(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "isEmpty() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let range = extract_receiver!(args, Range, "isEmpty")?;
    Ok(Value::Boolean(range.len() == 0))
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
        Value::Int(n) => range.contains(n),
        Value::Number(n) if n.fract() == 0.0 && f64_fits_i64(n) => range.contains(n as i64),
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

/// Native implementation of Range.step(k)
/// Returns an array of the range's values, starting at its start and
/// advancing by k each time, honoring the inclusive/exclusive end.
pub fn native_range_step(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "step() expects 1 argument (k), got {}",
            args.len() - 1
        ));
    }

    let range = extract_receiver!(args, Range, "step")?;
    let k = extract_integer_arg(args, 1, "k", "step")?;
    if k < 1 {
        return Err(format!("step() k must be >= 1, got {}", k));
    }

    let len = range.len();
    let values = (0..len)
        .step_by(usize::try_from(k).unwrap_or(usize::MAX))
        .map(|i| Value::Int(range.get(i)))
        .collect();
    Ok(Value::new_array(values))
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
pub fn native_range_map(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    array_functions::native_array_map(vm, &materialize(args, "map")?)
}

/// Native implementation of Range.filter(fn)
pub fn native_range_filter(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    array_functions::native_array_filter(vm, &materialize(args, "filter")?)
}

/// Native implementation of Range.forEach(fn)
pub fn native_range_for_each(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    array_functions::native_array_for_each(vm, &materialize(args, "forEach")?)
}

/// Native implementation of Range.flatMap(fn)
pub fn native_range_flat_map(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    array_functions::native_array_flat_map(vm, &materialize(args, "flatMap")?)
}

/// Native implementation of Range.take(n)
pub fn native_range_take(args: &[Value]) -> Result<Value, String> {
    array_functions::native_array_take(&materialize(args, "take")?)
}

/// Native implementation of Range.drop(n)
pub fn native_range_drop(args: &[Value]) -> Result<Value, String> {
    array_functions::native_array_drop(&materialize(args, "drop")?)
}

/// Native implementation of Range.first()
pub fn native_range_first(args: &[Value]) -> Result<Value, String> {
    array_functions::native_array_first(&materialize(args, "first")?)
}

/// Native implementation of Range.last()
pub fn native_range_last(args: &[Value]) -> Result<Value, String> {
    array_functions::native_array_last(&materialize(args, "last")?)
}

/// Native implementation of Range.chunked(n)
pub fn native_range_chunked(args: &[Value]) -> Result<Value, String> {
    array_functions::native_array_chunked(&materialize(args, "chunked")?)
}

/// Native implementation of Range.zip(other)
pub fn native_range_zip(args: &[Value]) -> Result<Value, String> {
    array_functions::native_array_zip(&materialize(args, "zip")?)
}

/// Native implementation of Range.withIndex()
pub fn native_range_with_index(args: &[Value]) -> Result<Value, String> {
    array_functions::native_array_with_index(&materialize(args, "withIndex")?)
}

/// Native implementation of Range.reduce(fn, initial)
pub fn native_range_reduce(
    vm: &mut dyn NativeContext,
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
