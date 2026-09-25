use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};

/// Code generator for the multi-pass compiler
/// Generates bytecode from AST using the semantic pass's resolutions
use crate::common::opcodes::OpCode;
use crate::common::{Chunk, SourceLocation, Value};
use crate::compiler::ast::{BinaryOp, Expr, NodeId, Stmt, UnaryOp};
use crate::compiler::resolutions::{Capture, DeclId, Res, Resolutions};
use crate::{number, string};
use indexmap::IndexMap;
use std::collections::HashMap;

struct Local {
    depth: u32,
    is_captured: bool,
}

impl Local {
    fn new(depth: u32, is_captured: bool) -> Self {
        Local { depth, is_captured }
    }
}

struct LoopContext {
    #[allow(dead_code)]
    loop_start: u32,
    break_jumps: Vec<u32>,
    continue_jumps: Vec<u32>,
    /// Locals deeper than this are popped by a break/continue before it jumps.
    depth: u32,
}

enum LoopExit {
    Break,
    Continue,
}

struct FunctionCompiler {
    chunk: Chunk,
    locals: Vec<Local>,
    scope_depth: u32,
    loop_contexts: Vec<LoopContext>,
}

impl FunctionCompiler {
    fn new(name: &str) -> Self {
        FunctionCompiler {
            chunk: Chunk::new(name),
            locals: Vec::new(),
            scope_depth: 0,
            loop_contexts: Vec::new(),
        }
    }

    fn add_parameter(&mut self, local: Local) {
        // Parameters are already on the stack, just register them
        self.locals.push(local);
    }

    fn define_local(&mut self, local: Local, line: u32, column: u32) {
        self.locals.push(local);
        let index = (self.locals.len() - 1) as u32;
        self.chunk
            .write_op_code_variant(OpCode::SetLocal, index, line, column);
    }

    /// Drops locals declared deeper than `depth`, returning whether each one
    /// was captured (top-most local first), so the caller can emit
    /// CloseUpvalue instead of a plain Pop for it.
    fn pop_locals_above(&mut self, depth: u32) -> Vec<bool> {
        let mut captured = Vec::new();
        while let Some(local) = self.locals.last() {
            if local.depth <= depth {
                break;
            }
            captured.push(local.is_captured);
            self.locals.pop();
        }
        captured
    }

    /// Same as `pop_locals_above`, without removing the locals: for a
    /// break/continue jump, which unwinds the runtime stack early but
    /// leaves the compile-time locals in scope for the code that follows.
    fn captured_flags_above(&self, depth: u32) -> Vec<bool> {
        let mut captured = Vec::new();
        for local in self.locals.iter().rev() {
            if local.depth <= depth {
                break;
            }
            captured.push(local.is_captured);
        }
        captured
    }
}

pub struct CodeGenerator<'a> {
    /// One entry per level of function nesting, outermost (the script) first.
    functions: Vec<FunctionCompiler>,
    errors: Vec<CompilationError>,
    #[allow(dead_code)]
    builtin: indexmap::IndexMap<String, Value>,
    resolutions: &'a Resolutions,
    /// Slot of each declaration in the `locals` of the function that owns it.
    /// `DeclId`s are unique program-wide, so one map serves every function.
    decl_slots: HashMap<DeclId, u32>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(builtin: IndexMap<String, Value>, resolutions: &'a Resolutions) -> Self {
        CodeGenerator {
            functions: vec![FunctionCompiler::new("main")],
            errors: Vec::new(),
            builtin,
            resolutions,
            decl_slots: HashMap::new(),
        }
    }

    pub fn generate(&mut self, statements: &[Stmt]) -> CompilationResult<Chunk> {
        // First: Define all functions and structs with placeholders
        // This allows forward references to work
        for stmt in statements {
            match stmt {
                Stmt::Fn { id, location, .. } => {
                    // Define function with nil placeholder
                    self.emit_op_code(OpCode::Nil, *location);
                    let decl = self.resolutions.decl(*id);
                    self.bind_decl_local(decl, *location);
                }
                Stmt::Struct {
                    name,
                    fields,
                    id,
                    location,
                    ..
                } => {
                    // Create the struct value
                    let struct_value = Value::new_struct(name.clone(), fields.clone());
                    self.emit_constant(struct_value, *location);
                    let decl = self.resolutions.decl(*id);
                    self.bind_decl_local(decl, *location);
                }
                _ => {}
            }
        }

        // Then: Compile impl-block methods into closures and register them,
        // so a method call textually before its `impl` block still works.
        for stmt in statements {
            if let Stmt::Impl {
                type_name, methods, ..
            } = stmt
            {
                for method in methods {
                    if let Stmt::Fn {
                        name,
                        params,
                        body,
                        id,
                        location,
                    } = method
                    {
                        self.generate_closure(*id, name, params, body, *location);
                        let takes_self = params.first().map(String::as_str) == Some("self");
                        self.emit_define_method(type_name, name, takes_self, *location);
                    }
                }
            }
        }

        // Then: Generate code for all statements
        for stmt in statements {
            self.generate_stmt(stmt);
        }

        // Emit final return
        self.emit_return();

        if self.errors.is_empty() {
            Ok(self.functions.pop().unwrap().chunk)
        } else {
            Err(self.errors.clone())
        }
    }

    // ===== Helper Methods =====

    fn current(&mut self) -> &mut FunctionCompiler {
        self.functions.last_mut().unwrap()
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.current().chunk
    }

    fn emit_op_code(&mut self, op_code: OpCode, location: SourceLocation) {
        self.current_chunk()
            .write_op_code(op_code, location.line, location.column);
    }

    fn emit_op_code_variant(&mut self, op_code: OpCode, index: u32, location: SourceLocation) {
        self.current_chunk()
            .write_op_code_variant(op_code, index, location.line, location.column);
    }

    /// Slot of `decl` in the `locals` of the function that owns it.
    fn decl_slot(&self, decl: DeclId) -> u32 {
        *self
            .decl_slots
            .get(&decl)
            .unwrap_or_else(|| panic!("no slot recorded for {:?}", decl))
    }

    /// Pushes a new local bound to `decl`, capturing whether it's captured
    /// from the resolutions, and records its slot.
    fn bind_local(&mut self, decl: DeclId, push: impl FnOnce(&mut FunctionCompiler, Local)) {
        let is_captured = self.resolutions.is_captured(decl);
        let depth = self.current().scope_depth;
        let local = Local::new(depth, is_captured);
        push(self.current(), local);
        let slot = (self.current().locals.len() - 1) as u32;
        self.decl_slots.insert(decl, slot);
    }

    /// Pushes a new local bound to `decl` on top of the current function's
    /// stack. The value it binds must already be on the stack.
    fn bind_decl_local(&mut self, decl: DeclId, location: SourceLocation) {
        self.bind_local(decl, |function, local| {
            function.define_local(local, location.line, location.column)
        });
    }

    /// Registers a function parameter, already on the stack from the call,
    /// as a local bound to `decl`.
    fn bind_param(&mut self, decl: DeclId) {
        self.bind_local(decl, |function, local| function.add_parameter(local));
    }

    fn emit_variable_get(&mut self, id: NodeId, location: SourceLocation) {
        let (op_code, index) = match self.resolutions.res(id) {
            Res::Local(decl) => (OpCode::GetLocal, self.decl_slot(decl)),
            Res::Global(decl) => (OpCode::GetGlobal, self.decl_slot(decl)),
            Res::Upvalue(index) => (OpCode::GetUpvalue, index),
            Res::Builtin(index) => (OpCode::GetBuiltin, index),
        };
        self.emit_op_code_variant(op_code, index, location);
    }

    fn emit_variable_set(&mut self, id: NodeId, location: SourceLocation) {
        let (op_code, index) = match self.resolutions.res(id) {
            Res::Local(decl) => (OpCode::SetLocal, self.decl_slot(decl)),
            Res::Global(decl) => (OpCode::SetGlobal, self.decl_slot(decl)),
            Res::Upvalue(index) => (OpCode::SetUpvalue, index),
            Res::Builtin(_) => unreachable!("the semantic pass rejects assignment to a builtin"),
        };
        self.emit_op_code_variant(op_code, index, location);
    }

    fn emit_upvalue_metadata(&mut self, captures: &[Capture], location: SourceLocation) {
        let message = format!(
            "function captures too many variables: {} (maximum is {})",
            captures.len(),
            u8::MAX
        );
        if self
            .check_count_limit(captures.len(), u8::MAX as usize, message, location)
            .is_none()
        {
            return;
        }
        self.current_chunk().write_u8(captures.len() as u8);

        for capture in captures {
            let (is_local, index) = match *capture {
                Capture::Local(decl) => (true, self.decl_slot(decl)),
                Capture::Upvalue(index) => (false, index),
            };
            let message = format!(
                "captured variable index too large: {} (maximum is {})",
                index,
                u16::MAX
            );
            if self
                .check_count_limit(index as usize, u16::MAX as usize, message, location)
                .is_none()
            {
                return;
            }
            self.current_chunk().write_u8(if is_local { 1 } else { 0 });
            self.current_chunk().write_u16(index as u16);
        }
    }

    /// Verifies a count fits within `max` before it is narrowed into a
    /// bytecode operand, reporting a compile error naming the limit instead
    /// of letting the narrowing cast wrap silently.
    fn check_count_limit(
        &mut self,
        count: usize,
        max: usize,
        message: impl Into<String>,
        location: SourceLocation,
    ) -> Option<()> {
        if count > max {
            self.errors.push(CompilationError::new(
                CompilationPhase::Codegen,
                CompilationErrorKind::Other,
                message.into(),
                location,
            ));
            None
        } else {
            Some(())
        }
    }

    fn emit_constant(&mut self, value: Value, location: SourceLocation) {
        self.current_chunk()
            .write_constant(value, location.line, location.column);
    }

    fn emit_string(&mut self, value: Value, location: SourceLocation) {
        self.current_chunk()
            .write_string(value, location.line, location.column);
    }

    fn emit_return(&mut self) {
        let location = SourceLocation {
            offset: 0,
            line: 0,
            column: 0,
        };
        self.emit_op_code(OpCode::Nil, location);
        self.emit_op_code(OpCode::Return, location);
    }

    fn emit_jump(&mut self, op_code: OpCode, location: SourceLocation) -> u32 {
        self.current_chunk()
            .emit_jump(op_code, location.line, location.column)
    }

    fn patch_jump(&mut self, offset: u32) {
        self.current_chunk().patch_jump(offset);
    }

    fn emit_loop(&mut self, loop_start: u32, location: SourceLocation) {
        self.current_chunk()
            .emit_loop(loop_start, location.line, location.column);
    }

    // ===== Statement Generation =====

    fn generate_variable_declaration(
        &mut self,
        id: NodeId,
        initializer: &Option<Expr>,
        location: SourceLocation,
    ) {
        // Generate initializer or nil
        if let Some(init) = initializer {
            self.generate_expr(init);
        } else {
            self.emit_op_code(OpCode::Nil, location);
        }

        // Define local variable
        let decl = self.resolutions.decl(id);
        self.bind_decl_local(decl, location);
    }

    fn generate_fn_stmt(
        &mut self,
        id: NodeId,
        name: &str,
        params: &[String],
        body: &[Stmt],
        location: SourceLocation,
    ) {
        // A nested function isn't pre-defined by generate()'s pre-pass, so define it now, before compiling its body, so it can recurse.
        if self.current().scope_depth > 0 {
            self.emit_op_code(OpCode::Nil, location);
            let decl = self.resolutions.decl(id);
            self.bind_decl_local(decl, location);
        }

        self.generate_closure(id, name, params, body, location);

        // Store the closure into the local defined for the function's name.
        let slot = self.decl_slot(self.resolutions.decl(id));
        self.emit_op_code_variant(OpCode::SetLocal, slot, location);
        self.emit_op_code(OpCode::Pop, location); // Pop the function value from the stack
    }

    /// Compiles `params`/`body` into a closure and leaves it on top of the
    /// stack. Shared by named function declarations, which then store it
    /// into the variable defined for the name, and lambda expressions,
    /// which leave it as their expression value.
    fn generate_closure(
        &mut self,
        id: NodeId,
        name: &str,
        params: &[String],
        body: &[Stmt],
        location: SourceLocation,
    ) {
        self.functions
            .push(FunctionCompiler::new(&format!("function_{}", name)));

        // Enter function scope
        self.current().scope_depth += 1;

        // Define parameters as local variables in the function scope
        let resolutions = self.resolutions;
        for &decl in &resolutions.function(id).params {
            self.bind_param(decl);
        }

        // Compile function body
        for stmt in body {
            self.generate_stmt(stmt);
        }

        // Emit return at end of function
        self.emit_return();

        let compiler = self.functions.pop().unwrap();
        let function_value =
            Value::new_function(name.to_string(), params.len() as u8, compiler.chunk);

        // Wrap the function in a closure.
        let const_index = self.current_chunk().add_constant(function_value);
        self.emit_op_code_variant(OpCode::Closure, const_index, location);

        self.emit_upvalue_metadata(&resolutions.function(id).upvalues, location);
    }

    fn generate_expression_stmt(&mut self, expr: &Expr, location: SourceLocation) {
        self.generate_expr(expr);
        self.emit_op_code(OpCode::Pop, location);
    }

    fn generate_block_stmt(&mut self, statements: &[Stmt], location: SourceLocation) {
        self.current().scope_depth += 1;
        for stmt in statements {
            self.generate_stmt(stmt);
        }
        self.end_scope(location);
    }

    fn end_scope(&mut self, location: SourceLocation) {
        self.current().scope_depth -= 1;
        let captured = self.discard_locals_above_current_depth();
        self.emit_scope_exit(&captured, location);
    }

    fn discard_locals_above_current_depth(&mut self) -> Vec<bool> {
        let scope_depth = self.current().scope_depth;
        self.current().pop_locals_above(scope_depth)
    }

    /// Emits one instruction per popped local, top-most first: CloseUpvalue
    /// for one a nested function captured, Pop otherwise.
    fn emit_scope_exit(&mut self, captured: &[bool], location: SourceLocation) {
        for &is_captured in captured {
            let op_code = if is_captured {
                OpCode::CloseUpvalue
            } else {
                OpCode::Pop
            };
            self.emit_op_code(op_code, location);
        }
    }

    fn generate_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
        location: SourceLocation,
    ) {
        self.generate_expr(condition);

        let then_jump = self.emit_jump(OpCode::JumpIfFalse, location);
        self.emit_op_code(OpCode::Pop, location); // Pop condition if true (not jumping)
        self.generate_stmt(then_branch);
        let else_jump = self.emit_jump(OpCode::Jump, location);
        self.patch_jump(then_jump);
        self.emit_op_code(OpCode::Pop, location); // Pop condition if false (jumped here)

        if let Some(else_stmt) = else_branch {
            self.generate_stmt(else_stmt);
        }
        self.patch_jump(else_jump);
    }

    fn generate_while_stmt(&mut self, condition: &Expr, body: &Stmt, location: SourceLocation) {
        let loop_start = self.current_chunk().instruction_count() as u32;

        // Push loop context for break/continue tracking
        let depth = self.current().scope_depth;
        self.current().loop_contexts.push(LoopContext {
            loop_start,
            break_jumps: Vec::new(),
            continue_jumps: Vec::new(),
            depth,
        });

        self.generate_expr(condition);

        let exit_jump = self.emit_jump(OpCode::JumpIfFalse, location);
        self.emit_op_code(OpCode::Pop, location); // Pop the condition value for the true case

        self.generate_stmt(body);

        // Pop loop context; continue jumps land here, before the Loop back.
        let loop_context = self.current().loop_contexts.pop().unwrap();
        for continue_jump in loop_context.continue_jumps {
            self.patch_jump(continue_jump);
        }

        self.emit_loop(loop_start, location);

        self.patch_jump(exit_jump);
        self.emit_op_code(OpCode::Pop, location); // Pop the condition value for the false case (exiting loop)

        // Patch all break jumps
        for break_jump in loop_context.break_jumps {
            self.patch_jump(break_jump);
        }
    }

    /// Compiles a C-style for loop so each iteration gets its own binding of
    /// the loop variable (JS/Go 1.22 semantics): a closure made in the body
    /// keeps that iteration's value, one made in the condition or increment
    /// captures the next iteration's.
    ///
    /// Bytecode structure:
    ///   <initializer>            ; defines the loop variable as a local
    ///   Jump skip
    ///   loop_start:
    ///   <increment>  Pop
    ///   skip:
    ///   <condition>  JumpIfFalse exit  Pop
    ///   <body>                   ; break/continue pop body locals only
    ///   continue_target:
    ///   CloseUpvalueInPlace      ; only emitted if the loop variable is captured
    ///   Loop loop_start
    ///   exit: Pop                ; the false condition
    ///   break_target:
    ///   <end loop scope: Pop or CloseUpvalue for the loop variable>
    fn generate_for_stmt(
        &mut self,
        initializer: &Stmt,
        condition: &Expr,
        increment: &Expr,
        body: &Stmt,
        location: SourceLocation,
    ) {
        self.current().scope_depth += 1;
        self.generate_stmt(initializer);

        let skip_jump = self.emit_jump(OpCode::Jump, location);
        let loop_start = self.current_chunk().instruction_count() as u32;
        self.generate_expression_stmt(increment, *increment.location());
        self.patch_jump(skip_jump);

        let depth = self.current().scope_depth;
        self.current().loop_contexts.push(LoopContext {
            loop_start,
            break_jumps: Vec::new(),
            continue_jumps: Vec::new(),
            depth,
        });

        self.generate_expr(condition);
        let exit_jump = self.emit_jump(OpCode::JumpIfFalse, location);
        self.emit_op_code(OpCode::Pop, location); // Pop the condition value for the true case

        self.generate_stmt(body);

        // Pop loop context; continue jumps land here, before the Loop back.
        let loop_context = self.current().loop_contexts.pop().unwrap();
        for continue_jump in loop_context.continue_jumps {
            self.patch_jump(continue_jump);
        }

        let loop_variable_captured = self
            .current()
            .locals
            .last()
            .map(|local| local.is_captured)
            .unwrap_or(false);
        if loop_variable_captured {
            self.emit_op_code(OpCode::CloseUpvalueInPlace, location);
        }

        self.emit_loop(loop_start, location);

        self.patch_jump(exit_jump);
        self.emit_op_code(OpCode::Pop, location); // Pop the condition value for the false case (exiting loop)

        // Patch all break jumps
        for break_jump in loop_context.break_jumps {
            self.patch_jump(break_jump);
        }

        self.end_scope(location);
    }

    fn generate_return_stmt(&mut self, value: &Expr, location: SourceLocation) {
        self.generate_expr(value);
        self.emit_op_code(OpCode::Return, location);
    }

    // Leaves the locals in place; end_scope still owns them on fall-through.
    fn emit_loop_exit_pops(&mut self, depth: u32, location: SourceLocation) {
        let captured = self.current().captured_flags_above(depth);
        self.emit_scope_exit(&captured, location);
    }

    fn generate_loop_exit_stmt(&mut self, exit: LoopExit, location: SourceLocation) {
        // Emit a Jump opcode and record it for later patching. For continue,
        // this allows jumping to the right place, just before the Loop
        // instruction.
        let depth = self
            .current()
            .loop_contexts
            .last()
            .expect("semantic pass guarantees a loop context")
            .depth;
        self.emit_loop_exit_pops(depth, location);

        let jump_index = self.emit_jump(OpCode::Jump, location);
        let context = self.current().loop_contexts.last_mut().unwrap();
        let jumps = match exit {
            LoopExit::Break => &mut context.break_jumps,
            LoopExit::Continue => &mut context.continue_jumps,
        };
        jumps.push(jump_index);
    }

    fn generate_for_in_stmt(
        &mut self,
        id: NodeId,
        collection: &Expr,
        body: &Stmt,
        location: SourceLocation,
    ) {
        // For-in loop code generation strategy: uses iterator opcodes rather
        // than the increment/condition structure of a C-style for loop.
        //
        // Bytecode structure:
        //   <evaluate collection>
        //   GetIterator              ; Convert collection to iterator state (VM internal)
        //   loop_start:
        //   IteratorDone            ; Check if has more (pushes true if more, false if done)
        //   JumpIfFalse exit_jump   ; If false (done), exit loop
        //   Pop                     ; Pop the true value (has more)
        //   IteratorNext            ; Get next value (pushes value onto stack)
        //   <body with loop variable>       ; break/continue pop the loop variable
        //                                    ; and any body locals before jumping
        //   Pop                     ; Pop the loop variable value
        //   Loop loop_start         ; Jump back
        //   exit_jump:
        //   Pop                     ; Pop the false value (done)
        //   <break lands here>
        //   PopIterator             ; Pop the iterator from the VM's iterator stack

        // Evaluate the collection expression
        self.generate_expr(collection);

        // Convert collection to iterator (stores iterator state in VM)
        self.emit_op_code(OpCode::GetIterator, location);

        // Enter a block scope for the loop
        self.current().scope_depth += 1;

        // Mark the start of the loop
        let loop_start = self.current_chunk().instruction_count() as u32;

        // Push loop context for break/continue tracking
        // - 1 so break/continue also pop the loop variable itself.
        let depth = self.current().scope_depth - 1;
        self.current().loop_contexts.push(LoopContext {
            loop_start,
            break_jumps: Vec::new(),
            continue_jumps: Vec::new(),
            depth,
        });

        // Check if iterator has more elements (pushes true if more, false if done)
        self.emit_op_code(OpCode::IteratorDone, location);

        // JumpIfFalse exits when false (done/no more elements)
        let exit_jump = self.emit_jump(OpCode::JumpIfFalse, location);

        // Pop the true value (has more elements, continuing loop)
        self.emit_op_code(OpCode::Pop, location);

        // Get next value from iterator (pushes value)
        self.emit_op_code(OpCode::IteratorNext, location);

        // Define the loop variable (value is already on stack from IteratorNext)
        let decl = self.resolutions.decl(id);
        self.bind_decl_local(decl, location);

        // Generate the loop body
        self.generate_stmt(body);

        let loop_variable_captured = self
            .current()
            .locals
            .last()
            .map(|local| local.is_captured)
            .unwrap_or(false);
        let exit_op = if loop_variable_captured {
            OpCode::CloseUpvalue
        } else {
            OpCode::Pop
        };
        self.emit_op_code(exit_op, location);

        // Patch all continue jumps to point here (just before the Loop)
        // This allows continue to properly skip to the next iteration
        let loop_context = self.current().loop_contexts.pop().unwrap();
        for continue_jump in loop_context.continue_jumps {
            self.patch_jump(continue_jump);
        }

        // Jump back to loop start (will push next value)
        self.emit_loop(loop_start, location);

        // Patch the exit jump
        self.patch_jump(exit_jump);

        // Pop the false value (done/no more elements)
        self.emit_op_code(OpCode::Pop, location);

        // Patch all break jumps
        for break_jump in loop_context.break_jumps {
            self.patch_jump(break_jump);
        }

        // Pop the iterator from the VM's iterator stack
        self.emit_op_code(OpCode::PopIterator, location);

        // Exit the loop scope. The loop variable's slot was already popped
        // above, so only its Local entry needs dropping here.
        self.current().scope_depth -= 1;
        self.discard_locals_above_current_depth();
    }

    fn generate_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Val {
                initializer,
                id,
                location,
                ..
            } => {
                self.generate_variable_declaration(*id, initializer, *location);
            }
            Stmt::Var {
                initializer,
                id,
                location,
                ..
            } => {
                self.generate_variable_declaration(*id, initializer, *location);
            }
            Stmt::Fn {
                name,
                params,
                body,
                id,
                location,
            } => {
                self.generate_fn_stmt(*id, name, params, body, *location);
            }
            Stmt::Struct { .. } => {
                // Struct was already defined, nothing to do here
            }
            Stmt::Impl { .. } => {
                // Methods were already compiled and registered in generate()'s pre-pass.
            }
            Stmt::Expression { expr, location } => {
                self.generate_expression_stmt(expr, *location);
            }
            Stmt::Block {
                statements,
                location,
            } => {
                self.generate_block_stmt(statements, *location);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                location,
            } => {
                self.generate_if_stmt(condition, then_branch, else_branch, *location);
            }
            Stmt::While {
                condition,
                body,
                location,
            } => {
                self.generate_while_stmt(condition, body, *location);
            }
            Stmt::Return { value, location } => {
                self.generate_return_stmt(value, *location);
            }
            Stmt::Break { location } => {
                self.generate_loop_exit_stmt(LoopExit::Break, *location);
            }
            Stmt::Continue { location } => {
                self.generate_loop_exit_stmt(LoopExit::Continue, *location);
            }
            Stmt::ForIn {
                collection,
                body,
                id,
                location,
                ..
            } => {
                self.generate_for_in_stmt(*id, collection, body, *location);
            }
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
                location,
            } => {
                self.generate_for_stmt(initializer, condition, increment, body, *location);
            }
        }
    }

    // ===== Expression Generation =====

    fn generate_string_interpolation_expr(
        &mut self,
        parts: &[crate::compiler::ast::InterpolationPart],
        location: SourceLocation,
    ) {
        use crate::compiler::ast::InterpolationPart;

        // Generate code for each part and concatenate them
        let mut first = true;
        for part in parts {
            match part {
                InterpolationPart::Literal(s) => {
                    self.emit_string(string!(s.as_str()), location);
                }
                InterpolationPart::Expression(expr) => {
                    // Generate the expression
                    self.generate_expr(expr);
                    // Convert to string using ToString opcode
                    self.emit_op_code(OpCode::ToString, location);
                }
            }

            // Concatenate with previous parts (skip for first part)
            if !first {
                self.emit_op_code(OpCode::Add, location);
            }
            first = false;
        }

        // If there are no parts, emit an empty string
        if parts.is_empty() {
            self.emit_string(string!(""), location);
        }
    }

    fn generate_binary_expr(
        &mut self,
        left: &Expr,
        operator: &BinaryOp,
        right: &Expr,
        location: SourceLocation,
    ) {
        // Handle short-circuit operators specially
        match operator {
            BinaryOp::And => {
                // For `a && b`:
                // 1. Evaluate left operand
                self.generate_expr(left);
                // 2. If false, skip right operand and result is false
                let end_jump = self.emit_jump(OpCode::JumpIfFalse, location);
                // 3. Left was true, pop it and evaluate right
                self.emit_op_code(OpCode::Pop, location);
                self.generate_expr(right);
                // 4. Patch jump to end (if left was false, we skip here with false on stack)
                self.patch_jump(end_jump);
            }
            BinaryOp::Or => {
                // For `a || b`:
                // 1. Evaluate left operand
                self.generate_expr(left);
                // 2. If false, jump to evaluate right operand
                let else_jump = self.emit_jump(OpCode::JumpIfFalse, location);
                // 3. Left was true, jump to end with true result
                let end_jump = self.emit_jump(OpCode::Jump, location);
                // 4. Patch else jump (left was false, need to evaluate right)
                self.patch_jump(else_jump);
                // 5. Pop false value and evaluate right
                self.emit_op_code(OpCode::Pop, location);
                self.generate_expr(right);
                // 6. Patch end jump (left was true, skip right evaluation)
                self.patch_jump(end_jump);
            }
            _ => {
                // Regular binary operators: evaluate both operands first
                self.generate_expr(left);
                self.generate_expr(right);

                match operator {
                    BinaryOp::Add => self.emit_op_code(OpCode::Add, location),
                    BinaryOp::Subtract => self.emit_op_code(OpCode::Subtract, location),
                    BinaryOp::Multiply => self.emit_op_code(OpCode::Multiply, location),
                    BinaryOp::Divide => self.emit_op_code(OpCode::Divide, location),
                    BinaryOp::Modulo => self.emit_op_code(OpCode::Modulo, location),
                    BinaryOp::Exponent => self.emit_op_code(OpCode::Exponent, location),
                    BinaryOp::Equal => self.emit_op_code(OpCode::Equal, location),
                    BinaryOp::NotEqual => {
                        self.emit_op_code(OpCode::Equal, location);
                        self.emit_op_code(OpCode::Not, location);
                    }
                    BinaryOp::Greater => self.emit_op_code(OpCode::Greater, location),
                    BinaryOp::GreaterEqual => self.emit_op_code(OpCode::GreaterEqual, location),
                    BinaryOp::Less => self.emit_op_code(OpCode::Less, location),
                    BinaryOp::LessEqual => self.emit_op_code(OpCode::LessEqual, location),
                    BinaryOp::BitwiseAnd => self.emit_op_code(OpCode::BitwiseAnd, location),
                    BinaryOp::BitwiseOr => self.emit_op_code(OpCode::BitwiseOr, location),
                    BinaryOp::BitwiseXor => self.emit_op_code(OpCode::BitwiseXor, location),
                    BinaryOp::LeftShift => self.emit_op_code(OpCode::LeftShift, location),
                    BinaryOp::RightShift => self.emit_op_code(OpCode::RightShift, location),
                    BinaryOp::And | BinaryOp::Or => unreachable!(),
                }
            }
        }
    }

    fn generate_call_expr(&mut self, callee: &Expr, arguments: &[Expr], location: SourceLocation) {
        // Check if this is a global function call (e.g., print("hello"))
        let is_global_function = if let Expr::Variable { name, .. } = callee {
            // Check if this is a global function by looking it up with empty namespace
            crate::common::method_registry::get_native_method_index("", name).is_some()
        } else {
            false
        };

        // Check if this is a constructor call (e.g., File("path"))
        let is_constructor_call = if let Expr::Variable { name, .. } = callee {
            crate::common::method_registry::get_native_method_index(name, "new").is_some()
        } else {
            false
        };

        if is_global_function {
            self.generate_global_call_expr(callee, arguments, location);
        } else if is_constructor_call {
            self.generate_constructor_call_expr(callee, arguments, location);
        } else {
            self.generate_regular_call_expr(callee, arguments, location);
        }
    }

    fn generate_regular_call_expr(
        &mut self,
        callee: &Expr,
        arguments: &[Expr],
        location: SourceLocation,
    ) {
        // Unified calling convention: [callable, args...]
        self.generate_expr(callee);

        for arg in arguments {
            self.generate_expr(arg);
        }

        self.emit_call(arguments.len() as u8, location);
    }

    fn generate_constructor_call_expr(
        &mut self,
        callee: &Expr,
        arguments: &[Expr],
        location: SourceLocation,
    ) {
        // Constructor call: File("path")
        let type_name = if let Expr::Variable { name, .. } = callee {
            name.clone()
        } else {
            unreachable!("Already checked this is a Variable")
        };

        // Look up constructor index at compile time
        let index = crate::common::method_registry::get_native_method_index(&type_name, "new")
            .unwrap_or_else(|| panic!("Unknown constructor: {}.new", type_name));

        self.push_native_callable_by_index(
            format!("{}.new", type_name),
            index,
            arguments.len() as u8,
            location,
        );

        for arg in arguments {
            self.generate_expr(arg);
        }

        self.emit_call(arguments.len() as u8, location);
    }

    fn generate_global_call_expr(
        &mut self,
        callee: &Expr,
        arguments: &[Expr],
        location: SourceLocation,
    ) {
        // Global function call: print("hello")
        // Extract function name
        let function_name = if let Expr::Variable { name, .. } = callee {
            name.clone()
        } else {
            unreachable!("Already checked this is a Variable")
        };

        // Look up global function index at compile time
        let index = crate::common::method_registry::get_native_method_index("", &function_name)
            .unwrap_or_else(|| panic!("Unknown function: {}", function_name));

        self.push_native_callable_by_index(
            function_name.to_string(),
            index,
            arguments.len() as u8,
            location,
        );

        for arg in arguments {
            self.generate_expr(arg);
        }

        self.emit_call(arguments.len() as u8, location);
    }

    fn generate_method_call_expr(
        &mut self,
        object: &Expr,
        method: &str,
        arguments: &[Expr],
        location: SourceLocation,
    ) {
        // Check if this is a static method call (e.g., Math.abs)
        let is_static_call = if let Expr::Variable { name, .. } = object {
            crate::common::method_registry::is_static_method(name, method)
        } else {
            false
        };

        if is_static_call {
            self.generate_static_method_call_expr(object, method, arguments, location);
        } else {
            self.generate_instance_method_call_expr(object, method, arguments, location);
        }
    }

    fn generate_instance_method_call_expr(
        &mut self,
        callee: &Expr,
        method: &str,
        arguments: &[Expr],
        location: SourceLocation,
    ) {
        // Instance method call: arr.push(x), str.len(), etc.
        // Type is unknown at compile time, use NativeByName for runtime dispatch

        // The receiver takes one of the u8 argument-count slots alongside arguments
        const MAX_METHOD_CALL_ARGUMENTS: usize = u8::MAX as usize - 1;
        let message = format!(
            "method call too large: {} arguments (maximum is {})",
            arguments.len(),
            MAX_METHOD_CALL_ARGUMENTS
        );
        if self
            .check_count_limit(
                arguments.len(),
                MAX_METHOD_CALL_ARGUMENTS,
                message,
                location,
            )
            .is_none()
        {
            return;
        }

        let arity = (arguments.len() + 1) as u8;
        self.push_native_callable_by_name("".to_string(), method.to_string(), arity, location);

        self.generate_expr(callee);

        for arg in arguments {
            self.generate_expr(arg);
        }

        self.emit_call(arity, location);
    }

    fn generate_static_method_call_expr(
        &mut self,
        object: &Expr,
        method: &str,
        arguments: &[Expr],
        location: SourceLocation,
    ) {
        // Static method call: Math.abs(x)
        // Extract namespace name
        let namespace_name = if let Expr::Variable { name, .. } = object {
            name.clone()
        } else {
            unreachable!("Already checked this is a Variable")
        };

        // Look up static method index at compile time
        let index =
            crate::common::method_registry::get_native_method_index(&namespace_name, method)
                .unwrap_or_else(|| panic!("Unknown static method: {}.{}", namespace_name, method));

        self.push_native_callable_by_index(
            method.to_string(),
            index,
            arguments.len() as u8,
            location,
        );

        for arg in arguments {
            self.generate_expr(arg);
        }

        self.emit_call(arguments.len() as u8, location);
    }

    fn generate_array_literal_expr(&mut self, elements: &[Expr], location: SourceLocation) {
        let message = format!(
            "array literal too large: {} elements (maximum is {})",
            elements.len(),
            u16::MAX
        );
        if self
            .check_count_limit(elements.len(), u16::MAX as usize, message, location)
            .is_none()
        {
            return;
        }

        // Generate code for all elements
        for element in elements {
            self.generate_expr(element);
        }

        // Emit CreateArray with the count of elements
        self.emit_op_code(OpCode::CreateArray, location);
        self.current_chunk().write_u16(elements.len() as u16);
    }

    fn generate_postfix_operation(
        &mut self,
        operand: &Expr,
        operation: OpCode,
        location: SourceLocation,
    ) {
        let Expr::Variable { id, .. } = operand else {
            unreachable!("semantic pass guarantees a postfix operand is a variable")
        };

        // Load old value (will be the return value)
        self.emit_variable_get(*id, location);

        // Load old value again (for modification)
        self.emit_variable_get(*id, location);

        // Push 1 and perform operation (add or subtract)
        self.emit_constant(number!(1.0), location);
        self.emit_op_code(operation, location);

        // Store new value
        self.emit_variable_set(*id, location);

        // Pop the new value, leaving old value on stack
        self.emit_op_code(OpCode::Pop, location);
    }

    fn generate_postfix_increment_expr(&mut self, operand: &Expr, location: SourceLocation) {
        self.generate_postfix_operation(operand, OpCode::Add, location);
    }

    fn generate_postfix_decrement_expr(&mut self, operand: &Expr, location: SourceLocation) {
        self.generate_postfix_operation(operand, OpCode::Subtract, location);
    }

    fn generate_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number { value, location } => {
                self.emit_constant(number!(*value), *location);
            }
            Expr::String { value, location } => {
                self.emit_string(string!(value.as_str()), *location);
            }
            Expr::StringInterpolation { parts, location } => {
                self.generate_string_interpolation_expr(parts, *location);
            }
            Expr::Boolean { value, location } => {
                if *value {
                    self.emit_op_code(OpCode::True, *location);
                } else {
                    self.emit_op_code(OpCode::False, *location);
                }
            }
            Expr::Nil { location } => {
                self.emit_op_code(OpCode::Nil, *location);
            }
            Expr::Variable { id, location, .. } => {
                self.emit_variable_get(*id, *location);
            }
            Expr::Assign {
                value,
                id,
                location,
                ..
            } => {
                self.generate_expr(value);
                self.emit_variable_set(*id, *location);
            }
            Expr::Binary {
                left,
                operator,
                right,
                location,
            } => {
                self.generate_binary_expr(left, operator, right, *location);
            }
            Expr::Unary {
                operator,
                operand,
                location,
            } => {
                self.generate_expr(operand);
                match operator {
                    UnaryOp::Negate => self.emit_op_code(OpCode::Negate, *location),
                    UnaryOp::Not => self.emit_op_code(OpCode::Not, *location),
                    UnaryOp::BitwiseNot => self.emit_op_code(OpCode::BitwiseNot, *location),
                }
            }
            Expr::Call {
                callee,
                arguments,
                location,
                ..
            } => {
                // Check if this is a method call: Call { callee: GetField { object, field }, arguments }
                if let Expr::GetField { object, field, .. } = callee.as_ref() {
                    // This is a method call obj.method(args)
                    self.generate_method_call_expr(object, field, arguments, *location);
                } else {
                    // Regular function call
                    self.generate_call_expr(callee, arguments, *location);
                }
            }
            Expr::GetField {
                object,
                field,
                location,
            } => {
                self.generate_expr(object);
                let field_string = string!(field.as_str());
                let field_index = self.current_chunk().add_string(field_string);
                self.emit_op_code_variant(OpCode::GetField, field_index, *location);
            }
            Expr::SetField {
                object,
                field,
                value,
                location,
            } => {
                self.generate_expr(object);
                self.generate_expr(value);
                let field_string = string!(field.as_str());
                let field_index = self.current_chunk().add_string(field_string);
                self.emit_op_code_variant(OpCode::SetField, field_index, *location);
            }
            Expr::Grouping { expr, .. } => {
                self.generate_expr(expr);
            }
            Expr::MapLiteral { entries, location } => {
                let message = format!(
                    "map literal too large: {} entries (maximum is {})",
                    entries.len(),
                    u16::MAX
                );
                if self
                    .check_count_limit(entries.len(), u16::MAX as usize, message, *location)
                    .is_none()
                {
                    return;
                }

                for (key, value) in entries {
                    self.generate_expr(key);
                    self.generate_expr(value);
                }
                self.emit_op_code(OpCode::CreateMap, *location);
                self.current_chunk().write_u16(entries.len() as u16);
            }
            Expr::ArrayLiteral { elements, location } => {
                self.generate_array_literal_expr(elements, *location);
            }
            Expr::SetLiteral { elements, location } => {
                let message = format!(
                    "set literal too large: {} elements (maximum is {})",
                    elements.len(),
                    u16::MAX
                );
                if self
                    .check_count_limit(elements.len(), u16::MAX as usize, message, *location)
                    .is_none()
                {
                    return;
                }

                for element in elements {
                    self.generate_expr(element);
                }
                self.emit_op_code(OpCode::CreateSet, *location);
                self.current_chunk().write_u16(elements.len() as u16);
            }
            Expr::Index {
                object,
                index,
                location,
            } => {
                self.generate_expr(object);
                self.generate_expr(index);
                self.emit_op_code(OpCode::GetIndex, *location);
            }
            Expr::IndexAssign {
                object,
                index,
                value,
                location,
            } => {
                self.generate_expr(object);
                self.generate_expr(index);
                self.generate_expr(value);
                self.emit_op_code(OpCode::SetIndex, *location);
            }
            Expr::Range {
                start,
                end,
                inclusive,
                location,
            } => {
                self.generate_expr(start);
                self.generate_expr(end);
                self.emit_op_code(OpCode::CreateRange, *location);
                self.current_chunk()
                    .write_u8(if *inclusive { 1 } else { 0 });
            }
            Expr::PostfixIncrement { operand, location } => {
                self.generate_postfix_increment_expr(operand, *location);
            }
            Expr::PostfixDecrement { operand, location } => {
                self.generate_postfix_decrement_expr(operand, *location);
            }
            Expr::Conditional {
                condition,
                then_expr,
                else_expr,
                location,
            } => {
                // Generate condition
                self.generate_expr(condition);

                // Jump to else branch if condition is false
                let else_jump = self.emit_jump(OpCode::JumpIfFalse, *location);

                // Pop the condition value (it's still on the stack)
                self.emit_op_code(OpCode::Pop, *location);

                // Generate then expression (leaves value on stack)
                self.generate_expr(then_expr);

                // Jump over else branch
                let end_jump = self.emit_jump(OpCode::Jump, *location);

                // Patch the else jump to here
                self.patch_jump(else_jump);

                // Pop the condition value for the else path
                self.emit_op_code(OpCode::Pop, *location);

                // Generate else expression (leaves value on stack)
                self.generate_expr(else_expr);

                // Patch the end jump to here
                self.patch_jump(end_jump);
            }
            Expr::Function {
                params,
                body,
                id,
                location,
            } => {
                self.generate_closure(*id, "anonymous", params, body, *location);
            }
        }
    }

    /// Pushes the placeholder native callable for a call dispatched by
    /// registry index.
    fn push_native_callable_by_index(
        &mut self,
        type_name: String,
        index: usize,
        arity: u8,
        location: SourceLocation,
    ) {
        let callable = Value::new_native_function(type_name, arity, index as u32, "".to_string());
        self.emit_constant(callable, location);
    }

    /// Pushes the placeholder native callable for a call dispatched by
    /// method name at runtime (instance methods, whose receiver type isn't
    /// known at compile time).
    fn push_native_callable_by_name(
        &mut self,
        type_name: String,
        method_name: String,
        arity: u8,
        location: SourceLocation,
    ) {
        let callable = Value::new_native_function(type_name, arity, u32::MAX, method_name);
        self.emit_constant(callable, location);
    }

    fn emit_call(&mut self, argc: u8, location: SourceLocation) {
        self.emit_op_code(OpCode::Call, location);
        self.current_chunk().write_u8(argc);
    }

    /// Emits `DefineMethod`, popping the closure left on top of the stack by
    /// a preceding `generate_closure` call and registering it under
    /// `(type_name, method_name)`, along with whether it takes `self`.
    fn emit_define_method(
        &mut self,
        type_name: &str,
        method_name: &str,
        takes_self: bool,
        location: SourceLocation,
    ) {
        let type_index = self.current_chunk().add_string(string!(type_name));
        let method_index = self.current_chunk().add_string(string!(method_name));
        self.emit_op_code(OpCode::DefineMethod, location);
        self.current_chunk().write_u32(type_index);
        self.current_chunk().write_u32(method_index);
        self.current_chunk().write_u8(takes_self as u8);
    }
}
