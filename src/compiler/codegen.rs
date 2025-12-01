use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};
/// Code generator for the multi-pass compiler
/// Generates bytecode from AST using symbol table information
use crate::common::opcodes::OpCode;
use crate::common::{Bloq, Local, SourceLocation, Value};
use crate::compiler::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::{number, string};

/// Code generator that walks the AST and emits bytecode
pub struct CodeGenerator {
    /// Stack of bloqs (for nested function compilation)
    bloqs: Vec<Bloq>,
    /// Current scope depth for tracking locals
    scope_depth: u32,
    /// Errors encountered during code generation
    errors: Vec<CompilationError>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        let bloqs = vec![Bloq::new("main")];

        CodeGenerator {
            bloqs,
            scope_depth: 0,
            errors: Vec::new(),
        }
    }

    /// Generate bytecode from the AST
    pub fn generate(&mut self, statements: &[Stmt]) -> CompilationResult<Bloq> {
        // First: Define all functions and structs with placeholders
        // This allows forward references to work
        for stmt in statements {
            match stmt {
                Stmt::Fn { name, location, .. } => {
                    // Define function with nil placeholder
                    self.emit_op_code(OpCode::Nil, *location);
                    let local = Local::new(name.clone(), self.scope_depth, false);
                    self.current_bloq()
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
                    self.current_bloq()
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
            Ok(self.bloqs.pop().unwrap())
        } else {
            Err(self.errors.clone())
        }
    }

    // ===== Helper Methods =====

    fn current_bloq(&mut self) -> &mut Bloq {
        self.bloqs.last_mut().unwrap()
    }

    fn emit_op_code(&mut self, op_code: OpCode, location: SourceLocation) {
        self.current_bloq()
            .write_op_code(op_code, location.line, location.column);
    }

    fn emit_op_code_variant(&mut self, op_code: OpCode, index: u32, location: SourceLocation) {
        self.current_bloq()
            .write_op_code_variant(op_code, index, location.line, location.column);
    }

    fn emit_constant(&mut self, value: Value, location: SourceLocation) {
        self.current_bloq()
            .write_constant(value, location.line, location.column);
    }

    fn emit_string(&mut self, value: Value, location: SourceLocation) {
        self.current_bloq()
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
        self.current_bloq()
            .emit_jump(op_code, location.line, location.column)
    }

    fn patch_jump(&mut self, offset: u32) {
        self.current_bloq().patch_jump(offset);
    }

    fn emit_loop(&mut self, loop_start: u32, location: SourceLocation) {
        self.current_bloq()
            .emit_loop(loop_start, location.line, location.column);
    }

    fn get_variable_index(&self, name: &str) -> (Option<u32>, bool, bool) {
        // Returns: (index, is_mutable, is_global)

        // Special case: Math is a built-in global stored in the VM's globals HashMap
        // We use u32::MAX as a sentinel value to signal the VM to look up built-in globals
        if name == "Math" {
            return (Some(u32::MAX), false, true); // is_global = true to emit GetGlobal
        }

        // Search in bloq stack from innermost to outermost
        let current_bloq_idx = self.bloqs.len() - 1;

        // First try to find in current bloq (parameters and locals)
        let current_result = self.bloqs[current_bloq_idx].get_local_index(name);
        if current_result.0.is_some() {
            let index = current_result.0.unwrap();
            return (Some(index), current_result.1, false);
        }

        // Then try to find in parent bloqs (global scope for nested functions)
        if current_bloq_idx > 0 {
            for bloq_idx in (0..current_bloq_idx).rev() {
                let index = self.bloqs[bloq_idx].get_local_index(name);
                if index.0.is_some() {
                    return (Some(index.0.unwrap()), index.1, true); // is_global = true
                }
            }
        }

        (None, false, false)
    }

    // ===== Statement Generation =====

    fn generate_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Val {
                name,
                initializer,
                location,
            } => {
                // Generate initializer or nil
                if let Some(init) = initializer {
                    self.generate_expr(init);
                } else {
                    self.emit_op_code(OpCode::Nil, *location);
                }

                // Define local variable
                let local = Local::new(name.clone(), self.scope_depth, false);
                self.current_bloq()
                    .define_local(local, location.line, location.column);
            }
            Stmt::Var {
                name,
                initializer,
                location,
            } => {
                // Generate initializer or nil
                if let Some(init) = initializer {
                    self.generate_expr(init);
                } else {
                    self.emit_op_code(OpCode::Nil, *location);
                }

                // Define local variable (mutable)
                let local = Local::new(name.clone(), self.scope_depth, true);
                self.current_bloq()
                    .define_local(local, location.line, location.column);
            }
            Stmt::Fn {
                name,
                params,
                body,
                location,
            } => {
                // Function was already defined with nil placeholder
                // Now compile the function body and replace the placeholder

                // Create a new bloq for the function
                self.bloqs.push(Bloq::new(&format!("function_{}", name)));

                // Enter function scope
                self.scope_depth += 1;

                // Define parameters as local variables in the function scope
                for param in params {
                    let param_local = Local::new(param.clone(), self.scope_depth, false);
                    self.current_bloq().add_parameter(param_local);
                }

                // Compile function body
                for stmt in body {
                    self.generate_stmt(stmt);
                }

                // Emit return at end of function
                self.emit_return();

                // Exit function scope
                self.scope_depth -= 1;

                let function_bloq = self.bloqs.pop().unwrap();
                let function_value =
                    Value::new_function(name.clone(), params.len() as u8, function_bloq);

                // Replace the nil placeholder with the actual function
                self.emit_constant(function_value, *location);

                // Get the index of the function variable we defined earlier
                let (index, _is_mutable, is_global) = self.get_variable_index(name);
                let index = match index {
                    Some(idx) => idx,
                    None => {
                        self.errors.push(CompilationError::new(
                            CompilationPhase::Codegen,
                            CompilationErrorKind::Internal,
                            format!("Function '{}' was not found after definition", name),
                            *location,
                        ));
                        return;
                    }
                };

                // Emit the appropriate Set opcode to update the placeholder
                if is_global {
                    self.emit_op_code_variant(OpCode::SetGlobal, index, *location);
                } else {
                    self.emit_op_code_variant(OpCode::SetLocal, index, *location);
                }
                self.emit_op_code(OpCode::Pop, *location); // Pop the function value from the stack
            }
            Stmt::Struct { .. } => {
                // Struct was already defined, nothing to do here
            }
            Stmt::Print { expr, location } => {
                self.generate_expr(expr);
                self.emit_op_code(OpCode::Print, *location);
            }
            Stmt::Expression { expr, location } => {
                self.generate_expr(expr);
                self.emit_op_code(OpCode::Pop, *location);
            }
            Stmt::Block {
                statements,
                location,
            } => {
                self.scope_depth += 1;
                for stmt in statements {
                    self.generate_stmt(stmt);
                }
                self.scope_depth -= 1;
                let _ = location; // Suppress unused warning
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                location,
            } => {
                self.generate_expr(condition);

                let then_jump = self.emit_jump(OpCode::JumpIfFalse, *location);
                self.emit_op_code(OpCode::Pop, *location); // Pop condition if true (not jumping)
                self.generate_stmt(then_branch);
                let else_jump = self.emit_jump(OpCode::Jump, *location);
                self.patch_jump(then_jump);
                self.emit_op_code(OpCode::Pop, *location); // Pop condition if false (jumped here)

                if let Some(else_stmt) = else_branch {
                    self.generate_stmt(else_stmt);
                }
                self.patch_jump(else_jump);
            }
            Stmt::While {
                condition,
                body,
                location,
            } => {
                let loop_start = self.current_bloq().instruction_count() as u32;

                self.generate_expr(condition);

                let exit_jump = self.emit_jump(OpCode::JumpIfFalse, *location);
                self.emit_op_code(OpCode::Pop, *location); // Pop the condition value for the true case
                self.generate_stmt(body);
                self.emit_loop(loop_start, *location);

                self.patch_jump(exit_jump);
                self.emit_op_code(OpCode::Pop, *location); // Pop the condition value for the false case (exiting loop)
            }
            Stmt::Return { value, location } => {
                self.generate_expr(value);
                self.emit_op_code(OpCode::Return, *location);
            }
        }
    }

    // ===== Expression Generation =====

    fn generate_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number { value, location } => {
                self.emit_constant(number!(*value), *location);
            }
            Expr::String { value, location } => {
                self.emit_string(string!(value.as_str()), *location);
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
                let (maybe_index, _is_mutable, is_global) = self.get_variable_index(name);
                if let Some(index) = maybe_index {
                    if is_global {
                        self.emit_op_code_variant(OpCode::GetGlobal, index, *location);
                    } else {
                        self.emit_op_code_variant(OpCode::GetLocal, index, *location);
                    }
                } else {
                    self.errors.push(CompilationError::new(
                        CompilationPhase::Codegen,
                        CompilationErrorKind::UndefinedSymbol,
                        format!("Undefined variable '{}'", name),
                        *location,
                    ));
                }
            }
            Expr::Assign {
                name,
                value,
                location,
            } => {
                // Generate the value being assigned
                self.generate_expr(value);

                // Get the variable index
                let (maybe_index, _is_mutable, is_global) = self.get_variable_index(name);
                if let Some(index) = maybe_index {
                    if is_global {
                        self.emit_op_code_variant(OpCode::SetGlobal, index, *location);
                    } else {
                        self.emit_op_code_variant(OpCode::SetLocal, index, *location);
                    }
                } else {
                    self.errors.push(CompilationError::new(
                        CompilationPhase::Codegen,
                        CompilationErrorKind::UndefinedSymbol,
                        format!("Undefined variable '{}'", name),
                        *location,
                    ));
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
                location,
            } => {
                // Handle short-circuit operators specially
                match operator {
                    BinaryOp::And => {
                        // For `a && b`:
                        // 1. Evaluate left operand
                        self.generate_expr(left);
                        // 2. If false, skip right operand and result is false
                        let end_jump = self.emit_jump(OpCode::JumpIfFalse, *location);
                        // 3. Left was true, pop it and evaluate right
                        self.emit_op_code(OpCode::Pop, *location);
                        self.generate_expr(right);
                        // 4. Patch jump to end (if left was false, we skip here with false on stack)
                        self.patch_jump(end_jump);
                    }
                    BinaryOp::Or => {
                        // For `a || b`:
                        // 1. Evaluate left operand
                        self.generate_expr(left);
                        // 2. If false, jump to evaluate right operand
                        let else_jump = self.emit_jump(OpCode::JumpIfFalse, *location);
                        // 3. Left was true, jump to end with true result
                        let end_jump = self.emit_jump(OpCode::Jump, *location);
                        // 4. Patch else jump (left was false, need to evaluate right)
                        self.patch_jump(else_jump);
                        // 5. Pop false value and evaluate right
                        self.emit_op_code(OpCode::Pop, *location);
                        self.generate_expr(right);
                        // 6. Patch end jump (left was true, skip right evaluation)
                        self.patch_jump(end_jump);
                    }
                    _ => {
                        // Regular binary operators: evaluate both operands first
                        self.generate_expr(left);
                        self.generate_expr(right);

                        match operator {
                            BinaryOp::Add => self.emit_op_code(OpCode::Add, *location),
                            BinaryOp::Subtract => self.emit_op_code(OpCode::Subtract, *location),
                            BinaryOp::Multiply => self.emit_op_code(OpCode::Multiply, *location),
                            BinaryOp::Divide => self.emit_op_code(OpCode::Divide, *location),
                            BinaryOp::Modulo => self.emit_op_code(OpCode::Modulo, *location),
                            BinaryOp::Equal => self.emit_op_code(OpCode::Equal, *location),
                            BinaryOp::NotEqual => {
                                self.emit_op_code(OpCode::Equal, *location);
                                self.emit_op_code(OpCode::Not, *location);
                            }
                            BinaryOp::Greater => self.emit_op_code(OpCode::Greater, *location),
                            BinaryOp::GreaterEqual => {
                                self.emit_op_code(OpCode::Less, *location);
                                self.emit_op_code(OpCode::Not, *location);
                            }
                            BinaryOp::Less => self.emit_op_code(OpCode::Less, *location),
                            BinaryOp::LessEqual => {
                                self.emit_op_code(OpCode::Greater, *location);
                                self.emit_op_code(OpCode::Not, *location);
                            }
                            BinaryOp::And | BinaryOp::Or => unreachable!(),
                        }
                    }
                }
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
                // Evaluate the callee
                self.generate_expr(callee);

                // Evaluate all arguments
                for arg in arguments {
                    self.generate_expr(arg);
                }

                // Emit call instruction
                self.emit_op_code(OpCode::Call, *location);
                self.current_bloq().write_u8(arguments.len() as u8);
            }
            Expr::GetField {
                object,
                field,
                location,
            } => {
                self.generate_expr(object);

                // Store field name as string constant
                let field_string = string!(field.as_str());
                let field_index = self.current_bloq().add_string(field_string);
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

                // Store field name as string constant
                let field_string = string!(field.as_str());
                let field_index = self.current_bloq().add_string(field_string);
                self.emit_op_code_variant(OpCode::SetField, field_index, *location);
            }
            Expr::Grouping { expr, .. } => {
                self.generate_expr(expr);
            }
            Expr::MethodCall {
                object,
                method,
                arguments,
                location,
            } => {
                // Evaluate the receiver object
                self.generate_expr(object);

                // Evaluate all arguments
                for arg in arguments {
                    self.generate_expr(arg);
                }

                // Emit CallMethod instruction
                self.emit_op_code(OpCode::CallMethod, *location);
                self.current_bloq().write_u8(arguments.len() as u8);

                // Add method name to string constants and emit index
                let method_string = string!(method.as_str());
                let method_index = self.current_bloq().add_string(method_string);
                self.current_bloq().write_u8(method_index as u8);
            }
            Expr::MapLiteral { entries, location } => {
                // Generate bytecode for map literal creation
                // Strategy: emit code for each key, then each value, then CreateMap
                // This allows the VM to pop pairs from the stack in order

                // First, generate code for all keys
                for (key, _) in entries {
                    self.generate_expr(key);
                }

                // Then, generate code for all values
                for (_, value) in entries {
                    self.generate_expr(value);
                }

                // Emit CreateMap with the count of entries
                self.emit_op_code(OpCode::CreateMap, *location);
                self.current_bloq().write_u8(entries.len() as u8);
            }
            Expr::SetLiteral { elements: _, location } => {
                // TODO: Generate bytecode for set literal creation
                // This will be implemented when CreateSet opcode is added
                self.errors.push(CompilationError::new(
                    CompilationPhase::Codegen,
                    CompilationErrorKind::UnexpectedToken,
                    "Set literals are not yet supported in codegen".to_string(),
                    *location,
                ));
            }
            Expr::Index {
                object,
                index,
                location,
            } => {
                // Generate bytecode for index access (map["key"] or array[0])
                // First evaluate the object being indexed
                self.generate_expr(object);

                // Then evaluate the index expression
                self.generate_expr(index);

                // Emit GetIndex opcode
                self.emit_op_code(OpCode::GetIndex, *location);
            }
            Expr::IndexAssign {
                object,
                index,
                value,
                location,
            } => {
                // Generate bytecode for index assignment (map["key"] = value)
                // First evaluate the object being indexed
                self.generate_expr(object);

                // Then evaluate the index expression
                self.generate_expr(index);

                // Finally evaluate the value to be assigned
                self.generate_expr(value);

                // Emit SetIndex opcode
                self.emit_op_code(OpCode::SetIndex, *location);
            }
        }
    }
}
