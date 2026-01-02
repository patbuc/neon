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
}

pub struct CodeGenerator {
    chunks: Vec<Chunk>,
    scope_depth: u32,
    errors: Vec<CompilationError>,
    loop_contexts: Vec<LoopContext>,
    builtin: indexmap::IndexMap<String, Value>,
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
        let (maybe_index, _is_mutable, is_global, _is_builtin) = self.get_variable_index(name);
        if let Some(index) = maybe_index {
            if is_global {
                self.emit_op_code_variant(OpCode::GetGlobal, index, location);
            } else {
                self.emit_op_code_variant(OpCode::GetLocal, index, location);
            }
            Some(())
        } else {
            self.errors.push(CompilationError::new(
                CompilationPhase::Codegen,
                CompilationErrorKind::UndefinedSymbol,
                format!("Undefined variable '{}'", name),
                location,
            ));
            None
        }
    }

    fn emit_variable_set(&mut self, name: &str, location: SourceLocation) -> Option<()> {
        let (maybe_index, _is_mutable, is_global, _is_builtin) = self.get_variable_index(name);
        if let Some(index) = maybe_index {
            if is_global {
                self.emit_op_code_variant(OpCode::SetGlobal, index, location);
            } else {
                self.emit_op_code_variant(OpCode::SetLocal, index, location);
            }
            Some(())
        } else {
            self.errors.push(CompilationError::new(
                CompilationPhase::Codegen,
                CompilationErrorKind::UndefinedSymbol,
                format!("Undefined variable '{}'", name),
                location,
            ));
            None
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

    fn get_variable_index(&self, name: &str) -> (Option<u32>, bool, bool, bool) {
        // Returns: (index, is_mutable, is_global, is_builtin)

        if self.is_builtin(name) {
            let index = self.get_builtin_index(name);
            if let Some(index) = index {
                return (Some(index as u32), false, true, true); // is_global = true, is_builtin = true
            }
            return (None, false, false, false);
        }

        // Search in chunk stack from innermost to outermost
        let current_chunk_idx = self.chunks.len() - 1;

        // First try to find in current chunk (parameters and locals)
        let current_result = self.chunks[current_chunk_idx].get_local_index(name);
        if current_result.0.is_some() {
            let index = current_result.0.unwrap();
            return (Some(index), current_result.1, false, false);
        }

        // Then try to find in parent chunks (global scope for nested functions)
        if current_chunk_idx > 0 {
            for chunk_idx in (0..current_chunk_idx).rev() {
                let index = self.chunks[chunk_idx].get_local_index(name);
                if index.0.is_some() {
                    return (Some(index.0.unwrap()), index.1, true, false); // is_global = true
                }
            }
        }

        (None, false, false, false)
    }

    // ===== Statement Generation =====

    fn generate_val_stmt(
        &mut self,
        name: &str,
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
        let local = Local::new(name.to_string(), self.scope_depth, false);
        self.current_chunk()
            .define_local(local, location.line, location.column);
    }

    fn generate_var_stmt(
        &mut self,
        name: &str,
        initializer: &Option<Expr>,
        location: SourceLocation,
    ) {
        // Generate initializer or nil
        if let Some(init) = initializer {
            self.generate_expr(init);
        } else {
            self.emit_op_code(OpCode::Nil, location);
        }

        // Define local variable (mutable)
        let local = Local::new(name.to_string(), self.scope_depth, true);
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
        // Function was already defined with nil placeholder
        // Now compile the function body and replace the placeholder

        // Create a new chunk for the function
        self.chunks.push(Chunk::new(&format!("function_{}", name)));

        // Enter function scope
        self.scope_depth += 1;

        // Define parameters as local variables in the function scope
        for param in params {
            let param_local = Local::new(param.clone(), self.scope_depth, false);
            self.current_chunk().add_parameter(param_local);
        }

        // Compile function body
        for stmt in body {
            self.generate_stmt(stmt);
        }

        // Emit return at end of function
        self.emit_return();

        // Exit function scope
        self.scope_depth -= 1;

        let function_chunk = self.chunks.pop().unwrap();
        let function_value =
            Value::new_function(name.to_string(), params.len() as u8, function_chunk);

        // Replace the nil placeholder with the actual function
        self.emit_constant(function_value, location);

        // Get the index of the function variable we defined earlier
        let (index, _is_mutable, is_global, _is_builtin) = self.get_variable_index(name);
        let index = match index {
            Some(idx) => idx,
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
        if is_global {
            self.emit_op_code_variant(OpCode::SetGlobal, index, location);
        } else {
            self.emit_op_code_variant(OpCode::SetLocal, index, location);
        }
        self.emit_op_code(OpCode::Pop, location); // Pop the function value from the stack
    }

    fn generate_expression_stmt(&mut self, expr: &Expr, location: SourceLocation) {
        self.generate_expr(expr);
        self.emit_op_code(OpCode::Pop, location);
    }

    fn generate_block_stmt(&mut self, statements: &[Stmt]) {
        self.scope_depth += 1;
        for stmt in statements {
            self.generate_stmt(stmt);
        }
        self.scope_depth -= 1;
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
        self.loop_contexts.push(LoopContext {
            loop_start,
            break_jumps: Vec::new(),
            continue_jumps: Vec::new(),
        });

        self.generate_expr(condition);

        let exit_jump = self.emit_jump(OpCode::JumpIfFalse, location);
        self.emit_op_code(OpCode::Pop, location); // Pop the condition value for the true case

        // Check if this is a desugared C-style for loop (body is Block with 2 statements)
        // In that case, we need to patch continue jumps after the first statement but before the second (increment)
        if let Stmt::Block { statements, .. } = body {
            if statements.len() == 2 {
                // This is likely a desugared for loop: Block([user_body, increment])
                // Generate the user body first
                self.generate_stmt(&statements[0]);

                // Now patch continue jumps to point here (before the increment)
                let loop_context = self.loop_contexts.last_mut().unwrap();
                let continue_jumps = std::mem::take(&mut loop_context.continue_jumps);
                for continue_jump in continue_jumps {
                    self.patch_jump(continue_jump);
                }

                // Generate the increment
                self.generate_stmt(&statements[1]);
            } else {
                // Regular block, generate normally
                self.generate_stmt(body);
            }
        } else {
            // Not a block, generate normally
            self.generate_stmt(body);
        }

        // Pop loop context
        let loop_context = self.loop_contexts.pop().unwrap();

        // Patch any remaining continue jumps (for non-desugared while loops)
        // These should jump to just before the Loop instruction
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

    fn generate_return_stmt(&mut self, value: &Expr, location: SourceLocation) {
        self.generate_expr(value);
        self.emit_op_code(OpCode::Return, location);
    }

    fn generate_break_stmt(&mut self, location: SourceLocation) {
        // Emit a Jump opcode and record its location for later patching
        if self.loop_contexts.is_empty() {
            self.errors.push(CompilationError::new(
                CompilationPhase::Codegen,
                CompilationErrorKind::Other,
                "Cannot use 'break' outside of a loop".to_string(),
                location,
            ));
            return;
        }
        let jump_index = self.emit_jump(OpCode::Jump, location);
        self.loop_contexts
            .last_mut()
            .unwrap()
            .break_jumps
            .push(jump_index);
    }

    fn generate_continue_stmt(&mut self, location: SourceLocation) {
        // Emit a Jump opcode and record it for later patching
        // This allows continue to jump to the right place (before the Loop instruction)
        // which is crucial for C-style for loops where increment comes at the end
        if self.loop_contexts.is_empty() {
            self.errors.push(CompilationError::new(
                CompilationPhase::Codegen,
                CompilationErrorKind::Other,
                "Cannot use 'continue' outside of a loop".to_string(),
                location,
            ));
            return;
        }
        let jump_index = self.emit_jump(OpCode::Jump, location);
        self.loop_contexts
            .last_mut()
            .unwrap()
            .continue_jumps
            .push(jump_index);
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
        //   <body with loop variable>
        //   Pop                     ; Pop the loop variable value
        //   Loop loop_start         ; Jump back
        //   exit_jump:
        //   Pop                     ; Pop the false value (done)

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

        // Pop the old loop variable value before getting the next one
        self.emit_op_code(OpCode::Pop, location);

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

        // Note: The loop variable has already been popped in the last iteration
        // before jumping back. So we don't need to pop it here.

        // Pop the iterator from the VM's iterator stack
        self.emit_op_code(OpCode::PopIterator, location);

        // Patch all break jumps
        for break_jump in loop_context.break_jumps {
            self.patch_jump(break_jump);
        }

        // Exit the loop scope
        self.scope_depth -= 1;
    }

    fn generate_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Val {
                name,
                initializer,
                location,
            } => {
                self.generate_val_stmt(name, initializer, *location);
            }
            Stmt::Var {
                name,
                initializer,
                location,
            } => {
                self.generate_var_stmt(name, initializer, *location);
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
            Stmt::Expression { expr, location } => {
                self.generate_expression_stmt(expr, *location);
            }
            Stmt::Block { statements, .. } => {
                self.generate_block_stmt(statements);
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
                self.generate_break_stmt(*location);
            }
            Stmt::Continue { location } => {
                self.generate_continue_stmt(*location);
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
        let (maybe_index, _is_mutable, is_global, is_builtin) = self.get_variable_index(name);
        if let Some(index) = maybe_index {
            if is_builtin {
                self.emit_op_code_variant(OpCode::GetBuiltin, index, location);
            } else if is_global {
                self.emit_op_code_variant(OpCode::GetGlobal, index, location);
            } else {
                self.emit_op_code_variant(OpCode::GetLocal, index, location);
            }
        } else {
            self.errors.push(CompilationError::new(
                CompilationPhase::Codegen,
                CompilationErrorKind::UndefinedSymbol,
                format!("Undefined variable '{}'", name),
                location,
            ));
        }
    }

    fn generate_assign_expr(&mut self, name: &str, value: &Expr, location: SourceLocation) {
        // Generate the value being assigned
        self.generate_expr(value);

        // Get the variable index
        let (maybe_index, _is_mutable, is_global, _is_builtin) = self.get_variable_index(name);
        if let Some(index) = maybe_index {
            if is_global {
                self.emit_op_code_variant(OpCode::SetGlobal, index, location);
            } else {
                self.emit_op_code_variant(OpCode::SetLocal, index, location);
            }
        } else {
            self.errors.push(CompilationError::new(
                CompilationPhase::Codegen,
                CompilationErrorKind::UndefinedSymbol,
                format!("Undefined variable '{}'", name),
                location,
            ));
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
                    BinaryOp::FloorDivide => self.emit_op_code(OpCode::FloorDivide, location),
                    BinaryOp::Modulo => self.emit_op_code(OpCode::Modulo, location),
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
        // Check if array size exceeds u16 limit
        if elements.len() > u16::MAX as usize {
            self.errors.push(CompilationError::new(
                CompilationPhase::Codegen,
                CompilationErrorKind::Other,
                format!(
                    "array literal too large: {} elements (maximum is {})",
                    elements.len(),
                    u16::MAX
                ),
                location,
            ));
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
                for (key, _) in entries {
                    self.generate_expr(key);
                }
                for (_, value) in entries {
                    self.generate_expr(value);
                }
                self.emit_op_code(OpCode::CreateMap, *location);
                self.current_chunk().write_u8(entries.len() as u8);
            }
            Expr::ArrayLiteral { elements, location } => {
                self.generate_array_literal_expr(elements, *location);
            }
            Expr::SetLiteral { elements, location } => {
                for element in elements {
                    self.generate_expr(element);
                }
                self.emit_op_code(OpCode::CreateSet, *location);
                self.current_chunk().write_u8(elements.len() as u8);
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
        }
    }

    fn is_builtin(&self, name: &str) -> bool {
        self.builtin.keys().any(|k| k == name)
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
}
