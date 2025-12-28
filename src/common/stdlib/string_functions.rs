use crate::common::{Object, Value};
use crate::{extract_receiver, extract_arg, extract_string_value};
use crate::vm::VirtualMachine;

/// Native implementation of String.len()
/// Returns the number of Unicode characters in the string
pub fn native_string_len(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("string.len() requires a string receiver".to_string());
    }

    let obj_string = extract_receiver!(args, String, "len")?;
    let len = obj_string.value.chars().count();
    Ok(Value::Number(len as f64))
}

/// Native implementation of String.substring(start, end)
/// Returns a substring from start (inclusive) to end (exclusive)
/// Handles negative indices and bounds checking
pub fn native_string_substring(vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "substring() expects 2 arguments (start, end), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let obj_string = extract_receiver!(args, String, "substring")?;

    // Extract start and end indices
    let start_arg = extract_arg!(args, 1, Number, "start index", "substring")?;
    let end_arg = extract_arg!(args, 2, Number, "end index", "substring")?;

    // Collect characters for proper Unicode handling
    let chars: Vec<char> = obj_string.value.chars().collect();
    let str_len = chars.len() as i32;

    // Handle negative indices
    let start_idx = if start_arg < 0.0 {
        (str_len + start_arg as i32).max(0) as usize
    } else {
        (start_arg as i32).min(str_len) as usize
    };

    let end_idx = if end_arg < 0.0 {
        (str_len + end_arg as i32).max(0) as usize
    } else {
        (end_arg as i32).min(str_len) as usize
    };

    // Return empty string if start > end
    if start_idx > end_idx {
        return Ok(vm.intern_string(""));
    }

    // Extract substring
    let substring: String = chars[start_idx..end_idx].iter().collect();
    Ok(vm.intern_string(&substring))
}

/// Native implementation of String.replace(old, new)
/// Returns a new string with all occurrences of old replaced with new
pub fn native_string_replace(vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "replace() expects 2 arguments (old, new), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let obj_string = extract_receiver!(args, String, "replace")?;

    // Extract old substring
    let old_str = extract_string_value!(args, 1, "old", "replace");

    // Extract new substring
    let new_str = extract_string_value!(args, 2, "new", "replace");

    // Perform replacement
    let result = obj_string.value.replace(old_str, new_str);
    Ok(vm.intern_string(&result))
}

/// Native implementation of String.toInt()
/// Parses the string as an integer and returns it as a Number
/// Returns an error if the string cannot be parsed as an integer
pub fn native_string_to_int(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("toInt() requires a string receiver".to_string());
    }

    // Extract the string
    let obj_string = extract_receiver!(args, String, "toInt")?;

    // Trim whitespace and parse as i64
    let trimmed = obj_string.value.trim();
    match trimmed.parse::<i64>() {
        Ok(num) => Ok(Value::Number(num as f64)),
        Err(_) => Err(format!(
            "toInt() failed: '{}' is not a valid integer",
            obj_string.value
        )),
    }
}

/// Native implementation of String.toFloat()
/// Parses the string as a floating-point number and returns it as a Number
/// Returns an error if the string cannot be parsed as a float
pub fn native_string_to_float(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("toFloat() requires a string receiver".to_string());
    }

    // Extract the string
    let obj_string = extract_receiver!(args, String, "toFloat")?;

    // Trim whitespace and parse as f64
    let trimmed = obj_string.value.trim();
    match trimmed.parse::<f64>() {
        Ok(num) => Ok(Value::Number(num)),
        Err(_) => Err(format!(
            "toFloat() failed: '{}' is not a valid float",
            obj_string.value
        )),
    }
}

/// Native implementation of String.toBool()
/// Parses the string as a boolean and returns it as a Boolean
/// Accepts "true" or "false" (case-insensitive), returns an error for other input
pub fn native_string_to_bool(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("toBool() requires a string receiver".to_string());
    }

    // Extract the string
    let obj_string = extract_receiver!(args, String, "toBool")?;

    // Trim whitespace and convert to lowercase for case-insensitive comparison
    let normalized = obj_string.value.trim().to_lowercase();

    match normalized.as_str() {
        "true" => Ok(Value::Boolean(true)),
        "false" => Ok(Value::Boolean(false)),
        _ => Err(format!(
            "toBool() failed: '{}' is not a valid boolean (expected 'true' or 'false')",
            obj_string.value
        )),
    }
}

/// Native implementation of String.split(delimiter)
/// Returns an array of strings split by the delimiter
pub fn native_string_split(vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "split() expects 1 argument (delimiter), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let obj_string = extract_receiver!(args, String, "split")?;

    // Extract delimiter
    let delimiter = extract_string_value!(args, 1, "delimiter", "split");

    // Handle edge cases
    let parts: Vec<Value> = if delimiter.is_empty() {
        // Empty delimiter: split into individual characters
        obj_string
            .value
            .chars()
            .map(|c| vm.intern_string(&c.to_string()))
            .collect()
    } else if !obj_string.value.contains(delimiter) {
        // Delimiter not found: return array with original string
        vec![vm.intern_string(obj_string.value.as_ref())]
    } else {
        // Normal split
        obj_string
            .value
            .split(delimiter)
            .map(|s| vm.intern_string(s))
            .collect()
    };

    Ok(Value::new_array(parts))
}

/// Native implementation of String.trim()
/// Returns a new string with leading and trailing whitespace removed
pub fn native_string_trim(vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("trim() requires a string receiver".to_string());
    }

    // Extract the string
    let obj_string = extract_receiver!(args, String, "trim")?;

    let trimmed = obj_string.value.trim();
    Ok(vm.intern_string(trimmed))
}

/// Native implementation of String.startsWith(prefix)
/// Returns true if the string starts with the given prefix
pub fn native_string_starts_with(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "startsWith() expects 1 argument (prefix), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let obj_string = extract_receiver!(args, String, "startsWith")?;

    // Extract prefix
    let prefix = extract_string_value!(args, 1, "prefix", "startsWith");

    Ok(Value::Boolean(obj_string.value.starts_with(prefix)))
}

/// Native implementation of String.endsWith(suffix)
/// Returns true if the string ends with the given suffix
pub fn native_string_ends_with(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "endsWith() expects 1 argument (suffix), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let obj_string = extract_receiver!(args, String, "endsWith")?;

    // Extract suffix
    let suffix = extract_string_value!(args, 1, "suffix", "endsWith");

    Ok(Value::Boolean(obj_string.value.ends_with(suffix)))
}

/// Native implementation of String.indexOf(substring)
/// Returns the index of the first occurrence of substring, or -1 if not found
pub fn native_string_index_of(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "indexOf() expects 1 argument (substring), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let obj_string = extract_receiver!(args, String, "indexOf")?;

    // Extract substring
    let substring = extract_string_value!(args, 1, "substring", "indexOf");

    // Find the index (character-based, not byte-based)
    let chars: Vec<char> = obj_string.value.chars().collect();
    let substring_chars: Vec<char> = substring.chars().collect();

    if substring_chars.is_empty() {
        return Ok(Value::Number(0.0));
    }

    for (i, window) in chars.windows(substring_chars.len()).enumerate() {
        if window == substring_chars.as_slice() {
            return Ok(Value::Number(i as f64));
        }
    }

    Ok(Value::Number(-1.0))
}

/// Native implementation of String.charAt(index)
/// Returns the character at the given index as a string of length 1
pub fn native_string_char_at(vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "charAt() expects 1 argument (index), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let obj_string = extract_receiver!(args, String, "charAt")?;

    // Extract index
    let index_arg = extract_arg!(args, 1, Number, "index", "charAt")?;

    // Handle negative indices and bounds checking
    let chars: Vec<char> = obj_string.value.chars().collect();
    let str_len = chars.len() as i32;

    let index = if index_arg < 0.0 {
        (str_len + index_arg as i32).max(0) as usize
    } else {
        index_arg as usize
    };

    if index >= chars.len() {
        return Err(format!(
            "charAt() index {} out of bounds (string length: {})",
            index_arg,
            chars.len()
        ));
    }

    Ok(vm.intern_string(&chars[index].to_string()))
}

/// Native implementation of String.toUpperCase()
/// Returns a new string with all characters converted to uppercase
pub fn native_string_to_upper_case(vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("toUpperCase() requires a string receiver".to_string());
    }

    // Extract the string
    let obj_string = extract_receiver!(args, String, "toUpperCase")?;

    let uppercase = obj_string.value.to_uppercase();
    Ok(vm.intern_string(&uppercase))
}

/// Native implementation of String.toLowerCase()
/// Returns a new string with all characters converted to lowercase
pub fn native_string_to_lower_case(vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("toLowerCase() requires a string receiver".to_string());
    }

    // Extract the string
    let obj_string = extract_receiver!(args, String, "toLowerCase")?;

    let lowercase = obj_string.value.to_lowercase();
    Ok(vm.intern_string(&lowercase))
}
