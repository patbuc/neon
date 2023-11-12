mod vm;

use colored::Colorize;

use crate::vm::block::{Block, OpCode};

use crate::vm::vm::VirtualMachine;

fn main() {
    println!(
        "Hi, this is {} - a toy language you didn't wait for.",
        "neon".truecolor(240, 0, 255).bold()
    );

    let mut block = Block::new("ZeBlock");

    block.write_constant(1234.56, 2);
    block.write_op_code(OpCode::Negate, 3);
    block.write_constant(345.67, 4);
    block.write_op_code(OpCode::Add, 4);
    block.write_constant(1.2, 5);
    block.write_op_code(OpCode::Multiply, 6);
    block.write_op_code(OpCode::Return, 8);

    let mut vm = VirtualMachine::new(block);
    vm.interpret();
}
