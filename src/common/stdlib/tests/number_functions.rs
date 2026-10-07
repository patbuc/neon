use crate::vm::{InterpretResult, VirtualMachine};

// ============================================================================
// Number.toInt() / Number.toFloat()
// ============================================================================

#[test]
fn test_number_to_int_truncates_float() {
    let program = r#"
        use "std/math"
        print((5.7).toInt())
        print((-5.7).toInt())
        print((5).toInt())
        print(math.div((5.7).toInt(), 1))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("5\n-5\n5\n5", vm.get_output());
}

#[test]
fn test_number_to_int_out_of_range() {
    let program = r#"
        print((1e300).toInt())
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(InterpretResult::RuntimeError, result);
    assert!(vm
        .get_runtime_errors()
        .contains("toInt() result is out of range"));
}

#[test]
fn test_number_to_int_nan() {
    let program = r#"
        val nan = 0.0 / 0.0
        print(nan.toInt())
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(InterpretResult::RuntimeError, result);
    assert!(vm
        .get_runtime_errors()
        .contains("toInt() result is out of range"));
}

#[test]
fn test_number_to_float() {
    let program = r#"
        print((5).toFloat())
        print((5.5).toFloat())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("5\n5.5", vm.get_output());
}

#[test]
fn test_number_to_float_returns_float_variant() {
    let program = r#"
        use "std/math"
        print(math.div((5).toFloat(), 1))
    "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(InterpretResult::RuntimeError, result);
    assert!(vm
        .get_runtime_errors()
        .contains("div() expects two integers, got float"));
}

#[test]
fn test_number_to_float_large_int() {
    let program = r#"
        val n = 9007199254740993
        print(n.toFloat().toString())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("9007199254740992", vm.get_output());
}

#[test]
fn test_number_to_string_very_small() {
    let program = r#"
        val small = 0.000000000123
        val result = small.toString()
        print(result)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    let output = vm.get_output();
    assert!(output.starts_with("0.000000000"));
}
