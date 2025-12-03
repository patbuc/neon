use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};
use crate::common::method_registry::MethodRegistry;
use crate::common::SourceLocation;
/// Semantic analyzer for the multi-pass compiler
/// Performs semantic analysis on the AST, building symbol tables and validating program semantics
use crate::compiler::ast::{Expr, Stmt};
use crate::compiler::symbol_table::{Symbol, SymbolKind, SymbolTable};
use std::collections::HashMap;

/// Semantic analyzer that validates the AST and builds symbol tables
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    errors: Vec<CompilationError>,
    type_env: HashMap<String, String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut symbol_table = SymbolTable::new();
        // Pre-define Math as a built-in global constant
        // This corresponds to the Math object that will be available at runtime
        let math_symbol = Symbol {
            name: "Math".to_string(),
            kind: SymbolKind::Value,
            is_mutable: false,
            scope_depth: 0,
            location: SourceLocation {
                offset: 0,
                line: 0,
                column: 0,
            },
        };
        let _ = symbol_table.define(math_symbol); // Ignore error since this is initial setup

        SemanticAnalyzer {
            symbol_table,
            errors: Vec::new(),
            type_env: HashMap::new(),
        }
    }

    /// Analyze the AST and return the symbol table if successful
    pub fn analyze(&mut self, statements: &[Stmt]) -> CompilationResult<SymbolTable> {
        // First: collect all top-level declarations
        self.collect_declarations(statements);

        // Then: resolve all references and validate
        self.resolve_statements(statements);

        if self.errors.is_empty() {
            Ok(self.symbol_table.clone())
        } else {
            Err(self.errors.clone())
        }
    }

    // ===== First: Declaration Collection =====
    // Only collect function and struct declarations
    // Variables (val/var) are defined during resolution

    fn collect_declarations(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            match stmt {
                Stmt::Fn {
                    name,
                    params,
                    location,
                    ..
                } => {
                    let arity = params.len() as u8;
                    self.define_symbol(
                        name.clone(),
                        SymbolKind::Function { arity },
                        false,
                        *location,
                    );
                }
                Stmt::Struct {
                    name,
                    fields,
                    location,
                } => {
                    self.define_symbol(
                        name.clone(),
                        SymbolKind::Struct {
                            fields: fields.clone(),
                        },
                        false,
                        *location,
                    );
                }
                _ => {}
            }
        }
    }

    fn define_symbol(
        &mut self,
        name: String,
        kind: SymbolKind,
        is_mutable: bool,
        location: SourceLocation,
    ) {
        let depth = self.symbol_table.current_depth();
        let symbol = Symbol::new(name.clone(), kind, is_mutable, depth, location);

        if let Err(err) = self.symbol_table.define(symbol) {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::DuplicateSymbol,
                err,
                location,
            ));
        }
    }

    /// Infer the type of an expression based on its structure
    fn infer_expr_type(&self, expr: &Expr) -> Option<String> {
        match expr {
            // Literal types
            Expr::Number { .. } => Some("Number".to_string()),
            Expr::String { .. } => Some("String".to_string()),
            Expr::Boolean { .. } => Some("Boolean".to_string()),
            Expr::ArrayLiteral { .. } => Some("Array".to_string()),
            Expr::MapLiteral { .. } => Some("Map".to_string()),
            Expr::SetLiteral { .. } => Some("Set".to_string()),
            Expr::Nil { .. } => Some("Nil".to_string()),

            // Variable lookup
            Expr::Variable { name, .. } => {
                self.type_env.get(name).cloned()
            }

            // Grouping - infer from inner expression
            Expr::Grouping { expr, .. } => {
                self.infer_expr_type(expr)
            }

            // Method calls with known return types
            Expr::MethodCall { object, method, .. } => {
                let object_type = self.infer_expr_type(object)?;
                match (object_type.as_str(), method.as_str()) {
                    ("Map", "keys") => Some("Array".to_string()),
                    ("Map", "values") => Some("Array".to_string()),
                    ("Set", "toArray") => Some("Array".to_string()),
                    ("String", "split") => Some("Array".to_string()),
                    ("Array", "join") => Some("String".to_string()),
                    ("Array", "map") => Some("Array".to_string()),
                    ("Array", "filter") => Some("Array".to_string()),
                    _ => None,
                }
            }

            // Binary operations - basic type inference
            Expr::Binary { operator, .. } => {
                use crate::compiler::ast::BinaryOp;
                match operator {
                    BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply |
                    BinaryOp::Divide | BinaryOp::Modulo => {
                        // Arithmetic operations return Number
                        Some("Number".to_string())
                    }
                    BinaryOp::Equal | BinaryOp::NotEqual | BinaryOp::Greater |
                    BinaryOp::GreaterEqual | BinaryOp::Less | BinaryOp::LessEqual |
                    BinaryOp::And | BinaryOp::Or => {
                        // Comparison and logical operations return Boolean
                        Some("Boolean".to_string())
                    }
                }
            }

            // Unary operations
            Expr::Unary { operator, .. } => {
                use crate::compiler::ast::UnaryOp;
                match operator {
                    UnaryOp::Negate => Some("Number".to_string()),
                    UnaryOp::Not => Some("Boolean".to_string()),
                }
            }

            // For other expressions, we can't infer the type
            _ => None,
        }
    }

    // ===== Then: Reference Resolution =====

    fn resolve_statements(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Val {
                name,
                initializer,
                location,
            } => {
                // Resolve initializer first (if any)
                if let Some(init) = initializer {
                    self.resolve_expr(init);
                    // Infer and track type if possible
                    if let Some(inferred_type) = self.infer_expr_type(init) {
                        self.type_env.insert(name.clone(), inferred_type);
                    }
                }
                // Then define the variable in current scope
                self.define_symbol(name.clone(), SymbolKind::Value, false, *location);
            }
            Stmt::Var {
                name,
                initializer,
                location,
            } => {
                // Resolve initializer first (if any)
                if let Some(init) = initializer {
                    self.resolve_expr(init);
                    // Infer and track type if possible
                    if let Some(inferred_type) = self.infer_expr_type(init) {
                        self.type_env.insert(name.clone(), inferred_type);
                    }
                }
                // Then define the variable in current scope
                self.define_symbol(name.clone(), SymbolKind::Variable, true, *location);
            }
            Stmt::Fn {
                params,
                body,
                location,
                ..
            } => {
                // Enter function scope
                self.symbol_table.enter_scope();

                // Define parameters in function scope
                for param in params {
                    let param_location = *location; // Use function location for params
                    self.define_symbol(param.clone(), SymbolKind::Parameter, false, param_location);
                }

                // Resolve function body
                for stmt in body {
                    self.resolve_stmt(stmt);
                }

                // Exit function scope
                self.symbol_table.exit_scope();
            }
            Stmt::Struct { .. } => {
                // Struct declarations are already collected, nothing to resolve
            }
            Stmt::Print { expr, .. } => {
                self.resolve_expr(expr);
            }
            Stmt::Expression { expr, .. } => {
                self.resolve_expr(expr);
            }
            Stmt::Block { statements, .. } => {
                self.symbol_table.enter_scope();
                for stmt in statements {
                    self.resolve_stmt(stmt);
                }
                self.symbol_table.exit_scope();
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.resolve_expr(condition);
                self.resolve_stmt(then_branch);
                if let Some(else_stmt) = else_branch {
                    self.resolve_stmt(else_stmt);
                }
            }
            Stmt::While {
                condition, body, ..
            } => {
                self.resolve_expr(condition);
                self.resolve_stmt(body);
            }
            Stmt::Return { value, .. } => {
                self.resolve_expr(value);
            }
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number { .. } | Expr::String { .. } | Expr::Boolean { .. } | Expr::Nil { .. } => {
                // Literals need no resolution
            }
            Expr::Variable { name, location } => {
                // Check if variable is defined
                if self.symbol_table.resolve(name).is_none() {
                    self.errors.push(CompilationError::new(
                        CompilationPhase::Semantic,
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
                // Resolve the value being assigned
                self.resolve_expr(value);

                // Check if variable exists and is mutable
                match self.symbol_table.resolve(name) {
                    None => {
                        self.errors.push(CompilationError::new(
                            CompilationPhase::Semantic,
                            CompilationErrorKind::UndefinedSymbol,
                            format!("Undefined variable '{}'", name),
                            *location,
                        ));
                    }
                    Some(symbol) => {
                        if !symbol.is_mutable {
                            self.errors.push(CompilationError::new(
                                CompilationPhase::Semantic,
                                CompilationErrorKind::ImmutableAssignment,
                                format!("Cannot assign to immutable variable '{}'", name),
                                *location,
                            ));
                        } else {
                            // Update type tracking for mutable variables
                            if let Some(new_type) = self.infer_expr_type(value) {
                                self.type_env.insert(name.clone(), new_type);
                            } else {
                                // If we can't infer the new type, remove from tracking
                                self.type_env.remove(name);
                            }
                        }
                    }
                }
            }
            Expr::Binary {
                left,
                right,
                operator,
                location,
            } => {
                self.resolve_expr(left);
                self.resolve_expr(right);

                // Additional validation for specific operators could go here
                // For example, ensuring division by zero checks, etc.
                let _ = (operator, location); // Suppress unused warnings for now
            }
            Expr::Unary { operand, .. } => {
                self.resolve_expr(operand);
            }
            Expr::Call {
                callee,
                arguments,
                location,
            } => {
                self.resolve_expr(callee);

                // Validate that callee is a function if it's a variable reference
                if let Expr::Variable { name, .. } = callee.as_ref() {
                    if let Some(symbol) = self.symbol_table.resolve(name) {
                        match &symbol.kind {
                            SymbolKind::Function { arity } => {
                                // Check arity matches
                                if arguments.len() != *arity as usize {
                                    self.errors.push(CompilationError::new(
                                        CompilationPhase::Semantic,
                                        CompilationErrorKind::ArityExceeded,
                                        format!(
                                            "Function '{}' expects {} arguments but got {}",
                                            name,
                                            arity,
                                            arguments.len()
                                        ),
                                        *location,
                                    ));
                                }
                            }
                            SymbolKind::Struct { .. } => {
                                // Calling a struct is valid (constructor)
                            }
                            _ => {
                                self.errors.push(CompilationError::new(
                                    CompilationPhase::Semantic,
                                    CompilationErrorKind::UnexpectedToken,
                                    format!("'{}' is not a function", name),
                                    *location,
                                ));
                            }
                        }
                    }
                }

                // Resolve all arguments
                for arg in arguments {
                    self.resolve_expr(arg);
                }
            }
            Expr::GetField {
                object,
                field,
                location,
            } => {
                self.resolve_expr(object);
                // Field validation could be added here if we track struct types
                let _ = (field, location);
            }
            Expr::SetField {
                object,
                field,
                value,
                location,
            } => {
                self.resolve_expr(object);
                self.resolve_expr(value);
                // Field validation could be added here if we track struct types
                let _ = (field, location);
            }
            Expr::Grouping { expr, .. } => {
                self.resolve_expr(expr);
            }
            Expr::MethodCall {
                object,
                method,
                arguments,
                location,
            } => {
                // Resolve the object and all arguments
                self.resolve_expr(object);
                for arg in arguments {
                    self.resolve_expr(arg);
                }

                // Validate method if we can infer the object's type
                if let Some(object_type) = self.infer_expr_type(object) {
                    // Check if the method is valid for this type
                    if !MethodRegistry::is_valid_method(&object_type, method) {
                        // Method is invalid - try to suggest a correction
                        let error_message = if let Some(suggestion) = MethodRegistry::suggest_method(&object_type, method) {
                            // We found a close match - suggest it
                            format!(
                                "Type '{}' has no method named '{}'. Did you mean '{}'?",
                                object_type, method, suggestion
                            )
                        } else {
                            // No close match - list available methods
                            let available_methods = MethodRegistry::get_methods_for_type(&object_type);
                            if available_methods.is_empty() {
                                format!(
                                    "Type '{}' has no method named '{}' and no available methods",
                                    object_type, method
                                )
                            } else {
                                format!(
                                    "Type '{}' has no method named '{}'. Available methods: {}",
                                    object_type,
                                    method,
                                    available_methods.join(", ")
                                )
                            }
                        };

                        self.errors.push(CompilationError::new(
                            CompilationPhase::Semantic,
                            CompilationErrorKind::Other,
                            error_message,
                            *location,
                        ));
                    }
                }
            }
            Expr::MapLiteral { entries, .. } => {
                // Resolve all key-value pairs in the map literal
                for (key, value) in entries {
                    self.resolve_expr(key);
                    self.resolve_expr(value);
                }
            }
            Expr::ArrayLiteral { elements, .. } => {
                // Resolve all elements in the array literal
                for element in elements {
                    self.resolve_expr(element);
                }
            }
            Expr::SetLiteral { elements, .. } => {
                // Resolve all elements in the set literal
                for element in elements {
                    self.resolve_expr(element);
                }
            }
            Expr::Index { object, index, .. } => {
                // Resolve the object and index expression
                self.resolve_expr(object);
                self.resolve_expr(index);
            }
            Expr::IndexAssign {
                object,
                index,
                value,
                ..
            } => {
                // Resolve the object, index, and value expressions
                self.resolve_expr(object);
                self.resolve_expr(index);
                self.resolve_expr(value);
            }
        }
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
