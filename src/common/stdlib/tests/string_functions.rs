use crate::common::stdlib::string_functions::{
    native_string_char_code_at, native_string_from_char_code,
};
use crate::common::Value;
use crate::string;
use crate::vm::{InterpretResult, VirtualMachine};

// ============================================================================
// String.len() - Success Cases
// ============================================================================

#[test]
fn test_string_len() {
    let program = r#"
        print("hello".len())
        print("hello 🌍".len())
        print("".len())
        print("12345".len())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("5\n7\n0\n5", vm.get_output());
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
