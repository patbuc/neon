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
// Array.size() - Success Cases
// ============================================================================

#[test]
fn test_array_size() {
    let program = r#"
        val arr = [1, 2, 3]
        print(arr.size())
        print([].size())
        print(["a", "b"].size())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("3\n0\n2", vm.get_output());
}

#[test]
fn test_array_is_empty() {
    let program = r#"
        print([].isEmpty())
        print([1].isEmpty())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("true\nfalse", vm.get_output());
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

// ============================================================================
// Array.sortBy() - Success Cases
// ============================================================================

#[test]
fn test_array_sort_by() {
    let program = r#"
        val words = ["bb", "a", "ccc"]
        val sorted = words.sortBy(fn(s) { return s.size() })
        print(sorted)
        print(words)

        val ties = ["cc", "bb", "a"]
        print(ties.sortBy(fn(s) { return s.size() }))

        print([3, 1, 2].sortBy(fn(x) { return -x }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!(
        "[a, bb, ccc]\n[bb, a, ccc]\n[a, cc, bb]\n[3, 2, 1]",
        vm.get_output()
    );
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
fn test_array_sort_by_mixed_key_types_errors() {
    let program = r#"
        [1, "a"].sortBy(fn(x) { return x })
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("sortBy() keys must be all numbers or all strings"),
        "{}",
        errors
    );
}

// ============================================================================
// Array.minBy() / Array.maxBy() - Success Cases
// ============================================================================

#[test]
fn test_array_min_by_and_max_by() {
    let program = r#"
        val words = ["bb", "a", "ccc"]
        print(words.minBy(fn(s) { return s.size() }))
        print(words.maxBy(fn(s) { return s.size() }))

        val minTies = ["bb", "cc", "ddd"]
        print(minTies.minBy(fn(s) { return s.size() }))

        val maxTies = ["aa", "b", "cc"]
        print(maxTies.maxBy(fn(s) { return s.size() }))

        print([].minBy(fn(s) { return s.size() }))
        print([].maxBy(fn(s) { return s.size() }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("a\nccc\nbb\naa\nnil\nnil", vm.get_output());
}

#[test]
fn test_array_min_by_mixed_key_types_errors() {
    let program = r#"
        [1, "a"].minBy(fn(x) { return x })
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("minBy() keys must be all numbers or all strings"),
        "{}",
        errors
    );
}

#[test]
fn test_array_max_by_mixed_key_types_errors() {
    let program = r#"
        [1, "a"].maxBy(fn(x) { return x })
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("maxBy() keys must be all numbers or all strings"),
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
        print(arr.size())
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
fn test_array_size_wrong_arg_count() {
    let program = r#"
        val arr = [1, 2]
        arr.size(1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_size_on_non_array() {
    let program = r#"
        val x = 123
        x.size()
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

// ============================================================================
// Array.forEach(fn) / Array.flatMap(fn)
// ============================================================================

#[test]
fn test_array_for_each_prints_each_and_returns_nil() {
    let program = r#"
        print([1, 2, 3].forEach(fn(x) { print(x) }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("1\n2\n3\nnil", vm.get_output());
}

#[test]
fn test_array_flat_map() {
    let program = r#"
        print([1, 2].flatMap(fn(x) { return [x, x] }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 1, 2, 2]", vm.get_output());
}

#[test]
fn test_array_flat_map_non_array_return_errors() {
    let program = r#"
        [1, 2].flatMap(fn(x) { return 3 })
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("flatMap() callback must return an array, got Int"),
        "{}",
        errors
    );
}

#[test]
fn test_array_for_each_callback_mutating_receiver_iterates_snapshot() {
    let program = r#"
        val arr = [1, 2, 3]
        arr.forEach(fn(x) { arr.push(x) })
        print(arr)
        print(arr.size())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2, 3, 1, 2, 3]\n6", vm.get_output());
}

#[test]
fn test_array_flat_map_callback_mutating_receiver_iterates_snapshot() {
    let program = r#"
        val arr = [1, 2, 3]
        val result = arr.flatMap(fn(x) {
            arr.push(x)
            return [x]
        })
        print(result)
        print(arr.size())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2, 3]\n6", vm.get_output());
}

#[test]
fn test_array_for_each_callback_error_propagates() {
    let program = r#"
        [1, 2].forEach(fn(x) {
            val y = nil
            return y.missing
        })
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

#[test]
fn test_array_flat_map_callback_error_propagates() {
    let program = r#"
        [1, 2].flatMap(fn(x) {
            val y = nil
            return y.missing
        })
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
}

// ============================================================================
// Array.take() / Array.drop()
// ============================================================================

#[test]
fn test_array_take_and_drop() {
    let program = r#"
        print([1, 2, 3].take(2))
        print([1, 2, 3].take(9))
        print([1, 2, 3].drop(1))
        print([1, 2, 3].drop(9))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2]\n[1, 2, 3]\n[2, 3]\n[]", vm.get_output());
}

#[test]
fn test_array_take_negative_errors() {
    let program = r#"
        [1, 2, 3].take(-1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("take() n must be non-negative, got -1"),
        "{}",
        errors
    );
}

#[test]
fn test_array_drop_negative_errors() {
    let program = r#"
        [1, 2, 3].drop(-1)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("drop() n must be non-negative, got -1"),
        "{}",
        errors
    );
}

#[test]
fn test_array_take_non_integer_errors() {
    let program = r#"
        [1, 2, 3].take(1.5)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("take() n must be an integer, got 1.5"),
        "{}",
        errors
    );
}

#[test]
fn test_array_take_drop_does_not_mutate_receiver() {
    let program = r#"
        val arr = [1, 2, 3]
        arr.take(2)
        arr.drop(1)
        print(arr)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2, 3]", vm.get_output());
}

// ============================================================================
// Array.first() / Array.last()
// ============================================================================

#[test]
fn test_array_first_and_last() {
    let program = r#"
        print([].first())
        print([].last())
        print([1, 2].first())
        print([1, 2].last())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("nil\nnil\n1\n2", vm.get_output());
}

// ============================================================================
// Array.chunked()
// ============================================================================

#[test]
fn test_array_chunked() {
    let program = r#"
        print([1, 2, 3, 4, 5].chunked(2))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[1, 2], [3, 4], [5]]", vm.get_output());
}

#[test]
fn test_array_chunked_non_integer_errors() {
    let program = r#"
        [1, 2, 3].chunked("x")
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("chunked() n must be an integer"),
        "{}",
        errors
    );
}

#[test]
fn test_array_chunked_zero_errors() {
    let program = r#"
        [1, 2, 3].chunked(0)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("chunked() n must be >= 1, got 0"),
        "{}",
        errors
    );
}

// ============================================================================
// Array.zip() / Array.withIndex()
// ============================================================================

#[test]
fn test_array_zip() {
    let program = r#"
        print([1, 2, 3].zip(["a", "b"]))
        print((1..3).zip(1..3))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[1, a], [2, b]]\n[[1, 1], [2, 2]]", vm.get_output());
}

#[test]
fn test_array_zip_range_other() {
    let program = r#"
        print([1, 2, 3].zip(10..20))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[1, 10], [2, 11], [3, 12]]", vm.get_output());
}

#[test]
fn test_array_zip_huge_range_other_does_not_materialize() {
    let program = r#"
        print([1, 2].zip(0..10000000000))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[1, 0], [2, 1]]", vm.get_output());
}

#[test]
fn test_array_zip_other_wrong_type_errors() {
    let program = r#"
        [1, 2, 3].zip(5)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("zip() other must be an array or range, got Int"),
        "{}",
        errors
    );
}

#[test]
fn test_array_with_index() {
    let program = r#"
        print(["a", "b"].withIndex())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[[0, a], [1, b]]", vm.get_output());
}

// ============================================================================
// Array.groupBy() / Array.tally()
// ============================================================================

#[test]
fn test_array_group_by() {
    let program = r#"
        print([1, 2, 3, 4].groupBy(fn(x) { return x % 2 }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("{1: [1, 3], 0: [2, 4]}", vm.get_output());
}

#[test]
fn test_array_tally() {
    let program = r#"
        print(["a", "b", "a"].tally())
        print([[1], [1]].tally())
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("{a: 2, b: 1}\n{[1]: 2}", vm.get_output());
}

#[test]
fn test_array_tally_invalid_key_errors() {
    let program = r#"
        [{}].tally()
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("Invalid map key type: {}. Only strings, numbers, booleans, arrays, and enum variants can be used as map keys."),
        "{}",
        errors
    );
}

#[test]
fn test_array_sort_nan_last() {
    let program = r#"
        val nums = [3, 0.0 / 0.0, 1]
        nums.sort()
        print(nums)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 3, NaN]", vm.get_output());
}

#[test]
fn test_array_sort_by_nan_keys_do_not_crash() {
    let program = r#"
        var arr = []
        for i in 0..60 {
            if (i % 7 == 0) {
                arr.push(0.0 / 0.0)
            } else {
                arr.push((60 - i).toFloat())
            }
        }
        val sorted = arr.sortBy(fn(x) { return x })
        print(sorted)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));

    let output = vm.get_output();
    let values: Vec<&str> = output
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(", ")
        .collect();
    assert_eq!(60, values.len(), "{}", output);

    let nan_count = values.iter().filter(|v| **v == "NaN").count();
    assert_eq!(9, nan_count, "{}", output);

    let non_nan: Vec<f64> = values[..60 - nan_count]
        .iter()
        .map(|v| v.parse::<f64>().expect("non-NaN values should parse"))
        .collect();
    let mut sorted_non_nan = non_nan.clone();
    sorted_non_nan.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(sorted_non_nan, non_nan, "{}", output);

    for v in &values[60 - nan_count..] {
        assert_eq!("NaN", *v, "{}", output);
    }
}

#[test]
fn test_array_sort_by_stable_with_many_elements() {
    let program = r#"
        print((0..50).sortBy(fn(x) { return x % 2 }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));

    let output = vm.get_output();
    let values: Vec<i64> = output
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(", ")
        .map(|v| v.parse::<i64>().expect("value should parse"))
        .collect();
    assert_eq!(50, values.len());

    let evens: Vec<i64> = values[..25].to_vec();
    let odds: Vec<i64> = values[25..].to_vec();
    assert_eq!(
        (0..50).step_by(2).collect::<Vec<i64>>(),
        evens,
        "{}",
        output
    );
    assert_eq!((1..50).step_by(2).collect::<Vec<i64>>(), odds, "{}", output);
}

#[test]
fn test_array_sort_by_calls_key_fn_once_per_element() {
    let program = r#"
        var calls = 0
        print([3, 1, 2].sortBy(fn(x) {
            calls += 1
            return x
        }))
        print(calls)
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2, 3]\n3", vm.get_output());
}

#[test]
fn test_array_sort_by_key_fn_error_propagates() {
    let program = r#"
        [1, 2].sortBy(fn(x) { return [][0] })
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(errors.contains("index out of bounds"), "{}", errors);
}

#[test]
fn test_array_min_by_key_fn_error_propagates() {
    let program = r#"
        [1, 2].minBy(fn(x) { return [][0] })
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(errors.contains("index out of bounds"), "{}", errors);
}

#[test]
fn test_array_group_by_key_fn_error_propagates() {
    let program = r#"
        [1, 2].groupBy(fn(x) { return [][0] })
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(errors.contains("index out of bounds"), "{}", errors);
}

#[test]
fn test_array_sort_by_mixed_int_and_float_keys() {
    let program = r#"
        print([1, 2.5, 2].sortBy(fn(x) { return x }))
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(InterpretResult::Ok, vm.interpret(program.to_string()));
    assert_eq!("[1, 2, 2.5]", vm.get_output());
}

#[test]
fn test_array_group_by_invalid_key_errors() {
    let program = r#"
        [1].groupBy(fn(x) { return {} })
    "#;

    let mut vm = VirtualMachine::new();
    assert_eq!(
        InterpretResult::RuntimeError,
        vm.interpret(program.to_string())
    );
    let errors = vm.get_runtime_errors();
    assert!(
        errors.contains("Invalid map key type: {}. Only strings, numbers, booleans, arrays, and enum variants can be used as map keys."),
        "{}",
        errors
    );
}
