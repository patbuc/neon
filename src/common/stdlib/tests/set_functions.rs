use crate::vm::{InterpretResult, VirtualMachine};

// ============================================================================
// Set Functions - Success Cases
// ============================================================================

#[test]
fn test_set_to_array_mutation_isolated() {
    let program = r#"
        val s = #{[1, 2]}
        val arr = s.toArray()
        arr[0].push(9)
        print(s.contains([1, 2]))
        print(s.toArray()[0])
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\n[1, 2]", vm.get_output());
}

#[test]
fn test_set_for_in_array_element_fresh_copy() {
    let program = r#"
        val s = #{[1, 2]}
        for e in s {
            e.push(9)
        }
        print(s.contains([1, 2]))
        print(s.toArray()[0])
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\n[1, 2]", vm.get_output());
}

// ============================================================================
// Set Functions - Error Cases
// ============================================================================

#[test]
fn test_set_add_wrong_arg_count() {
    let program = r#"
        val s = #{1, 2}
        s.add()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_set_contains_wrong_arg_count() {
    let program = r#"
        val s = #{1, 2}
        s.contains()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_set_remove_wrong_arg_count() {
    let program = r#"
        val s = #{1, 2}
        s.remove()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}
