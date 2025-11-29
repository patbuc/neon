use crate::vm::{Result, VirtualMachine};

#[test]
fn can_create_empty_array() {
    let program = r#"
        val arr = []
        print arr
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("[]", vm.get_output());
}

#[test]
fn can_create_array_with_elements() {
    let program = r#"
        val arr = [1, 2, 3]
        print arr
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("[1, 2, 3]", vm.get_output());
}

#[test]
fn can_access_array_element() {
    let program = r#"
        val arr = [10, 20, 30]
        print arr[0]
        print arr[1]
        print arr[2]
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("10\n20\n30", vm.get_output());
}

#[test]
fn can_set_array_element() {
    let program = r#"
        val arr = [1, 2, 3]
        arr[1] = 42
        print arr
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("[1, 42, 3]", vm.get_output());
}

#[test]
fn can_create_nested_arrays() {
    let program = r#"
        val arr = [[1, 2], [3, 4]]
        print arr
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("[[1, 2], [3, 4]]", vm.get_output());
}

#[test]
fn can_access_nested_array_element() {
    let program = r#"
        val arr = [[1, 2], [3, 4]]
        print arr[0][1]
        print arr[1][0]
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("2\n3", vm.get_output());
}

#[test]
fn can_modify_nested_array_element() {
    let program = r#"
        val arr = [[1, 2], [3, 4]]
        arr[0][1] = 99
        print arr
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("[[1, 99], [3, 4]]", vm.get_output());
}

#[test]
fn array_assignment_returns_value() {
    let program = r#"
        val arr = [1, 2, 3]
        print arr[0] = 42
        print arr
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("42\n[42, 2, 3]", vm.get_output());
}

#[test]
fn arrays_with_mixed_types() {
    let program = r#"
        val arr = [1, "hello", true, nil]
        print arr
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!(r#"[1, hello, true, nil]"#, vm.get_output());
}

#[test]
fn error_on_out_of_bounds_access() {
    let program = r#"
        val arr = [1, 2, 3]
        print arr[10]
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn error_on_out_of_bounds_assignment() {
    let program = r#"
        val arr = [1, 2, 3]
        arr[10] = 42
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn error_on_negative_index() {
    let program = r#"
        val arr = [1, 2, 3]
        print arr[-1]
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn error_on_non_integer_index() {
    let program = r#"
        val arr = [1, 2, 3]
        print arr[1.5]
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn error_on_indexing_non_array() {
    let program = r#"
        val x = 42
        print x[0]
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn error_on_non_number_index() {
    let program = r#"
        val arr = [1, 2, 3]
        print arr["hello"]
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::RuntimeError, result);
}

#[test]
fn can_use_array_in_expression() {
    let program = r#"
        val arr = [1, 2, 3]
        print arr[0] + arr[1] + arr[2]
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("6", vm.get_output());
}

#[test]
fn can_use_variable_as_index() {
    let program = r#"
        val arr = [10, 20, 30]
        val i = 1
        print arr[i]
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("20", vm.get_output());
}

#[test]
fn can_use_expression_as_index() {
    let program = r#"
        val arr = [10, 20, 30]
        print arr[1 + 1]
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("30", vm.get_output());
}

#[test]
fn can_pass_array_as_function_parameter() {
    let program = r#"
        fn get_first(arr) {
            return arr[0]
        }

        val numbers = [42, 100, 200]
        print get_first(numbers)
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("42", vm.get_output());
}

#[test]
fn can_return_array_from_function() {
    let program = r#"
        fn make_array() {
            return [1, 2, 3]
        }

        val arr = make_array()
        print arr
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("[1, 2, 3]", vm.get_output());
}

#[test]
fn can_modify_array_in_function() {
    let program = r#"
        fn modify_array(arr) {
            arr[0] = 999
        }

        val numbers = [1, 2, 3]
        modify_array(numbers)
        print numbers
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("[999, 2, 3]", vm.get_output());
}

#[test]
fn can_pass_and_return_nested_arrays() {
    let program = r#"
        fn get_inner(matrix, i) {
            return matrix[i]
        }

        val matrix = [[1, 2], [3, 4], [5, 6]]
        val row = get_inner(matrix, 1)
        print row
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("[3, 4]", vm.get_output());
}

#[test]
fn can_use_arrays_in_recursive_function() {
    let program = r#"
        fn sum_array(arr, index, len) {
            if (index >= len) {
                return 0
            }
            return arr[index] + sum_array(arr, index + 1, len)
        }

        val numbers = [1, 2, 3, 4, 5]
        print sum_array(numbers, 0, 5)
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("15", vm.get_output());
}

#[test]
fn can_build_array_in_loop() {
    let program = r#"
        val arr = [0, 0, 0]
        var i = 0
        while (i < 3) {
            arr[i] = i * 10
            i = i + 1
        }
        print arr
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("[0, 10, 20]", vm.get_output());
}

#[test]
fn can_use_array_in_conditional() {
    let program = r#"
        val arr = [10, 20, 30]
        if (arr[1] > 15) {
            print "yes"
        } else {
            print "no"
        }
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("yes", vm.get_output());
}
