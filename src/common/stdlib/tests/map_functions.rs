use crate::vm::{Result, VirtualMachine};

// ============================================================================
// Map Functions - Success Cases
// ============================================================================

#[test]
fn test_map_basic_operations() {
    let program = r#"
        val m = {"name": "Alice", "age": 30}
        print(m.size())
        print(m.get("name"))
        print(m.get("age"))
        print(m.get("missing"))
        print(m.has("name"))
        print(m.has("missing"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("2\nAlice\n30\nnil\ntrue\nfalse", vm.get_output());
}

#[test]
fn test_map_subscript_assignment() {
    let program = r#"
        val m = {}
        print(m.size())

        m["x"] = 10
        m["y"] = 20
        print(m.size())
        print(m.get("x"))
        print(m.get("y"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("0\n2\n10\n20", vm.get_output());
}

#[test]
fn test_map_remove() {
    let program = r#"
        val m = {"a": 1, "b": 2, "c": 3}
        print(m.size())
        print(m.remove("b"))
        print(m.size())
        print(m.has("b"))
        print(m.remove("b"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\n2\n2\nfalse\nnil", vm.get_output());
}

#[test]
fn test_map_keys() {
    let program = r#"
        val m = {"name": "Alice", "age": 30}
        val k = m.keys()
        print(k.length())
        print(k.contains("name"))
        print(k.contains("age"))
        print(k.contains("missing"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("2\ntrue\ntrue\nfalse", vm.get_output());
}

#[test]
fn test_map_values() {
    let program = r#"
        val m = {"a": 1, "b": 2}
        val v = m.values()
        print(v.length())
        print(v.contains(1))
        print(v.contains(2))
        print(v.contains(3))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("2\ntrue\ntrue\nfalse", vm.get_output());
}

#[test]
fn test_map_entries() {
    let program = r#"
        val m = {"a": 1, "b": 2}
        val e = m.entries()
        print(e.length())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("2", vm.get_output());
}

#[test]
fn test_map_number_keys() {
    let program = r#"
        val m = {}
        m[1] = "one"
        m[2] = "two"
        print(m.get(1))
        print(m.get(2))
        print(m.has(1))
        print(m.has(3))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("one\ntwo\ntrue\nfalse", vm.get_output());
}

#[test]
fn test_map_mixed_values() {
    let program = r#"
        val m = {"str": "hello", "num": 42, "bool": true}
        print(m.get("str"))
        print(m.get("num"))
        print(m.get("bool"))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("hello\n42\ntrue", vm.get_output());
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
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_map_has_wrong_arg_count() {
    let program = r#"
        val m = {"a": 1}
        m.has()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_map_remove_wrong_arg_count() {
    let program = r#"
        val m = {"a": 1}
        m.remove()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}
