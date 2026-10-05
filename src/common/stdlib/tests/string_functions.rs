use crate::common::stdlib::string_functions::{
    native_string_char_code_at, native_string_from_char_code, native_string_last_index_of,
    native_string_pad_start, native_string_repeat,
};
use crate::common::Value;
use crate::string;
use crate::vm::{InterpretResult, VirtualMachine};

// ============================================================================
// String.chars() - Success Cases
// ============================================================================

#[test]
fn test_string_chars() {
    let program = r#"
        print("abc".chars())
        print("".chars())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[a, b, c]\n[]", vm.get_output());
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
// String.charCodeAt() - Success Cases
// ============================================================================

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

// ============================================================================
// String.repeat() - Success Cases
// ============================================================================

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
fn test_string_pad_start_negative_length() {
    let program = r#"
        print("ab".padStart(-5, "0"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("ab", vm.get_output());
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
