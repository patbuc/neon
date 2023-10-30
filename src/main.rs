mod block;

use colored::Colorize;
use crate::block::{Block, OpCode};

#[cfg(feature = "disassemble")]
use crate::block::{BlockDbg};

fn main() {
    println!(
        "Hi, this is {} - a toy language you didn't wait for.",
        "neon".truecolor(240,0,255).bold()
    );

    let mut block = Block::new("ZeBlock");
    block.push_op_code(OpCode::Return);

    #[cfg(feature = "disassemble")]
    block.disassemble_block();
}


