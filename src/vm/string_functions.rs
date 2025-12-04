use crate::common::{Value, Object};
use crate::vm::VirtualMachine;
use crate::string;

/// Native implementation of String.len()
/// Returns the number of Unicode characters in the string
pub fn native_string_len(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("string.len() requires a string receiver".to_string());
    }

    match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(obj_string) => {
                let len = obj_string.value.chars().count();
                Ok(Value::Number(len as f64))
            }
            _ => Err("len() can only be called on strings".to_string()),
        },
        _ => Err("len() can only be called on strings".to_string()),
    }
}

/// Native implementation of String.substring(start, end)
/// Returns a substring from start (inclusive) to end (exclusive)
/// Handles negative indices and bounds checking
pub fn native_string_substring(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "substring() expects 2 arguments (start, end), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let obj_string = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => s,
            _ => return Err("substring() can only be called on strings".to_string()),
        },
        _ => return Err("substring() can only be called on strings".to_string()),
    };

    // Extract start and end indices
    let start_arg = match &args[1] {
        Value::Number(n) => *n,
        _ => return Err("substring() start index must be a number".to_string()),
    };

    let end_arg = match &args[2] {
        Value::Number(n) => *n,
        _ => return Err("substring() end index must be a number".to_string()),
    };

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
        return Ok(string!(String::new()));
    }

    // Extract substring
    let substring: String = chars[start_idx..end_idx].iter().collect();
    Ok(string!(substring))
}

/// Native implementation of String.replace(old, new)
/// Returns a new string with all occurrences of old replaced with new
pub fn native_string_replace(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "replace() expects 2 arguments (old, new), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let obj_string = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => s,
            _ => return Err("replace() can only be called on strings".to_string()),
        },
        _ => return Err("replace() can only be called on strings".to_string()),
    };

    // Extract old substring
    let old_str = match &args[1] {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => s.value.as_ref(),
            _ => return Err("replace() old argument must be a string".to_string()),
        },
        _ => return Err("replace() old argument must be a string".to_string()),
    };

    // Extract new substring
    let new_str = match &args[2] {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => s.value.as_ref(),
            _ => return Err("replace() new argument must be a string".to_string()),
        },
        _ => return Err("replace() new argument must be a string".to_string()),
    };

    // Perform replacement
    let result = obj_string.value.replace(old_str, new_str);
    Ok(string!(result))
}

/// Native implementation of String.toInt()
/// Parses the string as an integer and returns it as a Number
/// Returns an error if the string cannot be parsed as an integer
pub fn native_string_to_int(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("toInt() requires a string receiver".to_string());
    }

    // Extract the string
    let obj_string = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => s,
            _ => return Err("toInt() can only be called on strings".to_string()),
        },
        _ => return Err("toInt() can only be called on strings".to_string()),
    };

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
    let obj_string = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => s,
            _ => return Err("toFloat() can only be called on strings".to_string()),
        },
        _ => return Err("toFloat() can only be called on strings".to_string()),
    };

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
    let obj_string = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => s,
            _ => return Err("toBool() can only be called on strings".to_string()),
        },
        _ => return Err("toBool() can only be called on strings".to_string()),
    };

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
pub fn native_string_split(_vm: &mut VirtualMachine, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "split() expects 1 argument (delimiter), got {}",
            args.len() - 1
        ));
    }

    // Extract the string
    let obj_string = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => s,
            _ => return Err("split() can only be called on strings".to_string()),
        },
        _ => return Err("split() can only be called on strings".to_string()),
    };

    // Extract delimiter
    let delimiter = match &args[1] {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => s.value.as_ref(),
            _ => return Err("split() delimiter must be a string".to_string()),
        },
        _ => return Err("split() delimiter must be a string".to_string()),
    };

    // Handle edge cases
    let parts: Vec<Value> = if delimiter.is_empty() {
        // Empty delimiter: split into individual characters
        obj_string.value.chars()
            .map(|c| string!(c.to_string()))
            .collect()
    } else if !obj_string.value.contains(delimiter) {
        // Delimiter not found: return array with original string
        vec![string!(obj_string.value.as_ref())]
    } else {
        // Normal split
        obj_string.value.split(delimiter)
            .map(|s| string!(s))
            .collect()
    };

    Ok(Value::new_array(parts))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::as_number;
    use crate::as_string;

    #[test]
    fn test_string_len_basic() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("hello");
        let args = vec![test_str];

        let result = native_string_len(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 5.0);
    }

    #[test]
    fn test_string_len_unicode() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("hello 🌍");
        let args = vec![test_str];

        let result = native_string_len(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 7.0); // 5 chars + 1 space + 1 emoji
    }

    #[test]
    fn test_string_len_empty() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("");
        let args = vec![test_str];

        let result = native_string_len(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 0.0);
    }

    #[test]
    fn test_substring_basic() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("hello world");
        let args = vec![test_str, Value::Number(0.0), Value::Number(5.0)];

        let result = native_string_substring(&mut vm, &args).unwrap();
        assert_eq!("hello", as_string!(result));
    }

    #[test]
    fn test_substring_middle() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("hello world");
        let args = vec![test_str, Value::Number(6.0), Value::Number(11.0)];

        let result = native_string_substring(&mut vm, &args).unwrap();
        assert_eq!("world", as_string!(result));
    }

    #[test]
    fn test_substring_negative_indices() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("hello world");
        let args = vec![test_str, Value::Number(-5.0), Value::Number(-1.0)];

        let result = native_string_substring(&mut vm, &args).unwrap();
        assert_eq!("worl", as_string!(result));
    }

    #[test]
    fn test_substring_out_of_bounds() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("hello");
        let args = vec![test_str, Value::Number(0.0), Value::Number(100.0)];

        let result = native_string_substring(&mut vm, &args).unwrap();
        assert_eq!("hello", as_string!(result));
    }

    #[test]
    fn test_substring_empty() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("hello");
        let args = vec![test_str, Value::Number(2.0), Value::Number(2.0)];

        let result = native_string_substring(&mut vm, &args).unwrap();
        assert_eq!("", as_string!(result));
    }

    #[test]
    fn test_substring_start_greater_than_end() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("hello");
        // start=5, end=0: should return empty string
        let args = vec![test_str, Value::Number(5.0), Value::Number(0.0)];

        let result = native_string_substring(&mut vm, &args).unwrap();
        assert_eq!("", as_string!(result));
    }

    #[test]
    fn test_replace_basic() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("hello world");
        let old = string!("world");
        let new = string!("rust");
        let args = vec![test_str, old, new];

        let result = native_string_replace(&mut vm, &args).unwrap();
        assert_eq!("hello rust", as_string!(result));
    }

    #[test]
    fn test_replace_multiple() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("foo bar foo");
        let old = string!("foo");
        let new = string!("baz");
        let args = vec![test_str, old, new];

        let result = native_string_replace(&mut vm, &args).unwrap();
        assert_eq!("baz bar baz", as_string!(result));
    }

    #[test]
    fn test_replace_not_found() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("hello world");
        let old = string!("xyz");
        let new = string!("abc");
        let args = vec![test_str, old, new];

        let result = native_string_replace(&mut vm, &args).unwrap();
        assert_eq!("hello world", as_string!(result));
    }

    #[test]
    fn test_replace_empty_string() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("");
        let old = string!("foo");
        let new = string!("bar");
        let args = vec![test_str, old, new];

        let result = native_string_replace(&mut vm, &args).unwrap();
        assert_eq!("", as_string!(result));
    }

    #[test]
    fn test_replace_with_empty() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("hello world");
        let old = string!(" ");
        let new = string!("");
        let args = vec![test_str, old, new];

        let result = native_string_replace(&mut vm, &args).unwrap();
        assert_eq!("helloworld", as_string!(result));
    }

    #[test]
    fn test_split_basic_comma() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("a,b,c");
        let delimiter = string!(",");
        let args = vec![test_str, delimiter];

        let result = native_string_split(&mut vm, &args).unwrap();

        // Verify it's an array
        match result {
            Value::Object(obj) => {
                match obj.as_ref() {
                    Object::Array(arr) => {
                        let elements = arr.borrow();
                        assert_eq!(elements.len(), 3);
                        assert_eq!("a", as_string!(elements[0]));
                        assert_eq!("b", as_string!(elements[1]));
                        assert_eq!("c", as_string!(elements[2]));
                    }
                    _ => panic!("Expected Array object"),
                }
            }
            _ => panic!("Expected Object value"),
        }
    }

    #[test]
    fn test_split_delimiter_not_found() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("hello world");
        let delimiter = string!(",");
        let args = vec![test_str, delimiter];

        let result = native_string_split(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => {
                match obj.as_ref() {
                    Object::Array(arr) => {
                        let elements = arr.borrow();
                        assert_eq!(elements.len(), 1);
                        assert_eq!("hello world", as_string!(elements[0]));
                    }
                    _ => panic!("Expected Array object"),
                }
            }
            _ => panic!("Expected Object value"),
        }
    }

    #[test]
    fn test_split_empty_string() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("");
        let delimiter = string!(",");
        let args = vec![test_str, delimiter];

        let result = native_string_split(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => {
                match obj.as_ref() {
                    Object::Array(arr) => {
                        let elements = arr.borrow();
                        assert_eq!(elements.len(), 1);
                        assert_eq!("", as_string!(elements[0]));
                    }
                    _ => panic!("Expected Array object"),
                }
            }
            _ => panic!("Expected Object value"),
        }
    }

    #[test]
    fn test_split_empty_delimiter() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("hello");
        let delimiter = string!("");
        let args = vec![test_str, delimiter];

        let result = native_string_split(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => {
                match obj.as_ref() {
                    Object::Array(arr) => {
                        let elements = arr.borrow();
                        assert_eq!(elements.len(), 5);
                        assert_eq!("h", as_string!(elements[0]));
                        assert_eq!("e", as_string!(elements[1]));
                        assert_eq!("l", as_string!(elements[2]));
                        assert_eq!("l", as_string!(elements[3]));
                        assert_eq!("o", as_string!(elements[4]));
                    }
                    _ => panic!("Expected Array object"),
                }
            }
            _ => panic!("Expected Object value"),
        }
    }

    #[test]
    fn test_split_multiple_consecutive_delimiters() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("a,,b,,c");
        let delimiter = string!(",");
        let args = vec![test_str, delimiter];

        let result = native_string_split(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => {
                match obj.as_ref() {
                    Object::Array(arr) => {
                        let elements = arr.borrow();
                        // Split should produce: ["a", "", "b", "", "c"]
                        assert_eq!(elements.len(), 5);
                        assert_eq!("a", as_string!(elements[0]));
                        assert_eq!("", as_string!(elements[1]));
                        assert_eq!("b", as_string!(elements[2]));
                        assert_eq!("", as_string!(elements[3]));
                        assert_eq!("c", as_string!(elements[4]));
                    }
                    _ => panic!("Expected Array object"),
                }
            }
            _ => panic!("Expected Object value"),
        }
    }

    #[test]
    fn test_split_space_delimiter() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("hello world");
        let delimiter = string!(" ");
        let args = vec![test_str, delimiter];

        let result = native_string_split(&mut vm, &args).unwrap();

        match result {
            Value::Object(obj) => {
                match obj.as_ref() {
                    Object::Array(arr) => {
                        let elements = arr.borrow();
                        assert_eq!(elements.len(), 2);
                        assert_eq!("hello", as_string!(elements[0]));
                        assert_eq!("world", as_string!(elements[1]));
                    }
                    _ => panic!("Expected Array object"),
                }
            }
            _ => panic!("Expected Object value"),
        }
    }

    // Tests for String.toInt()
    #[test]
    fn test_to_int_basic_positive() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("123");
        let args = vec![test_str];

        let result = native_string_to_int(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 123.0);
    }

    #[test]
    fn test_to_int_basic_negative() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("-456");
        let args = vec![test_str];

        let result = native_string_to_int(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), -456.0);
    }

    #[test]
    fn test_to_int_zero() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("0");
        let args = vec![test_str];

        let result = native_string_to_int(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 0.0);
    }

    #[test]
    fn test_to_int_with_whitespace() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("  789  ");
        let args = vec![test_str];

        let result = native_string_to_int(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 789.0);
    }

    #[test]
    fn test_to_int_large_number() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("9876543210");
        let args = vec![test_str];

        let result = native_string_to_int(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 9876543210.0);
    }

    #[test]
    fn test_to_int_invalid_float() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("123.45");
        let args = vec![test_str];

        let result = native_string_to_int(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not a valid integer"));
    }

    #[test]
    fn test_to_int_invalid_string() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("abc");
        let args = vec![test_str];

        let result = native_string_to_int(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not a valid integer"));
    }

    #[test]
    fn test_to_int_empty_string() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("");
        let args = vec![test_str];

        let result = native_string_to_int(&mut vm, &args);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_int_mixed_content() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("123abc");
        let args = vec![test_str];

        let result = native_string_to_int(&mut vm, &args);
        assert!(result.is_err());
    }

    // Tests for String.toFloat()
    #[test]
    fn test_to_float_basic_integer() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("123");
        let args = vec![test_str];

        let result = native_string_to_float(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 123.0);
    }

    #[test]
    fn test_to_float_basic_decimal() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("45.67");
        let args = vec![test_str];

        let result = native_string_to_float(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 45.67);
    }

    #[test]
    fn test_to_float_negative() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("-12.34");
        let args = vec![test_str];

        let result = native_string_to_float(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), -12.34);
    }

    #[test]
    fn test_to_float_zero() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("0.0");
        let args = vec![test_str];

        let result = native_string_to_float(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 0.0);
    }

    #[test]
    fn test_to_float_with_whitespace() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("  3.14  ");
        let args = vec![test_str];

        let result = native_string_to_float(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 3.14);
    }

    #[test]
    fn test_to_float_scientific_notation() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("1.23e4");
        let args = vec![test_str];

        let result = native_string_to_float(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 12300.0);
    }

    #[test]
    fn test_to_float_scientific_notation_negative_exponent() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("1.5e-2");
        let args = vec![test_str];

        let result = native_string_to_float(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 0.015);
    }

    #[test]
    fn test_to_float_no_decimal() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("42");
        let args = vec![test_str];

        let result = native_string_to_float(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 42.0);
    }

    #[test]
    fn test_to_float_leading_decimal() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!(".5");
        let args = vec![test_str];

        let result = native_string_to_float(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 0.5);
    }

    #[test]
    fn test_to_float_invalid_string() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("abc");
        let args = vec![test_str];

        let result = native_string_to_float(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not a valid float"));
    }

    #[test]
    fn test_to_float_empty_string() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("");
        let args = vec![test_str];

        let result = native_string_to_float(&mut vm, &args);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_float_mixed_content() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("12.34abc");
        let args = vec![test_str];

        let result = native_string_to_float(&mut vm, &args);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_float_infinity() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("inf");
        let args = vec![test_str];

        let result = native_string_to_float(&mut vm, &args).unwrap();
        assert!(as_number!(result).is_infinite() && as_number!(result) > 0.0);
    }

    #[test]
    fn test_to_float_negative_infinity() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("-inf");
        let args = vec![test_str];

        let result = native_string_to_float(&mut vm, &args).unwrap();
        assert!(as_number!(result).is_infinite() && as_number!(result) < 0.0);
    }

    // Tests for String.toBool()
    #[test]
    fn test_to_bool_lowercase_true() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("true");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_to_bool_lowercase_false() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("false");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_to_bool_uppercase_true() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("TRUE");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_to_bool_uppercase_false() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("FALSE");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_to_bool_mixed_case_true() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("TrUe");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_to_bool_mixed_case_false() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("FaLsE");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_to_bool_with_whitespace_true() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("  true  ");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_to_bool_with_whitespace_false() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("  false  ");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_to_bool_with_tabs_and_newlines() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("\t\ntrue\n\t");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_to_bool_invalid_string() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("yes");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not a valid boolean"));
    }

    #[test]
    fn test_to_bool_invalid_number() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("1");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not a valid boolean"));
    }

    #[test]
    fn test_to_bool_invalid_zero() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("0");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not a valid boolean"));
    }

    #[test]
    fn test_to_bool_empty_string() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not a valid boolean"));
    }

    #[test]
    fn test_to_bool_partial_match() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("truee");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_bool_mixed_content() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("true123");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_bool_with_surrounding_text() {
        let mut vm = VirtualMachine::new(Vec::new());
        let test_str = string!("the answer is true");
        let args = vec![test_str];

        let result = native_string_to_bool(&mut vm, &args);
        assert!(result.is_err());
    }
}
