use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};
use crate::common::method_registry::MethodRegistry;
use crate::common::module_types::{ExportInfo, ExportKind};
use crate::common::SourceLocation;
/// Semantic analyzer for the multi-pass compiler
/// Performs semantic analysis on the AST, building symbol tables and validating program semantics
use crate::compiler::ast::{Expr, Stmt};
use crate::compiler::symbol_table::{ModuleExport, Symbol, SymbolKind, SymbolTable};
use std::collections::HashMap;
use std::path::PathBuf;

/// Semantic analyzer that validates the AST and builds symbol tables
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    errors: Vec<CompilationError>,
    type_env: HashMap<String, String>,
    loop_depth: u32,
    /// Exports collected from Export statements
    exports: Vec<ExportInfo>,
    /// Current file path (for relative imports)
    current_file: Option<PathBuf>,
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

        // Pre-define File as a built-in global function
        // This corresponds to the File constructor that will be available at runtime
        let file_symbol = Symbol {
            name: "File".to_string(),
            kind: SymbolKind::Function { arity: 1 },
            is_mutable: false,
            scope_depth: 0,
            location: SourceLocation {
                offset: 0,
                line: 0,
                column: 0,
            },
        };
        let _ = symbol_table.define(file_symbol); // Ignore error since this is initial setup

        // Pre-define args as a built-in global constant (array)
        // This corresponds to the command-line arguments array that will be available at runtime
        let args_symbol = Symbol {
            name: "args".to_string(),
            kind: SymbolKind::Value,
            is_mutable: false,
            scope_depth: 0,
            location: SourceLocation {
                offset: 0,
                line: 0,
                column: 0,
            },
        };
        let _ = symbol_table.define(args_symbol); // Ignore error since this is initial setup

        let mut type_env = HashMap::new();
        // Track that args is an Array type for method validation
        type_env.insert("args".to_string(), "Array".to_string());

        SemanticAnalyzer {
            symbol_table,
            errors: Vec::new(),
            type_env,
            loop_depth: 0,
            exports: Vec::new(),
            current_file: None,
        }
    }

    /// Set the current file path (for relative imports)
    pub fn set_current_file(&mut self, path: PathBuf) {
        self.current_file = Some(path);
    }

    /// Get the collected exports
    pub fn exports(&self) -> &Vec<ExportInfo> {
        &self.exports
    }

    /// Resolve imports in the AST using the provided ModuleResolver
    /// This should be called after analyze() to register imported modules in the symbol table
    pub fn resolve_imports(
        &mut self,
        statements: &[Stmt],
        resolve_module: impl Fn(&str, SourceLocation) -> Result<(PathBuf, HashMap<String, ExportInfo>), CompilationError>,
    ) -> Result<(), Vec<CompilationError>> {
        let mut import_errors = Vec::new();

        for stmt in statements {
            if let Stmt::Import { module_path, location } = stmt {
                match resolve_module(module_path, *location) {
                    Ok((resolved_path, module_exports)) => {
                        // Convert module exports to symbol table format
                        let mut exports_map = HashMap::new();
                        for (name, export_info) in module_exports {
                            exports_map.insert(
                                name,
                                ModuleExport {
                                    kind: export_info.kind,
                                    global_index: export_info.global_index,
                                },
                            );
                        }

                        // Extract module name from path (last component without extension)
                        let module_name = resolved_path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("module")
                            .to_string();

                        // Register the module as a symbol in the symbol table (at global scope)
                        let module_symbol = Symbol::new(
                            module_name,
                            SymbolKind::Module {
                                module_path: resolved_path,
                                exports: exports_map,
                            },
                            false,
                            0, // Global scope depth
                            *location,
                        );

                        if let Err(err) = self.symbol_table.define(module_symbol) {
                            import_errors.push(CompilationError::new(
                                CompilationPhase::Semantic,
                                CompilationErrorKind::DuplicateSymbol,
                                err,
                                *location,
                            ));
                        }
                    }
                    Err(err) => {
                        import_errors.push(err);
                    }
                }
            }
        }

        if import_errors.is_empty() {
            Ok(())
        } else {
            Err(import_errors)
        }
    }

    /// Analyze the AST and return the symbol table if successful
    pub fn analyze(&mut self, statements: &[Stmt], current_file: Option<PathBuf>) -> CompilationResult<SymbolTable> {
        // Set the current file path for module resolution
        if let Some(file_path) = current_file {
            self.set_current_file(file_path);
        }

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
            self.collect_declaration(stmt, false);
        }
    }

    fn collect_declaration(&mut self, stmt: &Stmt, is_exported: bool) {
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

                // If this is exported, add to exports
                if is_exported {
                    self.add_export(
                        name.clone(),
                        ExportKind::Function { arity },
                        usize::MAX, // Sentinel value; actual global_index will be set during code generation
                    );
                }
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

                // If this is exported, add to exports
                if is_exported {
                    self.add_export(
                        name.clone(),
                        ExportKind::Struct {
                            fields: fields.clone(),
                        },
                        usize::MAX, // Sentinel value; actual global_index will be set during code generation
                    );
                }
            }
            Stmt::Var {
                name,
                ..
            } => {
                // Variables - just track export, definition happens in resolve_stmt
                if is_exported {
                    self.add_export(
                        name.clone(),
                        ExportKind::Variable,
                        usize::MAX, // Sentinel value; actual global_index will be set during code generation
                    );
                }
            }
            Stmt::Val {
                name,
                ..
            } => {
                // Values - just track export, definition happens in resolve_stmt
                if is_exported {
                    self.add_export(
                        name.clone(),
                        ExportKind::Variable,
                        usize::MAX, // Sentinel value; actual global_index will be set during code generation
                    );
                }
            }
            Stmt::Export { declaration, .. } => {
                // Recursively collect with export flag set
                self.collect_declaration(declaration, true);
            }
            _ => {}
        }
    }

    fn add_export(&mut self, name: String, kind: ExportKind, global_index: usize) {
        self.exports.push(ExportInfo {
            name,
            kind,
            global_index,
        });
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
            Expr::StringInterpolation { .. } => Some("String".to_string()),
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

            // Call expression
            Expr::Call { callee, arguments: _, .. } => {
                // Check if this is a method call: Call { callee: GetField { object, field }, arguments }
                if let Expr::GetField { object, field, .. } = callee.as_ref() {
                    // This is a method call obj.method(args)
                    let object_type = self.infer_expr_type(object)?;
                    match (object_type.as_str(), field.as_str()) {
                        ("Map", "keys") => Some("Array".to_string()),
                        ("Map", "values") => Some("Array".to_string()),
                        ("Set", "toArray") => Some("Array".to_string()),
                        ("String", "split") => Some("Array".to_string()),
                        ("String", "charAt") => Some("String".to_string()),
                        ("String", "toUpperCase") => Some("String".to_string()),
                        ("String", "toLowerCase") => Some("String".to_string()),
                        ("String", "trim") => Some("String".to_string()),
                        ("String", "toString") => Some("String".to_string()),
                        ("String", "toInt") => Some("Number".to_string()),
                        ("String", "toFloat") => Some("Number".to_string()),
                        ("Number", "toString") => Some("String".to_string()),
                        ("Array", "join") => Some("String".to_string()),
                        ("Array", "map") => Some("Array".to_string()),
                        ("Array", "filter") => Some("Array".to_string()),
                        _ => None,
                    }
                } else {
                    // Regular function call - can't easily infer return type without more info
                    None
                }
            }

            // Binary operations - basic type inference
            Expr::Binary { operator, left, right, .. } => {
                use crate::compiler::ast::BinaryOp;
                match operator {
                    BinaryOp::Add => {
                        // Add can be either string concatenation or numeric addition
                        let left_type = self.infer_expr_type(left);
                        let right_type = self.infer_expr_type(right);

                        // If both operands are strings, result is string
                        if let (Some(lt), Some(rt)) = (left_type, right_type) {
                            if lt == "String" && rt == "String" {
                                return Some("String".to_string());
                            }
                        }

                        // Otherwise, assume numeric addition
                        Some("Number".to_string())
                    }
                    BinaryOp::Subtract | BinaryOp::Multiply |
                    BinaryOp::Divide | BinaryOp::FloorDivide | BinaryOp::Modulo => {
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

    /// Helper method to check if a variable exists and is mutable
    fn check_variable_mutability(&mut self, name: &str, location: SourceLocation) {
        match self.symbol_table.resolve(name) {
            None => {
                self.errors.push(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::UndefinedSymbol,
                    format!("Undefined variable '{}'", name),
                    location,
                ));
            }
            Some(symbol) => {
                if !symbol.is_mutable {
                    self.errors.push(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::ImmutableAssignment,
                        format!("Cannot modify immutable variable '{}'", name),
                        location,
                    ));
                }
            }
        }
    }

    fn resolve_statements(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Val { name, initializer, location } => {
                self.resolve_val_declaration(name, initializer.as_ref(), *location);
            }
            Stmt::Var { name, initializer, location } => {
                self.resolve_var_declaration(name, initializer.as_ref(), *location);
            }
            Stmt::Fn { params, body, location, .. } => {
                self.resolve_function_declaration(params, body, *location);
            }
            Stmt::Struct { .. } => {
                // Struct declarations are already collected, nothing to resolve
            }
            Stmt::Expression { expr, .. } => {
                self.resolve_expr(expr);
            }
            Stmt::Block { statements, .. } => {
                self.resolve_block_statement(statements);
            }
            Stmt::If { condition, then_branch, else_branch, .. } => {
                self.resolve_if_statement(condition, then_branch, else_branch.as_ref().map(|v| &**v));
            }
            Stmt::While { condition, body, .. } => {
                self.resolve_while_statement(condition, body);
            }
            Stmt::Return { value, .. } => {
                self.resolve_expr(value);
            }
            Stmt::Break { location } => {
                self.validate_break_statement(*location);
            }
            Stmt::Continue { location } => {
                self.validate_continue_statement(*location);
            }
            Stmt::ForIn { variable, collection, body, location } => {
                self.resolve_for_in_statement(variable, collection, body, *location);
            }
            Stmt::Import { module_path, location } => {
                // Validate that the module path is not empty
                if module_path.is_empty() {
                    self.errors.push(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::Other,
                        "Module path cannot be empty".to_string(),
                        *location,
                    ));
                }

                // Import resolution happens in a separate phase with access to ModuleResolver
                // For now, we just validate the syntax
                // The actual module compilation and symbol registration will be done by
                // the compiler which has access to the ModuleResolver
            }
            Stmt::Export { declaration, .. } => {
                // Resolve the declaration being exported
                self.resolve_stmt(declaration);
                // Module system export validation will be implemented in later phases
            }
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number { .. } | Expr::String { .. } | Expr::Boolean { .. } | Expr::Nil { .. } => {
                // Literals need no resolution
            }
            Expr::StringInterpolation { parts, .. } => {
                self.resolve_string_interpolation(parts);
            }
            Expr::Variable { name, location } => {
                self.resolve_variable(name, *location);
            }
            Expr::Assign { name, value, location } => {
                self.resolve_assignment(name, value, *location);
            }
            Expr::Binary { left, right, operator, location } => {
                self.resolve_binary_expr(left, right, operator, *location);
            }
            Expr::Unary { operand, .. } => {
                self.resolve_expr(operand);
            }
            Expr::Call { callee, arguments, location } => {
                self.resolve_call_expr(callee, arguments, *location);
            }
            Expr::GetField { object, field, location } => {
                self.resolve_get_field(object, field, *location);
            }
            Expr::SetField { object, field, value, location } => {
                self.resolve_set_field(object, field, value, *location);
            }
            Expr::Grouping { expr, .. } => {
                self.resolve_expr(expr);
            }
            Expr::MapLiteral { entries, .. } => {
                self.resolve_map_literal(entries);
            }
            Expr::ArrayLiteral { elements, .. } => {
                self.resolve_array_literal(elements);
            }
            Expr::SetLiteral { elements, .. } => {
                self.resolve_set_literal(elements);
            }
            Expr::Index { object, index, .. } => {
                self.resolve_index_expr(object, index);
            }
            Expr::IndexAssign { object, index, value, .. } => {
                self.resolve_index_assignment(object, index, value);
            }
            Expr::Range { start, end, .. } => {
                self.resolve_range_expr(start, end);
            }
            Expr::PostfixIncrement { operand, location } => {
                self.resolve_postfix_increment(operand, *location);
            }
            Expr::PostfixDecrement { operand, location } => {
                self.resolve_postfix_decrement(operand, *location);
            }
        }
    }

    fn is_static_namespace(&self, name: &str) -> bool {
        matches!(name, "Math")
    }

    // Statement resolution methods

    fn resolve_val_declaration(&mut self, name: &str, initializer: Option<&Expr>, location: SourceLocation) {
        // Resolve initializer first (if any)
        if let Some(init) = initializer {
            self.resolve_expr(init);
            // Infer and track type if possible
            if let Some(inferred_type) = self.infer_expr_type(init) {
                self.type_env.insert(name.to_string(), inferred_type);
            }
        }
        // Then define the variable in current scope
        self.define_symbol(name.to_string(), SymbolKind::Value, false, location);
    }

    fn resolve_var_declaration(&mut self, name: &str, initializer: Option<&Expr>, location: SourceLocation) {
        // Resolve initializer first (if any)
        if let Some(init) = initializer {
            self.resolve_expr(init);
            // Infer and track type if possible
            if let Some(inferred_type) = self.infer_expr_type(init) {
                self.type_env.insert(name.to_string(), inferred_type);
            }
        }
        // Then define the variable in current scope
        self.define_symbol(name.to_string(), SymbolKind::Variable, true, location);
    }

    fn resolve_function_declaration(&mut self, params: &[String], body: &[Stmt], location: SourceLocation) {
        // Enter function scope
        self.symbol_table.enter_scope();

        // Define parameters in function scope
        for param in params {
            let param_location = location; // Use function location for params
            self.define_symbol(param.clone(), SymbolKind::Parameter, false, param_location);
        }

        // Resolve function body
        for stmt in body {
            self.resolve_stmt(stmt);
        }

        // Exit function scope
        self.symbol_table.exit_scope();
    }

    fn resolve_block_statement(&mut self, statements: &[Stmt]) {
        self.symbol_table.enter_scope();
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
        self.symbol_table.exit_scope();
    }

    fn resolve_if_statement(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: Option<&Stmt>) {
        self.resolve_expr(condition);
        self.resolve_stmt(then_branch);
        if let Some(else_stmt) = else_branch {
            self.resolve_stmt(else_stmt);
        }
    }

    fn resolve_while_statement(&mut self, condition: &Expr, body: &Stmt) {
        self.resolve_expr(condition);
        self.loop_depth += 1;
        self.resolve_stmt(body);
        self.loop_depth -= 1;
    }

    fn resolve_for_in_statement(&mut self, variable: &str, collection: &Expr, body: &Stmt, location: SourceLocation) {
        // Resolve the collection expression
        self.resolve_expr(collection);

        // Enter a new scope for the loop
        self.symbol_table.enter_scope();

        // Define the loop variable as immutable (always val)
        self.define_symbol(variable.to_string(), SymbolKind::Value, false, location);

        // Track loop depth for break/continue validation
        self.loop_depth += 1;

        // Resolve the loop body
        self.resolve_stmt(body);

        // Exit loop depth tracking
        self.loop_depth -= 1;

        // Exit the loop scope
        self.symbol_table.exit_scope();
    }

    fn validate_break_statement(&mut self, location: SourceLocation) {
        if self.loop_depth == 0 {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::Other,
                "Cannot use 'break' outside of a loop".to_string(),
                location,
            ));
        }
    }

    fn validate_continue_statement(&mut self, location: SourceLocation) {
        if self.loop_depth == 0 {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::Other,
                "Cannot use 'continue' outside of a loop".to_string(),
                location,
            ));
        }
    }

    // Expression resolution methods

    fn resolve_string_interpolation(&mut self, parts: &[crate::compiler::ast::InterpolationPart]) {
        use crate::compiler::ast::InterpolationPart;
        // Resolve all expression parts
        for part in parts {
            if let InterpolationPart::Expression(expr) = part {
                self.resolve_expr(expr);
            }
        }
    }

    fn resolve_variable(&mut self, name: &str, location: SourceLocation) {
        // Check if variable is defined
        if self.symbol_table.resolve(name).is_none() {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::UndefinedSymbol,
                format!("Undefined variable '{}'", name),
                location,
            ));
        }
    }

    fn resolve_assignment(&mut self, name: &str, value: &Expr, location: SourceLocation) {
        // Resolve the value being assigned
        self.resolve_expr(value);

        // Check if variable exists and is mutable
        match self.symbol_table.resolve(name) {
            None => {
                self.errors.push(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::UndefinedSymbol,
                    format!("Undefined variable '{}'", name),
                    location,
                ));
            }
            Some(symbol) => {
                if !symbol.is_mutable {
                    self.errors.push(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::ImmutableAssignment,
                        format!("Cannot assign to immutable variable '{}'", name),
                        location,
                    ));
                } else {
                    // Update type tracking for mutable variables
                    if let Some(new_type) = self.infer_expr_type(value) {
                        self.type_env.insert(name.to_string(), new_type);
                    } else {
                        // If we can't infer the new type, remove from tracking
                        self.type_env.remove(name);
                    }
                }
            }
        }
    }

    fn resolve_binary_expr(&mut self, left: &Expr, right: &Expr, _operator: &crate::compiler::ast::BinaryOp, _location: SourceLocation) {
        self.resolve_expr(left);
        self.resolve_expr(right);

        // Additional validation for specific operators could go here
        // For example, ensuring division by zero checks, etc.
    }

    fn resolve_call_expr(&mut self, callee: &Expr, arguments: &[Expr], location: SourceLocation) {
        // Check if this is a method call: Call { callee: GetField { object, field }, arguments }
        if let Expr::GetField { object, field, .. } = callee {
            self.resolve_method_call(object, field, arguments, location);
        } else {
            self.resolve_function_call(callee, arguments, location);
        }
    }

    fn resolve_method_call(&mut self, object: &Expr, method: &str, arguments: &[Expr], location: SourceLocation) {
        // This is a method call obj.method(args)
        self.resolve_expr(object);
        for arg in arguments {
            self.resolve_expr(arg);
        }

        // Check if this is a module function call (module.function())
        if let Expr::Variable { name, .. } = object {
            if let Some(symbol) = self.symbol_table.resolve(name) {
                if let SymbolKind::Module { exports, .. } = &symbol.kind {
                    // Validate that the field exists and is a function
                    if let Some(export) = exports.get(method) {
                        if let ExportKind::Function { arity } = export.kind {
                            // Check arity matches (arity validation for module functions)
                            // Note: We don't have full arity info yet in ExportKind
                            // This will be enhanced in later phases
                            let _ = arity;
                        } else {
                            self.errors.push(CompilationError::new(
                                CompilationPhase::Semantic,
                                CompilationErrorKind::UnexpectedToken,
                                format!("'{}' in module '{}' is not a function", method, name),
                                location,
                            ));
                        }
                    }
                    // If export doesn't exist, error already reported in GetField validation
                    return;
                }
            }

            // Check if this is a static method call (e.g., Math.abs)
            if self.is_static_namespace(name) {
                self.validate_static_method(name, method, location);
                return;
            }
        }

        // Instance method call - validate method if we can infer the object's type
        if let Some(object_type) = self.infer_expr_type(object) {
            self.validate_instance_method(&object_type, method, location);
        }
    }

    fn resolve_function_call(&mut self, callee: &Expr, arguments: &[Expr], location: SourceLocation) {
        // Check if this is a global function call first
        let is_global_function = if let Expr::Variable { name, .. } = callee {
            crate::common::method_registry::get_native_method_index("", name).is_some()
        } else {
            false
        };

        if is_global_function {
            // Don't resolve the callee as a variable for global functions
            // Just validate the arguments
            for arg in arguments {
                self.resolve_expr(arg);
            }
        } else {
            // Regular function call - resolve callee as normal
            self.resolve_expr(callee);

            // Validate that callee is a function if it's a variable reference
            if let Expr::Variable { name, .. } = callee {
                self.validate_function_call(name, arguments, location);
            }

            // Resolve all arguments
            for arg in arguments {
                self.resolve_expr(arg);
            }
        }
    }

    fn resolve_get_field(&mut self, object: &Expr, field: &str, location: SourceLocation) {
        self.resolve_expr(object);

        // Special handling for module access (module.symbol)
        if let Expr::Variable { name, .. } = object {
            if let Some(symbol) = self.symbol_table.resolve(name) {
                if let SymbolKind::Module { exports, .. } = &symbol.kind {
                    // Validate that the field exists in the module's exports
                    if !exports.contains_key(field) {
                        let available_exports: Vec<&String> = exports.keys().collect();
                        let error_message = if available_exports.is_empty() {
                            format!("Module '{}' has no exported symbols", name)
                        } else {
                            format!(
                                "Module '{}' has no export named '{}'. Available exports: {}",
                                name,
                                field,
                                available_exports.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
                            )
                        };
                        self.errors.push(CompilationError::new(
                            CompilationPhase::Semantic,
                            CompilationErrorKind::UndefinedSymbol,
                            error_message,
                            location,
                        ));
                    }// Module field access validated
                }
            }
        }

        // Regular field validation could be added here if we track struct types
    }

    fn resolve_set_field(&mut self, object: &Expr, _field: &str, value: &Expr, _location: SourceLocation) {
        self.resolve_expr(object);
        self.resolve_expr(value);
        // Field validation could be added here if we track struct types
    }

    fn resolve_map_literal(&mut self, entries: &[(Expr, Expr)]) {
        // Resolve all key-value pairs in the map literal
        for (key, value) in entries {
            self.resolve_expr(key);
            self.resolve_expr(value);
        }
    }

    fn resolve_array_literal(&mut self, elements: &[Expr]) {
        // Resolve all elements in the array literal
        for element in elements {
            self.resolve_expr(element);
        }
    }

    fn resolve_set_literal(&mut self, elements: &[Expr]) {
        // Resolve all elements in the set literal
        for element in elements {
            self.resolve_expr(element);
        }
    }

    fn resolve_index_expr(&mut self, object: &Expr, index: &Expr) {
        // Resolve the object and index expression
        self.resolve_expr(object);
        self.resolve_expr(index);
    }

    fn resolve_index_assignment(&mut self, object: &Expr, index: &Expr, value: &Expr) {
        // Resolve the object, index, and value expressions
        self.resolve_expr(object);
        self.resolve_expr(index);
        self.resolve_expr(value);
    }

    fn resolve_range_expr(&mut self, start: &Expr, end: &Expr) {
        // Resolve the start and end expressions
        self.resolve_expr(start);
        self.resolve_expr(end);
    }

    fn resolve_postfix_increment(&mut self, operand: &Expr, location: SourceLocation) {
        // Postfix increment can only be applied to simple variables
        match operand {
            Expr::Variable { name, .. } => {
                // Check if variable exists and is mutable
                self.check_variable_mutability(name, location);
            }
            _ => {
                self.errors.push(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::Other,
                    "Increment operator can only be applied to variables".to_string(),
                    location,
                ));
            }
        }
    }

    fn resolve_postfix_decrement(&mut self, operand: &Expr, location: SourceLocation) {
        // Postfix decrement can only be applied to simple variables
        match operand {
            Expr::Variable { name, .. } => {
                // Check if variable exists and is mutable
                self.check_variable_mutability(name, location);
            }
            _ => {
                self.errors.push(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::Other,
                    "Decrement operator can only be applied to variables".to_string(),
                    location,
                ));
            }
        }
    }

    // Validation helper methods

    fn validate_static_method(&mut self, namespace: &str, method: &str, location: SourceLocation) {
        // Static method call - validate against method registry
        if crate::common::method_registry::get_native_method_index(namespace, method).is_none() {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::Other,
                format!("Static method '{}' not found in namespace '{}'", method, namespace),
                location,
            ));
        }
    }

    fn validate_instance_method(&mut self, object_type: &str, method: &str, location: SourceLocation) {
        // Check if the method is valid for this type
        if !MethodRegistry::is_valid_method(object_type, method) {
            // Method is invalid - try to suggest a correction
            let error_message = if let Some(suggestion) = MethodRegistry::suggest_method(object_type, method) {
                // We found a close match - suggest it
                format!(
                    "Type '{}' has no method named '{}'. Did you mean '{}'?",
                    object_type, method, suggestion
                )
            } else {
                // No close match - list available methods
                let available_methods = MethodRegistry::get_methods_for_type(object_type);
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
                location,
            ));
        }
    }

    fn validate_function_call(&mut self, function_name: &str, arguments: &[Expr], location: SourceLocation) {
        if let Some(symbol) = self.symbol_table.resolve(function_name) {
            match &symbol.kind {
                SymbolKind::Function { arity } => {
                    // Check arity matches
                    if arguments.len() != *arity as usize {
                        self.errors.push(CompilationError::new(
                            CompilationPhase::Semantic,
                            CompilationErrorKind::ArityExceeded,
                            format!(
                                "Function '{}' expects {} arguments but got {}",
                                function_name,
                                arity,
                                arguments.len()
                            ),
                            location,
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
                        format!("'{}' is not a function", function_name),
                        location,
                    ));
                }
            }
        }
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
