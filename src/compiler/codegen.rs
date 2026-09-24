use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};

/// Code generator for the multi-pass compiler
/// Generates bytecode from AST using symbol table information
use crate::common::opcodes::OpCode;
use crate::common::{Chunk, Local, SourceLocation, Value};
use crate::compiler::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::{number, string};
use indexmap::IndexMap;

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

enum VariableScope {
    Local,
    Global,
    Builtin,
    /// A variable captured from an enclosing function, addressed by index
    /// into the current function's own upvalue array.
    Upvalue,
}

struct VariableRef {
    index: u32,
    scope: VariableScope,
}

/// One entry in a function's upvalue array: where to find the value when
/// the function's closure is created. `is_local` means a local slot of the
/// immediately enclosing function; otherwise it's one of that enclosing
/// function's own upvalues (chaining a capture through multiple levels).
#[derive(Clone, Copy, PartialEq)]
struct UpvalueDescriptor {
    is_local: bool,
    index: u32,
}

/// Upvalue bookkeeping for one function currently being compiled.
#[derive(Default)]
struct FunctionContext {
    upvalues: Vec<UpvalueDescriptor>,
}

pub struct CodeGenerator {
    chunks: Vec<Chunk>,
    scope_depth: u32,
    errors: Vec<CompilationError>,
    loop_contexts: Vec<LoopContext>,
    builtin: indexmap::IndexMap<String, Value>,
    /// Upvalues captured by each function currently being compiled,
    /// outermost first; `chunks[i + 1]` is that function's own chunk.
    function_contexts: Vec<FunctionContext>,
}

impl CodeGenerator {
    pub fn new(builtin: IndexMap<String, Value>) -> Self {
        let chunks = vec![Chunk::new("main")];

        CodeGenerator {
            chunks,
            scope_depth: 0,
            errors: Vec::new(),
            loop_contexts: Vec::new(),
            builtin,
            function_contexts: Vec::new(),
        }
    }

    pub fn generate(&mut self, statements: &[Stmt]) -> CompilationResult<Chunk> {
        // First: Define all functions and structs with placeholders
        // This allows forward references to work
        for stmt in statements {
            match stmt {
                Stmt::Fn { name, location, .. } => {
                    // Define function with nil placeholder
                    self.emit_op_code(OpCode::Nil, *location);
                    let local = Local::new(name.clone(), self.scope_depth, false);
                    self.current_chunk()
                        .define_local(local, location.line, location.column);
                }
                Stmt::Struct {
                    name,
                    fields,
                    location,
                } => {
                    // Create the struct value
                    let struct_value = Value::new_struct(name.clone(), fields.clone());
                    self.emit_constant(struct_value, *location);
                    let local = Local::new(name.clone(), self.scope_depth, false);
                    self.current_chunk()
                        .define_local(local, location.line, location.column);
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
                        location,
                    } = method
                    {
                        self.generate_closure(name, params, body, *location);
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
            Ok(self.chunks.pop().unwrap())
        } else {
            Err(self.errors.clone())
        }
    }

    // ===== Helper Methods =====

    fn current_chunk(&mut self) -> &mut Chunk {
        self.chunks.last_mut().unwrap()
    }

    fn emit_op_code(&mut self, op_code: OpCode, location: SourceLocation) {
        self.current_chunk()
            .write_op_code(op_code, location.line, location.column);
    }

    fn emit_op_code_variant(&mut self, op_code: OpCode, index: u32, location: SourceLocation) {
        self.current_chunk()
            .write_op_code_variant(op_code, index, location.line, location.column);
    }

    fn emit_variable_get(&mut self, name: &str, location: SourceLocation) -> Option<()> {
        match self.get_variable_index(name) {
            Some(var) => {
                let op_code = match var.scope {
                    VariableScope::Builtin => OpCode::GetBuiltin,
                    VariableScope::Global => OpCode::GetGlobal,
                    VariableScope::Local => OpCode::GetLocal,
                    VariableScope::Upvalue => OpCode::GetUpvalue,
                };
                self.emit_op_code_variant(op_code, var.index, location);
                Some(())
            }
            None => {
                self.errors.push(CompilationError::new(
                    CompilationPhase::Codegen,
                    CompilationErrorKind::UndefinedSymbol,
                    format!("Undefined variable '{}'", name),
                    location,
                ));
                None
            }
        }
    }

    fn emit_set_for_variable(&mut self, var: &VariableRef, location: SourceLocation) {
        let op_code = match var.scope {
            VariableScope::Local => OpCode::SetLocal,
            VariableScope::Global => OpCode::SetGlobal,
            VariableScope::Upvalue => OpCode::SetUpvalue,
            VariableScope::Builtin => unreachable!("Builtins are never assignment targets"),
        };
        self.emit_op_code_variant(op_code, var.index, location);
    }

    fn emit_variable_set(&mut self, name: &str, location: SourceLocation) -> Option<()> {
        match self.get_variable_index(name) {
            Some(var) => {
                self.emit_set_for_variable(&var, location);
                Some(())
            }
            None => {
                self.errors.push(CompilationError::new(
                    CompilationPhase::Codegen,
                    CompilationErrorKind::UndefinedSymbol,
                    format!("Undefined variable '{}'", name),
                    location,
                ));
                None
            }
        }
    }

    fn emit_upvalue_metadata(&mut self, upvalues: &[UpvalueDescriptor], location: SourceLocation) {
        let message = format!(
            "function captures too many variables: {} (maximum is {})",
            upvalues.len(),
            u8::MAX
        );
        if self
            .check_count_limit(upvalues.len(), u8::MAX as usize, message, location)
            .is_none()
        {
            return;
        }
        self.current_chunk().write_u8(upvalues.len() as u8);

        for upvalue in upvalues {
            let message = format!(
                "captured variable index too large: {} (maximum is {})",
                upvalue.index,
                u16::MAX
            );
            if self
                .check_count_limit(upvalue.index as usize, u16::MAX as usize, message, location)
                .is_none()
            {
                return;
            }
            self.current_chunk()
                .write_u8(if upvalue.is_local { 1 } else { 0 });
            self.current_chunk().write_u16(upvalue.index as u16);
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

    fn get_variable_index(&mut self, name: &str) -> Option<VariableRef> {
        // Search in chunk stack from innermost to outermost
        let current_chunk_idx = self.chunks.len() - 1;

        // First try to find in current chunk (parameters and locals)
        let (index, _) = self.chunks[current_chunk_idx].get_local_index(name);
        if let Some(index) = index {
            return Some(VariableRef {
                index,
                scope: VariableScope::Local,
            });
        }

        if current_chunk_idx != 0 {
            // Next, an enclosing function's local (captured as an upvalue,
            // possibly chained through several levels of nesting).
            if let Some(upvalue_index) = self.resolve_upvalue(current_chunk_idx, name) {
                return Some(VariableRef {
                    index: upvalue_index,
                    scope: VariableScope::Upvalue,
                });
            }

            // The script chunk's locals stay reachable from any nesting depth.
            let (index, _) = self.chunks[0].get_local_index(name);
            if let Some(index) = index {
                return Some(VariableRef {
                    index,
                    scope: VariableScope::Global,
                });
            }
        }

        if let Some(index) = self.get_builtin_index(name) {
            return Some(VariableRef {
                index: index as u32,
                scope: VariableScope::Builtin,
            });
        }

        None
    }

    /// Resolves `name` as an upvalue of the function compiled into
    /// `chunks[chunk_idx]`: a local of the immediately enclosing chunk, or
    /// one of *that* chunk's own upvalues, chaining the capture through as
    /// many levels of nesting as needed. A depth-0 chunk-0 local (a true
    /// top-level script variable) is never captured this way — it's read
    /// as a global instead, since it's never popped.
    fn resolve_upvalue(&mut self, chunk_idx: usize, name: &str) -> Option<u32> {
        if chunk_idx == 0 {
            return None;
        }
        let enclosing_idx = chunk_idx - 1;

        let (local_index, _) = self.chunks[enclosing_idx].get_local_index(name);
        if let Some(local_index) = local_index {
            if enclosing_idx == 0 && self.chunks[0].locals[local_index as usize].depth == 0 {
                return None;
            }
            self.chunks[enclosing_idx].mark_captured(local_index);
            return Some(self.add_upvalue(
                chunk_idx,
                UpvalueDescriptor {
                    is_local: true,
                    index: local_index,
                },
            ));
        }

        let enclosing_upvalue = self.resolve_upvalue(enclosing_idx, name)?;
        Some(self.add_upvalue(
            chunk_idx,
            UpvalueDescriptor {
                is_local: false,
                index: enclosing_upvalue,
            },
        ))
    }

    /// Adds `descriptor` to the upvalue array of the function compiled into
    /// `chunks[chunk_idx]`, reusing a matching existing entry instead of
    /// duplicating it.
    fn add_upvalue(&mut self, chunk_idx: usize, descriptor: UpvalueDescriptor) -> u32 {
        let upvalues = &mut self.function_contexts[chunk_idx - 1].upvalues;
        if let Some(existing) = upvalues.iter().position(|d| *d == descriptor) {
            return existing as u32;
        }
        upvalues.push(descriptor);
        (upvalues.len() - 1) as u32
    }

    // ===== Statement Generation =====

    fn generate_variable_declaration(
        &mut self,
        name: &str,
        initializer: &Option<Expr>,
        is_mutable: bool,
        location: SourceLocation,
    ) {
        // Generate initializer or nil
        if let Some(init) = initializer {
            self.generate_expr(init);
        } else {
            self.emit_op_code(OpCode::Nil, location);
        }

        // Define local variable
        let local = Local::new(name.to_string(), self.scope_depth, is_mutable);
        self.current_chunk()
            .define_local(local, location.line, location.column);
    }

    fn generate_fn_stmt(
        &mut self,
        name: &str,
        params: &[String],
        body: &[Stmt],
        location: SourceLocation,
    ) {
        // A nested function isn't pre-defined by generate()'s pre-pass, so define it now, before compiling its body, so it can recurse.
        if self.scope_depth > 0 {
            self.emit_op_code(OpCode::Nil, location);
            let local = Local::new(name.to_string(), self.scope_depth, false);
            self.current_chunk()
                .define_local(local, location.line, location.column);
        }

        self.generate_closure(name, params, body, location);

        // Get the index of the function variable we defined earlier
        let var = match self.get_variable_index(name) {
            Some(var) => var,
            None => {
                self.errors.push(CompilationError::new(
                    CompilationPhase::Codegen,
                    CompilationErrorKind::Internal,
                    format!("Function '{}' was not found after definition", name),
                    location,
                ));
                return;
            }
        };

        // Emit the appropriate Set opcode to update the placeholder
        self.emit_set_for_variable(&var, location);
        self.emit_op_code(OpCode::Pop, location); // Pop the function value from the stack
    }

    /// Compiles `params`/`body` into a closure and leaves it on top of the
    /// stack. Shared by named function declarations, which then store it
    /// into the variable defined for the name, and lambda expressions,
    /// which leave it as their expression value.
    fn generate_closure(
        &mut self,
        name: &str,
        params: &[String],
        body: &[Stmt],
        location: SourceLocation,
    ) {
        // Create a new chunk for the function
        self.chunks.push(Chunk::new(&format!("function_{}", name)));
        self.function_contexts.push(FunctionContext::default());

        // Enter function scope
        self.scope_depth += 1;

        // Define parameters as local variables in the function scope
        for param in params {
            let param_local = Local::new(param.clone(), self.scope_depth, false);
            self.current_chunk().add_parameter(param_local);
        }

        // Reserve the slot the calling convention leaves the callable in, unconsumed, right after the last param.
        let reserved_slot = Local::new(String::new(), self.scope_depth, false);
        self.current_chunk().add_parameter(reserved_slot);

        // Compile function body
        for stmt in body {
            self.generate_stmt(stmt);
        }

        // Emit return at end of function
        self.emit_return();

        self.scope_depth -= 1;
        let context = self.function_contexts.pop().unwrap();

        let function_chunk = self.chunks.pop().unwrap();
        let function_value =
            Value::new_function(name.to_string(), params.len() as u8, function_chunk);

        // Wrap the function in a closure.
        let const_index = self.current_chunk().add_constant(function_value);
        self.emit_op_code_variant(OpCode::Closure, const_index, location);
        self.emit_upvalue_metadata(&context.upvalues, location);
    }

    fn generate_expression_stmt(&mut self, expr: &Expr, location: SourceLocation) {
        self.generate_expr(expr);
        self.emit_op_code(OpCode::Pop, location);
    }

    fn generate_block_stmt(&mut self, statements: &[Stmt], location: SourceLocation) {
        self.scope_depth += 1;
        for stmt in statements {
            self.generate_stmt(stmt);
        }
        self.end_scope(location);
    }

    fn end_scope(&mut self, location: SourceLocation) {
        self.scope_depth -= 1;
        let captured = self.discard_locals_above_current_depth();
        self.emit_scope_exit(&captured, location);
    }

    fn discard_locals_above_current_depth(&mut self) -> Vec<bool> {
        let scope_depth = self.scope_depth;
        self.current_chunk().pop_locals_above(scope_depth)
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

    fn generate_while_stmt(
        &mut self,
        condition: &Expr,
        body: &Stmt,
        increment: Option<&Stmt>,
        location: SourceLocation,
    ) {
        let loop_start = self.current_chunk().instruction_count() as u32;

        // Push loop context for break/continue tracking
        self.loop_contexts.push(LoopContext {
            loop_start,
            break_jumps: Vec::new(),
            continue_jumps: Vec::new(),
            depth: self.scope_depth,
        });

        self.generate_expr(condition);

        let exit_jump = self.emit_jump(OpCode::JumpIfFalse, location);
        self.emit_op_code(OpCode::Pop, location); // Pop the condition value for the true case

        self.generate_stmt(body);

        // Pop loop context; continue jumps land here, before the increment.
        let loop_context = self.loop_contexts.pop().unwrap();
        for continue_jump in loop_context.continue_jumps {
            self.patch_jump(continue_jump);
        }

        if let Some(increment) = increment {
            self.generate_stmt(increment);
        }

        self.emit_loop(loop_start, location);

        self.patch_jump(exit_jump);
        self.emit_op_code(OpCode::Pop, location); // Pop the condition value for the false case (exiting loop)

        // Patch all break jumps
        for break_jump in loop_context.break_jumps {
            self.patch_jump(break_jump);
        }
    }

    fn generate_return_stmt(&mut self, value: &Expr, location: SourceLocation) {
        self.generate_expr(value);
        self.emit_op_code(OpCode::Return, location);
    }

    // Leaves the locals in chunk.locals; end_scope still owns them on fall-through.
    fn emit_loop_exit_pops(&mut self, depth: u32, location: SourceLocation) {
        let captured = self.current_chunk().captured_flags_above(depth);
        self.emit_scope_exit(&captured, location);
    }

    fn generate_loop_exit_stmt(&mut self, exit: LoopExit, location: SourceLocation) {
        // Emit a Jump opcode and record it for later patching. For continue,
        // this allows jumping to the right place (before the Loop
        // instruction), which is crucial for C-style for loops where the
        // increment comes at the end.
        let keyword = match exit {
            LoopExit::Break => "break",
            LoopExit::Continue => "continue",
        };
        if self.loop_contexts.is_empty() {
            self.errors.push(CompilationError::new(
                CompilationPhase::Codegen,
                CompilationErrorKind::Other,
                format!("Cannot use '{}' outside of a loop", keyword),
                location,
            ));
            return;
        }
        let depth = self.loop_contexts.last().unwrap().depth;
        self.emit_loop_exit_pops(depth, location);

        let jump_index = self.emit_jump(OpCode::Jump, location);
        let context = self.loop_contexts.last_mut().unwrap();
        let jumps = match exit {
            LoopExit::Break => &mut context.break_jumps,
            LoopExit::Continue => &mut context.continue_jumps,
        };
        jumps.push(jump_index);
    }

    fn generate_for_in_stmt(
        &mut self,
        variable: &str,
        collection: &Expr,
        body: &Stmt,
        location: SourceLocation,
    ) {
        // For-in loop code generation strategy:
        // Unlike C-style for loops, we don't desugar this - we use iterator opcodes
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
        self.scope_depth += 1;

        // Mark the start of the loop
        let loop_start = self.current_chunk().instruction_count() as u32;

        // Push loop context for break/continue tracking
        self.loop_contexts.push(LoopContext {
            loop_start,
            break_jumps: Vec::new(),
            continue_jumps: Vec::new(),
            // - 1 so break/continue also pop the loop variable itself.
            depth: self.scope_depth - 1,
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
        let local = Local::new(variable.to_string(), self.scope_depth, false);
        self.current_chunk()
            .define_local(local, location.line, location.column);

        // Generate the loop body
        self.generate_stmt(body);

        let loop_variable_captured = self
            .current_chunk()
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
        let loop_context = self.loop_contexts.pop().unwrap();
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
        self.scope_depth -= 1;
        self.discard_locals_above_current_depth();
    }

    fn generate_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Val {
                name,
                initializer,
                location,
            } => {
                self.generate_variable_declaration(name, initializer, false, *location);
            }
            Stmt::Var {
                name,
                initializer,
                location,
            } => {
                self.generate_variable_declaration(name, initializer, true, *location);
            }
            Stmt::Fn {
                name,
                params,
                body,
                location,
            } => {
                self.generate_fn_stmt(name, params, body, *location);
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
                increment,
                location,
            } => {
                self.generate_while_stmt(condition, body, increment.as_deref(), *location);
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
                variable,
                collection,
                body,
                location,
            } => {
                self.generate_for_in_stmt(variable, collection, body, *location);
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

    fn generate_variable_expr(&mut self, name: &str, location: SourceLocation) {
        self.emit_variable_get(name, location);
    }

    fn generate_assign_expr(&mut self, name: &str, value: &Expr, location: SourceLocation) {
        // Generate the value being assigned
        self.generate_expr(value);
        self.emit_variable_set(name, location);
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
                    BinaryOp::GreaterEqual => {
                        self.emit_op_code(OpCode::Less, location);
                        self.emit_op_code(OpCode::Not, location);
                    }
                    BinaryOp::Less => self.emit_op_code(OpCode::Less, location),
                    BinaryOp::LessEqual => {
                        self.emit_op_code(OpCode::Greater, location);
                        self.emit_op_code(OpCode::Not, location);
                    }
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
        // Regular function call - could be user-defined function
        // Unified calling convention: [args..., callable]

        // Evaluate all arguments FIRST
        for arg in arguments {
            self.generate_expr(arg);
        }

        // Evaluate the callee last to get the function object on top of stack
        self.generate_expr(callee);

        // Emit unified CALL instruction
        self.emit_op_code(OpCode::Call, location);
        self.current_chunk().write_u8(arguments.len() as u8);
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

        // Evaluate all arguments (no callee!)
        for arg in arguments {
            self.generate_expr(arg);
        }

        // Look up constructor index at compile time
        let index = crate::common::method_registry::get_native_method_index(&type_name, "new")
            .unwrap_or_else(|| panic!("Unknown constructor: {}.new", type_name));

        self.emit_native_call_by_index(
            format!("{}.new", type_name),
            index,
            arguments.len() as u8,
            location,
        );
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

        // Evaluate all arguments (no callee!)
        for arg in arguments {
            self.generate_expr(arg);
        }

        // Look up global function index at compile time
        let index = crate::common::method_registry::get_native_method_index("", &function_name)
            .unwrap_or_else(|| panic!("Unknown function: {}", function_name));

        self.emit_native_call_by_index(
            function_name.to_string(),
            index,
            arguments.len() as u8,
            location,
        );
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

        // Evaluate receiver first
        self.generate_expr(callee);

        // Evaluate all arguments
        for arg in arguments {
            self.generate_expr(arg);
        }

        self.emit_native_call_by_name(
            "".to_string(),
            method.to_string(),
            (arguments.len() + 1) as u8,
            location,
        );
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

        // Evaluate all arguments FIRST
        for arg in arguments {
            self.generate_expr(arg);
        }

        // Look up static method index at compile time
        let index =
            crate::common::method_registry::get_native_method_index(&namespace_name, method)
                .unwrap_or_else(|| panic!("Unknown static method: {}.{}", namespace_name, method));

        self.emit_native_call_by_index(method.to_string(), index, arguments.len() as u8, location);
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
        operation_name: &str,
        location: SourceLocation,
    ) {
        // Semantic analysis ensures operand is a Variable
        if let Expr::Variable { name, .. } = operand {
            // Load old value (will be the return value)
            if self.emit_variable_get(name, location).is_none() {
                return;
            }

            // Load old value again (for modification)
            if self.emit_variable_get(name, location).is_none() {
                return;
            }

            // Push 1 and perform operation (add or subtract)
            self.emit_constant(number!(1.0), location);
            self.emit_op_code(operation, location);

            // Store new value
            if self.emit_variable_set(name, location).is_none() {
                return;
            }

            // Pop the new value, leaving old value on stack
            self.emit_op_code(OpCode::Pop, location);
        } else {
            self.errors.push(CompilationError::new(
                CompilationPhase::Codegen,
                CompilationErrorKind::Other,
                format!("Postfix {} operand must be a variable", operation_name),
                location,
            ));
        }
    }

    fn generate_postfix_increment_expr(&mut self, operand: &Expr, location: SourceLocation) {
        self.generate_postfix_operation(operand, OpCode::Add, "increment", location);
    }

    fn generate_postfix_decrement_expr(&mut self, operand: &Expr, location: SourceLocation) {
        self.generate_postfix_operation(operand, OpCode::Subtract, "decrement", location);
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
            Expr::Variable { name, location } => {
                self.generate_variable_expr(name, *location);
            }
            Expr::Assign {
                name,
                value,
                location,
            } => {
                self.generate_assign_expr(name, value, *location);
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

                for (key, _) in entries {
                    self.generate_expr(key);
                }
                for (_, value) in entries {
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
                location,
            } => {
                self.generate_closure("anonymous", params, body, *location);
            }
        }
    }

    fn get_builtin_index(&self, name: &str) -> Option<usize> {
        self.builtin.get_index_of(name)
    }

    /// Helper: Emit a native callable and CALL instruction
    fn emit_native_call_by_index(
        &mut self,
        type_name: String,
        index: usize,
        arity: u8,
        location: SourceLocation,
    ) {
        let callable = Value::new_native_function(type_name, arity, index as u32, "".to_string());
        self.emit_constant(callable, location);
        self.emit_op_code(OpCode::Call, location);
        self.current_chunk().write_u8(arity);
    }

    fn emit_native_call_by_name(
        &mut self,
        type_name: String,
        method_name: String,
        arity: u8,
        location: SourceLocation,
    ) {
        let callable = Value::new_native_function(type_name, arity, u32::MAX, method_name);
        self.emit_constant(callable, location);
        self.emit_op_code(OpCode::Call, location);
        self.current_chunk().write_u8(arity);
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
