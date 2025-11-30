use crate::common::opcodes::OpCode;
use crate::common::Bloq;
use crate::common::Value;
use crate::vm::{Result, VirtualMachine};
use crate::vm::native_functions;
use crate::{as_number, number};
use std::assert_eq;

#[test]
fn can_create_vm() {
    let vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();

    let result = vm.run_bloq(bloq);
    assert_eq!(Result::Ok, result);
    assert_eq!(3.5, as_number!(vm.pop()));
}

#[test]
fn can_print_hello_world() {
    let program = r#"
        print "Hello World 🌍"
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
}

#[test]
fn can_print_the_answer_to_everything_times_pi() {
    let program = r#"
        print 42 * 3.14
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
}

#[test]
fn can_run_multi_line_statements() {
    let program = r#"
        print "Hello World 🌎"
        print 13
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(Result::Ok, result);
}

#[test]
fn can_define_a_global_value() {
    let program = r#"
        val greeting = "Hello World 🌎"
        print greeting
        "#;

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("-42", vm.get_output());
}

#[test]
fn can_compare_numbers_equal() {
    let program = r#"
        print 42 == 42
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_compare_numbers_not_equal() {
    let program = r#"
        print 42 == 43
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn can_compare_greater_than() {
    let program = r#"
        print 43 > 42
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_compare_less_than() {
    let program = r#"
        print 41 < 42
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_use_logical_not() {
    let program = r#"
        print !false
        "#;

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn can_handle_string_concatenation() {
    let program = r#"
        print "Hello" + " " + "World"
        "#;

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("75", vm.get_output());
}

#[test]
fn can_handle_string_comparison() {
    let program = r#"
        print "hello" == "hello"
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_handle_multiple_boolean_operations() {
    let program = r#"
        print true == !false
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_handle_division_by_integers() {
    let program = r#"
        print 100 / 20
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("5", vm.get_output());
}

#[test]
fn can_handle_float_division() {
    let program = r#"
        print 10.0 / 3.0
        "#;

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("42", vm.get_output());
}

#[test]
fn can_handle_boolean_arithmetic() {
    let program = r#"
        print true == true == true
        "#;

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Hello World!", vm.get_output());
}

#[test]
fn can_handle_multiple_negations() {
    let program = r#"
        print !!true
        "#;

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("Hello\nWorld", vm.get_output());
}

#[test]
fn cannot_call_undefined_function() {
    let program = r#"
        undefined_function()
        "#;

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("3\n4", vm.get_output());
}

#[test]
fn can_call_native_function_directly() {
    // Test calling a native function directly (not through VM bytecode)
    let mut vm = VirtualMachine::new();

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
    let mut vm = VirtualMachine::new();

    let args = vec![Value::Number(5.0)];
    let result = native_functions::native_add(&mut vm, &args);

    assert!(result.is_err());
    assert_eq!("native_add expects 2 arguments, got 1", result.unwrap_err());
}

#[test]
fn native_function_rejects_wrong_types() {
    let mut vm = VirtualMachine::new();

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

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_and_true_false() {
    let program = r#"
        print true && false
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_logical_and_false_true() {
    let program = r#"
        print false && true
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_logical_and_false_false() {
    let program = r#"
        print false && false
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_logical_or_true_true() {
    let program = r#"
        print true || true
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_or_true_false() {
    let program = r#"
        print true || false
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_or_false_true() {
    let program = r#"
        print false || true
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_or_false_false() {
    let program = r#"
        print false || false
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_logical_operators_with_comparisons() {
    let program = r#"
        print 5 > 3 && 10 < 20
        "#;

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_precedence_or_and() {
    let program = r#"
        print false || true && false
        "#;

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn test_logical_chained_and_with_false() {
    let program = r#"
        print true && false && true
        "#;

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn test_logical_chained_or() {
    let program = r#"
        print false || false || true
        "#;

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
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

    let mut vm = VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(Result::Ok, result);
    // (true && false) = false
    // !false = true
    // (true && true) = true
    // false || true = true
    assert_eq!("true", vm.get_output());
}
