use crate::vm::{InterpretResult, VirtualMachine};

// ============================================================================
// Number.toString() - Success Cases
// ============================================================================

#[test]
fn test_number_to_string() {
    let program = r#"
        print((123).toString())
        print((45.67).toString())
        print((0).toString())
        print((-42).toString())
        print((-3.15).toString())
        print((1000000).toString())
        print((0.001).toString())
        print((12300000000).toString())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!(
        "123\n45.67\n0\n-42\n-3.15\n1000000\n0.001\n12300000000",
        vm.get_output()
    );
}

#[test]
fn test_number_to_string_special_values() {
    let program = r#"
        val inf = 1.0 / 0.0
        val neg_inf = -1.0 / 0.0
        val nan = 0.0 / 0.0
        print(inf.toString())
        print(neg_inf.toString())
        print(nan.toString())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("inf\n-inf\nNaN", vm.get_output());
}

#[test]
fn test_number_to_string_large_int() {
    let program = r#"
        print(9007199254740993.toString())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("9007199254740993", vm.get_output());
}

// ============================================================================
// Number.toInt() / Number.toFloat()
// ============================================================================

#[test]
fn test_number_to_int_truncates_float() {
    let program = r#"
        print((5.7).toInt())
        print((-5.7).toInt())
        print((5).toInt())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("5\n-5\n5", vm.get_output());
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
