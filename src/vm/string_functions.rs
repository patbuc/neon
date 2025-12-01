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

    // Ensure start <= end
    let (start_idx, end_idx) = if start_idx > end_idx {
        (end_idx, start_idx)
    } else {
        (start_idx, end_idx)
    };

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
        let mut vm = VirtualMachine::new();
        let test_str = string!("hello");
        let args = vec![test_str];

        let result = native_string_len(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 5.0);
    }

    #[test]
    fn test_string_len_unicode() {
        let mut vm = VirtualMachine::new();
        let test_str = string!("hello 🌍");
        let args = vec![test_str];

        let result = native_string_len(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 7.0); // 5 chars + 1 space + 1 emoji
    }

    #[test]
    fn test_string_len_empty() {
        let mut vm = VirtualMachine::new();
        let test_str = string!("");
        let args = vec![test_str];

        let result = native_string_len(&mut vm, &args).unwrap();
        assert_eq!(as_number!(result), 0.0);
    }

    #[test]
    fn test_substring_basic() {
        let mut vm = VirtualMachine::new();
        let test_str = string!("hello world");
        let args = vec![test_str, Value::Number(0.0), Value::Number(5.0)];

        let result = native_string_substring(&mut vm, &args).unwrap();
        assert_eq!("hello", as_string!(result));
    }

    #[test]
    fn test_substring_middle() {
        let mut vm = VirtualMachine::new();
        let test_str = string!("hello world");
        let args = vec![test_str, Value::Number(6.0), Value::Number(11.0)];

        let result = native_string_substring(&mut vm, &args).unwrap();
        assert_eq!("world", as_string!(result));
    }

    #[test]
    fn test_substring_negative_indices() {
        let mut vm = VirtualMachine::new();
        let test_str = string!("hello world");
        let args = vec![test_str, Value::Number(-5.0), Value::Number(-1.0)];

        let result = native_string_substring(&mut vm, &args).unwrap();
        assert_eq!("worl", as_string!(result));
    }

    #[test]
    fn test_substring_out_of_bounds() {
        let mut vm = VirtualMachine::new();
        let test_str = string!("hello");
        let args = vec![test_str, Value::Number(0.0), Value::Number(100.0)];

        let result = native_string_substring(&mut vm, &args).unwrap();
        assert_eq!("hello", as_string!(result));
    }

    #[test]
    fn test_substring_empty() {
        let mut vm = VirtualMachine::new();
        let test_str = string!("hello");
        let args = vec![test_str, Value::Number(2.0), Value::Number(2.0)];

        let result = native_string_substring(&mut vm, &args).unwrap();
        assert_eq!("", as_string!(result));
    }

    #[test]
    fn test_replace_basic() {
        let mut vm = VirtualMachine::new();
        let test_str = string!("hello world");
        let old = string!("world");
        let new = string!("rust");
        let args = vec![test_str, old, new];

        let result = native_string_replace(&mut vm, &args).unwrap();
        assert_eq!("hello rust", as_string!(result));
    }

    #[test]
    fn test_replace_multiple() {
        let mut vm = VirtualMachine::new();
        let test_str = string!("foo bar foo");
        let old = string!("foo");
        let new = string!("baz");
        let args = vec![test_str, old, new];

        let result = native_string_replace(&mut vm, &args).unwrap();
        assert_eq!("baz bar baz", as_string!(result));
    }

    #[test]
    fn test_replace_not_found() {
        let mut vm = VirtualMachine::new();
        let test_str = string!("hello world");
        let old = string!("xyz");
        let new = string!("abc");
        let args = vec![test_str, old, new];

        let result = native_string_replace(&mut vm, &args).unwrap();
        assert_eq!("hello world", as_string!(result));
    }

    #[test]
    fn test_replace_empty_string() {
        let mut vm = VirtualMachine::new();
        let test_str = string!("");
        let old = string!("foo");
        let new = string!("bar");
        let args = vec![test_str, old, new];

        let result = native_string_replace(&mut vm, &args).unwrap();
        assert_eq!("", as_string!(result));
    }

    #[test]
    fn test_replace_with_empty() {
        let mut vm = VirtualMachine::new();
        let test_str = string!("hello world");
        let old = string!(" ");
        let new = string!("");
        let args = vec![test_str, old, new];

        let result = native_string_replace(&mut vm, &args).unwrap();
        assert_eq!("helloworld", as_string!(result));
    }

    #[test]
    fn test_split_basic_comma() {
        let mut vm = VirtualMachine::new();
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
        let mut vm = VirtualMachine::new();
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
        let mut vm = VirtualMachine::new();
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
        let mut vm = VirtualMachine::new();
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
        let mut vm = VirtualMachine::new();
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
        let mut vm = VirtualMachine::new();
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
}
