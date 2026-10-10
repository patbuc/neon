use crate::common::stdlib::extraction_macros::extract_integer_arg;
use crate::common::NativeContext;
use crate::common::{compare_numeric, MapKey, NativeCallError, Numeric, Value};
use crate::{extract_arg, extract_receiver, extract_string_value, is_false_like};
use indexmap::IndexMap;

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

/// Native implementation of Array.removeAt(index)
/// Removes and returns the element at index; negative indices count from the end
pub fn native_array_remove_at(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "removeAt() expects 1 argument (index), got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "removeAt")?;
    let index = extract_integer_arg(args, 1, "index", "removeAt")?;

    let mut array = array_ref.borrow_mut();
    let len = array.len() as i64;
    let actual_index = if index < 0 { len + index } else { index };

    if actual_index < 0 || actual_index >= len {
        return Err(format!(
            "Array index out of bounds: index {} (normalized: {}) on array of length {}.",
            index, actual_index, len
        ));
    }

    Ok(array.remove(actual_index as usize))
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

/// Orders two `Numeric`s with NaN sorting after every other number, so the
/// result is a total order even when one side is an unordered float.
fn compare_numeric_nan_last(a: Numeric, b: Numeric) -> std::cmp::Ordering {
    compare_numeric(a, b).unwrap_or_else(|| a.as_f64().is_nan().cmp(&b.as_f64().is_nan()))
}

/// Default ascending order shared by `sort()`'s no-comparator path,
/// `sortBy`, `minBy`, and `maxBy`: numbers compare by value (NaN last),
/// strings compare lexically, anything else falls back to `sort_rank`.
fn default_order(a: &Value, b: &Value) -> std::cmp::Ordering {
    match (Numeric::from_value(a), Numeric::from_value(b)) {
        (Some(na), Some(nb)) => compare_numeric_nan_last(na, nb),
        _ => match (a, b) {
            (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
            _ => sort_rank(a).cmp(&sort_rank(b)),
        },
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
            array_ref.borrow_mut().sort_by(default_order);
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

/// Calls `callback` with each element to produce its sort/min/max key, then
/// checks the keys are all numbers or all strings.
fn compute_and_validate_keys(
    vm: &mut dyn NativeContext,
    elements: &[Value],
    callback: &Value,
    method: &str,
) -> Result<Vec<Value>, NativeCallError> {
    let mut keys = Vec::with_capacity(elements.len());
    for element in elements {
        keys.push(vm.call_value(callback.clone(), std::slice::from_ref(element))?);
    }

    let all_numbers = keys.iter().all(|k| Numeric::from_value(k).is_some());
    let all_strings = keys.iter().all(|k| matches!(k, Value::String(_)));
    if !all_numbers && !all_strings {
        return Err(format!("{}() keys must be all numbers or all strings", method).into());
    }

    Ok(keys)
}

/// Native implementation of Array.sortBy(fn)
/// Returns a new array sorted ascending by fn's key for each element
/// (stable); the receiver is unchanged. Keys must be all numbers or all
/// strings.
pub fn native_array_sort_by(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "sortBy() expects 1 argument (function), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "sortBy")?;
    let callback = args[1].clone();
    let elements: Vec<Value> = array_ref.borrow().clone();
    let keys = compute_and_validate_keys(vm, &elements, &callback, "sortBy")?;

    let mut indexed: Vec<(usize, Value)> = elements.into_iter().enumerate().collect();
    indexed.sort_by(|a, b| default_order(&keys[a.0], &keys[b.0]));
    let sorted = indexed.into_iter().map(|(_, value)| value).collect();

    Ok(Value::new_array(sorted))
}

/// Finds the element whose key (from fn) compares as `wanted` against every
/// other key; `None` on an empty array. Shared by minBy and maxBy.
fn extremum_by(
    vm: &mut dyn NativeContext,
    elements: Vec<Value>,
    callback: &Value,
    method: &str,
    wanted: std::cmp::Ordering,
) -> Result<Option<Value>, NativeCallError> {
    if elements.is_empty() {
        return Ok(None);
    }

    let keys = compute_and_validate_keys(vm, &elements, callback, method)?;
    let mut best = 0;
    for i in 1..elements.len() {
        if default_order(&keys[i], &keys[best]) == wanted {
            best = i;
        }
    }

    Ok(Some(elements[best].clone()))
}

/// Native implementation of Array.minBy(fn)
/// Returns the first element with the smallest key from fn, or nil on an
/// empty array. Keys must be all numbers or all strings.
pub fn native_array_min_by(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "minBy() expects 1 argument (function), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "minBy")?;
    let callback = args[1].clone();
    let elements: Vec<Value> = array_ref.borrow().clone();

    Ok(
        extremum_by(vm, elements, &callback, "minBy", std::cmp::Ordering::Less)?
            .unwrap_or(Value::Nil),
    )
}

/// Native implementation of Array.maxBy(fn)
/// Returns the first element with the largest key from fn, or nil on an
/// empty array. Keys must be all numbers or all strings.
pub fn native_array_max_by(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "maxBy() expects 1 argument (function), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "maxBy")?;
    let callback = args[1].clone();
    let elements: Vec<Value> = array_ref.borrow().clone();

    Ok(extremum_by(
        vm,
        elements,
        &callback,
        "maxBy",
        std::cmp::Ordering::Greater,
    )?
    .unwrap_or(Value::Nil))
}

/// Native implementation of Array.groupBy(fn)
/// Returns a map from each element's key (from fn) to an array of the
/// elements that produced it, in first-key insertion order. Keys must be
/// valid map keys.
pub fn native_array_group_by(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "groupBy() expects 1 argument (function), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "groupBy")?;
    let callback = args[1].clone();
    let elements: Vec<Value> = array_ref.borrow().clone();

    let mut groups: IndexMap<MapKey, Vec<Value>> = IndexMap::new();
    for element in elements {
        let key_value = vm.call_value(callback.clone(), std::slice::from_ref(&element))?;
        let key = MapKey::from_value(&key_value, "map key")?;
        groups.entry(key).or_default().push(element);
    }

    let groups = groups
        .into_iter()
        .map(|(key, group)| (key, Value::new_array(group)))
        .collect();
    Ok(Value::new_map(groups))
}

/// Native implementation of Array.tally()
/// Returns a map from each distinct element to how many times it occurs, in
/// first-occurrence order. Elements must be valid map keys.
pub fn native_array_tally(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "tally() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "tally")?;
    let elements: Vec<Value> = array_ref.borrow().clone();

    let mut counts: IndexMap<MapKey, i64> = IndexMap::new();
    for element in elements {
        let key = MapKey::from_value(&element, "map key")?;
        *counts.entry(key).or_insert(0) += 1;
    }

    let counts = counts
        .into_iter()
        .map(|(key, count)| (key, Value::Int(count)))
        .collect();
    Ok(Value::new_map(counts))
}

/// Native implementation of Array.distinct()
/// Returns a new array without repeated elements, keeping the first of each.
/// Elements must be valid map keys.
pub fn native_array_distinct(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "distinct() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "distinct")?;
    let elements: Vec<Value> = array_ref.borrow().clone();

    let mut seen: IndexMap<MapKey, Value> = IndexMap::new();
    for element in elements {
        let key = MapKey::from_value(&element, "map key")?;
        seen.entry(key).or_insert(element);
    }

    Ok(Value::new_array(seen.into_values().collect()))
}

/// Native implementation of Array.scan(initial, fn)
/// Like reduce, but returns every intermediate accumulator, starting with
/// initial.
pub fn native_array_scan(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 3 {
        return Err(format!(
            "scan() expects 2 arguments (initial value, function), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "scan")?;
    let callback = args[2].clone();
    let elements: Vec<Value> = array_ref.borrow().clone();

    let mut accumulator = args[1].clone();
    let mut results = Vec::with_capacity(elements.len() + 1);
    results.push(accumulator.clone());
    for element in elements {
        accumulator = vm.call_value(callback.clone(), &[accumulator, element])?;
        results.push(accumulator.clone());
    }

    Ok(Value::new_array(results))
}

/// Native implementation of Array.windowed(n)
/// Returns every run of n consecutive elements, sliding one at a time.
pub fn native_array_windowed(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "windowed() expects 1 argument (n), got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "windowed")?;
    let n = extract_integer_arg(args, 1, "n", "windowed")?;
    if n < 1 {
        return Err(format!("windowed() n must be >= 1, got {}", n));
    }

    let array = array_ref.borrow();
    let windows: Vec<Value> = array
        .windows(usize::try_from(n).unwrap_or(usize::MAX))
        .map(|window| Value::new_array(window.to_vec()))
        .collect();
    Ok(Value::new_array(windows))
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
            other => {
                return Err(format!(
                    "flatMap() callback must return an array, got {}",
                    other.type_name()
                )
                .into())
            }
        }
    }

    Ok(Value::new_array(flattened))
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

/// Index of the first element for which predicate is falsy, or the length
/// if there is none. Shared by takeWhile and dropWhile.
fn leading_truthy_count(
    vm: &mut dyn NativeContext,
    elements: &[Value],
    predicate: &Value,
) -> Result<usize, NativeCallError> {
    for (index, element) in elements.iter().enumerate() {
        let result = vm.call_value(predicate.clone(), std::slice::from_ref(element))?;
        if is_false_like!(result) {
            return Ok(index);
        }
    }
    Ok(elements.len())
}

/// Native implementation of Array.takeWhile(fn)
/// Returns the leading elements for which fn is truthy, stopping at the
/// first one that isn't.
pub fn native_array_take_while(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "takeWhile() expects 1 argument (predicate), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "takeWhile")?;
    let elements: Vec<Value> = array_ref.borrow().clone();
    let end = leading_truthy_count(vm, &elements, &args[1])?;

    Ok(Value::new_array(elements[..end].to_vec()))
}

/// Native implementation of Array.dropWhile(fn)
/// Returns the elements from the first one for which fn is falsy onwards.
pub fn native_array_drop_while(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "dropWhile() expects 1 argument (predicate), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "dropWhile")?;
    let elements: Vec<Value> = array_ref.borrow().clone();
    let start = leading_truthy_count(vm, &elements, &args[1])?;

    Ok(Value::new_array(elements[start..].to_vec()))
}

/// Native implementation of Array.partition(fn)
/// Returns [matching, nonMatching], splitting the elements by fn's truthiness.
pub fn native_array_partition(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 2 {
        return Err(format!(
            "partition() expects 1 argument (predicate), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "partition")?;
    let predicate = args[1].clone();
    let elements: Vec<Value> = array_ref.borrow().clone();

    let mut matching = Vec::new();
    let mut rest = Vec::new();
    for element in elements {
        let result = vm.call_value(predicate.clone(), std::slice::from_ref(&element))?;
        if is_false_like!(result) {
            rest.push(element);
        } else {
            matching.push(element);
        }
    }

    Ok(Value::new_array(vec![
        Value::new_array(matching),
        Value::new_array(rest),
    ]))
}

/// Native implementation of Array.reduce(initial, fn)
/// Folds the array from the left, calling fn(accumulator, element).
pub fn native_array_reduce(
    vm: &mut dyn NativeContext,
    args: &[Value],
) -> Result<Value, NativeCallError> {
    if args.len() != 3 {
        return Err(format!(
            "reduce() expects 2 arguments (initial value, function), got {}",
            args.len() - 1
        )
        .into());
    }

    let array_ref = extract_receiver!(args, Array, "reduce")?;
    let mut accumulator = args[1].clone();
    let callback = args[2].clone();
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

/// Native implementation of Array.take(n)
/// Returns a new array of the first n elements, clamped to the array's length.
pub fn native_array_take(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "take() expects 1 argument (n), got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "take")?;
    let n = extract_integer_arg(args, 1, "n", "take")?;
    if n < 0 {
        return Err(format!("take() n must be non-negative, got {}", n));
    }

    let array = array_ref.borrow();
    let end = usize::try_from(n).unwrap_or(usize::MAX).min(array.len());
    Ok(Value::new_array(array[..end].to_vec()))
}

/// Native implementation of Array.drop(n)
/// Returns a new array with the first n elements removed, clamped to the
/// array's length.
pub fn native_array_drop(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "drop() expects 1 argument (n), got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "drop")?;
    let n = extract_integer_arg(args, 1, "n", "drop")?;
    if n < 0 {
        return Err(format!("drop() n must be non-negative, got {}", n));
    }

    let array = array_ref.borrow();
    let start = usize::try_from(n).unwrap_or(usize::MAX).min(array.len());
    Ok(Value::new_array(array[start..].to_vec()))
}

/// Native implementation of Array.first()
/// Returns the first element, or nil if the array is empty.
pub fn native_array_first(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "first() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "first")?;
    let array = array_ref.borrow();
    Ok(array.first().cloned().unwrap_or(Value::Nil))
}

/// Native implementation of Array.last()
/// Returns the last element, or nil if the array is empty.
pub fn native_array_last(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "last() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "last")?;
    let array = array_ref.borrow();
    Ok(array.last().cloned().unwrap_or(Value::Nil))
}

/// Native implementation of Array.chunked(n)
/// Splits the array into arrays of n elements each; the last chunk may be
/// shorter.
pub fn native_array_chunked(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "chunked() expects 1 argument (n), got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "chunked")?;
    let n = extract_integer_arg(args, 1, "n", "chunked")?;
    if n < 1 {
        return Err(format!("chunked() n must be >= 1, got {}", n));
    }

    let array = array_ref.borrow();
    let chunks: Vec<Value> = array
        .chunks(usize::try_from(n).unwrap_or(usize::MAX))
        .map(|chunk| Value::new_array(chunk.to_vec()))
        .collect();
    Ok(Value::new_array(chunks))
}

/// The elements of an array or range value, for methods that accept either.
/// A range only yields up to `limit` elements, so a huge range isn't
/// materialized when the caller needs just a few of them.
fn elements_of(value: &Value, limit: usize) -> Option<Vec<Value>> {
    match value {
        Value::Array(arr) => Some(arr.borrow().clone()),
        Value::Range(range) => Some(range.elements_upto(limit)),
        _ => None,
    }
}

/// Native implementation of Array.zip(other)
/// Pairs each element with the element at the same position in other (an
/// array or range), stopping at the shorter length.
pub fn native_array_zip(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "zip() expects 1 argument (other), got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "zip")?;
    let array = array_ref.borrow();
    let other = elements_of(&args[1], array.len()).ok_or_else(|| {
        format!(
            "zip() other must be an array or range, got {}",
            args[1].type_name()
        )
    })?;

    let len = array.len().min(other.len());
    let pairs = array[..len]
        .iter()
        .zip(other[..len].iter())
        .map(|(a, b)| Value::new_array(vec![a.clone(), b.clone()]))
        .collect();
    Ok(Value::new_array(pairs))
}

/// Native implementation of Array.withIndex()
/// Returns `[[0, element0], [1, element1], ...]`.
pub fn native_array_with_index(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "withIndex() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let array_ref = extract_receiver!(args, Array, "withIndex")?;
    let array = array_ref.borrow();
    let pairs = array
        .iter()
        .enumerate()
        .map(|(i, element)| Value::new_array(vec![Value::Int(i as i64), element.clone()]))
        .collect();
    Ok(Value::new_array(pairs))
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
