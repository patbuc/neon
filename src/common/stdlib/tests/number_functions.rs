use crate::vm::{Result, VirtualMachine};

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
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("123\n45.67\n0\n-42\n-3.15\n1000000\n0.001\n12300000000", vm.get_output());
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
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("inf\n-inf\nNaN", vm.get_output());
}

#[test]
fn test_number_to_string_very_small() {
    let program = r#"
        val small = 0.000000000123
        val result = small.toString()
        print(result)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    let output = vm.get_output();
    assert!(output.starts_with("0.000000000"));
}
