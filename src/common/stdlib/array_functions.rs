use crate::common::stdlib::extraction_macros::extract_integer_arg;
use crate::common::NativeContext;
use crate::common::{compare_numeric, NativeCallError, Numeric, Value};
use crate::{extract_arg, extract_receiver, extract_string_value, is_false_like};

/// Native implementation of Array.push(value)
/// Adds an element to the end of the array and returns nil
/// New calling convention: [receiver, args...]
pub fn native_array_push(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "push() expects 1 argument (value), got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array_ref = extract_receiver!(args, Array, "push")?;

    // Extract the value to push (args[1] in new convention)
    let value = &args[1];

    // Push the value onto the array
    let mut array = array_ref.borrow_mut();
    array.push(value.clone());

    Ok(Value::Nil)
}

/// Native implementation of Array.pop()
/// Removes and returns the last element of the array, or nil if the array is empty
pub fn native_array_pop(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "pop() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array_ref = extract_receiver!(args, Array, "pop")?;

    // Pop the last element
    let mut array = array_ref.borrow_mut();
    Ok(array.pop().unwrap_or(Value::Nil))
}

/// Native implementation of Array.size()
/// Returns the number of elements in the array
pub fn native_array_size(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "size() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "size")?;
    let elements = array_ref.borrow();
    Ok(Value::Int(elements.len() as i64))
}

/// Native implementation of Array.isEmpty()
/// Returns true if the array has no elements
pub fn native_array_is_empty(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "isEmpty() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "isEmpty")?;
    let elements = array_ref.borrow();
    Ok(Value::Boolean(elements.is_empty()))
}

/// Native implementation of Array.contains(element)
/// Returns true if the array contains the specified element
pub fn native_array_contains(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "contains() expects 1 argument (element), got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array = extract_receiver!(args, Array, "contains")?;

    // Get the element to search for
    let element = &args[1];

    // Check if the array contains the element
    let elements = array.borrow();
    let contains = elements.iter().any(|e| e == element);

    Ok(Value::Boolean(contains))
}

/// Sort bucket for `sort()`'s mixed-type ordering: numbers first, then
/// everything else, then booleans/nil/uninitialized last.
fn sort_rank(value: &Value) -> u8 {
    match value {
        Value::Number(_) | Value::Int(_) => 0,
        Value::Boolean(_) | Value::Nil | Value::Uninitialized(_) => 2,
        _ => 1,
    }
}

/// Native implementation of Array.sort() / Array.sort(comparator)
/// Sorts in place (default order, or by calling the comparator on each
/// pair) and returns the same array.
pub fn native_array_sort(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 1 && args.len() != 2 {
        return Err(format!("sort() expects 0 or 1 arguments, got {}", args.len() - 1).into());
    }

    // Extract the array
    let array_ref = extract_receiver!(args, Array, "sort")?;

    match args.get(1) {
        None => {
            array_ref.borrow_mut().sort_by(|a, b| {
                match (Numeric::from_value(a), Numeric::from_value(b)) {
                    (Some(na), Some(nb)) => {
                        compare_numeric(na, nb).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    _ => match (a, b) {
                        (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
                        _ => sort_rank(a).cmp(&sort_rank(b)),
                    },
                }
            });
        }
        Some(comparator) => {
            let comparator = comparator.clone();
            // Snapshot so the comparator can mutate the array without us
            // holding a RefCell borrow across the callback.
            let elements: Vec<Value> = array_ref.borrow().clone();
            let sorted = merge_sort_by(vm, elements, &comparator)?;
            *array_ref.borrow_mut() = sorted;
        }
    }

    Ok(args[0].clone())
}

/// Stable merge sort driven by a user comparator.
fn merge_sort_by(
    vm: &mut dyn NativeContext,
    mut values: Vec<Value>,
    comparator: &Value,
) -> Result<Vec<Value>, NativeCallError> {
    if values.len() <= 1 {
        return Ok(values);
    }

    let right = values.split_off(values.len() / 2);
    let left = merge_sort_by(vm, values, comparator)?;
    let right = merge_sort_by(vm, right, comparator)?;
    merge_by(vm, left, right, comparator)
}

/// Merges two already-sorted runs, favoring the left run on a tie so the
/// merge is stable.
#[allow(clippy::expect_used)]
fn merge_by(
    vm: &mut dyn NativeContext,
    left: Vec<Value>,
    right: Vec<Value>,
    comparator: &Value,
) -> Result<Vec<Value>, NativeCallError> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left = left.into_iter().peekable();
    let mut right = right.into_iter().peekable();

    while let (Some(a), Some(b)) = (left.peek(), right.peek()) {
        let order = compare(vm, comparator, a, b)?;
        if order > 0.0 {
            result.push(right.next().expect("peek just confirmed Some"));
        } else {
            result.push(left.next().expect("peek just confirmed Some"));
        }
    }
    result.extend(left);
    result.extend(right);

    Ok(result)
}

/// Converts a comparator's return value to the signed number `sort()` needs.
fn comparator_result_to_f64(value: Value) -> Result<f64, NativeCallError> {
    match value {
        Value::Number(n) => Ok(n),
        Value::Int(i) => Ok(i as f64),
        other => Err(format!(
            "sort() comparator must return a number, got {}",
            other.type_name()
        )
        .into()),
    }
}

/// Calls the comparator with (a, b) and requires a number result.
fn compare(
    vm: &mut dyn NativeContext,
    comparator: &Value,
    a: &Value,
    b: &Value,
) -> Result<f64, NativeCallError> {
    comparator_result_to_f64(vm.call_value(comparator.clone(), &[a.clone(), b.clone()])?)
}

/// Native implementation of Array.reverse()
/// Reverses array in place
pub fn native_array_reverse(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "reverse() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array_ref = extract_receiver!(args, Array, "reverse")?;

    // Reverse the array
    let mut array = array_ref.borrow_mut();
    array.reverse();

    Ok(Value::Nil)
}

/// Native implementation of Array.slice(start, end)
/// Extracts a subarray (supports negative indices)
pub fn native_array_slice(args: &[Value]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "slice() expects 2 arguments (start, end), got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array_ref = extract_receiver!(args, Array, "slice")?;

    // Extract start index
    let start = extract_arg!(args, 1, Number, "start index", "slice")? as i64;

    // Extract end index
    let end = extract_arg!(args, 2, Number, "end index", "slice")? as i64;

    let array = array_ref.borrow();
    let len = array.len() as i64;

    // Handle negative indices
    let start_idx = if start < 0 {
        (len + start).max(0) as usize
    } else {
        start.min(len) as usize
    };

    let end_idx = if end < 0 {
        (len + end).max(0) as usize
    } else {
        end.min(len) as usize
    };

    // Extract the slice
    let sliced = if start_idx < end_idx {
        array[start_idx..end_idx].to_vec()
    } else {
        vec![]
    };

    Ok(Value::new_array(sliced))
}

/// Native implementation of Array.join(delimiter)
/// Joins array elements into string
pub fn native_array_join(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "join() expects 1 argument (delimiter), got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array_ref = extract_receiver!(args, Array, "join")?;

    // Extract delimiter
    let delimiter = extract_string_value!(args, 1, "delimiter", "join");

    let array = array_ref.borrow();
    let parts: Vec<String> = array.iter().map(|v| format!("{}", v)).collect();
    let result = parts.join(delimiter);

    Ok(Value::String(std::rc::Rc::new(result)))
}

/// Native implementation of Array.indexOf(element)
/// Finds first occurrence index (-1 if not found)
pub fn native_array_index_of(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "indexOf() expects 1 argument (element), got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array_ref = extract_receiver!(args, Array, "indexOf")?;

    let element = &args[1];
    let array = array_ref.borrow();

    // Find the element
    let index = array.iter().position(|e| e == element);

    match index {
        Some(idx) => Ok(Value::Int(idx as i64)),
        None => Ok(Value::Int(-1)),
    }
}

/// Native implementation of Array.sum()
/// Sums numeric array (error if non-numeric)
pub fn native_array_sum(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "sum() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array_ref = extract_receiver!(args, Array, "sum")?;

    let array = array_ref.borrow();

    let mut numbers = Vec::with_capacity(array.len());
    for (i, value) in array.iter().enumerate() {
        match Numeric::from_value(value) {
            Some(n) => numbers.push(n),
            None => {
                return Err(format!(
                    "sum() requires all elements to be numbers, but element at index {} is not",
                    i
                ))
            }
        }
    }

    // Any float makes the sum a float.
    if numbers.iter().any(|n| matches!(n, Numeric::Float(_))) {
        let sum = numbers.iter().map(|n| n.as_f64()).sum();
        return Ok(Value::Number(sum));
    }

    let mut sum: i64 = 0;
    for n in numbers {
        if let Numeric::Int(i) = n {
            sum = sum
                .checked_add(i)
                .ok_or_else(|| "integer overflow in sum()".to_string())?;
        }
    }

    Ok(Value::Int(sum))
}

/// Native implementation of Array.min()
/// Finds minimum value in array
pub fn native_array_min(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "min() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array_ref = extract_receiver!(args, Array, "min")?;

    let array = array_ref.borrow();

    if array.is_empty() {
        return Err("min() cannot be called on an empty array".to_string());
    }

    // Find minimum - handle both numbers and strings
    let mut min = &array[0];

    for value in array.iter().skip(1) {
        let is_less = match (Numeric::from_value(value), Numeric::from_value(min)) {
            (Some(a), Some(b)) => compare_numeric(a, b) == Some(std::cmp::Ordering::Less),
            _ => match (value, min) {
                (Value::String(s1), Value::String(s2)) => s1 < s2,
                _ => return Err("min() can only compare numbers or strings".to_string()),
            },
        };

        if is_less {
            min = value;
        }
    }

    Ok(min.clone())
}

/// Native implementation of Array.max()
/// Finds maximum value in array
pub fn native_array_max(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "max() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    // Extract the array
    let array_ref = extract_receiver!(args, Array, "max")?;

    let array = array_ref.borrow();

    if array.is_empty() {
        return Err("max() cannot be called on an empty array".to_string());
    }

    // Find maximum - handle both numbers and strings
    let mut max = &array[0];

    for value in array.iter().skip(1) {
        let is_greater = match (Numeric::from_value(value), Numeric::from_value(max)) {
            (Some(a), Some(b)) => compare_numeric(a, b) == Some(std::cmp::Ordering::Greater),
            _ => match (value, max) {
                (Value::String(s1), Value::String(s2)) => s1 > s2,
                _ => return Err("max() can only compare numbers or strings".to_string()),
            },
        };

        if is_greater {
            max = value;
        }
    }

    Ok(max.clone())
}

/// Native implementation of Array.map(fn)
/// Returns a new array with fn applied to each element.
/// Snapshots the elements before calling fn, so a callback that mutates
/// the receiving array doesn't change what map iterates over.
pub fn native_array_map(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "map() expects 1 argument (function), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "map")?;
    let callback = args[1].clone();
    let elements: Vec<Value> = array_ref.borrow().clone();

    let mut mapped = Vec::with_capacity(elements.len());
    for element in elements {
        mapped.push(vm.call_value(callback.clone(), &[element])?);
    }

    Ok(Value::new_array(mapped))
}

/// Native implementation of Array.forEach(fn)
/// Calls fn with each element in order. Returns nil.
pub fn native_array_for_each(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "forEach() expects 1 argument (function), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "forEach")?;
    let callback = args[1].clone();
    let elements: Vec<Value> = array_ref.borrow().clone();

    for element in elements {
        vm.call_value(callback.clone(), &[element])?;
    }

    Ok(Value::Nil)
}

/// Native implementation of Array.flatMap(fn)
/// Maps fn over the elements and concatenates the resulting arrays.
pub fn native_array_flat_map(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "flatMap() expects 1 argument (function), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "flatMap")?;
    let callback = args[1].clone();
    let elements: Vec<Value> = array_ref.borrow().clone();

    let mut flattened = Vec::with_capacity(elements.len());
    for element in elements {
        let mapped = vm.call_value(callback.clone(), &[element])?;
        match mapped {
            Value::Array(mapped_ref) => flattened.extend(mapped_ref.borrow().iter().cloned()),
            other => return Err(flat_map_type_error(&other)),
        }
    }

    Ok(Value::new_array(flattened))
}

/// Builds the "flatMap() callback must return an array" error, naming the
/// value's type the way Neon spells it elsewhere (`Int`, `String`, ...)
/// rather than `type_name()`'s lowercase runtime label.
fn flat_map_type_error(value: &Value) -> NativeCallError {
    let name = if matches!(value, Value::Int(_)) {
        "Int".to_string()
    } else {
        let lower = value.type_name();
        lower[..1].to_uppercase() + &lower[1..]
    };
    format!("flatMap() callback must return an array, got {}", name).into()
}

/// Native implementation of Array.filter(fn)
/// Returns a new array of the elements for which fn is truthy.
pub fn native_array_filter(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "filter() expects 1 argument (predicate), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "filter")?;
    let predicate = args[1].clone();
    let elements: Vec<Value> = array_ref.borrow().clone();

    let mut kept = Vec::new();
    for element in elements {
        let result = vm.call_value(predicate.clone(), std::slice::from_ref(&element))?;
        if !is_false_like!(result) {
            kept.push(element);
        }
    }

    Ok(Value::new_array(kept))
}

/// Native implementation of Array.reduce(fn, initial)
/// Folds the array from the left, calling fn(accumulator, element).
pub fn native_array_reduce(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 3 {
        return Err(format!(
            "reduce() expects 2 arguments (function, initial value), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "reduce")?;
    let callback = args[1].clone();
    let mut accumulator = args[2].clone();
    let elements: Vec<Value> = array_ref.borrow().clone();

    for element in elements {
        accumulator = vm.call_value(callback.clone(), &[accumulator, element])?;
    }

    Ok(accumulator)
}

/// Calls predicate on each element, stopping as soon as one's truthiness
/// matches `wanted` and returning it; `None` if none did. Shared by find,
/// some and every, which only differ in `wanted` and how they read the
/// result.
fn find_by_truthiness(
    vm: &mut dyn NativeContext,
    elements: Vec<Value>,
    predicate: &Value,
    wanted: bool,
) -> Result<Option<Value>, NativeCallError> {
    for element in elements {
        let result = vm.call_value(predicate.clone(), std::slice::from_ref(&element))?;
        let truthy = !is_false_like!(result);
        if truthy == wanted {
            return Ok(Some(element));
        }
    }
    Ok(None)
}

/// Native implementation of Array.find(fn)
/// Returns the first element for which fn is truthy, or nil.
pub fn native_array_find(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "find() expects 1 argument (predicate), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "find")?;
    let elements: Vec<Value> = array_ref.borrow().clone();

    Ok(find_by_truthiness(vm, elements, &args[1], true)?.unwrap_or(Value::Nil))
}

/// Native implementation of Array.some(fn)
/// Returns true if fn is truthy for any element, stopping at the first one.
/// False on an empty array.
pub fn native_array_some(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "some() expects 1 argument (predicate), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "some")?;
    let elements: Vec<Value> = array_ref.borrow().clone();

    let found = find_by_truthiness(vm, elements, &args[1], true)?;
    Ok(Value::Boolean(found.is_some()))
}

/// Native implementation of Array.every(fn)
/// Returns true if fn is truthy for every element, stopping at the first
/// one that isn't. True on an empty array.
pub fn native_array_every(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "every() expects 1 argument (predicate), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "every")?;
    let elements: Vec<Value> = array_ref.borrow().clone();

    let found_falsy = find_by_truthiness(vm, elements, &args[1], false)?;
    Ok(Value::Boolean(found_falsy.is_none()))
}

/// Native implementation of Array.flat()
/// Flattens one level: arrays inside are spliced in, other elements kept.
pub fn native_array_flat(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "flat() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "flat")?;
    let array = array_ref.borrow();

    let mut flattened = Vec::new();
    for element in array.iter() {
        match element {
            Value::Array(inner) => flattened.extend(inner.borrow().iter().cloned()),
            other => flattened.push(other.clone()),
        }
    }

    Ok(Value::new_array(flattened))
}

/// Native implementation of Array.copy()
/// Makes a shallow copy of the array.
pub fn native_array_copy(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "copy() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "copy")?;
    let elements: Vec<Value> = array_ref.borrow().clone();

    Ok(Value::new_array(elements))
}

const MAX_ARRAY_LEN: usize = 100_000_000;

fn is_callable(value: &Value) -> bool {
    matches!(value, Value::Closure(_) | Value::NativeFunction(_))
}

/// Native implementation of the Array(n, init) constructor.
/// Builds an array of n elements. If init is callable (a closure, function,
/// or native function), it's called with each index 0..n to produce that
/// element; otherwise init is stored (the same reference) in every element.
pub fn native_array_constructor(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!("Array() expects 2 arguments (n, init), got {}", args.len()).into());
    }

    let n = extract_integer_arg(args, 0, "n", "Array")?;
    if n < 0 {
        return Err(format!("Array() n must be non-negative, got {}", n).into());
    }
    if n > MAX_ARRAY_LEN as i64 {
        return Err(format!("Array() n exceeds {} elements", MAX_ARRAY_LEN).into());
    }
    let n = n as usize;

    let init = &args[1];
    let mut elements = Vec::new();
    elements
        .try_reserve_exact(n)
        .map_err(|_| format!("Array() n exceeds available memory: {}", n))?;

    if is_callable(init) {
        for i in 0..n {
            elements.push(vm.call_value(init.clone(), &[Value::Int(i as i64)])?);
        }
    } else {
        for _ in 0..n {
            elements.push(init.clone());
        }
    }

    Ok(Value::new_array(elements))
}
