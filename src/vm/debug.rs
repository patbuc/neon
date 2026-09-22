//! Debug infrastructure for the Neon VM.
//!
//! This module defines the core types and traits that enable step-through debugging
//! of Neon programs. It provides a contract between the VM and debug frontends
//! (CLI, IDE, etc.) through the `DebugHandler` trait.

use crate::common::{CallFrame, Value};

/// Commands that can be issued by a debug handler to control execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugCommand {
    /// Execute the next instruction and pause.
    Step,
    /// Continue execution until the next breakpoint or completion.
    Continue,
    /// Stop execution and exit the debugger.
    Quit,
}

/// Context information provided to debug handlers at each step.
///
/// This structure gives the debug handler a read-only view of the VM's
/// execution state, including the call stack, value stack, and instruction pointer.
#[derive(Debug)]
pub struct DebugContext<'a> {
    /// The current call frames (function call stack).
    pub call_frames: &'a [CallFrame],
    /// The current value stack.
    pub stack: &'a [Value],
    /// The current instruction pointer (offset in bytecode).
    pub current_ip: usize,
    /// The stack slot where the current frame's local variables start.
    /// Can be -1 for the script frame.
    pub slot_start: isize,
}

/// Trait for implementing debug handlers.
///
/// A debug handler is called at each step during VM execution when debugging
/// is enabled. It can inspect the execution state and return commands to
/// control further execution.
pub trait DebugHandler {
    /// Called at each step of execution.
    ///
    /// # Parameters
    /// - `context`: Read-only view of the current VM state
    ///
    /// # Returns
    /// A `DebugCommand` indicating how execution should proceed.
    fn on_step(&mut self, context: &DebugContext) -> DebugCommand;
}
