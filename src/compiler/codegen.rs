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
        CodeGenerator {
            bloqs: vec![Bloq::new("main")],
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
        // Search in bloq stack from innermost to outermost
        let current_bloq_idx = self.bloqs.len() - 1;

        // First try to find in current bloq (parameters and locals)
        let current_result = self.bloqs[current_bloq_idx].get_local_index(name);
        if current_result.0.is_some() {
            return (current_result.0, current_result.1, false);
        }

        // Then try to find in parent bloqs (global scope for nested functions)
        if current_bloq_idx > 0 {
            for bloq_idx in (0..current_bloq_idx).rev() {
                let index = self.bloqs[bloq_idx].get_local_index(name);
                if index.0.is_some() {
                    return (index.0, index.1, true); // is_global = true
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
                self.generate_stmt(then_branch);
                let else_jump = self.emit_jump(OpCode::Jump, *location);
                self.patch_jump(then_jump);

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
            Expr::Array { elements, location } => {
                // Array literal generation
                for element in elements {
                    self.generate_expr(element);
                }
                // TODO: Emit array creation instruction when VM supports arrays
                let _ = location; // Suppress unused warning
            }
            Expr::Map { entries, location } => {
                // Create empty map
                self.emit_op_code(OpCode::Map, *location);

                // Add each key-value pair
                for (key, value) in entries {
                    // Generate the value expression
                    self.generate_expr(value);

                    // Store key as string constant
                    let key_string = string!(key.as_str());
                    let key_index = self.current_bloq().add_string(key_string);
                    self.emit_op_code_variant(OpCode::MapSet, key_index, *location);
                }
            }
            Expr::Set { elements, location } => {
                // Create empty set
                self.emit_op_code(OpCode::Set, *location);

                // Add each element
                for element in elements {
                    // Generate the element expression
                    self.generate_expr(element);
                    // Add element to set
                    self.emit_op_code(OpCode::SetAdd, *location);
                }
            }
            Expr::MethodCall {
                object,
                method,
                arguments,
                location,
            } => {
                // Generate object expression
                self.generate_expr(object);

                // Handle map and set method calls
                match method.as_str() {
                    // Map methods
                    "get" => {
                        if arguments.len() != 1 {
                            self.errors.push(CompilationError::new(
                                CompilationPhase::Codegen,
                                CompilationErrorKind::ArityExceeded,
                                format!("Map.get() expects 1 argument, got {}", arguments.len()),
                                *location,
                            ));
                            return;
                        }
                        // Key must be a string literal
                        if let Expr::String { value: key, .. } = &arguments[0] {
                            let key_string = string!(key.as_str());
                            let key_index = self.current_bloq().add_string(key_string);
                            self.emit_op_code_variant(OpCode::MapGet, key_index, *location);
                        } else {
                            self.errors.push(CompilationError::new(
                                CompilationPhase::Codegen,
                                CompilationErrorKind::Other,
                                "Map key must be a string literal".to_string(),
                                *location,
                            ));
                        }
                    }
                    "set" => {
                        if arguments.len() != 2 {
                            self.errors.push(CompilationError::new(
                                CompilationPhase::Codegen,
                                CompilationErrorKind::ArityExceeded,
                                format!("Map.set() expects 2 arguments, got {}", arguments.len()),
                                *location,
                            ));
                            return;
                        }
                        // Generate value first (will be on stack)
                        self.generate_expr(&arguments[1]);
                        // Key must be a string literal
                        if let Expr::String { value: key, .. } = &arguments[0] {
                            let key_string = string!(key.as_str());
                            let key_index = self.current_bloq().add_string(key_string);
                            self.emit_op_code_variant(OpCode::MapSet, key_index, *location);
                        } else {
                            self.errors.push(CompilationError::new(
                                CompilationPhase::Codegen,
                                CompilationErrorKind::Other,
                                "Map key must be a string literal".to_string(),
                                *location,
                            ));
                        }
                    }
                    "has" => {
                        if arguments.len() != 1 {
                            self.errors.push(CompilationError::new(
                                CompilationPhase::Codegen,
                                CompilationErrorKind::ArityExceeded,
                                format!(
                                    "Map/Set.has() expects 1 argument, got {}",
                                    arguments.len()
                                ),
                                *location,
                            ));
                            return;
                        }
                        // Check if key is string (map) or value (set)
                        if let Expr::String { value: key, .. } = &arguments[0] {
                            // Map.has()
                            let key_string = string!(key.as_str());
                            let key_index = self.current_bloq().add_string(key_string);
                            self.emit_op_code_variant(OpCode::MapHas, key_index, *location);
                        } else {
                            // Set.has()
                            self.generate_expr(&arguments[0]);
                            self.emit_op_code(OpCode::SetHas, *location);
                        }
                    }
                    "remove" => {
                        if arguments.len() != 1 {
                            self.errors.push(CompilationError::new(
                                CompilationPhase::Codegen,
                                CompilationErrorKind::ArityExceeded,
                                format!(
                                    "Map/Set.remove() expects 1 argument, got {}",
                                    arguments.len()
                                ),
                                *location,
                            ));
                            return;
                        }
                        // Check if key is string (map) or value (set)
                        if let Expr::String { value: key, .. } = &arguments[0] {
                            // Map.remove()
                            let key_string = string!(key.as_str());
                            let key_index = self.current_bloq().add_string(key_string);
                            self.emit_op_code_variant(OpCode::MapRemove, key_index, *location);
                        } else {
                            // Set.remove()
                            self.generate_expr(&arguments[0]);
                            self.emit_op_code(OpCode::SetRemove, *location);
                        }
                    }
                    "keys" => {
                        if !arguments.is_empty() {
                            self.errors.push(CompilationError::new(
                                CompilationPhase::Codegen,
                                CompilationErrorKind::ArityExceeded,
                                "Map.keys() expects no arguments".to_string(),
                                *location,
                            ));
                            return;
                        }
                        self.emit_op_code(OpCode::MapKeys, *location);
                    }
                    "values" => {
                        if !arguments.is_empty() {
                            self.errors.push(CompilationError::new(
                                CompilationPhase::Codegen,
                                CompilationErrorKind::ArityExceeded,
                                "Map/Set.values() expects no arguments".to_string(),
                                *location,
                            ));
                            return;
                        }
                        // Try MapValues first, will handle SetValues in VM based on object type
                        self.emit_op_code(OpCode::MapValues, *location);
                    }
                    "size" => {
                        if !arguments.is_empty() {
                            self.errors.push(CompilationError::new(
                                CompilationPhase::Codegen,
                                CompilationErrorKind::ArityExceeded,
                                "Map/Set.size() expects no arguments".to_string(),
                                *location,
                            ));
                            return;
                        }
                        // Try MapSize first, will handle SetSize in VM based on object type
                        self.emit_op_code(OpCode::MapSize, *location);
                    }
                    "add" => {
                        if arguments.len() != 1 {
                            self.errors.push(CompilationError::new(
                                CompilationPhase::Codegen,
                                CompilationErrorKind::ArityExceeded,
                                format!("Set.add() expects 1 argument, got {}", arguments.len()),
                                *location,
                            ));
                            return;
                        }
                        self.generate_expr(&arguments[0]);
                        self.emit_op_code(OpCode::SetAdd, *location);
                    }
                    _ => {
                        self.errors.push(CompilationError::new(
                            CompilationPhase::Codegen,
                            CompilationErrorKind::Other,
                            format!("Unknown method '{}'", method),
                            *location,
                        ));
                    }
                }
            }
        }
    }
}
