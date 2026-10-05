use crate::vm::{InterpretResult, VirtualMachine};

// ============================================================================
// Map Functions - Success Cases
// ============================================================================

#[test]
fn test_map_remove() {
    let program = r#"
        val m = {"a": 1, "b": 2, "c": 3}
        print(m.size())
        print(m.remove("b"))
        print(m.size())
        print(m.contains("b"))
        print(m.remove("b"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\n2\n2\nfalse\nnil", vm.get_output());
}

#[test]
fn test_map_number_keys() {
    let program = r#"
        val m = {}
        m[1] = "one"
        m[2] = "two"
        print(m.get(1))
        print(m.get(2))
        print(m.contains(1))
        print(m.contains(3))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("one\ntwo\ntrue\nfalse", vm.get_output());
}

// ============================================================================
// Map Functions - Error Cases
// ============================================================================

#[test]
fn test_map_get_wrong_arg_count() {
    let program = r#"
        val m = {"a": 1}
        m.get()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_map_contains_wrong_arg_count() {
    let program = r#"
        val m = {"a": 1}
        m.contains()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_map_remove_wrong_arg_count() {
    let program = r#"
        val m = {"a": 1}
        m.remove()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}
