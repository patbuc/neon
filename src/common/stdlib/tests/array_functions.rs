use crate::vm::{Result, VirtualMachine};

// ============================================================================
// Array.push() - Success Cases
// ============================================================================

#[test]
fn test_array_push() {
    let program = r#"
        val arr = [1, 2]
        arr.push(3)
        print arr
        arr.push(4)
        print arr
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2, 3]\n[1, 2, 3, 4]", vm.get_output());
}

#[test]
fn test_array_push_to_empty() {
    let program = r#"
        val arr = []
        arr.push(42)
        print arr
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("[42]", vm.get_output());
}

#[test]
fn test_array_push_different_types() {
    let program = r#"
        val arr = [1]
        arr.push("hello")
        arr.push(true)
        arr.push(nil)
        print arr
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, hello, true, nil]", vm.get_output());
}

// ============================================================================
// Array.pop() - Success Cases
// ============================================================================

#[test]
fn test_array_pop() {
    let program = r#"
        val arr = [1, 2, 3]
        print arr.pop()
        print arr
        print arr.pop()
        print arr
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\n[1, 2]\n2\n[1]", vm.get_output());
}

// ============================================================================
// Array.length() and Array.size() - Success Cases
// ============================================================================

#[test]
fn test_array_length() {
    let program = r#"
        val arr = [1, 2, 3]
        print arr.length()
        print [].length()
        print ["a", "b"].length()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\n0\n2", vm.get_output());
}

#[test]
fn test_array_size() {
    let program = r#"
        val arr = [1, 2, 3]
        print arr.size()
        print [].size()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\n0", vm.get_output());
}

// ============================================================================
// Array.contains() - Success Cases
// ============================================================================

#[test]
fn test_array_contains() {
    let program = r#"
        val arr = [1, 2, 3, "hello"]
        print arr.contains(2)
        print arr.contains(5)
        print arr.contains("hello")
        print arr.contains("world")
        print [].contains(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\nfalse\ntrue\nfalse\nfalse", vm.get_output());
}

// ============================================================================
// Array.sort() - Success Cases
// ============================================================================

#[test]
fn test_array_sort() {
    let program = r#"
        val nums = [3, 1, 4, 1, 5, 9, 2, 6]
        nums.sort()
        print nums

        val strs = ["zebra", "apple", "mango", "banana"]
        strs.sort()
        print strs
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 1, 2, 3, 4, 5, 6, 9]\n[apple, banana, mango, zebra]", vm.get_output());
}

// ============================================================================
// Array.reverse() - Success Cases
// ============================================================================

#[test]
fn test_array_reverse() {
    let program = r#"
        val arr = [1, 2, 3, 4, 5]
        arr.reverse()
        print arr

        val single = [42]
        single.reverse()
        print single

        val empty = []
        empty.reverse()
        print empty
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("[5, 4, 3, 2, 1]\n[42]\n[]", vm.get_output());
}

// ============================================================================
// Array.slice() - Success Cases
// ============================================================================

#[test]
fn test_array_slice() {
    let program = r#"
        val arr = [1, 2, 3, 4, 5]
        print arr.slice(1, 3)
        print arr.slice(0, 2)
        print arr.slice(2, 5)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("[2, 3]\n[1, 2]\n[3, 4, 5]", vm.get_output());
}

#[test]
fn test_array_slice_negative_indices() {
    let program = r#"
        val arr = [1, 2, 3, 4, 5]
        print arr.slice(-3, -1)
        print arr.slice(-2, 5)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("[3, 4]\n[4, 5]", vm.get_output());
}

// ============================================================================
// Array.join() - Success Cases
// ============================================================================

#[test]
fn test_array_join() {
    let program = r#"
        val arr = ["hello", "world", "test"]
        print arr.join(", ")
        print arr.join("")
        print arr.join(" - ")

        val nums = [1, 2, 3]
        print nums.join(", ")

        print [].join(", ")
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("hello, world, test\nhelloworldtest\nhello - world - test\n1, 2, 3", vm.get_output());
}

// ============================================================================
// Array.indexOf() - Success Cases
// ============================================================================

#[test]
fn test_array_index_of() {
    let program = r#"
        val arr = [10, 20, 30, 40, 20]
        print arr.indexOf(20)
        print arr.indexOf(40)
        print arr.indexOf(99)
        print [].indexOf(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("1\n3\n-1\n-1", vm.get_output());
}

// ============================================================================
// Array.sum() - Success Cases
// ============================================================================

#[test]
fn test_array_sum() {
    let program = r#"
        val nums = [1, 2, 3, 4, 5]
        print nums.sum()

        val decimals = [1.5, 2.5, 3.0]
        print decimals.sum()

        print [].sum()

        print [42].sum()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("15\n7\n0\n42", vm.get_output());
}

// ============================================================================
// Array.min() - Success Cases
// ============================================================================

#[test]
fn test_array_min() {
    let program = r#"
        val nums = [5, 2, 8, 1, 9]
        print nums.min()

        val negatives = [-5, -2, -10]
        print negatives.min()

        print [42].min()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("1\n-10\n42", vm.get_output());
}

// ============================================================================
// Array.max() - Success Cases
// ============================================================================

#[test]
fn test_array_max() {
    let program = r#"
        val nums = [5, 2, 8, 1, 9]
        print nums.max()

        val negatives = [-5, -2, -10]
        print negatives.max()

        print [42].max()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("9\n-2\n42", vm.get_output());
}

#[test]
fn test_array_max_strings() {
    let program = r#"
        val strs = ["zebra", "apple", "mango"]
        print strs.max()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("zebra", vm.get_output());
}

// ============================================================================
// Array Operations - Combined Tests
// ============================================================================

#[test]
fn test_array_operations_sequence() {
    let program = r#"
        val arr = []
        arr.push(1)
        arr.push(2)
        arr.push(3)
        print arr.length()
        print arr.contains(2)
        arr.reverse()
        print arr
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\ntrue\n[3, 2, 1]", vm.get_output());
}

// ============================================================================
// Array Functions - Error Cases
// ============================================================================

#[test]
fn test_array_push_wrong_arg_count() {
    let program = r#"
        val arr = []
        arr.push()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_array_push_on_non_array() {
    let program = r#"
        val x = 42
        x.push(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::CompileError, vm.interpret(program.to_string()));
}

#[test]
fn test_array_pop_wrong_arg_count() {
    let program = r#"
        val arr = [1, 2]
        arr.pop(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_array_pop_on_non_array() {
    let program = r#"
        val x = "not an array"
        x.pop()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::CompileError, vm.interpret(program.to_string()));
}

#[test]
fn test_array_length_wrong_arg_count() {
    let program = r#"
        val arr = [1, 2]
        arr.length(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_array_length_on_non_array() {
    let program = r#"
        val x = 123
        x.length()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::CompileError, vm.interpret(program.to_string()));
}

#[test]
fn test_array_contains_wrong_arg_count() {
    let program = r#"
        val arr = [1, 2]
        arr.contains()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_array_contains_wrong_type() {
    let program = r#"
        val x = true
        x.contains(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::CompileError, vm.interpret(program.to_string()));
}

#[test]
fn test_array_sum_non_numeric() {
    let program = r#"
        val arr = [1, "two", 3]
        arr.sum()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_array_min_empty() {
    let program = r#"
        val arr = []
        arr.min()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}

#[test]
fn test_array_max_empty() {
    let program = r#"
        val arr = []
        arr.max()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(Result::RuntimeError, vm.interpret(program.to_string()));
}
