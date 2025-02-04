use crate::vm::Value;
use std::assert_eq;

#[test]
fn can_create_vm() {
    let vm = super::VirtualMachine::new();
    assert_eq!(0, vm.ip);
    assert_eq!(0, vm.stack.len());
}

#[test]
fn can_execute_simple_arithmetics() {
    let mut block = super::Block::new("ZeBlock");

    block.write_constant(crate::number!(1.0), 0);
    block.write_constant(crate::number!(2.0), 0);
    block.write_op_code(super::OpCode::Add, 0);
    block.write_constant(crate::number!(3.0), 0);
    block.write_op_code(super::OpCode::Multiply, 0);
    block.write_constant(crate::number!(2.0), 0);
    block.write_op_code(super::OpCode::Subtract, 0);
    block.write_constant(crate::number!(2.0), 0);
    block.write_op_code(super::OpCode::Divide, 0);
    block.write_op_code(super::OpCode::Return, 0);

    let mut vm = super::VirtualMachine::new();

    let result = vm.run(&block);
    assert_eq!(super::Result::Ok, result);
    assert_eq!(3.5, crate::as_number!(vm.pop()));
}

#[test]
fn can_print_hello_world() {
    let program = r#"
        print "Hello World 🌍"
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(super::Result::Ok, result);
}

#[test]
fn can_print_the_answer_to_everything_times_pi() {
    let program = r#"
        print 42 * 3.14
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(super::Result::Ok, result);
}

#[test]
fn can_run_multi_line_statements() {
    let program = r#"
        print "Hello World 🌎"
        print 13
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(super::Result::Ok, result);
}

#[test]
fn can_define_a_global_value() {
    let program = r#"
        val greeting = "Hello World 🌎"
        print greeting
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());

    assert_eq!(super::Result::Ok, result);
    assert_eq!("Hello World 🌎", vm.get_output())
}

#[test]
fn can_negate_numbers() {
    let program = r#"
        val x = 42
        print -x
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("-42", vm.get_output());
}

#[test]
fn can_compare_numbers_equal() {
    let program = r#"
        print 42 == 42
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_compare_numbers_not_equal() {
    let program = r#"
        print 42 == 43
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn can_compare_greater_than() {
    let program = r#"
        print 43 > 42
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_compare_less_than() {
    let program = r#"
        print 41 < 42
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_use_logical_not() {
    let program = r#"
        print !false
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_handle_nil() {
    let program = r#"
        val x = nil
        print x
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("nil", vm.get_output());
}

#[test]
fn can_handle_boolean_true() {
    let program = r#"
        val x = true
        print x
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_handle_boolean_false() {
    let program = r#"
        val x = false
        print x
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("false", vm.get_output());
}

#[test]
fn can_handle_string_concatenation() {
    let program = r#"
        print "Hello" + " " + "World"
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("Hello World", vm.get_output());
}

#[test]
fn can_handle_multiple_global_variables() {
    let program = r#"
        val x = 40
        val y = 2
        print x + y
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("42", vm.get_output());
}

#[test]
fn can_handle_complex_arithmetic() {
    let program = r#"
        val x = 10
        val y = 5
        print (x + y) * (x - y)
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("75", vm.get_output());
}

#[test]
fn can_handle_string_comparison() {
    let program = r#"
        print "hello" == "hello"
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_handle_multiple_boolean_operations() {
    let program = r#"
        print true == !false
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("true", vm.get_output());
}

#[test]
fn can_handle_division_by_integers() {
    let program = r#"
        print 100 / 20
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("5", vm.get_output());
}

#[test]
fn can_handle_float_division() {
    let program = r#"
        print 10.0 / 3.0
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("3.3333333333333335", vm.get_output());
}

#[test]
fn can_handle_negative_numbers() {
    let program = r#"
        val x = -42
        print -x
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("42", vm.get_output());
}

#[test]
fn can_handle_boolean_arithmetic() {
    let program = r#"
        print true == true == true
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
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

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("Hello World!", vm.get_output());
}

#[test]
fn can_handle_multiple_negations() {
    let program = r#"
        print !!true
        "#;

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
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

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
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

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
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

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
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

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
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

    let mut vm = super::VirtualMachine::new();
    let result = vm.interpret(program.to_string());
    assert_eq!(super::Result::Ok, result);
    assert_eq!("The beginning", vm.get_output());
}
