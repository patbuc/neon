use crate::common::stdlib::string_functions::*;
use crate::common::Value;
use crate::{as_number, as_string, string};

#[test]
fn test_string_len_basic() {
    let test_str = string!("hello");
    let args = vec![test_str];

    let result = native_string_len(&args).unwrap();
    assert_eq!(as_number!(result), 5.0);
}

#[test]
fn test_string_len_unicode() {
    let test_str = string!("hello 🌍");
    let args = vec![test_str];

    let result = native_string_len(&args).unwrap();
    assert_eq!(as_number!(result), 7.0); // 5 chars + 1 space + 1 emoji
}

#[test]
fn test_string_len_empty() {
    let test_str = string!("");
    let args = vec![test_str];

    let result = native_string_len(&args).unwrap();
    assert_eq!(as_number!(result), 0.0);
}

#[test]
fn test_substring_basic() {
    let test_str = string!("hello world");
    let args = vec![test_str, Value::Number(0.0), Value::Number(5.0)];

    let result = native_string_substring(&args).unwrap();
    assert_eq!("hello", as_string!(result));
}

#[test]
fn test_substring_middle() {
    let test_str = string!("hello world");
    let args = vec![test_str, Value::Number(6.0), Value::Number(11.0)];

    let result = native_string_substring(&args).unwrap();
    assert_eq!("world", as_string!(result));
}

#[test]
fn test_substring_negative_indices() {
    let test_str = string!("hello world");
    let args = vec![test_str, Value::Number(-5.0), Value::Number(-1.0)];

    let result = native_string_substring(&args).unwrap();
    assert_eq!("worl", as_string!(result));
}

#[test]
fn test_substring_out_of_bounds() {
    let test_str = string!("hello");
    let args = vec![test_str, Value::Number(0.0), Value::Number(100.0)];

    let result = native_string_substring(&args).unwrap();
    assert_eq!("hello", as_string!(result));
}

#[test]
fn test_substring_empty() {
    let test_str = string!("hello");
    let args = vec![test_str, Value::Number(2.0), Value::Number(2.0)];

    let result = native_string_substring(&args).unwrap();
    assert_eq!("", as_string!(result));
}

#[test]
fn test_substring_start_greater_than_end() {
    let test_str = string!("hello");
    // start=5, end=0: should return empty string
    let args = vec![test_str, Value::Number(5.0), Value::Number(0.0)];

    let result = native_string_substring(&args).unwrap();
    assert_eq!("", as_string!(result));
}

#[test]
fn test_replace_basic() {
    let test_str = string!("hello world");
    let old = string!("world");
    let new = string!("rust");
    let args = vec![test_str, old, new];

    let result = native_string_replace(&args).unwrap();
    assert_eq!("hello rust", as_string!(result));
}

#[test]
fn test_replace_multiple() {
    let test_str = string!("foo bar foo");
    let old = string!("foo");
    let new = string!("baz");
    let args = vec![test_str, old, new];

    let result = native_string_replace(&args).unwrap();
    assert_eq!("baz bar baz", as_string!(result));
}

#[test]
fn test_replace_not_found() {
    let test_str = string!("hello world");
    let old = string!("xyz");
    let new = string!("abc");
    let args = vec![test_str, old, new];

    let result = native_string_replace(&args).unwrap();
    assert_eq!("hello world", as_string!(result));
}

#[test]
fn test_replace_empty_string() {
    let test_str = string!("");
    let old = string!("foo");
    let new = string!("bar");
    let args = vec![test_str, old, new];

    let result = native_string_replace(&args).unwrap();
    assert_eq!("", as_string!(result));
}

#[test]
fn test_replace_with_empty() {
    let test_str = string!("hello world");
    let old = string!(" ");
    let new = string!("");
    let args = vec![test_str, old, new];

    let result = native_string_replace(&args).unwrap();
    assert_eq!("helloworld", as_string!(result));
}

#[test]
fn test_split_basic_comma() {
    let test_str = string!("a,b,c");
    let delimiter = string!(",");
    let args = vec![test_str, delimiter];

    let result = native_string_split(&args).unwrap();

    // Verify it's an array
    match result {
        Value::Object(obj) => match obj.as_ref() {
            crate::common::Object::Array(arr) => {
                let elements = arr.borrow();
                assert_eq!(elements.len(), 3);
                assert_eq!("a", as_string!(elements[0]));
                assert_eq!("b", as_string!(elements[1]));
                assert_eq!("c", as_string!(elements[2]));
            }
            _ => panic!("Expected Array object"),
        },
        _ => panic!("Expected Object value"),
    }
}

#[test]
fn test_split_delimiter_not_found() {
    let test_str = string!("hello world");
    let delimiter = string!(",");
    let args = vec![test_str, delimiter];

    let result = native_string_split(&args).unwrap();

    match result {
        Value::Object(obj) => match obj.as_ref() {
            crate::common::Object::Array(arr) => {
                let elements = arr.borrow();
                assert_eq!(elements.len(), 1);
                assert_eq!("hello world", as_string!(elements[0]));
            }
            _ => panic!("Expected Array object"),
        },
        _ => panic!("Expected Object value"),
    }
}

#[test]
fn test_split_empty_string() {
    let test_str = string!("");
    let delimiter = string!(",");
    let args = vec![test_str, delimiter];

    let result = native_string_split(&args).unwrap();

    match result {
        Value::Object(obj) => match obj.as_ref() {
            crate::common::Object::Array(arr) => {
                let elements = arr.borrow();
                assert_eq!(elements.len(), 1);
                assert_eq!("", as_string!(elements[0]));
            }
            _ => panic!("Expected Array object"),
        },
        _ => panic!("Expected Object value"),
    }
}

#[test]
fn test_split_empty_delimiter() {
    let test_str = string!("hello");
    let delimiter = string!("");
    let args = vec![test_str, delimiter];

    let result = native_string_split(&args).unwrap();

    match result {
        Value::Object(obj) => match obj.as_ref() {
            crate::common::Object::Array(arr) => {
                let elements = arr.borrow();
                assert_eq!(elements.len(), 5);
                assert_eq!("h", as_string!(elements[0]));
                assert_eq!("e", as_string!(elements[1]));
                assert_eq!("l", as_string!(elements[2]));
                assert_eq!("l", as_string!(elements[3]));
                assert_eq!("o", as_string!(elements[4]));
            }
            _ => panic!("Expected Array object"),
        },
        _ => panic!("Expected Object value"),
    }
}

#[test]
fn test_split_multiple_consecutive_delimiters() {
    let test_str = string!("a,,b,,c");
    let delimiter = string!(",");
    let args = vec![test_str, delimiter];

    let result = native_string_split(&args).unwrap();

    match result {
        Value::Object(obj) => {
            match obj.as_ref() {
                crate::common::Object::Array(arr) => {
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
    let test_str = string!("hello world");
    let delimiter = string!(" ");
    let args = vec![test_str, delimiter];

    let result = native_string_split(&args).unwrap();

    match result {
        Value::Object(obj) => match obj.as_ref() {
            crate::common::Object::Array(arr) => {
                let elements = arr.borrow();
                assert_eq!(elements.len(), 2);
                assert_eq!("hello", as_string!(elements[0]));
                assert_eq!("world", as_string!(elements[1]));
            }
            _ => panic!("Expected Array object"),
        },
        _ => panic!("Expected Object value"),
    }
}

// Tests for String.toInt()
#[test]
fn test_to_int_basic_positive() {
    let test_str = string!("123");
    let args = vec![test_str];

    let result = native_string_to_int(&args).unwrap();
    assert_eq!(as_number!(result), 123.0);
}

#[test]
fn test_to_int_basic_negative() {
    let test_str = string!("-456");
    let args = vec![test_str];

    let result = native_string_to_int(&args).unwrap();
    assert_eq!(as_number!(result), -456.0);
}

#[test]
fn test_to_int_zero() {
    let test_str = string!("0");
    let args = vec![test_str];

    let result = native_string_to_int(&args).unwrap();
    assert_eq!(as_number!(result), 0.0);
}

#[test]
fn test_to_int_with_whitespace() {
    let test_str = string!("  789  ");
    let args = vec![test_str];

    let result = native_string_to_int(&args).unwrap();
    assert_eq!(as_number!(result), 789.0);
}

#[test]
fn test_to_int_large_number() {
    let test_str = string!("9876543210");
    let args = vec![test_str];

    let result = native_string_to_int(&args).unwrap();
    assert_eq!(as_number!(result), 9876543210.0);
}

#[test]
fn test_to_int_invalid_float() {
    let test_str = string!("123.45");
    let args = vec![test_str];

    let result = native_string_to_int(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not a valid integer"));
}

#[test]
fn test_to_int_invalid_string() {
    let test_str = string!("abc");
    let args = vec![test_str];

    let result = native_string_to_int(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not a valid integer"));
}

#[test]
fn test_to_int_empty_string() {
    let test_str = string!("");
    let args = vec![test_str];

    let result = native_string_to_int(&args);
    assert!(result.is_err());
}

#[test]
fn test_to_int_mixed_content() {
    let test_str = string!("123abc");
    let args = vec![test_str];

    let result = native_string_to_int(&args);
    assert!(result.is_err());
}

// Tests for String.toFloat()
#[test]
fn test_to_float_basic_integer() {
    let test_str = string!("123");
    let args = vec![test_str];

    let result = native_string_to_float(&args).unwrap();
    assert_eq!(as_number!(result), 123.0);
}

#[test]
fn test_to_float_basic_decimal() {
    let test_str = string!("45.67");
    let args = vec![test_str];

    let result = native_string_to_float(&args).unwrap();
    assert_eq!(as_number!(result), 45.67);
}

#[test]
fn test_to_float_negative() {
    let test_str = string!("-12.34");
    let args = vec![test_str];

    let result = native_string_to_float(&args).unwrap();
    assert_eq!(as_number!(result), -12.34);
}

#[test]
fn test_to_float_zero() {
    let test_str = string!("0.0");
    let args = vec![test_str];

    let result = native_string_to_float(&args).unwrap();
    assert_eq!(as_number!(result), 0.0);
}

#[test]
fn test_to_float_with_whitespace() {
    let test_str = string!("  3.15  ");
    let args = vec![test_str];

    let result = native_string_to_float(&args).unwrap();
    assert_eq!(as_number!(result), 3.15);
}

#[test]
fn test_to_float_scientific_notation() {
    let test_str = string!("1.23e4");
    let args = vec![test_str];

    let result = native_string_to_float(&args).unwrap();
    assert_eq!(as_number!(result), 12300.0);
}

#[test]
fn test_to_float_scientific_notation_negative_exponent() {
    let test_str = string!("1.5e-2");
    let args = vec![test_str];

    let result = native_string_to_float(&args).unwrap();
    assert_eq!(as_number!(result), 0.015);
}

#[test]
fn test_to_float_no_decimal() {
    let test_str = string!("42");
    let args = vec![test_str];

    let result = native_string_to_float(&args).unwrap();
    assert_eq!(as_number!(result), 42.0);
}

#[test]
fn test_to_float_leading_decimal() {
    let test_str = string!(".5");
    let args = vec![test_str];

    let result = native_string_to_float(&args).unwrap();
    assert_eq!(as_number!(result), 0.5);
}

#[test]
fn test_to_float_invalid_string() {
    let test_str = string!("abc");
    let args = vec![test_str];

    let result = native_string_to_float(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not a valid float"));
}

#[test]
fn test_to_float_empty_string() {
    let test_str = string!("");
    let args = vec![test_str];

    let result = native_string_to_float(&args);
    assert!(result.is_err());
}

#[test]
fn test_to_float_mixed_content() {
    let test_str = string!("12.34abc");
    let args = vec![test_str];

    let result = native_string_to_float(&args);
    assert!(result.is_err());
}

#[test]
fn test_to_float_infinity() {
    let test_str = string!("inf");
    let args = vec![test_str];

    let result = native_string_to_float(&args).unwrap();
    assert!(as_number!(result).is_infinite() && as_number!(result) > 0.0);
}

#[test]
fn test_to_float_negative_infinity() {
    let test_str = string!("-inf");
    let args = vec![test_str];

    let result = native_string_to_float(&args).unwrap();
    assert!(as_number!(result).is_infinite() && as_number!(result) < 0.0);
}

// Tests for String.toBool()
#[test]
fn test_to_bool_lowercase_true() {
    let test_str = string!("true");
    let args = vec![test_str];

    let result = native_string_to_bool(&args).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_to_bool_lowercase_false() {
    let test_str = string!("false");
    let args = vec![test_str];

    let result = native_string_to_bool(&args).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_to_bool_uppercase_true() {
    let test_str = string!("TRUE");
    let args = vec![test_str];

    let result = native_string_to_bool(&args).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_to_bool_uppercase_false() {
    let test_str = string!("FALSE");
    let args = vec![test_str];

    let result = native_string_to_bool(&args).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_to_bool_mixed_case_true() {
    let test_str = string!("TrUe");
    let args = vec![test_str];

    let result = native_string_to_bool(&args).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_to_bool_mixed_case_false() {
    let test_str = string!("FaLsE");
    let args = vec![test_str];

    let result = native_string_to_bool(&args).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_to_bool_with_whitespace_true() {
    let test_str = string!("  true  ");
    let args = vec![test_str];

    let result = native_string_to_bool(&args).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_to_bool_with_whitespace_false() {
    let test_str = string!("  false  ");
    let args = vec![test_str];

    let result = native_string_to_bool(&args).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_to_bool_with_tabs_and_newlines() {
    let test_str = string!("\t\ntrue\n\t");
    let args = vec![test_str];

    let result = native_string_to_bool(&args).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_to_bool_invalid_string() {
    let test_str = string!("yes");
    let args = vec![test_str];

    let result = native_string_to_bool(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not a valid boolean"));
}

#[test]
fn test_to_bool_invalid_number() {
    let test_str = string!("1");
    let args = vec![test_str];

    let result = native_string_to_bool(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not a valid boolean"));
}

#[test]
fn test_to_bool_invalid_zero() {
    let test_str = string!("0");
    let args = vec![test_str];

    let result = native_string_to_bool(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not a valid boolean"));
}

#[test]
fn test_to_bool_empty_string() {
    let test_str = string!("");
    let args = vec![test_str];

    let result = native_string_to_bool(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not a valid boolean"));
}

#[test]
fn test_to_bool_partial_match() {
    let test_str = string!("truee");
    let args = vec![test_str];

    let result = native_string_to_bool(&args);
    assert!(result.is_err());
}

#[test]
fn test_to_bool_mixed_content() {
    let test_str = string!("true123");
    let args = vec![test_str];

    let result = native_string_to_bool(&args);
    assert!(result.is_err());
}

#[test]
fn test_to_bool_with_surrounding_text() {
    let test_str = string!("the answer is true");
    let args = vec![test_str];

    let result = native_string_to_bool(&args);
    assert!(result.is_err());
}

// Tests for String.trim()
#[test]
fn test_trim_basic() {
    let test_str = string!("  hello world  ");
    let args = vec![test_str];

    let result = native_string_trim(&args).unwrap();
    assert_eq!("hello world", as_string!(result));
}

#[test]
fn test_trim_leading_only() {
    let test_str = string!("  hello");
    let args = vec![test_str];

    let result = native_string_trim(&args).unwrap();
    assert_eq!("hello", as_string!(result));
}

#[test]
fn test_trim_trailing_only() {
    let test_str = string!("hello  ");
    let args = vec![test_str];

    let result = native_string_trim(&args).unwrap();
    assert_eq!("hello", as_string!(result));
}

#[test]
fn test_trim_no_whitespace() {
    let test_str = string!("hello");
    let args = vec![test_str];

    let result = native_string_trim(&args).unwrap();
    assert_eq!("hello", as_string!(result));
}

#[test]
fn test_trim_empty_string() {
    let test_str = string!("");
    let args = vec![test_str];

    let result = native_string_trim(&args).unwrap();
    assert_eq!("", as_string!(result));
}

#[test]
fn test_trim_only_whitespace() {
    let test_str = string!("   ");
    let args = vec![test_str];

    let result = native_string_trim(&args).unwrap();
    assert_eq!("", as_string!(result));
}

#[test]
fn test_trim_tabs_and_newlines() {
    let test_str = string!("\t\nhello\n\t");
    let args = vec![test_str];

    let result = native_string_trim(&args).unwrap();
    assert_eq!("hello", as_string!(result));
}

// Tests for String.startsWith()
#[test]
fn test_starts_with_true() {
    let test_str = string!("hello world");
    let prefix = string!("hello");
    let args = vec![test_str, prefix];

    let result = native_string_starts_with(&args).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_starts_with_false() {
    let test_str = string!("hello world");
    let prefix = string!("world");
    let args = vec![test_str, prefix];

    let result = native_string_starts_with(&args).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_starts_with_empty_prefix() {
    let test_str = string!("hello");
    let prefix = string!("");
    let args = vec![test_str, prefix];

    let result = native_string_starts_with(&args).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_starts_with_exact_match() {
    let test_str = string!("hello");
    let prefix = string!("hello");
    let args = vec![test_str, prefix];

    let result = native_string_starts_with(&args).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_starts_with_longer_prefix() {
    let test_str = string!("hi");
    let prefix = string!("hello");
    let args = vec![test_str, prefix];

    let result = native_string_starts_with(&args).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

// Tests for String.endsWith()
#[test]
fn test_ends_with_true() {
    let test_str = string!("hello world");
    let suffix = string!("world");
    let args = vec![test_str, suffix];

    let result = native_string_ends_with(&args).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_ends_with_false() {
    let test_str = string!("hello world");
    let suffix = string!("hello");
    let args = vec![test_str, suffix];

    let result = native_string_ends_with(&args).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_ends_with_empty_suffix() {
    let test_str = string!("hello");
    let suffix = string!("");
    let args = vec![test_str, suffix];

    let result = native_string_ends_with(&args).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_ends_with_exact_match() {
    let test_str = string!("hello");
    let suffix = string!("hello");
    let args = vec![test_str, suffix];

    let result = native_string_ends_with(&args).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_ends_with_longer_suffix() {
    let test_str = string!("hi");
    let suffix = string!("hello");
    let args = vec![test_str, suffix];

    let result = native_string_ends_with(&args).unwrap();
    assert_eq!(result, Value::Boolean(false));
}

// Tests for String.indexOf()
#[test]
fn test_index_of_found_at_start() {
    let test_str = string!("hello world");
    let substring = string!("hello");
    let args = vec![test_str, substring];

    let result = native_string_index_of(&args).unwrap();
    assert_eq!(as_number!(result), 0.0);
}

#[test]
fn test_index_of_found_at_end() {
    let test_str = string!("hello world");
    let substring = string!("world");
    let args = vec![test_str, substring];

    let result = native_string_index_of(&args).unwrap();
    assert_eq!(as_number!(result), 6.0);
}

#[test]
fn test_index_of_found_in_middle() {
    let test_str = string!("hello world");
    let substring = string!("lo w");
    let args = vec![test_str, substring];

    let result = native_string_index_of(&args).unwrap();
    assert_eq!(as_number!(result), 3.0);
}

#[test]
fn test_index_of_not_found() {
    let test_str = string!("hello world");
    let substring = string!("xyz");
    let args = vec![test_str, substring];

    let result = native_string_index_of(&args).unwrap();
    assert_eq!(as_number!(result), -1.0);
}

#[test]
fn test_index_of_empty_substring() {
    let test_str = string!("hello");
    let substring = string!("");
    let args = vec![test_str, substring];

    let result = native_string_index_of(&args).unwrap();
    assert_eq!(as_number!(result), 0.0);
}

#[test]
fn test_index_of_single_char() {
    let test_str = string!("hello");
    let substring = string!("l");
    let args = vec![test_str, substring];

    let result = native_string_index_of(&args).unwrap();
    assert_eq!(as_number!(result), 2.0); // First 'l' is at index 2
}

#[test]
fn test_index_of_unicode() {
    let test_str = string!("hello 🌍 world");
    let substring = string!("🌍");
    let args = vec![test_str, substring];

    let result = native_string_index_of(&args).unwrap();
    assert_eq!(as_number!(result), 6.0);
}

// Tests for String.charAt()
#[test]
fn test_char_at_basic() {
    let test_str = string!("hello");
    let args = vec![test_str, Value::Number(0.0)];

    let result = native_string_char_at(&args).unwrap();
    assert_eq!("h", as_string!(result));
}

#[test]
fn test_char_at_middle() {
    let test_str = string!("hello");
    let args = vec![test_str, Value::Number(2.0)];

    let result = native_string_char_at(&args).unwrap();
    assert_eq!("l", as_string!(result));
}

#[test]
fn test_char_at_last() {
    let test_str = string!("hello");
    let args = vec![test_str, Value::Number(4.0)];

    let result = native_string_char_at(&args).unwrap();
    assert_eq!("o", as_string!(result));
}

#[test]
fn test_char_at_negative_index() {
    let test_str = string!("hello");
    let args = vec![test_str, Value::Number(-1.0)];

    let result = native_string_char_at(&args).unwrap();
    assert_eq!("o", as_string!(result));
}

#[test]
fn test_char_at_out_of_bounds() {
    let test_str = string!("hello");
    let args = vec![test_str, Value::Number(10.0)];

    let result = native_string_char_at(&args);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("out of bounds"));
}

#[test]
fn test_char_at_unicode() {
    let test_str = string!("hello 🌍");
    let args = vec![test_str, Value::Number(6.0)];

    let result = native_string_char_at(&args).unwrap();
    assert_eq!("🌍", as_string!(result));
}

// Tests for String.toUpperCase()
#[test]
fn test_to_upper_case_basic() {
    let test_str = string!("hello");
    let args = vec![test_str];

    let result = native_string_to_upper_case(&args).unwrap();
    assert_eq!("HELLO", as_string!(result));
}

#[test]
fn test_to_upper_case_mixed() {
    let test_str = string!("HeLLo WoRLd");
    let args = vec![test_str];

    let result = native_string_to_upper_case(&args).unwrap();
    assert_eq!("HELLO WORLD", as_string!(result));
}

#[test]
fn test_to_upper_case_already_upper() {
    let test_str = string!("HELLO");
    let args = vec![test_str];

    let result = native_string_to_upper_case(&args).unwrap();
    assert_eq!("HELLO", as_string!(result));
}

#[test]
fn test_to_upper_case_empty() {
    let test_str = string!("");
    let args = vec![test_str];

    let result = native_string_to_upper_case(&args).unwrap();
    assert_eq!("", as_string!(result));
}

#[test]
fn test_to_upper_case_numbers_and_symbols() {
    let test_str = string!("hello123!@#");
    let args = vec![test_str];

    let result = native_string_to_upper_case(&args).unwrap();
    assert_eq!("HELLO123!@#", as_string!(result));
}

#[test]
fn test_to_upper_case_unicode() {
    let test_str = string!("hello 🌍");
    let args = vec![test_str];

    let result = native_string_to_upper_case(&args).unwrap();
    assert_eq!("HELLO 🌍", as_string!(result));
}

// Tests for String.toLowerCase()
#[test]
fn test_to_lower_case_basic() {
    let test_str = string!("HELLO");
    let args = vec![test_str];

    let result = native_string_to_lower_case(&args).unwrap();
    assert_eq!("hello", as_string!(result));
}

#[test]
fn test_to_lower_case_mixed() {
    let test_str = string!("HeLLo WoRLd");
    let args = vec![test_str];

    let result = native_string_to_lower_case(&args).unwrap();
    assert_eq!("hello world", as_string!(result));
}

#[test]
fn test_to_lower_case_already_lower() {
    let test_str = string!("hello");
    let args = vec![test_str];

    let result = native_string_to_lower_case(&args).unwrap();
    assert_eq!("hello", as_string!(result));
}

#[test]
fn test_to_lower_case_empty() {
    let test_str = string!("");
    let args = vec![test_str];

    let result = native_string_to_lower_case(&args).unwrap();
    assert_eq!("", as_string!(result));
}

#[test]
fn test_to_lower_case_numbers_and_symbols() {
    let test_str = string!("HELLO123!@#");
    let args = vec![test_str];

    let result = native_string_to_lower_case(&args).unwrap();
    assert_eq!("hello123!@#", as_string!(result));
}

#[test]
fn test_to_lower_case_unicode() {
    let test_str = string!("HELLO 🌍");
    let args = vec![test_str];

    let result = native_string_to_lower_case(&args).unwrap();
    assert_eq!("hello 🌍", as_string!(result));
}
