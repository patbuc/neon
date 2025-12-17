use crate::vm::{Result, VirtualMachine};

// ============================================================================
// Set Functions - Success Cases
// ============================================================================

#[test]
fn test_set_add() {
    let program = r#"
        val s = {1, 2}
        print s.add(3)
        print s.size()
        print s.add(2)
        print s.size()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\n3\nfalse\n3", vm.get_output());
}

#[test]
fn test_set_has() {
    let program = r#"
        val s = {1, 2, 3}
        print s.has(2)
        print s.has(5)
        print s.has("hello")
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\nfalse\nfalse", vm.get_output());
}

#[test]
fn test_set_remove() {
    let program = r#"
        val s = {1, 2, 3}
        print s.size()
        print s.remove(2)
        print s.size()
        print s.has(2)
        print s.remove(2)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\ntrue\n2\nfalse\nfalse", vm.get_output());
}

#[test]
fn test_set_size() {
    let program = r#"
        val s = {1, 2, 3}
        print s.size()

        val empty = {}
        print empty.size()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\n0", vm.get_output());
}

#[test]
fn test_set_clear() {
    let program = r#"
        val s = {1, 2, 3}
        print s.size()
        s.clear()
        print s.size()
        print s.has(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\n0\nfalse", vm.get_output());
}

#[test]
fn test_set_to_array() {
    let program = r#"
        val s = {3, 1, 2}
        val arr = s.toArray()
        print arr.length()
        print arr.contains(1)
        print arr.contains(2)
        print arr.contains(3)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\ntrue\ntrue\ntrue", vm.get_output());
}

#[test]
fn test_set_union() {
    let program = r#"
        val s1 = {1, 2, 3}
        val s2 = {3, 4, 5}
        val result = s1.union(s2)
        print result.size()
        print result.has(1)
        print result.has(3)
        print result.has(5)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("5\ntrue\ntrue\ntrue", vm.get_output());
}

#[test]
fn test_set_intersection() {
    let program = r#"
        val s1 = {1, 2, 3}
        val s2 = {2, 3, 4}
        val result = s1.intersection(s2)
        print result.size()
        print result.has(2)
        print result.has(3)
        print result.has(1)
        print result.has(4)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("2\ntrue\ntrue\nfalse\nfalse", vm.get_output());
}

#[test]
fn test_set_difference() {
    let program = r#"
        val s1 = {1, 2, 3, 4}
        val s2 = {3, 4, 5}
        val result = s1.difference(s2)
        print result.size()
        print result.has(1)
        print result.has(2)
        print result.has(3)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("2\ntrue\ntrue\nfalse", vm.get_output());
}

#[test]
fn test_set_is_subset() {
    let program = r#"
        val s1 = {1, 2}
        val s2 = {1, 2, 3, 4}
        print s1.isSubset(s2)
        print s2.isSubset(s1)

        val s3 = {1, 2}
        val s4 = {1, 2}
        print s3.isSubset(s4)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\nfalse\ntrue", vm.get_output());
}

#[test]
fn test_set_different_types() {
    let program = r#"
        val s = {1, "hello", true}
        print s.size()
        print s.has(1)
        print s.has("hello")
        print s.has(true)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\ntrue\ntrue\ntrue", vm.get_output());
}

// ============================================================================
// Set Functions - Error Cases
// ============================================================================

#[test]
fn test_set_add_wrong_arg_count() {
    let program = r#"
        val s = {1, 2}
        s.add()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_set_has_wrong_arg_count() {
    let program = r#"
        val s = {1, 2}
        s.has()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_set_remove_wrong_arg_count() {
    let program = r#"
        val s = {1, 2}
        s.remove()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}
