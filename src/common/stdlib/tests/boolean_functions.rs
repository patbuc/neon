use crate::vm::{Result, VirtualMachine};

// ============================================================================
// Boolean.toString() - Success Cases
// ============================================================================

#[test]
fn test_boolean_to_string() {
    let program = r#"
        print true.toString()
        print false.toString()
        print true.toString()
        print false.toString()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\nfalse\ntrue\nfalse", vm.get_output());
}
