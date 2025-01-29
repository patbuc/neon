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
