use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};

use crate::common::SourceLocation;
/// Semantic analyzer for the multi-pass compiler
/// Performs semantic analysis on the AST, building symbol tables and validating program semantics
use crate::compiler::ast::{Expr, Stmt};
use crate::compiler::symbol_table::{Symbol, SymbolKind, SymbolTable};
use std::collections::HashMap;

/// A method's call signature: the rule that decides whether it's callable
/// as `receiver.method(...)` or `Type.method(...)` is a single fact - does
/// its first parameter literally read `self`.
#[derive(Clone, Copy)]
struct MethodSignature {
    param_count: u8,
    takes_self: bool,
}

/// Which syntactic form a method call used.
#[derive(Clone, Copy, PartialEq)]
enum MethodCallKind {
    /// `Type.method(...)`
    Static,
    /// `receiver.method(...)`
    Instance,
}

/// Semantic analyzer that validates the AST and builds symbol tables
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    errors: Vec<CompilationError>,
    // One map per active scope, mirroring the symbol table's scope chain.
    // A present key shadows any outer type for that name; its value is
    // the known static type, or None if the type is unknown.
    type_env: Vec<HashMap<String, Option<String>>>,
    loop_depth: u32,
    // Methods contributed by `impl` blocks, keyed by type name (a struct or
    // a builtin type) then method name.
    struct_methods: HashMap<String, HashMap<String, MethodSignature>>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut symbol_table = SymbolTable::new();
        let mut type_env = vec![HashMap::new()];

        // Namespaces (Math, File, ...) come from the method registry, the
        // single source of truth for what's callable as `Name.method(...)`
        // or constructible as `Name(...)`.
        for namespace in crate::common::method_registry::namespaces() {
            let symbol = Symbol {
                name: namespace.to_string(),
                kind: SymbolKind::Namespace,
                is_mutable: false,
                scope_depth: 0,
                location: SourceLocation::default(),
            };
            let _ = symbol_table.define(symbol); // Ignore error since this is initial setup
        }

        // Runtime builtin values (args, ...) come from the same list the VM
        // uses to construct them.
        for (name, type_name) in crate::common::stdlib::BUILTIN_VALUES {
            let symbol = Symbol {
                name: name.to_string(),
                kind: SymbolKind::Value,
                is_mutable: false,
                scope_depth: 0,
                location: SourceLocation::default(),
            };
            let _ = symbol_table.define(symbol); // Ignore error since this is initial setup
            type_env[0].insert(name.to_string(), Some(type_name.to_string()));
        }

        SemanticAnalyzer {
            symbol_table,
            errors: Vec::new(),
            type_env,
            loop_depth: 0,
            struct_methods: HashMap::new(),
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
                    ..
                } => {
                    if crate::common::method_registry::BUILTIN_TYPE_NAMES.contains(&name.as_str()) {
                        self.errors.push(CompilationError::new(
                            CompilationPhase::Semantic,
                            CompilationErrorKind::Other,
                            format!("Struct name '{}' is reserved for a builtin type", name),
                            *location,
                        ));
                        continue;
                    }
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

        // Second pass: impl blocks, so a method call anywhere validates.
        for stmt in statements {
            if let Stmt::Impl {
                type_name,
                methods,
                location,
            } = stmt
            {
                self.collect_impl_block(type_name, methods, *location);
            }
        }

        // Third pass: resolve method bodies now that every impl block has
        // contributed its methods. Skip an impl for an undefined type - its
        // methods were never registered, so resolving bodies would only add
        // follow-on errors on top of the undefined-type error.
        for stmt in statements {
            if let Stmt::Impl {
                type_name, methods, ..
            } = stmt
            {
                if !self.is_struct_type(type_name)
                    && !crate::common::method_registry::BUILTIN_TYPE_NAMES
                        .contains(&type_name.as_str())
                {
                    continue;
                }
                for method in methods {
                    if let Stmt::Fn {
                        params,
                        body,
                        location,
                        ..
                    } = method
                    {
                        self.resolve_function_body(params, body, *location, Some(type_name));
                    }
                }
            }
        }
    }

    /// Register the methods an `impl` block contributes to a struct or a
    /// builtin type, flagging an impl for an undefined type, a method
    /// already defined for this type, one shadowing a field name, or one
    /// shadowing a native method (builtin types only).
    fn collect_impl_block(&mut self, type_name: &str, methods: &[Stmt], location: SourceLocation) {
        let is_builtin_type =
            crate::common::method_registry::BUILTIN_TYPE_NAMES.contains(&type_name);
        let field_names = if is_builtin_type {
            Vec::new()
        } else {
            match self.symbol_table.resolve(type_name) {
                Some(Symbol {
                    kind: SymbolKind::Struct { fields },
                    ..
                }) => fields.clone(),
                _ => {
                    self.errors.push(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::UndefinedSymbol,
                        format!("Cannot implement undefined type '{}'", type_name),
                        location,
                    ));
                    return;
                }
            }
        };

        for method in methods {
            let Stmt::Fn {
                name,
                params,
                location: method_location,
                ..
            } = method
            else {
                continue;
            };

            if crate::common::method_registry::is_valid_method(type_name, name) {
                self.errors.push(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::Other,
                    format!(
                        "Method '{}' is already a native method of {}",
                        name, type_name
                    ),
                    *method_location,
                ));
                continue;
            }

            let takes_self = params.first().map(String::as_str) == Some("self");
            if is_builtin_type && !takes_self {
                self.errors.push(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::Other,
                    format!(
                        "Method '{}' on {} must take self; static methods are only supported on structs",
                        name, type_name
                    ),
                    *method_location,
                ));
                continue;
            }

            if field_names.iter().any(|f| f == name) {
                self.errors.push(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::Other,
                    format!(
                        "Method '{}' has the same name as field '{}' on struct '{}'",
                        name, name, type_name
                    ),
                    *method_location,
                ));
                continue;
            }

            let entry = self
                .struct_methods
                .entry(type_name.to_string())
                .or_default();
            if entry.contains_key(name) {
                self.errors.push(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::DuplicateSymbol,
                    format!(
                        "Method '{}' is already defined for type '{}'",
                        name, type_name
                    ),
                    *method_location,
                ));
                continue;
            }

            entry.insert(
                name.clone(),
                MethodSignature {
                    param_count: params.len() as u8,
                    takes_self,
                },
            );
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

    /// Enter a new lexical scope, keeping the type environment in step
    /// with the symbol table.
    fn enter_scope(&mut self) {
        self.symbol_table.enter_scope();
        self.type_env.push(HashMap::new());
    }

    /// Exit the current lexical scope, keeping the type environment in
    /// step with the symbol table.
    fn exit_scope(&mut self) {
        self.symbol_table.exit_scope();
        self.type_env.pop();
    }

    /// Define a name's static type (or None if unknown) in the current
    /// scope, shadowing any outer type recorded for the same name.
    fn define_type(&mut self, name: &str, ty: Option<String>) {
        self.type_env
            .last_mut()
            .expect("global scope always present")
            .insert(name.to_string(), ty);
    }

    /// Update a name's static type in the scope where it was last
    /// defined, searching outward from the current scope. Falls back to
    /// defining it in the current scope if it isn't tracked yet.
    fn set_type(&mut self, name: &str, ty: Option<String>) {
        for scope in self.type_env.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), ty);
                return;
            }
        }
        self.define_type(name, ty);
    }

    /// Look up a name's static type, searching from the innermost scope
    /// outward. A name bound with no known type stops the search there.
    fn lookup_type(&self, name: &str) -> Option<String> {
        for scope in self.type_env.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return ty.clone();
            }
        }
        None
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
            Expr::Variable { name, .. } => self.lookup_type(name),

            // Grouping - infer from inner expression
            Expr::Grouping { expr, .. } => self.infer_expr_type(expr),

            // Call expression
            Expr::Call { callee, .. } => {
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
                } else if let Expr::Variable { name, .. } = callee.as_ref() {
                    // A direct call to a known struct is a constructor;
                    // its result is statically an instance of that struct.
                    match self.symbol_table.resolve(name) {
                        Some(Symbol {
                            kind: SymbolKind::Struct { .. },
                            ..
                        }) => Some(name.clone()),
                        _ => None,
                    }
                } else {
                    // Regular function call - can't easily infer return type without more info
                    None
                }
            }

            // Binary operations - basic type inference
            Expr::Binary {
                operator,
                left,
                right,
                ..
            } => {
                use crate::compiler::ast::BinaryOp;
                match operator {
                    BinaryOp::Add => {
                        // Add can be either string concatenation or numeric
                        // addition. Only fn_add's own outcomes are valid at
                        // runtime (String + String, or Number + Number), so
                        // an unknown operand can never rule out String: it
                        // could still turn out to be one at runtime.
                        let left_type = self.infer_expr_type(left);
                        let right_type = self.infer_expr_type(right);

                        if left_type.as_deref() == Some("String")
                            || right_type.as_deref() == Some("String")
                        {
                            Some("String".to_string())
                        } else if left_type.as_deref() == Some("Number")
                            && right_type.as_deref() == Some("Number")
                        {
                            Some("Number".to_string())
                        } else {
                            None
                        }
                    }
                    BinaryOp::Subtract
                    | BinaryOp::Multiply
                    | BinaryOp::Divide
                    | BinaryOp::Modulo
                    | BinaryOp::Exponent => {
                        // Arithmetic operations return Number
                        Some("Number".to_string())
                    }
                    BinaryOp::Equal
                    | BinaryOp::NotEqual
                    | BinaryOp::Greater
                    | BinaryOp::GreaterEqual
                    | BinaryOp::Less
                    | BinaryOp::LessEqual
                    | BinaryOp::And
                    | BinaryOp::Or => {
                        // Comparison and logical operations return Boolean
                        Some("Boolean".to_string())
                    }
                    BinaryOp::BitwiseAnd
                    | BinaryOp::BitwiseOr
                    | BinaryOp::BitwiseXor
                    | BinaryOp::LeftShift
                    | BinaryOp::RightShift => {
                        // Bitwise operations return Number
                        Some("Number".to_string())
                    }
                }
            }

            // Unary operations
            Expr::Unary { operator, .. } => {
                use crate::compiler::ast::UnaryOp;
                match operator {
                    UnaryOp::Negate => Some("Number".to_string()),
                    UnaryOp::Not => Some("Boolean".to_string()),
                    UnaryOp::BitwiseNot => Some("Number".to_string()),
                }
            }

            // Conditional (ternary) - try to infer type from branches
            Expr::Conditional {
                then_expr,
                else_expr,
                ..
            } => {
                let then_type = self.infer_expr_type(then_expr);
                let else_type = self.infer_expr_type(else_expr);
                // Only trust the type when both branches agree; a mismatch
                // (including one side being unknown) means the result could
                // be either at runtime, so it's unknown too.
                if then_type == else_type {
                    then_type
                } else {
                    None
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
            Stmt::Val {
                name,
                initializer,
                location,
                ..
            } => {
                self.resolve_variable_declaration(
                    name,
                    initializer.as_ref(),
                    SymbolKind::Value,
                    false,
                    *location,
                );
            }
            Stmt::Var {
                name,
                initializer,
                location,
                ..
            } => {
                self.resolve_variable_declaration(
                    name,
                    initializer.as_ref(),
                    SymbolKind::Variable,
                    true,
                    *location,
                );
            }
            Stmt::Fn {
                name,
                params,
                body,
                location,
                ..
            } => {
                self.resolve_function_declaration(name, params, body, *location);
            }
            Stmt::Struct {
                name,
                fields,
                location,
                ..
            } => {
                // A top-level struct was already resolved in collect_declarations.
                if self.symbol_table.current_depth() != 0 {
                    self.errors.push(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::Other,
                        format!("Struct '{}' must be declared at the top level", name),
                        *location,
                    ));
                    self.define_symbol(
                        name.clone(),
                        SymbolKind::Struct {
                            fields: fields.clone(),
                        },
                        false,
                        *location,
                    );
                }
            }
            Stmt::Impl { location, .. } => {
                // A top-level impl was already resolved in collect_declarations.
                if self.symbol_table.current_depth() != 0 {
                    self.errors.push(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::Other,
                        "'impl' blocks are only allowed at the top level".to_string(),
                        *location,
                    ));
                }
            }
            Stmt::Expression { expr, .. } => {
                self.resolve_expr(expr);
            }
            Stmt::Block { statements, .. } => {
                self.resolve_block_statement(statements);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.resolve_if_statement(
                    condition,
                    then_branch,
                    else_branch.as_ref().map(|v| &**v),
                );
            }
            Stmt::While {
                condition, body, ..
            } => {
                self.resolve_while_statement(condition, body);
            }
            Stmt::Return { value, .. } => {
                self.resolve_expr(value);
            }
            Stmt::Break { location } => {
                self.validate_loop_control_statement("break", *location);
            }
            Stmt::Continue { location } => {
                self.validate_loop_control_statement("continue", *location);
            }
            Stmt::ForIn {
                variable,
                collection,
                body,
                location,
                ..
            } => {
                self.resolve_for_in_statement(variable, collection, body, *location);
            }
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
                ..
            } => {
                self.resolve_for_statement(initializer, condition, increment, body);
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
            Expr::Variable { name, location, .. } => {
                self.resolve_variable(name, *location);
            }
            Expr::Assign {
                name,
                value,
                location,
                ..
            } => {
                self.resolve_assignment(name, value, *location);
            }
            Expr::Binary {
                left,
                right,
                operator,
                location,
            } => {
                self.resolve_binary_expr(left, right, operator, *location);
            }
            Expr::Unary { operand, .. } => {
                self.resolve_expr(operand);
            }
            Expr::Call {
                callee,
                arguments,
                location,
                ..
            } => {
                self.resolve_call_expr(callee, arguments, *location);
            }
            Expr::GetField {
                object,
                field,
                location,
            } => {
                self.resolve_get_field(object, field, *location);
            }
            Expr::SetField {
                object,
                field,
                value,
                location,
            } => {
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
            Expr::IndexAssign {
                object,
                index,
                value,
                ..
            } => {
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
            Expr::Conditional {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                self.resolve_expr(condition);
                self.resolve_expr(then_expr);
                self.resolve_expr(else_expr);
            }
            Expr::Function {
                params,
                body,
                location,
                ..
            } => {
                self.resolve_function_body(params, body, *location, None);
            }
        }
    }

    // Statement resolution methods

    fn resolve_variable_declaration(
        &mut self,
        name: &str,
        initializer: Option<&Expr>,
        kind: SymbolKind,
        is_mutable: bool,
        location: SourceLocation,
    ) {
        // Resolve initializer first (if any), tracking its type (or
        // unknown) in the current scope - this shadows any outer type
        // recorded for the same name.
        let inferred_type = initializer.map(|init| {
            self.resolve_expr(init);
            self.infer_expr_type(init)
        });
        self.define_type(name, inferred_type.flatten());
        // Then define the variable in current scope
        self.define_symbol(name.to_string(), kind, is_mutable, location);
    }

    fn resolve_function_declaration(
        &mut self,
        name: &str,
        params: &[String],
        body: &[Stmt],
        location: SourceLocation,
    ) {
        // A nested function isn't hoisted by collect_declarations, so define it now, before resolving its body, so it can recurse.
        if self.symbol_table.current_depth() > 0 {
            let arity = params.len() as u8;
            self.define_symbol(
                name.to_string(),
                SymbolKind::Function { arity },
                false,
                location,
            );
        }

        self.resolve_function_body(params, body, location, None);
    }

    /// Resolves a function's parameters and body in a fresh scope. Shared by
    /// named function declarations, lambda expressions, and impl methods.
    /// `self_type` names the struct a leading `self` parameter is typed as;
    /// it's `None` outside of an impl method.
    fn resolve_function_body(
        &mut self,
        params: &[String],
        body: &[Stmt],
        location: SourceLocation,
        self_type: Option<&str>,
    ) {
        // Enter function scope
        self.enter_scope();

        // A loop enclosing this declaration must not let break/continue
        // inside the function body see themselves as inside that loop.
        let saved_loop_depth = self.loop_depth;
        self.loop_depth = 0;

        // Define parameters in function scope. Their type is unknown and
        // explicitly shadows any outer type recorded for the same name,
        // except a leading `self` inside an impl method, which is typed as
        // the struct being implemented.
        for (i, param) in params.iter().enumerate() {
            let param_location = location; // Use function location for params
            let param_type = if i == 0 && param == "self" {
                self_type.map(|t| t.to_string())
            } else {
                None
            };
            self.define_type(param, param_type);
            self.define_symbol(param.clone(), SymbolKind::Parameter, false, param_location);
        }

        // Resolve function body
        for stmt in body {
            self.resolve_stmt(stmt);
        }

        self.loop_depth = saved_loop_depth;

        // Exit function scope
        self.exit_scope();
    }

    fn resolve_block_statement(&mut self, statements: &[Stmt]) {
        self.enter_scope();
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
        self.exit_scope();
    }

    fn resolve_if_statement(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
    ) {
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

    fn resolve_for_statement(
        &mut self,
        initializer: &Stmt,
        condition: &Expr,
        increment: &Expr,
        body: &Stmt,
    ) {
        self.enter_scope();

        self.resolve_stmt(initializer);
        self.resolve_expr(condition);

        self.loop_depth += 1;
        self.resolve_stmt(body);
        self.loop_depth -= 1;

        self.resolve_expr(increment);

        self.exit_scope();
    }

    fn resolve_for_in_statement(
        &mut self,
        variable: &str,
        collection: &Expr,
        body: &Stmt,
        location: SourceLocation,
    ) {
        // Resolve the collection expression
        self.resolve_expr(collection);

        // Enter a new scope for the loop
        self.enter_scope();

        // Define the loop variable as immutable (always val). Its type is
        // unknown and explicitly shadows any outer type of the same name.
        self.define_type(variable, None);
        self.define_symbol(variable.to_string(), SymbolKind::Value, false, location);

        // Track loop depth for break/continue validation
        self.loop_depth += 1;

        // Resolve the loop body
        self.resolve_stmt(body);

        // Exit loop depth tracking
        self.loop_depth -= 1;

        // Exit the loop scope
        self.exit_scope();
    }

    fn validate_loop_control_statement(&mut self, keyword: &str, location: SourceLocation) {
        if self.loop_depth == 0 {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::Other,
                format!("Cannot use '{}' outside of a loop", keyword),
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
                if symbol.kind == SymbolKind::Namespace {
                    self.errors.push(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::Other,
                        format!("'{}' is a namespace, not a value", name),
                        location,
                    ));
                }
            }
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
                    // This analysis is flow-insensitive: an assignment may
                    // happen conditionally (e.g. inside an if branch), so
                    // adopting the new value's type here would wrongly
                    // apply it even on paths that never assign. Mark the
                    // type unknown instead, wherever it's tracked.
                    self.set_type(name, None);
                }
            }
        }
    }

    fn resolve_binary_expr(
        &mut self,
        left: &Expr,
        right: &Expr,
        _operator: &crate::compiler::ast::BinaryOp,
        _location: SourceLocation,
    ) {
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

    fn resolve_method_call(
        &mut self,
        object: &Expr,
        method: &str,
        arguments: &[Expr],
        location: SourceLocation,
    ) {
        // This is a method call obj.method(args)
        for arg in arguments {
            self.resolve_expr(arg);
        }

        // Check if this is a static method call (e.g., Math.abs). The
        // namespace name isn't a variable reference, so don't resolve it
        // as one - that would (rightly) reject it as "not a value".
        if let Expr::Variable { name, .. } = object {
            if crate::common::method_registry::is_static_namespace(name) {
                self.validate_static_method(name, method, location);
                return;
            }

            // A static call on a struct's own name, e.g. Point.origin().
            // The receiver is the type itself, not a value of that type.
            if self.is_struct_type(name) {
                let struct_name = name.clone();
                self.validate_struct_method_call(
                    &struct_name,
                    method,
                    arguments.len(),
                    location,
                    MethodCallKind::Static,
                );
                return;
            }

            // A static call on a builtin type name, e.g. Array.second().
            if crate::common::method_registry::BUILTIN_TYPE_NAMES.contains(&name.as_str())
                && self.symbol_table.resolve(name).is_none()
            {
                self.errors.push(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::Other,
                    "Static methods are only supported on structs".to_string(),
                    location,
                ));
                return;
            }
        }

        self.resolve_expr(object);

        // Instance method call - validate method if we can infer the object's type
        if let Some(object_type) = self.infer_expr_type(object) {
            self.validate_instance_method(&object_type, method, arguments.len(), location);
        }
    }

    fn resolve_function_call(
        &mut self,
        callee: &Expr,
        arguments: &[Expr],
        location: SourceLocation,
    ) {
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
            return;
        }

        // Constructor call on a namespace (e.g. File(path)). The namespace
        // name isn't a variable reference, so validate its arity from the
        // registry instead of resolving it as one.
        if let Expr::Variable { name, .. } = callee {
            if let Some(arity) = crate::common::method_registry::constructor_arity(name) {
                self.validate_arity("Function", name, arity, arguments.len(), location);
                for arg in arguments {
                    self.resolve_expr(arg);
                }
                return;
            }
        }

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

    fn resolve_get_field(&mut self, object: &Expr, field: &str, location: SourceLocation) {
        self.resolve_expr(object);
        if let Some(struct_name) = self.infer_expr_type(object) {
            self.validate_struct_field(&struct_name, field, location);
        }
    }

    fn resolve_set_field(
        &mut self,
        object: &Expr,
        field: &str,
        value: &Expr,
        location: SourceLocation,
    ) {
        self.resolve_expr(object);
        self.resolve_expr(value);
        if let Some(struct_name) = self.infer_expr_type(object) {
            self.validate_struct_field(&struct_name, field, location);
        }
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
                format!(
                    "Static method '{}' not found in namespace '{}'",
                    method, namespace
                ),
                location,
            ));
        }
    }

    /// True when `name` is a declared struct type, as opposed to a builtin
    /// type name (Array, Map, ...) or an unresolved name.
    fn is_struct_type(&self, name: &str) -> bool {
        matches!(
            self.symbol_table.resolve(name),
            Some(Symbol {
                kind: SymbolKind::Struct { .. },
                ..
            })
        )
    }

    /// Validate a call to a struct's own method, whether the receiver is an
    /// instance (`p.len()`) or the struct name itself (`Point.origin()`).
    /// Unknown methods get a "Did you mean" suggestion drawn from the
    /// struct's own methods, since a struct never has builtin methods.
    fn validate_struct_method_call(
        &mut self,
        struct_name: &str,
        method: &str,
        arg_count: usize,
        location: SourceLocation,
        call_kind: MethodCallKind,
    ) {
        let signature = self
            .struct_methods
            .get(struct_name)
            .and_then(|methods| methods.get(method).copied());

        if let Some(signature) = signature {
            match (call_kind, signature.takes_self) {
                (MethodCallKind::Static, true) => {
                    self.errors.push(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::Other,
                        format!(
                            "Method '{}' needs an instance; call it on a {} value",
                            method, struct_name
                        ),
                        location,
                    ));
                }
                (MethodCallKind::Instance, false) => {
                    self.errors.push(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::Other,
                        format!(
                            "Method '{}' is static; call it as {}.{}()",
                            method, struct_name, method
                        ),
                        location,
                    ));
                }
                (MethodCallKind::Static, false) => {
                    self.validate_arity(
                        "Method",
                        method,
                        signature.param_count,
                        arg_count,
                        location,
                    );
                }
                (MethodCallKind::Instance, true) => {
                    self.validate_arity(
                        "Method",
                        method,
                        signature.param_count - 1,
                        arg_count,
                        location,
                    );
                }
            }
            return;
        }

        // No method by this name; a field can still be called (its value is
        // callable or not only known at runtime). Fields only exist on
        // instances, so a static call keeps erroring.
        if call_kind == MethodCallKind::Instance && self.struct_has_field(struct_name, method) {
            return;
        }

        let candidates: Vec<String> = self
            .struct_methods
            .get(struct_name)
            .map(|methods| methods.keys().cloned().collect())
            .unwrap_or_default();
        let candidate_refs: Vec<&str> = candidates.iter().map(String::as_str).collect();

        let error_message = unknown_method_error(struct_name, method, &candidate_refs);

        self.errors.push(CompilationError::new(
            CompilationPhase::Semantic,
            CompilationErrorKind::Other,
            error_message,
            location,
        ));
    }

    fn validate_instance_method(
        &mut self,
        object_type: &str,
        method: &str,
        arg_count: usize,
        location: SourceLocation,
    ) {
        if self.is_struct_type(object_type) {
            self.validate_struct_method_call(
                object_type,
                method,
                arg_count,
                location,
                MethodCallKind::Instance,
            );
            return;
        }

        // A native method always wins; a user method can never shadow one.
        if crate::common::method_registry::is_valid_method(object_type, method) {
            return;
        }

        // A user method contributed by an `impl` block on this builtin type.
        if let Some(signature) = self
            .struct_methods
            .get(object_type)
            .and_then(|methods| methods.get(method).copied())
        {
            self.validate_arity(
                "Method",
                method,
                signature.param_count - 1,
                arg_count,
                location,
            );
            return;
        }

        let mut candidates: Vec<String> =
            crate::common::method_registry::get_methods_for_type(object_type)
                .into_iter()
                .map(String::from)
                .collect();
        if let Some(user_methods) = self.struct_methods.get(object_type) {
            candidates.extend(user_methods.keys().cloned());
        }
        let candidate_refs: Vec<&str> = candidates.iter().map(String::as_str).collect();
        let error_message = unknown_method_error(object_type, method, &candidate_refs);

        self.errors.push(CompilationError::new(
            CompilationPhase::Semantic,
            CompilationErrorKind::Other,
            error_message,
            location,
        ));
    }

    /// True when `struct_name` is a known struct type declaring a field
    /// named `field`.
    fn struct_has_field(&self, struct_name: &str, field: &str) -> bool {
        matches!(
            self.symbol_table.resolve(struct_name),
            Some(Symbol {
                kind: SymbolKind::Struct { fields },
                ..
            }) if fields.iter().any(|f| f == field)
        )
    }

    /// Check that a field access/set on a receiver of statically known
    /// struct type refers to one of the struct's declared fields.
    fn validate_struct_field(&mut self, struct_name: &str, field: &str, location: SourceLocation) {
        if !matches!(
            self.symbol_table.resolve(struct_name),
            Some(Symbol {
                kind: SymbolKind::Struct { .. },
                ..
            })
        ) {
            // Not a known struct type - nothing to check.
            return;
        }

        if !self.struct_has_field(struct_name, field) {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::Other,
                format!("Struct '{}' has no field named '{}'", struct_name, field),
                location,
            ));
        }
    }

    fn validate_function_call(
        &mut self,
        function_name: &str,
        arguments: &[Expr],
        location: SourceLocation,
    ) {
        if let Some(symbol) = self.symbol_table.resolve(function_name) {
            match &symbol.kind {
                SymbolKind::Function { arity } => {
                    let arity = *arity;
                    self.validate_arity(
                        "Function",
                        function_name,
                        arity,
                        arguments.len(),
                        location,
                    );
                }
                SymbolKind::Struct { fields } => {
                    let arity = fields.len() as u8;
                    self.validate_arity(
                        "Function",
                        function_name,
                        arity,
                        arguments.len(),
                        location,
                    );
                }
                SymbolKind::Value | SymbolKind::Variable | SymbolKind::Parameter => {
                    // Holds an arbitrary value; whether it's callable, and
                    // with how many arguments, is only known at runtime.
                }
                SymbolKind::Namespace => {
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

    fn validate_arity(
        &mut self,
        kind_label: &str,
        name: &str,
        expected: u8,
        actual: usize,
        location: SourceLocation,
    ) {
        if actual != expected as usize {
            let kind = if actual < expected as usize {
                CompilationErrorKind::TooFewArguments
            } else {
                CompilationErrorKind::ArityExceeded
            };
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                kind,
                format!(
                    "{} '{}' expects {} arguments but got {}",
                    kind_label, name, expected, actual
                ),
                location,
            ));
        }
    }
}

/// Builds the "unknown method" error message for `type_name`, suggesting
/// the closest match among `candidates` when one exists.
fn unknown_method_error(type_name: &str, method: &str, candidates: &[&str]) -> String {
    if let Some(suggestion) =
        crate::common::string_similarity::find_closest_match(method, candidates)
    {
        format!(
            "Type '{}' has no method named '{}'. Did you mean '{}'?",
            type_name, method, suggestion
        )
    } else if candidates.is_empty() {
        format!(
            "Type '{}' has no method named '{}' and no available methods",
            type_name, method
        )
    } else {
        format!(
            "Type '{}' has no method named '{}'. Available methods: {}",
            type_name,
            method,
            candidates.join(", ")
        )
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
