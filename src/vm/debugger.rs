//! CLI-based debugger implementation for the Neon VM.
//!
//! This module provides an interactive stdin/stdout debugger that allows
//! step-through execution of Neon programs with runtime state inspection.

use std::io::{self, Write};

use crate::common::opcodes::OpCode;
use crate::common::Chunk;
use crate::vm::debug::{DebugCommand, DebugContext, DebugHandler};

/// CLI debugger that provides interactive step-through debugging via stdin/stdout.
///
/// Displays the call stack, current instruction, and value stack at each step,
/// and allows the user to control execution with simple commands.
pub struct CliDebugger;

impl CliDebugger {
    /// Creates a new CLI debugger instance.
    pub fn new() -> Self {
        Self
    }

    /// Displays the current call stack with function names and instruction pointers.
    fn display_call_stack(&self, context: &DebugContext) {
        println!("\n=== Call Stack ===");
        if context.call_frames.is_empty() {
            println!("  (empty)");
            return;
        }

        for (i, frame) in context.call_frames.iter().enumerate() {
            let marker = if i == context.call_frames.len() - 1 {
                "→"
            } else {
                " "
            };
            println!(
                "  {} [{}] {} (ip: 0x{:04x})",
                marker, i, frame.function.name, frame.ip
            );
        }
    }

    /// Displays the current instruction with disassembly.
    fn display_current_instruction(&self, context: &DebugContext) {
        println!("\n=== Current Instruction ===");

        let current_frame = match context.call_frames.last() {
            Some(frame) => frame,
            None => {
                println!("  (no active frame)");
                return;
            }
        };

        let chunk = &current_frame.function.chunk;
        let ip = context.current_ip;

        // Display instruction pointer and line number
        print!("  0x{:04x} ", ip);

        if let Some(source_loc) = chunk.get_source_location(ip) {
            print!("{}:{} ", source_loc.line, source_loc.column);
        } else {
            print!("      ");
        }

        // Disassemble the instruction
        self.disassemble_instruction(chunk, ip);
    }

    /// Disassembles a single instruction at the given offset.
    ///
    /// This is a simplified version adapted from the Chunk disassembler,
    /// returning formatted output instead of advancing through instructions.
    fn disassemble_instruction(&self, chunk: &Chunk, offset: usize) {
        if offset >= chunk.instruction_count() {
            println!("(out of bounds)");
            return;
        }

        let instruction = OpCode::from_u8(chunk.read_u8(offset));

        match instruction {
            // Simple instructions (no operands)
            OpCode::Return
            | OpCode::Negate
            | OpCode::Add
            | OpCode::Subtract
            | OpCode::Multiply
            | OpCode::Divide
            | OpCode::FloorDivide
            | OpCode::Modulo
            | OpCode::Exponent
            | OpCode::Nil
            | OpCode::True
            | OpCode::False
            | OpCode::Equal
            | OpCode::Greater
            | OpCode::Less
            | OpCode::Not
            | OpCode::Pop
            | OpCode::Loop
            | OpCode::GetIndex
            | OpCode::SetIndex
            | OpCode::GetIterator
            | OpCode::IteratorNext
            | OpCode::IteratorDone
            | OpCode::PopIterator
            | OpCode::ToString
            | OpCode::BitwiseAnd
            | OpCode::BitwiseOr
            | OpCode::BitwiseXor
            | OpCode::BitwiseNot
            | OpCode::LeftShift
            | OpCode::RightShift => {
                println!("{:?}", instruction);
            }

            // Constant instructions
            OpCode::Constant | OpCode::Constant2 | OpCode::Constant4 => {
                let index = self.read_operand_index(chunk, &instruction, offset + 1);
                let constant = chunk.read_constant(index);
                println!("{:?} {} '{}'", instruction, index, constant);
            }

            // String instructions
            OpCode::String | OpCode::String2 | OpCode::String4 => {
                let index = self.read_operand_index(chunk, &instruction, offset + 1);
                let string = chunk.read_string(index);
                println!("{:?} {} '{}'", instruction, index, string);
            }

            // Local slots, named after the chunk's local table
            OpCode::GetLocal
            | OpCode::GetLocal2
            | OpCode::GetLocal4
            | OpCode::SetLocal
            | OpCode::SetLocal2
            | OpCode::SetLocal4 => {
                let index = self.read_operand_index(chunk, &instruction, offset + 1);
                match chunk.locals.get(index) {
                    Some(local) => println!("{:?} {} '{}'", instruction, index, local.name),
                    None => println!("{:?} {}", instruction, index),
                }
            }

            // Global and built-in slots live outside the current chunk
            OpCode::GetGlobal
            | OpCode::GetGlobal2
            | OpCode::GetGlobal4
            | OpCode::SetGlobal
            | OpCode::SetGlobal2
            | OpCode::SetGlobal4
            | OpCode::GetBuiltin
            | OpCode::GetBuiltin2
            | OpCode::GetBuiltin4 => {
                let index = self.read_operand_index(chunk, &instruction, offset + 1);
                println!("{:?} {}", instruction, index);
            }

            // Field instructions
            OpCode::GetField
            | OpCode::GetField2
            | OpCode::GetField4
            | OpCode::SetField
            | OpCode::SetField2
            | OpCode::SetField4 => {
                let index = self.read_operand_index(chunk, &instruction, offset + 1);
                let field_name = chunk.read_string(index);
                println!("{:?} {} '{}'", instruction, index, field_name);
            }

            // Jump instructions
            OpCode::JumpIfFalse | OpCode::Jump => {
                let jump = chunk.read_u32(offset + 1);
                println!(
                    "{:?} 0x{:04x} -> 0x{:04x}",
                    instruction,
                    offset,
                    offset + 5 + jump as usize
                );
            }

            // Call instruction
            OpCode::Call => {
                let arg_count = chunk.read_u8(offset + 1);
                println!("Call (args: {})", arg_count);
            }

            // Collection constructors
            OpCode::CreateMap => {
                println!("CreateMap (entries: {})", chunk.read_u8(offset + 1));
            }
            OpCode::CreateArray => {
                println!("CreateArray (elements: {})", chunk.read_u16(offset + 1));
            }
            OpCode::CreateSet => {
                println!("CreateSet (elements: {})", chunk.read_u8(offset + 1));
            }
            OpCode::CreateRange => {
                println!(
                    "CreateRange (inclusive: {})",
                    chunk.read_u8(offset + 1) != 0
                );
            }
        }
    }

    /// Reads the operand index based on instruction variant (u8, u16, or u32).
    fn read_operand_index(&self, chunk: &Chunk, instruction: &OpCode, offset: usize) -> usize {
        // Determine size based on instruction variant
        let is_4byte = matches!(
            instruction,
            OpCode::Constant4
                | OpCode::String4
                | OpCode::GetLocal4
                | OpCode::SetLocal4
                | OpCode::GetBuiltin4
                | OpCode::GetGlobal4
                | OpCode::SetGlobal4
                | OpCode::GetField4
                | OpCode::SetField4
        );

        let is_2byte = matches!(
            instruction,
            OpCode::Constant2
                | OpCode::String2
                | OpCode::GetLocal2
                | OpCode::SetLocal2
                | OpCode::GetBuiltin2
                | OpCode::GetGlobal2
                | OpCode::SetGlobal2
                | OpCode::GetField2
                | OpCode::SetField2
        );

        if is_4byte {
            chunk.read_u32(offset) as usize
        } else if is_2byte {
            chunk.read_u16(offset) as usize
        } else {
            chunk.read_u8(offset) as usize
        }
    }

    /// Displays the value stack with indices and frame marker.
    fn display_value_stack(&self, context: &DebugContext) {
        println!("\n=== Value Stack ===");

        if context.stack.is_empty() {
            println!("  (empty)");
            return;
        }

        for (i, value) in context.stack.iter().enumerate() {
            let i_signed = i as isize;
            let marker = if i_signed == context.slot_start {
                " <frame>"
            } else {
                ""
            };
            println!("  [{}] {}{}", i, value, marker);
        }
    }

    /// Displays the current frame's local variables by name.
    ///
    /// Locals occupy consecutive stack slots from `slot_start + 1` in
    /// declaration order; slots not yet pushed are skipped.
    fn display_locals(&self, context: &DebugContext) {
        println!("\n=== Locals ===");

        let Some(frame) = context.call_frames.last() else {
            println!("  (no active frame)");
            return;
        };

        let first_slot = (context.slot_start + 1) as usize;
        let mut shown = false;
        for (index, local) in frame.function.chunk.locals.iter().enumerate() {
            if let Some(value) = context.stack.get(first_slot + index) {
                println!("  {} = {}", local.name, value);
                shown = true;
            }
        }
        if !shown {
            println!("  (none)");
        }
    }

    /// Displays help text for available commands.
    fn display_help(&self) {
        println!("\n=== Debugger Commands ===");
        println!("  step, s, <Enter>  - Execute the next instruction");
        println!("  continue, c       - Continue execution until completion");
        println!("  stack             - Show the value stack");
        println!("  locals            - Show the current frame's local variables");
        println!("  quit, q           - Quit debugger and exit program");
        println!("  help, h           - Show this help message");
    }

    /// Prompts the user for a command and returns the corresponding DebugCommand.
    ///
    /// Inspection commands re-prompt; EOF or a read error is treated as quit.
    fn prompt_user(&mut self, context: &DebugContext) -> DebugCommand {
        loop {
            print!("\n> ");
            if io::stdout().flush().is_err() {
                return DebugCommand::Quit;
            }

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    println!("\n(EOF - exiting debugger)");
                    return DebugCommand::Quit;
                }
                Ok(_) => match input.trim() {
                    "" | "step" | "s" => return DebugCommand::Step,
                    "continue" | "c" => return DebugCommand::Continue,
                    "quit" | "q" => return DebugCommand::Quit,
                    "stack" => self.display_value_stack(context),
                    "locals" => self.display_locals(context),
                    "help" | "h" => self.display_help(),
                    other => println!("Unknown command: '{}'. Type 'help' for help.", other),
                },
                Err(_) => {
                    println!("\n(read error - exiting debugger)");
                    return DebugCommand::Quit;
                }
            }
        }
    }
}

impl DebugHandler for CliDebugger {
    fn on_step(&mut self, context: &DebugContext) -> DebugCommand {
        // Display current state
        self.display_call_stack(context);
        self.display_current_instruction(context);
        self.display_value_stack(context);

        // Get user command
        self.prompt_user(context)
    }
}

impl Default for CliDebugger {
    fn default() -> Self {
        Self::new()
    }
}
