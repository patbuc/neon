use crate::vm::{Result, VirtualMachine};

#[test]
fn test_math_min_variadic() {
    let program = r#"
        print Math.min(5, 2, 8, 1)
        print Math.min(3, 7)
        print Math.min(42)
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n3\n42", vm.get_output());
}

#[test]
fn test_math_max_variadic() {
    let program = r#"
        print Math.max(5, 2, 8, 1)
        print Math.max(3, 7)
        print Math.max(42)
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("8\n7\n42", vm.get_output());
}

#[test]
fn test_math_min_max_negative() {
    let program = r#"
        print Math.min(-5, -2, -8)
        print Math.max(-5, -2, -8)
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("-8\n-2", vm.get_output());
}

#[test]
fn test_math_min_max_mixed() {
    let program = r#"
        print Math.min(10, -3, 5, -10)
        print Math.max(10, -3, 5, -10)
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("-10\n10", vm.get_output());
}
