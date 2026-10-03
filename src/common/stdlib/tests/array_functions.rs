use crate::common::stdlib::array_functions::{
    native_array_max, native_array_min, native_array_sort, native_array_sum,
};
use crate::common::Value;
use crate::vm::{InterpretResult, VirtualMachine};

// ============================================================================
// Array.push() - Success Cases
// ============================================================================

#[test]
fn test_array_push() {
    let program = r#"
        val arr = [1, 2]
        arr.push(3)
        print(arr)
        arr.push(4)
        print(arr)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2, 3]\n[1, 2, 3, 4]", vm.get_output());
}

#[test]
fn test_array_push_to_empty() {
    let program = r#"
        val arr = []
        arr.push(42)
        print(arr)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[42]", vm.get_output());
}

#[test]
fn test_array_push_different_types() {
    let program = r#"
        val arr = [1]
        arr.push("hello")
        arr.push(true)
        arr.push(nil)
        print(arr)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, hello, true, nil]", vm.get_output());
}

// ============================================================================
// Array.pop() - Success Cases
// ============================================================================

#[test]
fn test_array_pop() {
    let program = r#"
        val arr = [1, 2, 3]
        print(arr.pop())
        print(arr)
        print(arr.pop())
        print(arr)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\n[1, 2]\n2\n[1]", vm.get_output());
}

// ============================================================================
// Array.length() and Array.size() - Success Cases
// ============================================================================

#[test]
fn test_array_length() {
    let program = r#"
        val arr = [1, 2, 3]
        print(arr.length())
        print([].length())
        print(["a", "b"].length())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\n0\n2", vm.get_output());
}

#[test]
fn test_array_size() {
    let program = r#"
        val arr = [1, 2, 3]
        print(arr.size())
        print([].size())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\n0", vm.get_output());
}

// ============================================================================
// Array.contains() - Success Cases
// ============================================================================

#[test]
fn test_array_contains() {
    let program = r#"
        val arr = [1, 2, 3, "hello"]
        print(arr.contains(2))
        print(arr.contains(5))
        print(arr.contains("hello"))
        print(arr.contains("world"))
        print([].contains(1))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
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
        print(nums)

        val strs = ["zebra", "apple", "mango", "banana"]
        strs.sort()
        print(strs)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!(
        "[1, 1, 2, 3, 4, 5, 6, 9]\n[apple, banana, mango, zebra]",
        vm.get_output()
    );
}

#[test]
fn test_array_sort_mixed_types() {
    let program = r#"
        val a = [nil, [1], true]
        a.sort()
        print(a)

        val b = [[1], "a"]
        b.sort()
        print(b)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[1], nil, true]\n[[1], a]", vm.get_output());
}

#[test]
fn test_array_sort_returns_same_array() {
    let program = r#"
        val a = [3, 1, 2]
        val b = a.sort()
        print(b)
        b.push(4)
        print(a)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2, 3]\n[1, 2, 3, 4]", vm.get_output());
}

#[test]
fn test_array_sort_with_comparator() {
    let program = r#"
        print([3, 1, 2].sort(fn(a, b) { return b - a }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[3, 2, 1]", vm.get_output());
}

#[test]
fn test_array_sort_with_comparator_zero_result_keeps_order() {
    let program = r#"
        val pairs = [[1, "a"], [1, "b"], [0, "c"], [1, "d"]]
        val sorted = pairs.sort(fn(a, b) { return 0 })
        print(sorted)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[1, a], [1, b], [0, c], [1, d]]", vm.get_output());
}

#[test]
fn test_array_sort_with_comparator_is_stable() {
    let program = r#"
        val pairs = [[1, "a"], [1, "b"], [0, "c"], [1, "d"]]
        val sorted = pairs.sort(fn(x, y) { return x[0] - y[0] })
        print(sorted)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[0, c], [1, a], [1, b], [1, d]]", vm.get_output());
}

#[test]
fn test_array_sort_comparator_non_number_result() {
    let program = r#"
        [1, 2].sort(fn(a, b) { return "nope" })
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("sort() comparator must return a number, got string"),
        "{}",
        errors
    );
}

#[test]
fn test_array_sort_comparator_error_propagates() {
    let program = r#"
        fn boom(a, b) {
            val x = nil
            return x.missing
        }
        [1, 2].sort(boom)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_sort_wrong_arg_count() {
    let program = r#"
        [1, 2].sort(1, 2)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("sort() expects 0 or 1 arguments, got 2"),
        "{}",
        errors
    );
}

#[test]
fn test_array_sort_ints() {
    let array = Value::new_array(vec![Value::Int(3), Value::Int(1), Value::Int(2)]);
    let mut vm = VirtualMachine::new();
    let result = native_array_sort(&mut vm, &[array]).unwrap();
    let Value::Array(sorted) = result else {
        panic!("expected an array");
    };
    let order: Vec<String> = sorted.borrow().iter().map(|v| v.to_string()).collect();
    assert_eq!(order, vec!["1", "2", "3"]);
}

#[test]
fn test_array_sort_mixed_int_and_number() {
    let array = Value::new_array(vec![Value::Number(2.5), Value::Int(3), Value::Int(1)]);
    let mut vm = VirtualMachine::new();
    let result = native_array_sort(&mut vm, &[array]).unwrap();
    let Value::Array(sorted) = result else {
        panic!("expected an array");
    };
    let order: Vec<String> = sorted.borrow().iter().map(|v| v.to_string()).collect();
    assert_eq!(order, vec!["1", "2.5", "3"]);
}

// ============================================================================
// Array.reverse() - Success Cases
// ============================================================================

#[test]
fn test_array_reverse() {
    let program = r#"
        val arr = [1, 2, 3, 4, 5]
        arr.reverse()
        print(arr)

        val single = [42]
        single.reverse()
        print(single)

        val empty = []
        empty.reverse()
        print(empty)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[5, 4, 3, 2, 1]\n[42]\n[]", vm.get_output());
}

// ============================================================================
// Array.slice() - Success Cases
// ============================================================================

#[test]
fn test_array_slice() {
    let program = r#"
        val arr = [1, 2, 3, 4, 5]
        print(arr.slice(1, 3))
        print(arr.slice(0, 2))
        print(arr.slice(2, 5))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[2, 3]\n[1, 2]\n[3, 4, 5]", vm.get_output());
}

#[test]
fn test_array_slice_negative_indices() {
    let program = r#"
        val arr = [1, 2, 3, 4, 5]
        print(arr.slice(-3, -1))
        print(arr.slice(-2, 5))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[3, 4]\n[4, 5]", vm.get_output());
}

// ============================================================================
// Array.join() - Success Cases
// ============================================================================

#[test]
fn test_array_join() {
    let program = r#"
        val arr = ["hello", "world", "test"]
        print(arr.join(", "))
        print(arr.join(""))
        print(arr.join(" - "))

        val nums = [1, 2, 3]
        print(nums.join(", "))

        print([].join(", "))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!(
        "hello, world, test\nhelloworldtest\nhello - world - test\n1, 2, 3",
        vm.get_output()
    );
}

// ============================================================================
// Array.indexOf() - Success Cases
// ============================================================================

#[test]
fn test_array_index_of() {
    let program = r#"
        val arr = [10, 20, 30, 40, 20]
        print(arr.indexOf(20))
        print(arr.indexOf(40))
        print(arr.indexOf(99))
        print([].indexOf(1))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("1\n3\n-1\n-1", vm.get_output());
}

// ============================================================================
// Array.sum() - Success Cases
// ============================================================================

#[test]
fn test_array_sum() {
    let program = r#"
        val nums = [1, 2, 3, 4, 5]
        print(nums.sum())

        val decimals = [1.5, 2.5, 3.0]
        print(decimals.sum())

        print([].sum())

        print([42].sum())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("15\n7\n0\n42", vm.get_output());
}

#[test]
fn test_array_sum_ints() {
    let array = Value::new_array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
    let result = native_array_sum(&[array]).unwrap();
    assert!(matches!(result, Value::Int(6)));
}

#[test]
fn test_array_sum_exact_large_int() {
    let array = Value::new_array(vec![Value::Int(9007199254740993), Value::Int(1)]);
    let result = native_array_sum(&[array]).unwrap();
    assert!(matches!(result, Value::Int(9007199254740994)));
}

#[test]
fn test_array_sum_mixed_int_and_float() {
    let array = Value::new_array(vec![Value::Int(1), Value::Number(2.5)]);
    let result = native_array_sum(&[array]).unwrap();
    assert!(matches!(result, Value::Number(n) if n == 3.5));
}

#[test]
fn test_array_sum_overflow_order_independent() {
    let forward = Value::new_array(vec![
        Value::Int(i64::MAX),
        Value::Int(1),
        Value::Number(0.5),
    ]);
    let backward = Value::new_array(vec![
        Value::Number(0.5),
        Value::Int(1),
        Value::Int(i64::MAX),
    ]);

    let forward_result = native_array_sum(&[forward]).unwrap();
    let backward_result = native_array_sum(&[backward]).unwrap();

    assert!(matches!(forward_result, Value::Number(_)));
    assert_eq!(forward_result.to_string(), backward_result.to_string());
}

#[test]
fn test_array_sum_int_only_overflow_errors() {
    let array = Value::new_array(vec![Value::Int(i64::MAX), Value::Int(1)]);
    let result = native_array_sum(&[array]);
    assert_eq!(result, Err("integer overflow in sum()".to_string()));
}

// ============================================================================
// Array.min() - Success Cases
// ============================================================================

#[test]
fn test_array_min() {
    let program = r#"
        val nums = [5, 2, 8, 1, 9]
        print(nums.min())

        val negatives = [-5, -2, -10]
        print(negatives.min())

        print([42].min())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("1\n-10\n42", vm.get_output());
}

#[test]
fn test_array_min_mixed_int_and_number() {
    let array = Value::new_array(vec![Value::Number(2.5), Value::Int(1)]);
    let result = native_array_min(&[array]).unwrap();
    assert_eq!(result.to_string(), "1");
}

// ============================================================================
// Array.max() - Success Cases
// ============================================================================

#[test]
fn test_array_max() {
    let program = r#"
        val nums = [5, 2, 8, 1, 9]
        print(nums.max())

        val negatives = [-5, -2, -10]
        print(negatives.max())

        print([42].max())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("9\n-2\n42", vm.get_output());
}

#[test]
fn test_array_max_mixed_int_and_number() {
    let array = Value::new_array(vec![Value::Number(2.5), Value::Int(3)]);
    let result = native_array_max(&[array]).unwrap();
    assert_eq!(result.to_string(), "3");
}

#[test]
fn test_array_max_strings() {
    let program = r#"
        val strs = ["zebra", "apple", "mango"]
        print(strs.max())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
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
        print(arr.length())
        print(arr.contains(2))
        arr.reverse()
        print(arr)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
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
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_push_on_non_array() {
    let program = r#"
        val x = 42
        x.push(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::CompileError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_pop_wrong_arg_count() {
    let program = r#"
        val arr = [1, 2]
        arr.pop(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_pop_on_non_array() {
    let program = r#"
        val x = "not an array"
        x.pop()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::CompileError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_length_wrong_arg_count() {
    let program = r#"
        val arr = [1, 2]
        arr.length(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_length_on_non_array() {
    let program = r#"
        val x = 123
        x.length()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::CompileError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_contains_wrong_arg_count() {
    let program = r#"
        val arr = [1, 2]
        arr.contains()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_contains_wrong_type() {
    let program = r#"
        val x = true
        x.contains(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::CompileError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_sum_non_numeric() {
    let program = r#"
        val arr = [1, "two", 3]
        arr.sum()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_min_empty() {
    let program = r#"
        val arr = []
        arr.min()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_max_empty() {
    let program = r#"
        val arr = []
        arr.max()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

// ============================================================================
// Array.map() / Array.filter() / Array.reduce()
// ============================================================================

#[test]
fn test_array_map_does_not_mutate_receiver() {
    let program = r#"
        val arr = [1, 2, 3]
        val doubled = arr.map(fn(x) { return x * 2 })
        print(arr)
        print(doubled)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2, 3]\n[2, 4, 6]", vm.get_output());
}

#[test]
fn test_array_map_wrong_arg_count() {
    let program = r#"
        val arr = [1, 2, 3]
        arr.map()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_map_non_callable() {
    let program = r#"
        val arr = [1, 2, 3]
        arr.map(5)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_filter_keeps_truthy_non_boolean_results() {
    let program = r#"
        print([1, 2, 3].filter(fn(x) { return x }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2, 3]", vm.get_output());
}

#[test]
fn test_array_filter_wrong_arg_count() {
    let program = r#"
        val arr = [1, 2, 3]
        arr.filter()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_reduce_with_closure() {
    let program = r#"
        val factor = 10
        val result = [1, 2, 3].reduce(fn(acc, x) { return acc + x * factor }, 0)
        print(result)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("60", vm.get_output());
}

#[test]
fn test_array_reduce_wrong_arg_count() {
    let program = r#"
        fn add(acc, x) { return acc + x }
        val arr = [1, 2, 3]
        arr.reduce(add)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

// ============================================================================
// Array.find(fn)
// ============================================================================

#[test]
fn test_array_find_match() {
    let program = r#"
        print([1, 2, 3].find(fn(x) { return x > 1 }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("2", vm.get_output());
}

#[test]
fn test_array_find_no_match() {
    let program = r#"
        print([1, 2, 3].find(fn(x) { return x > 10 }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("nil", vm.get_output());
}

#[test]
fn test_array_find_stops_at_first_match() {
    let program = r#"
        val calls = []
        [1, 2, 3].find(fn(x) {
            calls.push(x)
            return x >= 2
        })
        print(calls)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2]", vm.get_output());
}

#[test]
fn test_array_find_wrong_arg_count() {
    let program = r#"
        [1, 2, 3].find()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

// ============================================================================
// Array.some(fn) / Array.every(fn)
// ============================================================================

#[test]
fn test_array_some_true() {
    let program = r#"
        print([1, 2, 3].some(fn(x) { return x > 2 }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_array_some_false() {
    let program = r#"
        print([1, 2, 3].some(fn(x) { return x > 10 }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_array_some_empty_is_false() {
    let program = r#"
        print([].some(fn(x) { return true }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_array_some_stops_at_first_match() {
    let program = r#"
        val calls = []
        [1, 2, 3].some(fn(x) {
            calls.push(x)
            return x >= 2
        })
        print(calls)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2]", vm.get_output());
}

#[test]
fn test_array_every_true() {
    let program = r#"
        print([1, 2, 3].every(fn(x) { return x > 0 }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_array_every_false() {
    let program = r#"
        print([1, 2, 3].every(fn(x) { return x > 1 }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_array_every_empty_is_true() {
    let program = r#"
        print([].every(fn(x) { return false }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_array_every_stops_at_first_failure() {
    let program = r#"
        val calls = []
        [1, 2, 3].every(fn(x) {
            calls.push(x)
            return x < 2
        })
        print(calls)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2]", vm.get_output());
}

// ============================================================================
// Array.flat() / Array.copy()
// ============================================================================

#[test]
fn test_array_flat_one_level() {
    let program = r#"
        print([[1, 2], 3, [4, [5]]].flat())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2, 3, 4, [5]]", vm.get_output());
}

#[test]
fn test_array_flat_wrong_arg_count() {
    let program = r#"
        [1, 2].flat(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_copy_independent_of_original() {
    let program = r#"
        val a = [1, 2, 3]
        val b = a.copy()
        b.push(4)
        print(a)
        print(b)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2, 3]\n[1, 2, 3, 4]", vm.get_output());
}

#[test]
fn test_array_copy_shares_inner_arrays() {
    let program = r#"
        val a = [[1, 2]]
        val b = a.copy()
        b[0].push(3)
        print(a)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[1, 2, 3]]", vm.get_output());
}

// ============================================================================
// Array(n, init) constructor
// ============================================================================

#[test]
fn test_array_constructor_fills_with_value() {
    let program = r#"
        print(Array(3, 0))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[0, 0, 0]", vm.get_output());
}

#[test]
fn test_array_constructor_zero_length() {
    let program = r#"
        print(Array(0, 1))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[]", vm.get_output());
}

#[test]
fn test_array_constructor_negative_length_errors() {
    let program = r#"
        Array(-1, 0)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(errors.contains("Array()"), "{}", errors);
}

#[test]
fn test_array_constructor_non_integer_length_errors() {
    let program = r#"
        Array(1.5, 0)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(errors.contains("Array()"), "{}", errors);
}

#[test]
fn test_array_constructor_absurd_length_errors() {
    let program = r#"
        Array(1e15, 0)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(errors.contains("Array()"), "{}", errors);
}

#[test]
fn test_array_constructor_calls_init_with_index() {
    let program = r#"
        print(Array(3, fn(i) { return i * i }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[0, 1, 4]", vm.get_output());
}

#[test]
fn test_array_constructor_init_not_shared_across_elements() {
    let program = r#"
        val g = Array(2, fn(y) { return Array(2, 0) })
        g[0][0] = 1
        print(g)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[1, 0], [0, 0]]", vm.get_output());
}

#[test]
fn test_array_constructor_non_function_init_is_shared() {
    let program = r#"
        val s = Array(2, [])
        s[0].push(1)
        print(s)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[1], [1]]", vm.get_output());
}

#[test]
fn test_array_constructor_init_error_propagates() {
    let program = r#"
        Array(3, fn(i) {
            if (i == 1) {
                return Math.div(1, 0)
            }
            return i
        })
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_constructor_local_shadows_native() {
    let program = r#"
        fn f() {
            val Array = fn(n, v) { return "mine " + n.toString() }
            return Array(3, 0)
        }
        print(f())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("mine 3", vm.get_output());
}
