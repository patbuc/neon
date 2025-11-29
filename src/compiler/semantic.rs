use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};
use crate::common::SourceLocation;
/// Semantic analyzer for the multi-pass compiler
/// Performs semantic analysis on the AST, building symbol tables and validating program semantics
use crate::compiler::ast::{Expr, Stmt};
use crate::compiler::symbol_table::{Symbol, SymbolKind, SymbolTable};

/// Semantic analyzer that validates the AST and builds symbol tables
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    errors: Vec<CompilationError>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
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
            Expr::Array { elements, .. } => {
                // Resolve all elements in the array
                for element in elements {
                    self.resolve_expr(element);
                }
            }
            Expr::Index { object, index, .. } => {
                // Resolve the object being indexed
                self.resolve_expr(object);
                // Resolve the index expression
                self.resolve_expr(index);
            }
            Expr::SetIndex {
                object,
                index,
                value,
                ..
            } => {
                // Resolve the object being indexed
                self.resolve_expr(object);
                // Resolve the index expression
                self.resolve_expr(index);
                // Resolve the value being assigned
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
