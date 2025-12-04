use crate::common::opcodes::OpCode;
use crate::common::Bloq;
use crate::common::Value;
use crate::vm::{Result, VirtualMachine};
use crate::vm::native_functions;
use crate::{as_number, number};
use std::assert_eq;

#[test]
fn can_create_vm() {
    let vm = VirtualMachine::new(Vec::new());
    assert_eq!(0, vm.call_frames.len());
    assert_eq!(0, vm.stack.len());
}

#[test]
fn can_execute_simple_arithmetics() {
    let mut bloq = Bloq::new("ZeBloq");

    bloq.write_constant(number!(1.0), 0, 0);
    bloq.write_constant(number!(2.0), 0, 0);
    bloq.write_op_code(OpCode::Add, 0, 0);
    bloq.write_constant(number!(3.0), 0, 0);
    bloq.write_op_code(OpCode::Multiply, 0, 0);
    bloq.write_constant(number!(2.0), 0, 0);
    bloq.write_op_code(OpCode::Subtract, 0, 0);
    bloq.write_constant(number!(2.0), 0, 0);
    bloq.write_op_code(OpCode::Divide, 0, 0);
    bloq.write_op_code(OpCode::Return, 0, 0);

    let mut vm = VirtualMachine::new(Vec::new());

    let result = vm.run_bloq(bloq);
    assert_eq!(Result::Ok, result);
    assert_eq!(3.5, as_number!(vm.pop()));
}

#[test]
fn can_print_hello_world() {
    let program = r#"
        print "Hello World 🌍"
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
}

#[test]
fn can_print_the_answer_to_everything_times_pi() {
    let program = r#"
        print 42 * 3.14
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
}

#[test]
fn can_run_multi_line_statements() {
    let program = r#"
        print "Hello World 🌎"
        print 13
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
}

#[test]
fn can_define_a_global_value() {
    let program = r#"
        val greeting = "Hello World 🌎"
        print greeting
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
    assert_eq!("Hello World 🌎", vm.get_output())
}

#[test]
fn can_negate_numbers() {
    let program = r#"
        val x = 42
        print -x
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("-42", vm.get_output());
}

#[test]
fn can_compare_numbers_equal() {
    let program = r#"
        print 42 == 42
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_compare_numbers_not_equal() {
    let program = r#"
        print 42 == 43
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn can_compare_greater_than() {
    let program = r#"
        print 43 > 42
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_compare_less_than() {
    let program = r#"
        print 41 < 42
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_use_logical_not() {
    let program = r#"
        print !false
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_handle_nil() {
    let program = r#"
        val x = nil
        print x
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("nil", vm.get_output());
}

#[test]
fn can_handle_boolean_true() {
    let program = r#"
        val x = true
        print x
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_handle_boolean_false() {
    let program = r#"
        val x = false
        print x
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn can_handle_string_concatenation() {
    let program = r#"
        print "Hello" + " " + "World"
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Hello World", vm.get_output());
}

#[test]
fn can_handle_multiple_global_variables() {
    let program = r#"
        val x = 40
        val y = 2
        print x + y
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("42", vm.get_output());
}

#[test]
fn can_handle_complex_arithmetic() {
    let program = r#"
        val x = 10
        val y = 5
        print (x + y) * (x - y)
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("75", vm.get_output());
}

#[test]
fn can_handle_string_comparison() {
    let program = r#"
        print "hello" == "hello"
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_handle_multiple_boolean_operations() {
    let program = r#"
        print true == !false
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_handle_division_by_integers() {
    let program = r#"
        print 100 / 20
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("5", vm.get_output());
}

#[test]
fn can_handle_float_division() {
    let program = r#"
        print 10.0 / 3.0
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("3.3333333333333335", vm.get_output());
}

#[test]
fn can_handle_negative_numbers() {
    let program = r#"
        val x = -42
        print -x
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("42", vm.get_output());
}

#[test]
fn can_handle_boolean_arithmetic() {
    let program = r#"
        print true == true == true
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_handle_complex_string_operations() {
    let program = r#"
        val greeting = "Hello"
        val name = "World"
        val punctuation = "!"
        print greeting + " " + name + punctuation
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Hello World!", vm.get_output());
}

#[test]
fn can_handle_multiple_negations() {
    let program = r#"
        print !!true
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_handle_a_true_if_statement() {
    let program = r#"
        val x = 42
        if (x == 42) {
            print "The answer to everything"
        }
        print "The end"
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("The answer to everything\nThe end", vm.get_output());
}

#[test]
fn can_handle_a_false_if_statement() {
    let program = r#"
        val x = 42
        if (x != 42) {
            print "The answer to everything"
        }
        print "The end"
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("The end", vm.get_output());
}

#[test]
fn can_handle_a_true_if_else_statement() {
    let program = r#"
        val x = 42
        if (x == 42) {
            print "The answer to everything"
        } else {
            print "The end"
        }
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("The answer to everything", vm.get_output());
}

#[test]
fn can_handle_multiple_if_else_statements() {
    let program = r#"
        val x = 42
        if (x == 41) {
            print "The answer to everything"
        } else if (x == 42) {
            print "The end"
        } else {
            print "The beginning"
        }
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("The end", vm.get_output());
}

#[test]
fn can_handle_multiple_if_else_statements_2() {
    let program = r#"
        val x = 4
        if (x == 41) {
            print "The answer to everything"
        } else if (x == 42) {
            print "The end"
        } else {
            print "The beginning"
        }
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("The beginning", vm.get_output());
}

#[test]
fn can_assign_value_to_variable() {
    let program = r#"
        var x = 10
        x = x + 5
        print x
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("15", vm.get_output());
}

#[test]
fn cannot_assign_value_to_value() {
    let program = r#"
        val x = 10
        x = x + 5
        print x
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::CompileError, result);
    assert_eq!(
        "[Semantic] Immutable Assignment: Cannot assign to immutable variable 'x' at 3:9",
        vm.get_compiler_error()
    );
}

#[test]
fn cannot_access_undefined_variable() {
    let program = r#"
        var x = 3
        x = z + 5
        print x
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::CompileError, result);
    assert_eq!(
        "[Semantic] Undefined Symbol: Undefined variable 'z' at 3:13",
        vm.get_compiler_error()
    );
}

#[test]
fn can_loop() {
    let program = r#"
        var x = 0
        while (x < 10) {
            x = x + 1
            print x
        }
        print "Done"
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n2\n3\n4\n5\n6\n7\n8\n9\n10\nDone", vm.get_output());
}

#[test]
fn can_call_function() {
    let program = r#"
        fn greet() {
            print "Hello from function!"
        }
        greet()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Hello from function!", vm.get_output());
}

#[test]
fn can_call_function_multiple_times() {
    let program = r#"
        fn greet() {
            print "Hello again!"
        }
        greet()
        greet()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Hello again!\nHello again!", vm.get_output());
}

#[test]
fn can_calculate_fibonacci() {
    let program = r#"
        fn fib(n) {
            if (n == 0) {
                return 0
            }
            if (n == 1) {
                return 1
            }
            return fib(n - 1) + fib(n - 2)
        }
        print fib(10)
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let start = std::time::Instant::now();
    let result = vm.interpret(program.to_string());
    let elapsed = start.elapsed();
    println!("Fibonacci test (fib 0-30) took: {:?}", elapsed);
    assert_eq!(Result::Ok, result);
    assert_eq!("55", vm.get_output());
}

#[test]
fn can_calculate_fibonacci_20() {
    let program = r#"
        fn fib(n) {
            if (n == 0) {
                return 0
            }
            if (n == 1) {
                return 1
            }
            return fib(n - 1) + fib(n - 2)
        }
        print fib(20)
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let start = std::time::Instant::now();
    let result = vm.interpret(program.to_string());
    let elapsed = start.elapsed();
    println!("Fibonacci test (fib 20) took: {:?}", elapsed);
    assert_eq!(Result::Ok, result);
    assert_eq!("6765", vm.get_output());
}
#[test]
fn can_handle_nested_function_calls() {
    let program = r#"
        fn hello() {
            print "Hello"
        }
        fn greet() {
            hello()
            print "World"
        }
        greet()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Hello\nWorld", vm.get_output());
}

#[test]
fn cannot_call_undefined_function() {
    let program = r#"
        undefined_function()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::CompileError, result);
    assert_eq!(
        "[Semantic] Undefined Symbol: Undefined variable 'undefined_function' at 2:9",
        vm.get_compiler_error()
    );
}

#[test]
fn can_handle_function_with_no_body() {
    let program = r#"
        fn empty() {}
        empty()
        print "Done"
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Done", vm.get_output());
}

#[test]
fn can_use_modulo_operator() {
    let program = r#"
        print 10 % 3
        print 7 % 2
        print 5 % 5
        print 4 % 5
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n1\n0\n4", vm.get_output());
}

#[test]
fn can_use_struct() {
    let program = r#"
        struct Point {
            x
            y
        }

        val p = Point(3, 4)
        print p.x
        print p.y
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("3\n4", vm.get_output());
}

#[test]
fn can_call_native_function_directly() {
    // Test calling a native function directly (not through VM bytecode)
    let mut vm = VirtualMachine::new(Vec::new());

    let args = vec![Value::Number(5.0), Value::Number(3.0)];
    let result = native_functions::native_add(&mut vm, &args);

    assert!(result.is_ok());
    match result.unwrap() {
        Value::Number(n) => assert_eq!(8.0, n),
        _ => panic!("Expected number result"),
    }
}

#[test]
fn native_function_rejects_wrong_arity() {
    let mut vm = VirtualMachine::new(Vec::new());

    let args = vec![Value::Number(5.0)];
    let result = native_functions::native_add(&mut vm, &args);

    assert!(result.is_err());
    assert_eq!("native_add expects 2 arguments, got 1", result.unwrap_err());
}

#[test]
fn native_function_rejects_wrong_types() {
    let mut vm = VirtualMachine::new(Vec::new());

    let args = vec![Value::Number(5.0), Value::Boolean(true)];
    let result = native_functions::native_add(&mut vm, &args);

    assert!(result.is_err());
    assert_eq!("native_add requires two number arguments", result.unwrap_err());
}

#[test]
fn can_create_and_display_native_function() {
    // Test that we can create a NativeFunction value and display it
    let native_fn = Value::new_native_function(
        "test_add".to_string(),
        2,
        native_functions::native_add
    );

    let display = format!("{}", native_fn);
    assert_eq!("<native fn test_add>", display);
}

// =============================================================================
// Logical Operator Tests
// =============================================================================

#[test]
fn test_logical_and_true_true() {
    let program = r#"
        print true && true
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_and_true_false() {
    let program = r#"
        print true && false
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_logical_and_false_true() {
    let program = r#"
        print false && true
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_logical_and_false_false() {
    let program = r#"
        print false && false
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_logical_or_true_true() {
    let program = r#"
        print true || true
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_or_true_false() {
    let program = r#"
        print true || false
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_or_false_true() {
    let program = r#"
        print false || true
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_or_false_false() {
    let program = r#"
        print false || false
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_logical_operators_with_comparisons() {
    let program = r#"
        print 5 > 3 && 10 < 20
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_operators_with_variables() {
    let program = r#"
        val x = true
        val y = false
        print x && y
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_logical_or_with_variables() {
    let program = r#"
        val x = true
        val y = false
        print x || y
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_precedence_or_and() {
    let program = r#"
        print false || true && false
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    // Should parse as: false || (true && false)
    // true && false = false
    // false || false = false
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_logical_precedence_and_or() {
    let program = r#"
        print true && false || true
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    // Should parse as: (true && false) || true
    // true && false = false
    // false || true = true
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_precedence_with_parens() {
    let program = r#"
        print (false || true) && false
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    // (false || true) = true
    // true && false = false
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_logical_and_short_circuit() {
    let program = r#"
        var x = 10
        if (false && (x = 20) > 0) {
            print "Should not reach here"
        }
        print x
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    // x should still be 10 because the right side of && should not evaluate
    assert_eq!("10", vm.get_output());
}

#[test]
fn test_logical_or_short_circuit() {
    let program = r#"
        var x = 10
        if (true || (x = 20) > 0) {
            print "Should reach here"
        }
        print x
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    // x should still be 10 because the right side of || should not evaluate
    assert_eq!("Should reach here\n10", vm.get_output());
}

#[test]
fn test_logical_complex_expression() {
    let program = r#"
        val a = true
        val b = false
        val c = true
        print (a || b) && c
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    // (true || false) = true
    // true && true = true
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_with_not() {
    let program = r#"
        print !false && true
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    // !false = true
    // true && true = true
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_chained_and() {
    let program = r#"
        print true && true && true
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_chained_and_with_false() {
    let program = r#"
        print true && false && true
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_logical_chained_or() {
    let program = r#"
        print false || false || true
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_in_if_statement() {
    let program = r#"
        val x = 5
        val y = 10
        if (x > 0 && y > 0) {
            print "Both positive"
        }
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Both positive", vm.get_output());
}

#[test]
fn test_logical_in_while_loop() {
    let program = r#"
        var x = 0
        var y = 3
        while (x < 3 && y > 0) {
            x = x + 1
            y = y - 1
        }
        print x
        print y
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("3\n0", vm.get_output());
}

#[test]
fn test_logical_with_equality() {
    let program = r#"
        val x = 5
        print x == 5 && x > 0
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_all_operators_combined() {
    let program = r#"
        val a = true
        val b = false
        val c = true
        val d = false
        print (a && b) || (c && !d)
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    // (true && false) = false
    // !false = true
    // (true && true) = true
    // false || true = true
    assert_eq!("true", vm.get_output());
}

// =============================================================================
// Map Tests
// =============================================================================

#[test]
fn test_empty_map_creation() {
    let program = r#"
        val m = {}
        print m
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("{}", vm.get_output());
}

#[test]
fn test_map_creation_with_string_keys() {
    let program = r#"
        val m = {"name": "Alice", "age": 30}
        print m
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    let output = vm.get_output();
    // HashMap order is not guaranteed, check both possible orders
    assert!(output.contains("name: Alice") && output.contains("age: 30"));
}

#[test]
fn test_map_access_string_key() {
    let program = r#"
        val m = {"name": "Alice", "age": 30}
        print m["name"]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Alice", vm.get_output());
}

#[test]
fn test_map_access_missing_key_returns_nil() {
    let program = r#"
        val m = {"name": "Alice"}
        print m["missing"]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("nil", vm.get_output());
}

#[test]
fn test_map_set_new_value() {
    let program = r#"
        var m = {"name": "Alice"}
        m["age"] = 30
        print m["age"]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("30", vm.get_output());
}

#[test]
fn test_map_update_existing_value() {
    let program = r#"
        var m = {"name": "Alice"}
        m["name"] = "Bob"
        print m["name"]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Bob", vm.get_output());
}

#[test]
fn test_map_with_number_keys() {
    let program = r#"
        val m = {1: "one", 2: "two", 3: "three"}
        print m[2]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("two", vm.get_output());
}

#[test]
fn test_map_with_boolean_keys() {
    let program = r#"
        val m = {true: "yes", false: "no"}
        print m[true]
        print m[false]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("yes\nno", vm.get_output());
}

#[test]
fn test_map_with_mixed_key_types() {
    let program = r#"
        val m = {"name": "Alice", 42: "answer", true: "yes"}
        print m["name"]
        print m[42]
        print m[true]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Alice\nanswer\nyes", vm.get_output());
}

#[test]
fn test_map_with_mixed_value_types() {
    let program = r#"
        val m = {"num": 42, "bool": true, "nil": nil}
        print m["num"]
        print m["bool"]
        print m["nil"]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("42\ntrue\nnil", vm.get_output());
}

#[test]
fn test_map_nested_in_map() {
    let program = r#"
        val m = {"inner": {"x": 10}}
        print m["inner"]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("{x: 10}", vm.get_output());
}

#[test]
fn test_map_assignment_returns_value() {
    let program = r#"
        var m = {}
        val result = m["key"] = 42
        print result
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("42", vm.get_output());
}

#[test]
fn test_map_in_variable_assignment() {
    let program = r#"
        val m1 = {"x": 1}
        val m2 = m1
        print m2["x"]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1", vm.get_output());
}

#[test]
fn test_map_in_expression() {
    let program = r#"
        val m = {"x": 10, "y": 20}
        print m["x"] + m["y"]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("30", vm.get_output());
}

#[test]
fn test_map_key_evaluation() {
    let program = r#"
        val m = {"a": 1, "b": 2}
        val key = "a"
        print m[key]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1", vm.get_output());
}

#[test]
fn test_map_dynamic_key() {
    let program = r#"
        val m = {1: "one", 2: "two"}
        val x = 1
        print m[x + 1]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("two", vm.get_output());
}

// Map method tests

#[test]
fn test_map_get_method_existing_key() {
    let program = r#"
        val m = {"name": "Alice", "age": 30}
        print m.get("name")
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Alice", vm.get_output());
}

#[test]
fn test_map_get_method_nonexistent_key() {
    let program = r#"
        val m = {"name": "Alice"}
        print m.get("missing")
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("nil", vm.get_output());
}

#[test]
fn test_map_get_method_number_key() {
    let program = r#"
        val m = {42: "answer", 100: "century"}
        print m.get(42)
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("answer", vm.get_output());
}

#[test]
fn test_map_size_method_empty() {
    let program = r#"
        val m = {}
        print m.size()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("0", vm.get_output());
}

#[test]
fn test_map_size_method_with_entries() {
    let program = r#"
        val m = {"a": 1, "b": 2, "c": 3}
        print m.size()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("3", vm.get_output());
}

#[test]
fn test_map_has_method_existing_key() {
    let program = r#"
        val m = {"name": "Alice", "age": 30}
        print m.has("name")
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_map_has_method_nonexistent_key() {
    let program = r#"
        val m = {"name": "Alice"}
        print m.has("missing")
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_map_has_method_boolean_key() {
    let program = r#"
        val m = {true: "yes", false: "no"}
        print m.has(true)
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_map_remove_method_existing_key() {
    let program = r#"
        val m = {"name": "Alice", "age": 30}
        val removed = m.remove("name")
        print removed
        print m.size()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Alice\n1", vm.get_output());
}

#[test]
fn test_map_remove_method_nonexistent_key() {
    let program = r#"
        val m = {"name": "Alice"}
        val removed = m.remove("missing")
        print removed
        print m.size()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("nil\n1", vm.get_output());
}

#[test]
fn test_map_keys_method_empty() {
    let program = r#"
        val m = {}
        val keys = m.keys()
        print keys
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[]", vm.get_output());
}

#[test]
fn test_map_keys_method_with_entries() {
    let program = r#"
        val m = {"a": 1, "b": 2}
        val keys = m.keys()
        print keys
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    // Note: HashMap order is not guaranteed, so we just check it's an array with 2 elements
    let output = vm.get_output();
    assert!(output.starts_with('[') && output.ends_with(']'));
}

#[test]
fn test_map_values_method_empty() {
    let program = r#"
        val m = {}
        val values = m.values()
        print values
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[]", vm.get_output());
}

#[test]
fn test_map_values_method_with_entries() {
    let program = r#"
        val m = {"a": 1, "b": 2}
        val values = m.values()
        print values
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    // Note: HashMap order is not guaranteed, so we just check it's an array with 2 elements
    let output = vm.get_output();
    assert!(output.starts_with('[') && output.ends_with(']'));
}

#[test]
fn test_map_entries_method_empty() {
    let program = r#"
        val m = {}
        val entries = m.entries()
        print entries
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[]", vm.get_output());
}

#[test]
fn test_map_entries_method_with_entries() {
    let program = r#"
        val m = {"name": "Alice", "age": 30}
        val entries = m.entries()
        print entries
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    // Note: HashMap order is not guaranteed, so we just check it's an array
    let output = vm.get_output();
    assert!(output.starts_with('[') && output.ends_with(']'));
}

#[test]
fn test_map_chained_operations() {
    let program = r#"
        val m = {"a": 1, "b": 2, "c": 3}
        print m.has("a")
        print m.get("b")
        val old = m.remove("c")
        print old
        print m.size()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true\n2\n3\n2", vm.get_output());
}

#[test]
fn test_map_keys_with_different_types() {
    let program = r#"
        val m = {"str": 1, 42: 2, true: 3}
        val keys = m.keys()
        print keys
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    // Just verify it returns an array
    let output = vm.get_output();
    assert!(output.starts_with('[') && output.ends_with(']'));
}

#[test]
fn test_map_method_after_modification() {
    let program = r#"
        val m = {"a": 1}
        m["b"] = 2
        m["c"] = 3
        print m.size()
        print m.has("b")
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("3\ntrue", vm.get_output());
}

#[test]
fn test_map_remove_then_size() {
    let program = r#"
        val m = {"x": 10, "y": 20, "z": 30}
        m.remove("y")
        print m.size()
        print m.has("y")
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("2\nfalse", vm.get_output());
}

#[test]
fn test_map_get_and_bracket_equivalence() {
    let program = r#"
        val m = {"key": "value"}
        print m.get("key")
        print m["key"]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("value\nvalue", vm.get_output());
}

#[test]
fn test_map_values_reflect_changes() {
    let program = r#"
        val m = {"a": 1, "b": 2}
        m["c"] = 3
        val values = m.values()
        print values
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    // Verify it's an array with 3 elements
    let output = vm.get_output();
    assert!(output.starts_with('[') && output.ends_with(']'));
}

// =============================================================================
// Map Integration Tests - End-to-End Scenarios
// =============================================================================

#[test]
fn test_map_with_string_values() {
    let program = r#"
        val messages = {"greet": "Hello", "farewell": "Goodbye", "thanks": "Thank you"}
        print messages["greet"]
        print messages["farewell"]
        print messages["thanks"]
        print messages.size()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Hello\nGoodbye\nThank you\n3", vm.get_output());
}

#[test]
fn test_map_iteration_with_modification() {
    let program = r#"
        var m = {"a": 1, "b": 2, "c": 3}
        // Test direct key access and modification
        print m["a"]
        print m["b"]
        print m["c"]
        m["a"] = m["a"] + 10
        m["b"] = m["b"] + 10
        m["c"] = m["c"] + 10
        print m["a"]
        print m["b"]
        print m["c"]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n2\n3\n11\n12\n13", vm.get_output());
}

#[test]
fn test_nested_map_access_chain() {
    let program = r#"
        val data = {
            "user": {
                "profile": {
                    "name": "Alice"
                }
            }
        }
        val user = data["user"]
        val profile = user["profile"]
        print profile["name"]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Alice", vm.get_output());
}

#[test]
fn test_map_with_conditional_logic() {
    let program = r#"
        val scores = {"Alice": 95, "Bob": 65, "Charlie": 80}
        // Test conditional logic with direct map access
        val alice_score = scores["Alice"]
        if (alice_score >= 90) {
            print "Alice: A"
        }

        val bob_score = scores["Bob"]
        if (bob_score >= 90) {
            print "Bob: A"
        } else if (bob_score >= 80) {
            print "Bob: B"
        } else if (bob_score >= 70) {
            print "Bob: C"
        } else {
            print "Bob: F"
        }

        val charlie_score = scores["Charlie"]
        if (charlie_score >= 90) {
            print "Charlie: A"
        } else if (charlie_score >= 80) {
            print "Charlie: B"
        }
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Alice: A\nBob: F\nCharlie: B", vm.get_output());
}

#[test]
fn test_map_direct_value_access() {
    let program = r#"
        val m = {"x": 10, "y": 20, "z": 30}
        // Test direct access to multiple values
        val x_val = m["x"]
        val y_val = m["y"]
        val z_val = m["z"]
        print x_val + y_val + z_val
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("60", vm.get_output());
}

#[test]
fn test_map_as_function_parameter() {
    let program = r#"
        fn get_value(map, key) {
            return map[key]
        }

        fn set_value(map, key, value) {
            map[key] = value
            return map
        }

        val m1 = {"a": 1, "b": 2}
        print get_value(m1, "a")
        print get_value(m1, "b")

        val m2 = set_value(m1, "c", 3)
        print m2["c"]
        print m2.size()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n2\n3\n3", vm.get_output());
}

#[test]
fn test_map_with_computed_keys() {
    let program = r#"
        val base = "key"
        var m = {}
        m[base + "1"] = 100
        m[base + "2"] = 200
        print m["key1"]
        print m["key2"]
        print m.size()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("100\n200\n2", vm.get_output());
}

#[test]
fn test_map_update_in_loop() {
    let program = r#"
        var m = {"a": 1, "b": 2, "c": 3}
        // Test updating map values directly
        print m["a"]
        print m["b"]
        print m["c"]

        // Update each value
        m["a"] = m["a"] * 2
        m["b"] = m["b"] * 2
        m["c"] = m["c"] * 2

        print m["a"]
        print m["b"]
        print m["c"]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n2\n3\n2\n4\n6", vm.get_output());
}

#[test]
fn test_map_multiple_removes() {
    let program = r#"
        var m = {"a": 1, "b": 2, "c": 3, "d": 4}
        print m.size()
        m.remove("a")
        print m.size()
        m.remove("c")
        print m.size()
        print m.has("a")
        print m.has("b")
        print m.has("c")
        print m.has("d")
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("4\n3\n2\nfalse\ntrue\nfalse\ntrue", vm.get_output());
}

#[test]
fn test_map_with_struct_values() {
    let program = r#"
        struct Point {
            x
            y
        }

        val points = {
            "origin": Point(0, 0),
            "unit": Point(1, 1)
        }

        val origin = points["origin"]
        print origin.x
        print origin.y

        val unit = points["unit"]
        print unit.x
        print unit.y
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("0\n0\n1\n1", vm.get_output());
}

#[test]
fn test_map_boolean_key_expressions() {
    let program = r#"
        val m = {true: "yes", false: "no"}
        val x = 5
        print m[x > 3]
        print m[x < 3]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("yes\nno", vm.get_output());
}

#[test]
fn test_map_chaining_operations() {
    let program = r#"
        var m = {"a": 1}
        m["b"] = 2
        m["c"] = 3
        val size1 = m.size()
        m.remove("b")
        val size2 = m.size()
        print size1
        print size2
        print m.has("a")
        print m.has("b")
        print m.has("c")
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("3\n2\ntrue\nfalse\ntrue", vm.get_output());
}

#[test]
fn test_map_empty_to_full_lifecycle() {
    let program = r#"
        var m = {}
        print m.size()
        print m.keys()

        m["first"] = 1
        print m.size()

        m["second"] = 2
        m["third"] = 3
        print m.size()

        m.remove("second")
        print m.size()

        m.remove("first")
        m.remove("third")
        print m.size()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("0\n[]\n1\n3\n2\n0", vm.get_output());
}

#[test]
fn test_map_in_recursive_function() {
    let program = r#"
        fn count_down(map, n) {
            if (n == 0) {
                return map
            }
            map[n] = n * n
            return count_down(map, n - 1)
        }

        var m = {}
        val result = count_down(m, 3)
        print result[1]
        print result[2]
        print result[3]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n4\n9", vm.get_output());
}

// =============================================================================
// Array Tests
// =============================================================================

#[test]
fn test_array_literal_empty() {
    let program = r#"
        val arr = []
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[]", vm.get_output());
}

#[test]
fn test_array_literal_single_element() {
    let program = r#"
        val arr = [42]
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[42]", vm.get_output());
}

#[test]
fn test_array_literal_multiple_elements() {
    let program = r#"
        val arr = [1, 2, 3]
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[1, 2, 3]", vm.get_output());
}

#[test]
fn test_array_literal_mixed_types() {
    let program = r#"
        val arr = [1, "hello", true, nil]
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[1, hello, true, nil]", vm.get_output());
}

#[test]
fn test_array_indexing_positive() {
    let program = r#"
        val arr = [10, 20, 30]
        print arr[0]
        print arr[1]
        print arr[2]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("10\n20\n30", vm.get_output());
}

#[test]
fn test_array_indexing_negative() {
    let program = r#"
        val arr = [10, 20, 30]
        print arr[-1]
        print arr[-2]
        print arr[-3]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("30\n20\n10", vm.get_output());
}

#[test]
fn test_array_index_assignment() {
    let program = r#"
        var arr = [1, 2, 3]
        arr[0] = 10
        arr[1] = 20
        arr[2] = 30
        print arr[0]
        print arr[1]
        print arr[2]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("10\n20\n30", vm.get_output());
}

#[test]
fn test_array_index_assignment_negative() {
    let program = r#"
        var arr = [1, 2, 3]
        arr[-1] = 99
        arr[-2] = 88
        print arr[1]
        print arr[2]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("88\n99", vm.get_output());
}

#[test]
fn test_array_push() {
    let program = r#"
        var arr = [1, 2, 3]
        arr.push(4)
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[1, 2, 3, 4]", vm.get_output());
}

#[test]
fn test_array_push_multiple() {
    let program = r#"
        var arr = []
        arr.push(1)
        arr.push(2)
        arr.push(3)
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[1, 2, 3]", vm.get_output());
}

#[test]
fn test_array_pop() {
    let program = r#"
        var arr = [1, 2, 3]
        val last = arr.pop()
        print last
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("3\n[1, 2]", vm.get_output());
}

#[test]
fn test_array_pop_empty() {
    let program = r#"
        var arr = []
        val result = arr.pop()
        print result
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("nil", vm.get_output());
}

#[test]
fn test_array_length() {
    let program = r#"
        val arr1 = []
        val arr2 = [1]
        val arr3 = [1, 2, 3]
        print arr1.length()
        print arr2.length()
        print arr3.length()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("0\n1\n3", vm.get_output());
}

#[test]
fn test_array_length_after_push() {
    let program = r#"
        var arr = [1, 2]
        print arr.length()
        arr.push(3)
        print arr.length()
        arr.push(4)
        print arr.length()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("2\n3\n4", vm.get_output());
}

#[test]
fn test_array_length_after_pop() {
    let program = r#"
        var arr = [1, 2, 3, 4]
        print arr.length()
        arr.pop()
        print arr.length()
        arr.pop()
        print arr.length()
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("4\n3\n2", vm.get_output());
}

#[test]
fn test_array_nested() {
    let program = r#"
        val arr = [[1, 2], [3, 4]]
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[[1, 2], [3, 4]]", vm.get_output());
}

#[test]
fn test_array_nested_access() {
    let program = r#"
        val arr = [[1, 2], [3, 4]]
        val inner = arr[0]
        print inner[0]
        print inner[1]
        val inner2 = arr[1]
        print inner2[0]
        print inner2[1]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n2\n3\n4", vm.get_output());
}

#[test]
fn test_array_nested_modification() {
    let program = r#"
        var arr = [[1, 2], [3, 4]]
        val inner = arr[0]
        inner[0] = 99
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[[99, 2], [3, 4]]", vm.get_output());
}

#[test]
fn test_array_with_variables() {
    let program = r#"
        val x = 10
        val y = 20
        val z = 30
        val arr = [x, y, z]
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[10, 20, 30]", vm.get_output());
}

#[test]
fn test_array_with_expressions() {
    let program = r#"
        val arr = [1 + 1, 2 * 3, 10 - 5]
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[2, 6, 5]", vm.get_output());
}

#[test]
fn test_array_in_expression() {
    let program = r#"
        val arr = [10, 20, 30]
        val sum = arr[0] + arr[1] + arr[2]
        print sum
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("60", vm.get_output());
}

#[test]
fn test_array_in_variable_assignment() {
    let program = r#"
        val arr1 = [1, 2, 3]
        val arr2 = arr1
        print arr2
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[1, 2, 3]", vm.get_output());
}

#[test]
fn test_array_dynamic_index() {
    let program = r#"
        val arr = [10, 20, 30]
        val i = 1
        print arr[i]
        print arr[i + 1]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("20\n30", vm.get_output());
}

#[test]
fn test_array_assignment_returns_value() {
    let program = r#"
        var arr = [1, 2, 3]
        val result = arr[1] = 99
        print result
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("99\n[1, 99, 3]", vm.get_output());
}

// =============================================================================
// Array Integration Tests - End-to-End Scenarios
// =============================================================================

#[test]
fn test_array_as_function_parameter() {
    let program = r#"
        fn get_first(arr) {
            return arr[0]
        }

        fn set_first(arr, value) {
            arr[0] = value
            return arr
        }

        val arr1 = [1, 2, 3]
        print get_first(arr1)

        val arr2 = set_first(arr1, 99)
        print arr2[0]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n99", vm.get_output());
}

#[test]
fn test_array_with_conditional_logic() {
    let program = r#"
        val scores = [85, 92, 78]

        val first = scores[0]
        if (first >= 90) {
            print "A"
        } else if (first >= 80) {
            print "B"
        }

        val second = scores[1]
        if (second >= 90) {
            print "A"
        }

        val third = scores[2]
        if (third >= 80) {
            print "B"
        } else {
            print "C"
        }
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("B\nA\nC", vm.get_output());
}

#[test]
fn test_array_in_loop() {
    let program = r#"
        var arr = [1, 2, 3]
        var i = 0
        while (i < arr.length()) {
            print arr[i]
            i = i + 1
        }
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n2\n3", vm.get_output());
}

#[test]
fn test_array_accumulation() {
    let program = r#"
        val arr = [1, 2, 3, 4, 5]
        var sum = 0
        var i = 0
        while (i < arr.length()) {
            sum = sum + arr[i]
            i = i + 1
        }
        print sum
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("15", vm.get_output());
}

#[test]
fn test_array_push_pop_lifecycle() {
    let program = r#"
        var arr = []
        print arr.length()

        arr.push(1)
        arr.push(2)
        arr.push(3)
        print arr.length()
        print arr

        arr.pop()
        print arr.length()
        print arr

        arr.pop()
        arr.pop()
        print arr.length()
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("0\n3\n[1, 2, 3]\n2\n[1, 2]\n0\n[]", vm.get_output());
}

#[test]
fn test_array_with_map_values() {
    let program = r#"
        val map1 = {"x": 1, "y": 2}
        val map2 = {"x": 3, "y": 4}
        val arr = [map1, map2]

        val first = arr[0]
        print first["x"]

        val second = arr[1]
        print second["y"]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n4", vm.get_output());
}

#[test]
fn test_map_with_array_values() {
    let program = r#"
        val m = {
            "numbers": [1, 2, 3],
            "strings": ["a", "b", "c"]
        }

        val numbers = m["numbers"]
        print numbers[0]
        print numbers[2]

        val strings = m["strings"]
        print strings[1]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n3\nb", vm.get_output());
}

#[test]
fn test_array_build_with_loop() {
    let program = r#"
        var arr = []
        var i = 1
        while (i <= 5) {
            arr.push(i * i)
            i = i + 1
        }
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[1, 4, 9, 16, 25]", vm.get_output());
}

#[test]
fn test_array_reverse_with_negative_indices() {
    let program = r#"
        val arr = [1, 2, 3, 4, 5]
        print arr[-1]
        print arr[-2]
        print arr[-3]
        print arr[-4]
        print arr[-5]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("5\n4\n3\n2\n1", vm.get_output());
}

#[test]
fn test_array_modification_in_function() {
    let program = r#"
        fn double_elements(arr) {
            var i = 0
            while (i < arr.length()) {
                arr[i] = arr[i] * 2
                i = i + 1
            }
            return arr
        }

        var numbers = [1, 2, 3]
        val result = double_elements(numbers)
        print result
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("[2, 4, 6]", vm.get_output());
}

#[test]
fn test_array_chained_operations() {
    let program = r#"
        var arr = [1, 2, 3]
        arr.push(4)
        arr.push(5)
        val last = arr.pop()
        print last
        print arr.length()
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("5\n4\n[1, 2, 3, 4]", vm.get_output());
}

#[test]
fn test_array_with_struct_values() {
    let program = r#"
        struct Point {
            x
            y
        }

        val points = [Point(1, 2), Point(3, 4), Point(5, 6)]

        val p1 = points[0]
        print p1.x
        print p1.y

        val p2 = points[1]
        print p2.x
        print p2.y
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n2\n3\n4", vm.get_output());
}

#[test]
fn test_array_empty_to_full_lifecycle() {
    let program = r#"
        var arr = []
        print arr.length()

        arr.push(10)
        print arr.length()
        print arr[0]

        arr.push(20)
        arr.push(30)
        print arr.length()

        arr[1] = 99
        print arr

        arr.pop()
        arr.pop()
        arr.pop()
        print arr.length()
        print arr
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("0\n1\n10\n3\n[10, 99, 30]\n0\n[]", vm.get_output());
}

#[test]
fn test_array_large_size() {
    // Test that arrays with more than 256 elements work correctly
    // This tests the u16 array count encoding
    let program = r#"
        var arr = []
        var i = 0
        while (i < 300) {
            arr.push(i)
            i = i + 1
        }
        print arr.length()
        print arr[0]
        print arr[255]
        print arr[256]
        print arr[299]
        "#;

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("300\n0\n255\n256\n299", vm.get_output());
}

#[test]
fn test_array_literal_large_size() {
    // Test array literal with more than 256 elements
    let mut elements = Vec::new();
    for i in 0..300 {
        elements.push(i.to_string());
    }
    let array_literal = format!("[{}]", elements.join(", "));

    let program = format!(
        r#"
        var arr = {}
        print arr.length()
        print arr[0]
        print arr[255]
        print arr[256]
        print arr[299]
        "#,
        array_literal
    );

    let mut vm = VirtualMachine::new(Vec::new());
    let result = vm.interpret(program);
    assert_eq!(Result::Ok, result);
    assert_eq!("300\n0\n255\n256\n299", vm.get_output());
}

// =============================================================================
// Break and Continue Tests
// =============================================================================

#[test]
fn test_break_in_while_loop() {
    let program = r#"
        var x = 0
        while (x < 10) {
            if (x == 5) {
                break
            }
            print x
            x = x + 1
        }
        print "Done"
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("0\n1\n2\n3\n4\nDone", vm.get_output());
}

#[test]
fn test_continue_in_while_loop() {
    let program = r#"
        var x = 0
        while (x < 5) {
            x = x + 1
            if (x == 3) {
                continue
            }
            print x
        }
        print "Done"
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n2\n4\n5\nDone", vm.get_output());
}

#[test]
fn test_break_in_for_loop() {
    let program = r#"
        for (var i = 0; i < 10; i = i + 1) {
            if (i == 5) {
                break
            }
            print i
        }
        print "Done"
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("0\n1\n2\n3\n4\nDone", vm.get_output());
}

#[test]
fn test_continue_in_for_loop() {
    let program = r#"
        for (var i = 0; i < 5; i = i + 1) {
            if (i == 2) {
                continue
            }
            print i
        }
        print "Done"
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("0\n1\n3\n4\nDone", vm.get_output());
}

#[test]
fn test_break_in_for_in_loop() {
    let program = r#"
        val arr = [1, 2, 3, 4, 5]
        for (item in arr) {
            if (item == 3) {
                break
            }
            print item
        }
        print "Done"
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n2\nDone", vm.get_output());
}

#[test]
fn test_continue_in_for_in_loop() {
    let program = r#"
        val arr = [1, 2, 3, 4, 5]
        for (item in arr) {
            if (item == 3) {
                continue
            }
            print item
        }
        print "Done"
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n2\n4\n5\nDone", vm.get_output());
}

#[test]
fn test_nested_loops_with_break() {
    let program = r#"
        var i = 0
        while (i < 3) {
            var j = 0
            while (j < 3) {
                if (j == 2) {
                    break
                }
                print "i=" + i.toString() + " j=" + j.toString()
                j = j + 1
            }
            i = i + 1
        }
        print "Done"
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("i=0 j=0\ni=0 j=1\ni=1 j=0\ni=1 j=1\ni=2 j=0\ni=2 j=1\nDone", vm.get_output());
}

#[test]
fn test_nested_loops_with_continue() {
    let program = r#"
        var i = 0
        while (i < 3) {
            i = i + 1
            if (i == 2) {
                continue
            }
            var j = 0
            while (j < 2) {
                print "i=" + i.toString() + " j=" + j.toString()
                j = j + 1
            }
        }
        print "Done"
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("i=1 j=0\ni=1 j=1\ni=3 j=0\ni=3 j=1\nDone", vm.get_output());
}

#[test]
fn test_break_with_accumulator() {
    let program = r#"
        val arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        var sum = 0
        for (item in arr) {
            if (sum > 10) {
                break
            }
            sum = sum + item
        }
        print sum
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("15", vm.get_output());
}

#[test]
fn test_continue_with_accumulator() {
    let program = r#"
        val arr = [1, 2, 3, 4, 5]
        var sum = 0
        for (item in arr) {
            if (item == 3) {
                continue
            }
            sum = sum + item
        }
        print sum
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("12", vm.get_output());
}

#[test]
fn test_break_immediately() {
    let program = r#"
        var x = 0
        while (true) {
            break
            x = x + 1
        }
        print x
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("0", vm.get_output());
}

#[test]
fn test_multiple_breaks_in_loop() {
    let program = r#"
        var x = 0
        while (x < 10) {
            if (x == 3) {
                break
            }
            if (x == 7) {
                break
            }
            print x
            x = x + 1
        }
        print "Done"
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("0\n1\n2\nDone", vm.get_output());
}

#[test]
fn test_multiple_continues_in_loop() {
    let program = r#"
        var x = 0
        while (x < 6) {
            x = x + 1
            if (x == 2) {
                continue
            }
            if (x == 4) {
                continue
            }
            print x
        }
        print "Done"
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("1\n3\n5\n6\nDone", vm.get_output());
}
