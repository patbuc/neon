//! CLI-based debugger implementation for the Neon VM.
//!
//! This module provides an interactive stdin/stdout debugger that allows
//! step-through execution of Neon programs with runtime state inspection.

use std::io::{self, Write};

use crate::common::opcodes::OpCode;
use crate::common::Bloq;
use crate::vm::debug::{DebugCommand, DebugContext, DebugHandler};

/// CLI debugger that provides interactive step-through debugging via stdin/stdout.
///
/// Displays the call stack, current instruction, and value stack at each step,
/// and allows the user to control execution with simple commands.
pub struct CliDebugger {
    /// Tracks whether we're in continuous execution mode
    continuous: bool,
}

impl CliDebugger {
    /// Creates a new CLI debugger instance.
    pub fn new() -> Self {
        Self { continuous: false }
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

        let bloq = &current_frame.function.bloq;
        let ip = context.current_ip;

        // Display instruction pointer and line number
        print!("  0x{:04x} ", ip);

        if let Some(source_loc) = bloq.get_source_location(ip) {
            print!("{}:{} ", source_loc.line, source_loc.column);
        } else {
            print!("      ");
        }

        // Disassemble the instruction
        self.disassemble_instruction(bloq, ip);
    }

    /// Disassembles a single instruction at the given offset.
    ///
    /// This is a simplified version adapted from the Bloq disassembler,
    /// returning formatted output instead of advancing through instructions.
    fn disassemble_instruction(&self, bloq: &Bloq, offset: usize) {
        if offset >= bloq.instruction_count() {
            println!("(out of bounds)");
            return;
        }

        let instruction = OpCode::from_u8(bloq.read_u8(offset));

        match instruction {
            // Simple instructions (no operands)
            OpCode::Return
            | OpCode::Negate
            | OpCode::Add
            | OpCode::Subtract
            | OpCode::Multiply
            | OpCode::Divide
            | OpCode::Modulo
            | OpCode::Nil
            | OpCode::True
            | OpCode::False
            | OpCode::Equal
            | OpCode::Greater
            | OpCode::Less
            | OpCode::Not
            | OpCode::Print
            | OpCode::Pop
            | OpCode::Loop => {
                println!("{:?}", instruction);
            }

            // Constant instructions
            OpCode::Constant | OpCode::Constant2 | OpCode::Constant4 => {
                let index = self.read_operand_index(bloq, &instruction, offset + 1);
                let constant = bloq.read_constant(index);
                println!("{:?} {} '{}'", instruction, index, constant);
            }

            // String instructions
            OpCode::String | OpCode::String2 | OpCode::String4 => {
                let index = self.read_operand_index(bloq, &instruction, offset + 1);
                let string = bloq.read_string(index);
                println!("{:?} {} '{}'", instruction, index, string);
            }

            // Variable instructions (local/global get/set)
            OpCode::GetLocal
            | OpCode::GetLocal2
            | OpCode::GetLocal4
            | OpCode::SetLocal
            | OpCode::SetLocal2
            | OpCode::SetLocal4
            | OpCode::GetGlobal
            | OpCode::GetGlobal2
            | OpCode::GetGlobal4
            | OpCode::SetGlobal
            | OpCode::SetGlobal2
            | OpCode::SetGlobal4 => {
                let index = self.read_operand_index(bloq, &instruction, offset + 1);
                let constant = bloq.read_constant(index);
                println!("{:?} {} '{}'", instruction, index, constant);
            }

            // Field instructions
            OpCode::GetField
            | OpCode::GetField2
            | OpCode::GetField4
            | OpCode::SetField
            | OpCode::SetField2
            | OpCode::SetField4 => {
                let index = self.read_operand_index(bloq, &instruction, offset + 1);
                let field_name = bloq.read_string(index);
                println!("{:?} {} '{}'", instruction, index, field_name);
            }

            // Jump instructions
            OpCode::JumpIfFalse | OpCode::Jump => {
                let jump = bloq.read_u32(offset + 1);
                println!(
                    "{:?} 0x{:04x} -> 0x{:04x}",
                    instruction,
                    offset,
                    offset + 5 + jump as usize
                );
            }

            // Call instruction
            OpCode::Call => {
                let arg_count = bloq.read_u8(offset + 1);
                println!("Call (args: {})", arg_count);
            }
        }
    }

    /// Reads the operand index based on instruction variant (u8, u16, or u32).
    fn read_operand_index(&self, bloq: &Bloq, instruction: &OpCode, offset: usize) -> usize {
        // Determine size based on instruction variant
        let is_4byte = matches!(
            instruction,
            OpCode::Constant4
                | OpCode::String4
                | OpCode::GetLocal4
                | OpCode::SetLocal4
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
                | OpCode::GetGlobal2
                | OpCode::SetGlobal2
                | OpCode::GetField2
                | OpCode::SetField2
        );

        if is_4byte {
            bloq.read_u32(offset) as usize
        } else if is_2byte {
            bloq.read_u16(offset) as usize
        } else {
            bloq.read_u8(offset) as usize
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

    /// Displays help text for available commands.
    fn display_help(&self) {
        println!("\n=== Debugger Commands ===");
        println!("  <Enter>  - Step to next instruction");
        println!("  c        - Continue execution (run until completion)");
        println!("  q        - Quit debugger and exit program");
        println!("  h        - Show this help message");
    }

    /// Prompts the user for a command and returns the corresponding DebugCommand.
    ///
    /// Handles EOF gracefully by treating it as a quit command.
    fn prompt_user(&mut self) -> DebugCommand {
        print!("\n> ");
        if let Err(_) = io::stdout().flush() {
            // If we can't flush stdout, treat as quit
            return DebugCommand::Quit;
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF reached, treat as quit
                println!("\n(EOF - exiting debugger)");
                DebugCommand::Quit
            }
            Ok(_) => {
                let trimmed = input.trim();
                match trimmed {
                    "" => DebugCommand::Step,
                    "c" => {
                        self.continuous = true;
                        DebugCommand::Continue
                    }
                    "q" => DebugCommand::Quit,
                    "h" => {
                        self.display_help();
                        // After showing help, prompt again
                        self.prompt_user()
                    }
                    _ => {
                        println!("Unknown command: '{}'. Press 'h' for help.", trimmed);
                        self.prompt_user()
                    }
                }
            }
            Err(_) => {
                // Read error, treat as quit
                println!("\n(read error - exiting debugger)");
                DebugCommand::Quit
            }
        }
    }
}

impl DebugHandler for CliDebugger {
    fn on_step(&mut self, context: &DebugContext) -> DebugCommand {
        // If in continuous mode, keep going
        if self.continuous {
            return DebugCommand::Continue;
        }

        // Display current state
        self.display_call_stack(context);
        self.display_current_instruction(context);
        self.display_value_stack(context);

        // Get user command
        self.prompt_user()
    }
}

impl Default for CliDebugger {
    fn default() -> Self {
        Self::new()
    }
}
