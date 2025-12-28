use crate::common::opcodes::OpCode;
use crate::common::{Chunk, MapKey, Object, Value};
use crate::vm::{Result, VirtualMachine};
use crate::string;
use std::rc::Rc;

/// Helper function to extract the Rc<str> from a Value if it's a string.
/// Returns None if the value is not a string.
fn extract_string_rc(value: &Value) -> Option<Rc<str>> {
    match value {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(obj_string) => Some(Rc::clone(&obj_string.value)),
            _ => None,
        },
        _ => None,
    }
}

#[test]
fn test_identical_string_literals_share_pointer() {
    let mut vm = VirtualMachine::new();
    let mut chunk = Chunk::new("test");

    // Add string literal "hello" twice using write_string
    // write_string both adds to strings table AND emits OpCode::String
    chunk.write_string(string!("hello"), 0, 0);
    chunk.write_string(string!("hello"), 0, 0);
    chunk.write_string(string!("world"), 0, 0);
    chunk.write_op_code(OpCode::Return, 0, 0);

    // Run chunk - strings will be interned at load time
    let result = vm.run_chunk(chunk);
    assert_eq!(Result::Ok, result);

    // Pop values from stack (in reverse order)
    let val3 = vm.pop(); // "world"
    let val2 = vm.pop(); // "hello" (second)
    let val1 = vm.pop(); // "hello" (first)

    // Extract Rc<str> and verify pointer equality
    let rc1 = extract_string_rc(&val1).expect("val1 should be string");
    let rc2 = extract_string_rc(&val2).expect("val2 should be string");
    let rc3 = extract_string_rc(&val3).expect("val3 should be string");

    // Verify content
    assert_eq!(rc1.as_ref(), "hello");
    assert_eq!(rc2.as_ref(), "hello");
    assert_eq!(rc3.as_ref(), "world");

    // Verify interning: identical strings share pointer
    assert!(
        Rc::ptr_eq(&rc1, &rc2),
        "Identical string literals should share Rc pointer"
    );

    // Different strings have different pointers
    assert!(
        !Rc::ptr_eq(&rc1, &rc3),
        "Different string literals should have different pointers"
    );
}

#[test]
fn test_runtime_concatenation_produces_interned_strings() {
    let mut vm = VirtualMachine::new();
    let mut chunk = Chunk::new("test");

    // First concatenation: "hel" + "lo"
    chunk.write_string(string!("hel"), 0, 0);
    chunk.write_string(string!("lo"), 0, 0);
    chunk.write_op_code(OpCode::Add, 0, 0); // String concatenation

    // Second concatenation: "hel" + "lo"
    chunk.write_string(string!("hel"), 0, 0);
    chunk.write_string(string!("lo"), 0, 0);
    chunk.write_op_code(OpCode::Add, 0, 0); // String concatenation

    // Literal "hello"
    chunk.write_string(string!("hello"), 0, 0);
    chunk.write_op_code(OpCode::Return, 0, 0);

    let result = vm.run_chunk(chunk);
    assert_eq!(Result::Ok, result);

    // Pop results (reverse order)
    let val3 = vm.pop(); // "hello" literal
    let val2 = vm.pop(); // "hel" + "lo" (second)
    let val1 = vm.pop(); // "hel" + "lo" (first)

    let rc1 = extract_string_rc(&val1).expect("val1 should be string");
    let rc2 = extract_string_rc(&val2).expect("val2 should be string");
    let rc3 = extract_string_rc(&val3).expect("val3 should be string");

    // All should produce "hello"
    assert_eq!(rc1.as_ref(), "hello");
    assert_eq!(rc2.as_ref(), "hello");
    assert_eq!(rc3.as_ref(), "hello");

    // All should share the same interned pointer
    assert!(
        Rc::ptr_eq(&rc1, &rc2),
        "Concatenated strings with same result should share pointer"
    );
    assert!(
        Rc::ptr_eq(&rc1, &rc3),
        "Concatenated string should share pointer with literal"
    );
}

#[test]
fn test_empty_string_interning() {
    let mut vm = VirtualMachine::new();
    let mut chunk = Chunk::new("test");

    // Add empty string literals
    chunk.write_string(string!(""), 0, 0);
    chunk.write_string(string!(""), 0, 0);

    // Create empty string via concatenation: "" + ""
    chunk.write_string(string!(""), 0, 0);
    chunk.write_string(string!(""), 0, 0);
    chunk.write_op_code(OpCode::Add, 0, 0);
    chunk.write_op_code(OpCode::Return, 0, 0);

    let result = vm.run_chunk(chunk);
    assert_eq!(Result::Ok, result);

    let val3 = vm.pop(); // concatenated ""
    let val2 = vm.pop(); // second ""
    let val1 = vm.pop(); // first ""

    let rc1 = extract_string_rc(&val1).expect("val1 should be string");
    let rc2 = extract_string_rc(&val2).expect("val2 should be string");
    let rc3 = extract_string_rc(&val3).expect("val3 should be string");

    // All should be empty
    assert_eq!(rc1.as_ref(), "");
    assert_eq!(rc2.as_ref(), "");
    assert_eq!(rc3.as_ref(), "");

    // All should share the same pointer
    assert!(
        Rc::ptr_eq(&rc1, &rc2),
        "Empty string literals should share pointer"
    );
    assert!(
        Rc::ptr_eq(&rc1, &rc3),
        "Empty string from concatenation should share pointer with literal"
    );
}

#[test]
fn test_unicode_string_interning() {
    let mut vm = VirtualMachine::new();
    let mut chunk = Chunk::new("test");

    // Unicode string literal twice
    chunk.write_string(string!("Hello, 世界"), 0, 0);
    chunk.write_string(string!("Hello, 世界"), 0, 0);

    // Unicode string via concatenation
    chunk.write_string(string!("Hello, "), 0, 0);
    chunk.write_string(string!("世界"), 0, 0);
    chunk.write_op_code(OpCode::Add, 0, 0);
    chunk.write_op_code(OpCode::Return, 0, 0);

    let result = vm.run_chunk(chunk);
    assert_eq!(Result::Ok, result);

    let val3 = vm.pop(); // concatenated
    let val2 = vm.pop(); // second literal
    let val1 = vm.pop(); // first literal

    let rc1 = extract_string_rc(&val1).expect("val1 should be string");
    let rc2 = extract_string_rc(&val2).expect("val2 should be string");
    let rc3 = extract_string_rc(&val3).expect("val3 should be string");

    // All should be the same Unicode string
    assert_eq!(rc1.as_ref(), "Hello, 世界");
    assert_eq!(rc2.as_ref(), "Hello, 世界");
    assert_eq!(rc3.as_ref(), "Hello, 世界");

    // All should share the same pointer
    assert!(
        Rc::ptr_eq(&rc1, &rc2),
        "Unicode string literals should share pointer"
    );
    assert!(
        Rc::ptr_eq(&rc1, &rc3),
        "Concatenated Unicode string should share pointer with literal"
    );
}

#[test]
fn test_different_vms_have_separate_intern_pools() {
    // Create two separate VMs
    let mut vm1 = VirtualMachine::new();
    let mut vm2 = VirtualMachine::new();

    // Create identical bytecode for both
    let mut chunk1 = Chunk::new("test1");
    chunk1.write_string(string!("hello"), 0, 0);
    chunk1.write_op_code(OpCode::Return, 0, 0);

    let mut chunk2 = Chunk::new("test2");
    chunk2.write_string(string!("hello"), 0, 0);
    chunk2.write_op_code(OpCode::Return, 0, 0);

    // Run in both VMs
    let result1 = vm1.run_chunk(chunk1);
    let result2 = vm2.run_chunk(chunk2);

    assert_eq!(Result::Ok, result1);
    assert_eq!(Result::Ok, result2);

    // Extract the string values
    let val1 = vm1.pop();
    let val2 = vm2.pop();

    let rc1 = extract_string_rc(&val1).expect("val1 should be string");
    let rc2 = extract_string_rc(&val2).expect("val2 should be string");

    // Both should have the same content
    assert_eq!(rc1.as_ref(), "hello");
    assert_eq!(rc2.as_ref(), "hello");

    // But they should NOT share the same pointer (different VMs = different pools)
    assert!(
        !Rc::ptr_eq(&rc1, &rc2),
        "Different VMs should have separate string intern pools"
    );
}

#[test]
fn test_string_equality_benefits_from_interning() {
    let mut vm = VirtualMachine::new();
    let mut chunk = Chunk::new("test");

    // Create two "hello" strings via different methods
    chunk.write_string(string!("hello"), 0, 0); // literal
    chunk.write_string(string!("hel"), 0, 0);
    chunk.write_string(string!("lo"), 0, 0);
    chunk.write_op_code(OpCode::Add, 0, 0); // concatenation

    // Compare them with Equal opcode
    chunk.write_op_code(OpCode::Equal, 0, 0);
    chunk.write_op_code(OpCode::Return, 0, 0);

    let result = vm.run_chunk(chunk);
    assert_eq!(Result::Ok, result);

    // Pop the equality result
    let equality_result = vm.pop();
    assert_eq!(equality_result, Value::Boolean(true));

    // We can't directly verify pointer equality after the fact since the
    // strings are consumed by Equal, but we verified it works
}

#[test]
fn test_many_duplicate_strings_share_memory() {
    let mut vm = VirtualMachine::new();
    let mut chunk = Chunk::new("test");

    // Create many instances of "duplicate"
    for _ in 0..10 {
        chunk.write_string(string!("duplicate"), 0, 0);
    }

    // Create more via concatenation
    for _ in 0..5 {
        chunk.write_string(string!("dup"), 0, 0);
        chunk.write_string(string!("licate"), 0, 0);
        chunk.write_op_code(OpCode::Add, 0, 0);
    }
    chunk.write_op_code(OpCode::Return, 0, 0);

    let result = vm.run_chunk(chunk);
    assert_eq!(Result::Ok, result);

    // Pop all 15 values
    let mut values = Vec::new();
    for _ in 0..15 {
        values.push(vm.pop());
    }

    // Extract all Rc pointers
    let rcs: Vec<Rc<str>> = values
        .iter()
        .map(|v| extract_string_rc(v).expect("should be string"))
        .collect();

    // All should have same content
    for rc in &rcs {
        assert_eq!(rc.as_ref(), "duplicate");
    }

    // All should share the same pointer
    for i in 1..rcs.len() {
        assert!(
            Rc::ptr_eq(&rcs[0], &rcs[i]),
            "All 'duplicate' strings should share the same pointer"
        );
    }
}
