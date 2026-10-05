use crate::common::stdlib::extraction_macros::extract_integer_arg;
use crate::common::Value;
use crate::string;
use crate::{extract_arg, extract_receiver, extract_string_value};

/// Native implementation of String.size()
/// Returns the number of Unicode characters in the string
pub fn native_string_size(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "size() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let string = extract_receiver!(args, String, "size")?;
    let len = string.chars().count();
    Ok(Value::Int(len as i64))
}

/// Native implementation of String.isEmpty()
/// Returns true if the string has no characters
pub fn native_string_is_empty(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "isEmpty() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let string = extract_receiver!(args, String, "isEmpty")?;
    Ok(Value::Boolean(string.is_empty()))
}

/// Native implementation of String.substring(start, end)
/// Returns a substring from start (inclusive) to end (exclusive)
/// Handles negative indices and bounds checking
pub fn native_string_substring(args: &[Value]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "substring() expects 2 arguments (start, end), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let string = extract_receiver!(args, String, "substring")?;

    // Extract start and end indices
    let start_arg = extract_arg!(args, 1, Number, "start index", "substring")?;
    let end_arg = extract_arg!(args, 2, Number, "end index", "substring")?;

    // Collect characters for proper Unicode handling
    let chars: Vec<char> = string.chars().collect();
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
        return Ok(string!(String::new()));
    }

    // Extract substring
    let substring: String = chars[start_idx..end_idx].iter().collect();
    Ok(string!(substring))
}

/// Native implementation of String.replace(old, new)
/// Returns a new string with all occurrences of old replaced with new
pub fn native_string_replace(args: &[Value]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "replace() expects 2 arguments (old, new), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let string = extract_receiver!(args, String, "replace")?;

    // Extract old substring
    let old_str = extract_string_value!(args, 1, "old", "replace");

    // Extract new substring
    let new_str = extract_string_value!(args, 2, "new", "replace");

    // Perform replacement
    let result = string.replace(old_str, new_str);
    Ok(string!(result))
}

/// Native implementation of String.toInt()
/// Parses the string as an integer and returns it as a Number
/// Returns an error if the string cannot be parsed as an integer
pub fn native_string_to_int(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "toInt() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let string = extract_receiver!(args, String, "toInt")?;

    // Trim whitespace and parse as i64
    let trimmed = string.trim();
    match trimmed.parse::<i64>() {
        Ok(num) => Ok(Value::Int(num)),
        Err(_) => Err(format!(
            "toInt() failed: '{}' is not a valid integer",
            string
        )),
    }
}

/// Native implementation of String.toFloat()
/// Parses the string as a floating-point number and returns it as a Number
/// Returns an error if the string cannot be parsed as a float
pub fn native_string_to_float(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "toFloat()() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let string = extract_receiver!(args, String, "toFloat")?;

    // Trim whitespace and parse as f64
    let trimmed = string.trim();
    match trimmed.parse::<f64>() {
        Ok(num) => Ok(Value::Number(num)),
        Err(_) => Err(format!(
            "toFloat() failed: '{}' is not a valid float",
            string
        )),
    }
}

/// Native implementation of String.toBool()
/// Parses the string as a boolean and returns it as a Boolean
/// Accepts "true" or "false" (case-insensitive), returns an error for other input
pub fn native_string_to_bool(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "toBool()() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let string = extract_receiver!(args, String, "toBool")?;

    // Trim whitespace and convert to lowercase for case-insensitive comparison
    let normalized = string.trim().to_lowercase();

    match normalized.as_str() {
        "true" => Ok(Value::Boolean(true)),
        "false" => Ok(Value::Boolean(false)),
        _ => Err(format!(
            "toBool() failed: '{}' is not a valid boolean (expected 'true' or 'false')",
            string
        )),
    }
}

/// Native implementation of String.split() and String.split(delimiter)
/// With no argument, splits on runs of Unicode whitespace and drops leading
/// and trailing empties. With a delimiter, splits on exact occurrences of it.
pub fn native_string_split(args: &[Value]) -> Result<Value, String> {
    if args.is_empty() || args.len() > 2 {
        return Err(format!(
            "split() expects 0 or 1 arguments (delimiter), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let string = extract_receiver!(args, String, "split")?;

    if args.len() == 1 {
        let parts: Vec<Value> = string.split_whitespace().map(|s| string!(s)).collect();
        return Ok(Value::new_array(parts));
    }

    // Extract delimiter
    let delimiter = extract_string_value!(args, 1, "delimiter", "split");

    // Handle edge cases
    let parts: Vec<Value> = if delimiter.is_empty() {
        // Empty delimiter: split into individual characters
        string.chars().map(|c| string!(c.to_string())).collect()
    } else if !string.contains(delimiter) {
        // Delimiter not found: return array with original string
        vec![string!(string.as_str())]
    } else {
        // Normal split
        string.split(delimiter).map(|s| string!(s)).collect()
    };

    Ok(Value::new_array(parts))
}

/// Native implementation of String.trim()
/// Returns a new string with leading and trailing whitespace removed
pub fn native_string_trim(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "trim()() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let string = extract_receiver!(args, String, "trim")?;

    let trimmed = string.trim();
    Ok(string!(trimmed))
}

/// Native implementation of String.startsWith(prefix)
/// Returns true if the string starts with the given prefix
pub fn native_string_starts_with(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "startsWith() expects 1 argument (prefix), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let string = extract_receiver!(args, String, "startsWith")?;

    // Extract prefix
    let prefix = extract_string_value!(args, 1, "prefix", "startsWith");

    Ok(Value::Boolean(string.starts_with(prefix)))
}

/// Native implementation of String.endsWith(suffix)
/// Returns true if the string ends with the given suffix
pub fn native_string_ends_with(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "endsWith() expects 1 argument (suffix), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let string = extract_receiver!(args, String, "endsWith")?;

    // Extract suffix
    let suffix = extract_string_value!(args, 1, "suffix", "endsWith");

    Ok(Value::Boolean(string.ends_with(suffix)))
}

/// Native implementation of String.indexOf(substring)
/// Returns the index of the first occurrence of substring, or -1 if not found
pub fn native_string_index_of(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "indexOf() expects 1 argument (substring), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let string = extract_receiver!(args, String, "indexOf")?;

    // Extract substring
    let substring = extract_string_value!(args, 1, "substring", "indexOf");

    // Find the index (character-based, not byte-based)
    let chars: Vec<char> = string.chars().collect();
    let substring_chars: Vec<char> = substring.chars().collect();

    if substring_chars.is_empty() {
        return Ok(Value::Int(0));
    }

    for (i, window) in chars.windows(substring_chars.len()).enumerate() {
        if window == substring_chars.as_slice() {
            return Ok(Value::Int(i as i64));
        }
    }

    Ok(Value::Int(-1))
}

/// Native implementation of String.charCodeAt(index)
/// Returns the Unicode code point of the character at index, indexed by
/// `char` like string indexing (`s[i]`).
pub fn native_string_char_code_at(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "charCodeAt() expects 1 argument (index), got {}",
            args.len() - 1
        ));
    }

    let string = extract_receiver!(args, String, "charCodeAt")?;
    let chars: Vec<char> = string.chars().collect();

    let index_arg = extract_integer_arg(args, 1, "index", "charCodeAt")?;

    let str_len = chars.len() as i64;
    let index = if index_arg < 0 {
        index_arg + str_len
    } else {
        index_arg
    };

    if index < 0 || index >= str_len {
        return Err(format!(
            "charCodeAt() index {} out of bounds (string length: {})",
            index_arg,
            chars.len()
        ));
    }

    Ok(Value::Int(chars[index as usize] as i64))
}

/// Native implementation of String.fromCharCode(n)
/// Returns a one-character string for the given Unicode code point.
pub fn native_string_from_char_code(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "fromCharCode() expects 1 argument, got {}",
            args.len()
        ));
    }

    let code = extract_integer_arg(args, 0, "argument", "fromCharCode")?;

    let code_point = u32::try_from(code)
        .ok()
        .and_then(char::from_u32)
        .ok_or_else(|| format!("fromCharCode() invalid code point: {}", code))?;

    Ok(string!(code_point.to_string()))
}

/// Native implementation of String.toUpperCase()
/// Returns a new string with all characters converted to uppercase
pub fn native_string_to_upper_case(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "toUpperCase()() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let string = extract_receiver!(args, String, "toUpperCase")?;

    let uppercase = string.to_uppercase();
    Ok(string!(uppercase))
}

/// Native implementation of String.toLowerCase()
/// Returns a new string with all characters converted to lowercase
pub fn native_string_to_lower_case(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "toLowerCase()() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let string = extract_receiver!(args, String, "toLowerCase")?;

    let lowercase = string.to_lowercase();
    Ok(string!(lowercase))
}

const MAX_RESULT_CHARS: usize = 100_000_000;

/// Native implementation of String.repeat(n)
/// Returns the string concatenated with itself n times. n must be a
/// non-negative integer.
pub fn native_string_repeat(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "repeat() expects 1 argument (count), got {}",
            args.len() - 1
        ));
    }

    let string = extract_receiver!(args, String, "repeat")?;
    let count = extract_integer_arg(args, 1, "count", "repeat")?;

    if count < 0 {
        return Err(format!(
            "repeat() count must be non-negative, got {}",
            count
        ));
    }

    let len = string.chars().count();
    let total = len as u128 * count as u128;
    if total > MAX_RESULT_CHARS as u128 {
        return Err(format!(
            "repeat() result exceeds {} chars",
            MAX_RESULT_CHARS
        ));
    }

    Ok(string!(string.repeat(count as usize)))
}

/// Build a fill string of `pad_count` chars by cycling `fill`'s chars,
/// cutting off the cycle when the target length is reached.
fn cycled_fill(fill: &str, pad_count: usize) -> String {
    let fill_chars: Vec<char> = fill.chars().collect();
    (0..pad_count)
        .map(|i| fill_chars[i % fill_chars.len()])
        .collect()
}

/// Shared implementation of String.padStart(len, fill) / padEnd(len, fill).
/// Pads with `fill` (cycled) until the string is `len` chars long, at the
/// start or end depending on `at_start`. Returns the string unchanged if
/// it is already that long, or if `len` is negative.
fn pad(args: &[Value], method: &str, at_start: bool) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "{}() expects 2 arguments (length, fill), got {}",
            method,
            args.len() - 1
        ));
    }

    let string = extract_receiver!(args, String, method)?;
    let target_len = extract_integer_arg(args, 1, "length", method)?;
    let fill = extract_string_value!(args, 2, "fill", method);

    if fill.is_empty() {
        return Err(format!("{}() fill must not be empty", method));
    }
    if target_len < 0 {
        return Ok(args[0].clone());
    }
    if target_len as u128 > MAX_RESULT_CHARS as u128 {
        return Err(format!(
            "{}() result exceeds {} chars",
            method, MAX_RESULT_CHARS
        ));
    }
    let target_len = target_len as usize;

    let chars: Vec<char> = string.chars().collect();
    if chars.len() >= target_len {
        return Ok(args[0].clone());
    }

    let fill_str = cycled_fill(fill, target_len - chars.len());
    Ok(if at_start {
        string!(format!("{}{}", fill_str, string))
    } else {
        string!(format!("{}{}", string, fill_str))
    })
}

/// Native implementation of String.padStart(len, fill)
pub fn native_string_pad_start(args: &[Value]) -> Result<Value, String> {
    pad(args, "padStart", true)
}

/// Native implementation of String.padEnd(len, fill)
pub fn native_string_pad_end(args: &[Value]) -> Result<Value, String> {
    pad(args, "padEnd", false)
}

/// Native implementation of String.lastIndexOf(substring)
/// Returns the char index of the last occurrence of substring, or -1 if
/// not found. An empty substring matches at the string's length.
pub fn native_string_last_index_of(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "lastIndexOf() expects 1 argument (substring), got {}",
            args.len() - 1
        ));
    }

    let string = extract_receiver!(args, String, "lastIndexOf")?;
    let substring = extract_string_value!(args, 1, "substring", "lastIndexOf");

    let chars: Vec<char> = string.chars().collect();
    let sub_chars: Vec<char> = substring.chars().collect();

    if sub_chars.is_empty() {
        return Ok(Value::Int(chars.len() as i64));
    }
    if sub_chars.len() > chars.len() {
        return Ok(Value::Int(-1));
    }

    for start in (0..=chars.len() - sub_chars.len()).rev() {
        if chars[start..start + sub_chars.len()] == sub_chars[..] {
            return Ok(Value::Int(start as i64));
        }
    }

    Ok(Value::Int(-1))
}

/// Native implementation of String.contains(substring)
/// Returns true if substring occurs anywhere in the string.
pub fn native_string_contains(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "contains() expects 1 argument (substring), got {}",
            args.len() - 1
        ));
    }

    let string = extract_receiver!(args, String, "contains")?;
    let substring = extract_string_value!(args, 1, "substring", "contains");

    Ok(Value::Boolean(string.contains(substring)))
}

/// Native implementation of String.chars()
/// Returns an array of the string's Unicode-scalar characters, each a one-character string
pub fn native_string_chars(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "chars() expects no arguments, got {}",
            args.len() - 1
        ));
    }

    let string = extract_receiver!(args, String, "chars")?;
    Ok(string_chars_array(string))
}

/// Converts a string into an array of one-character strings, one per Unicode scalar value.
/// Shared by String.chars() and the for-in iterator over strings.
pub fn string_chars_array(string: &str) -> Value {
    let chars: Vec<Value> = string.chars().map(|c| string!(c.to_string())).collect();
    Value::new_array(chars)
}
