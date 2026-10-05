use crate::common::stdlib::string_functions::{
    native_string_char_code_at, native_string_from_char_code, native_string_last_index_of,
    native_string_pad_start, native_string_repeat,
};
use crate::common::Value;
use crate::string;
use crate::vm::{InterpretResult, VirtualMachine};

// ============================================================================
// String.size() / isEmpty() / contains() - Success Cases
// ============================================================================

#[test]
fn test_string_size() {
    let program = r#"
        print("hello".size())
        print("hello 🌍".size())
        print("".size())
        print("12345".size())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("5\n7\n0\n5", vm.get_output());
}

#[test]
fn test_string_is_empty() {
    let program = r#"
        print("".isEmpty())
        print("abc".isEmpty())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\nfalse", vm.get_output());
}

#[test]
fn test_string_contains() {
    let program = r#"
        print("hello world".contains("world"))
        print("hello world".contains("xyz"))
        print("".contains(""))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\nfalse\ntrue", vm.get_output());
}

// ============================================================================
// String.substring() - Success Cases
// ============================================================================

#[test]
fn test_string_substring() {
    let program = r#"
        print("hello world".substring(0, 5))
        print("hello world".substring(6, 11))
        print("hello".substring(2, 2))
        print("hello".substring(0, 100))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("hello\nworld\n\nhello", vm.get_output());
}

#[test]
fn test_string_substring_negative() {
    let program = r#"
        print("hello world".substring(-5, -1))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("worl", vm.get_output());
}

// ============================================================================
// String.replace() - Success Cases
// ============================================================================

#[test]
fn test_string_replace() {
    let program = r#"
        print("hello world".replace("world", "rust"))
        print("foo bar foo".replace("foo", "baz"))
        print("hello".replace("x", "y"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("hello rust\nbaz bar baz\nhello", vm.get_output());
}

// ============================================================================
// String.split() - Success Cases
// ============================================================================

#[test]
fn test_string_split() {
    let program = r#"
        val words = "hello world test".split(" ")
        print(words)

        val csv = "a,b,c".split(",")
        print(csv)

        val single = "hello".split(",")
        print(single)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[hello, world, test]\n[a, b, c]\n[hello]", vm.get_output());
}

#[test]
fn test_string_split_whitespace() {
    let program = r#"
        print("  1  2\t3\n".split())
        print("".split())
        print("   ".split())
        print("move 3 from 1 to 2".split())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!(
        "[1, 2, 3]\n[]\n[]\n[move, 3, from, 1, to, 2]",
        vm.get_output()
    );
}

// ============================================================================
// String.toInt(), String.toFloat(), String.toBool() - Success Cases
// ============================================================================

#[test]
fn test_string_to_int() {
    let program = r#"
        print("42".toInt())
        print("-123".toInt())
        print("0".toInt())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("42\n-123\n0", vm.get_output());
}

#[test]
fn test_string_to_int_exact() {
    let program = r#"
        print("9007199254740993".toInt())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("9007199254740993", vm.get_output());
}

#[test]
fn test_string_to_float() {
    let program = r#"
        print("3.14".toFloat())
        print("-2.5".toFloat())
        print("42".toFloat())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("3.14\n-2.5\n42", vm.get_output());
}

#[test]
fn test_string_to_bool() {
    let program = r#"
        print("true".toBool())
        print("false".toBool())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\nfalse", vm.get_output());
}

// ============================================================================
// String.trim() - Success Cases
// ============================================================================

#[test]
fn test_string_trim() {
    let program = r#"
        print("  hello  ".trim())
        print("hello".trim())
        print("  ".trim())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("hello\nhello", vm.get_output());
}

// ============================================================================
// String.startsWith() and String.endsWith() - Success Cases
// ============================================================================

#[test]
fn test_string_starts_with() {
    let program = r#"
        print("hello world".startsWith("hello"))
        print("hello world".startsWith("world"))
        print("hello".startsWith(""))
        print("test".startsWith("testing"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\nfalse\ntrue\nfalse", vm.get_output());
}

#[test]
fn test_string_ends_with() {
    let program = r#"
        print("hello world".endsWith("world"))
        print("hello world".endsWith("hello"))
        print("hello".endsWith(""))
        print("test".endsWith("testing"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\nfalse\ntrue\nfalse", vm.get_output());
}

// ============================================================================
// String.indexOf() - Success Cases
// ============================================================================

#[test]
fn test_string_index_of() {
    let program = r#"
        print("hello world".indexOf("world"))
        print("hello world".indexOf("o"))
        print("hello".indexOf("x"))
        print("hello".indexOf(""))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("6\n4\n-1\n0", vm.get_output());
}

// ============================================================================
// String.charAt() - Success Cases
// ============================================================================

#[test]
fn test_string_char_at() {
    let program = r#"
        print("hello".charAt(0))
        print("hello".charAt(4))
        print("hello".charAt(-1))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("h\no\no", vm.get_output());
}

// ============================================================================
// String.charCodeAt() - Success Cases
// ============================================================================

#[test]
fn test_string_char_code_at() {
    let program = r#"
        print("abc".charCodeAt(0))
        print("é".charCodeAt(0))
        print("abc".charCodeAt(-1))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("97\n233\n99", vm.get_output());
}

#[test]
fn test_string_char_code_at_returns_int() {
    let result = native_string_char_code_at(&[string!("abc".to_string()), Value::Int(0)]).unwrap();
    assert!(matches!(result, Value::Int(97)));

    let result = native_string_char_code_at(&[string!("é".to_string()), Value::Int(0)]).unwrap();
    assert!(matches!(result, Value::Int(233)));
}

#[test]
fn test_string_char_code_at_negative_in_range() {
    let result = native_string_char_code_at(&[string!("abc".to_string()), Value::Int(-1)]).unwrap();
    assert!(matches!(result, Value::Int(99)));
}

// ============================================================================
// String.fromCharCode() - Success Cases
// ============================================================================

#[test]
fn test_string_from_char_code() {
    let program = r#"
        print(String.fromCharCode(97))
        print(String.fromCharCode(233))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("a\né", vm.get_output());
}

#[test]
fn test_string_from_char_code_returns_string() {
    let result = native_string_from_char_code(&[Value::Int(97)]).unwrap();
    assert!(matches!(result, Value::String(s) if s.as_str() == "a"));
}

// ============================================================================
// String.toUpperCase() and String.toLowerCase() - Success Cases
// ============================================================================

#[test]
fn test_string_to_upper_case() {
    let program = r#"
        print("hello".toUpperCase())
        print("WORLD".toUpperCase())
        print("Hello World".toUpperCase())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("HELLO\nWORLD\nHELLO WORLD", vm.get_output());
}

#[test]
fn test_string_to_lower_case() {
    let program = r#"
        print("HELLO".toLowerCase())
        print("world".toLowerCase())
        print("Hello World".toLowerCase())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("hello\nworld\nhello world", vm.get_output());
}

// ============================================================================
// String Functions - Error Cases
// ============================================================================

#[test]
fn test_string_to_int_invalid() {
    let program = r#"
        "not a number".toInt()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_to_float_invalid() {
    let program = r#"
        "not a number".toFloat()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_to_bool_invalid() {
    let program = r#"
        "not a bool".toBool()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_char_at_out_of_bounds() {
    let program = r#"
        "hello".charAt(10)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_substring_wrong_arg_count() {
    let program = r#"
        "hello".substring(0)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_replace_wrong_arg_count() {
    let program = r#"
        "hello".replace("h")
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_split_wrong_arg_count() {
    let program = r#"
        "hello".split(",", "extra")
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_starts_with_wrong_arg_count() {
    let program = r#"
        "hello".startsWith()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_ends_with_wrong_arg_count() {
    let program = r#"
        "hello".endsWith()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_index_of_wrong_arg_count() {
    let program = r#"
        "hello".indexOf()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_char_at_wrong_arg_count() {
    let program = r#"
        "hello".charAt()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_char_code_at_out_of_bounds() {
    let program = r#"
        "hello".charCodeAt(10)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_char_code_at_negative_out_of_bounds() {
    let program = r#"
        "abc".charCodeAt(-4)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_char_code_at_non_integer() {
    let program = r#"
        "hello".charCodeAt(0.5)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_from_char_code_invalid() {
    let program = r#"
        String.fromCharCode(-1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_from_char_code_surrogate() {
    let program = r#"
        String.fromCharCode(0xD800)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_from_char_code_non_integer() {
    let program = r#"
        String.fromCharCode(1.5)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

// ============================================================================
// String.repeat() - Success Cases
// ============================================================================

#[test]
fn test_string_repeat() {
    let program = r#"
        print("ab".repeat(3))
        print("ab".repeat(0))
        print("x".repeat(1))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("ababab\n\nx", vm.get_output());
}

#[test]
fn test_string_repeat_negative() {
    let program = r#"
        "ab".repeat(-1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_repeat_non_integer() {
    let program = r#"
        "ab".repeat(1.5)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_repeat_too_large() {
    let program = r#"
        "x".repeat(999999999999)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_repeat_cap_message() {
    let err =
        native_string_repeat(&[string!("x".to_string()), Value::Int(200_000_000)]).unwrap_err();
    assert_eq!("repeat() result exceeds 100000000 chars", err);
}

#[test]
fn test_string_repeat_empty_string_huge_count() {
    let program = r#"
        print("".repeat(999999999999))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("", vm.get_output());
}

#[test]
fn test_string_repeat_huge_float_out_of_range() {
    let err = native_string_repeat(&[string!("x".to_string()), Value::Number(1e300)]).unwrap_err();
    assert!(err.contains("out of range"), "got: {}", err);
}

#[test]
fn test_string_repeat_wrong_arg_count() {
    let program = r#"
        "ab".repeat()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

// ============================================================================
// String.padStart() / String.padEnd() - Success Cases
// ============================================================================

#[test]
fn test_string_pad_start() {
    let program = r#"
        print("101".padStart(6, "0"))
        print("abc".padStart(2, "0"))
        print("1".padStart(5, "ab"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("000101\nabc\nabab1", vm.get_output());
}

#[test]
fn test_string_pad_end() {
    let program = r#"
        print("7".padEnd(3, "."))
        print("abc".padEnd(2, "0"))
        print("1".padEnd(5, "ab"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("7..\nabc\n1abab", vm.get_output());
}

#[test]
fn test_string_pad_start_negative_length() {
    let program = r#"
        print("ab".padStart(-5, "0"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("ab", vm.get_output());
}

#[test]
fn test_string_pad_start_empty_fill() {
    let program = r#"
        "ab".padStart(5, "")
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_pad_end_empty_fill() {
    let program = r#"
        "ab".padEnd(5, "")
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_string_pad_start_multibyte_fill() {
    let program = r#"
        print("é".padStart(4, "äö"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("äöäé", vm.get_output());
}

#[test]
fn test_string_pad_end_multibyte_fill() {
    let program = r#"
        print("é".padEnd(4, "äö"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("éäöä", vm.get_output());
}

#[test]
fn test_string_pad_start_cap_message() {
    let err = native_string_pad_start(&[
        string!("x".to_string()),
        Value::Int(200_000_000),
        string!("0".to_string()),
    ])
    .unwrap_err();
    assert_eq!("padStart() result exceeds 100000000 chars", err);
}

#[test]
fn test_string_pad_start_wrong_arg_count() {
    let program = r#"
        "ab".padStart(5)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

// ============================================================================
// String.lastIndexOf() - Success Cases
// ============================================================================

#[test]
fn test_string_last_index_of() {
    let program = r#"
        print("a-b-c".lastIndexOf("-"))
        print("hello".lastIndexOf("x"))
        print("hello".lastIndexOf("l"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\n-1\n3", vm.get_output());
}

#[test]
fn test_string_last_index_of_multibyte_prefix() {
    let result =
        native_string_last_index_of(&[string!("é-a-b".to_string()), string!("a".to_string())])
            .unwrap();
    assert!(matches!(result, Value::Int(2)));
}

#[test]
fn test_string_last_index_of_overlapping_match() {
    let result =
        native_string_last_index_of(&[string!("aaa".to_string()), string!("aa".to_string())])
            .unwrap();
    assert!(matches!(result, Value::Int(1)));
}

#[test]
fn test_string_last_index_of_multibyte_substring() {
    let result =
        native_string_last_index_of(&[string!("xéé-éé".to_string()), string!("éé".to_string())])
            .unwrap();
    assert!(matches!(result, Value::Int(4)));
}

#[test]
fn test_string_last_index_of_empty_substring() {
    let program = r#"
        print("hello".lastIndexOf(""))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("5", vm.get_output());
}

#[test]
fn test_string_last_index_of_wrong_arg_count() {
    let program = r#"
        "hello".lastIndexOf()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

// ============================================================================
// String.contains() - Error Cases
// ============================================================================

#[test]
fn test_string_contains_wrong_arg_count() {
    let program = r#"
        "hello".contains()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}
