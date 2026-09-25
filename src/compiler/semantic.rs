use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};

use crate::common::static_type::StaticType;
use crate::common::SourceLocation;
/// Semantic analyzer for the multi-pass compiler
/// Performs semantic analysis on the AST, building symbol tables and validating program semantics,
/// and resolves every name use to where it lives at runtime.
use crate::compiler::ast::{Expr, NodeId, Stmt};
use crate::compiler::resolutions::{Capture, DeclId, FunctionResolution, Res, Resolutions};
use crate::compiler::symbol_table::{Symbol, SymbolKind, SymbolTable};
use std::collections::{HashMap, HashSet};

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

/// The fields of a `Symbol` a name use needs, copied out so they can outlive
/// the symbol table borrow.
#[derive(Clone, Copy)]
struct SymbolUse {
    is_mutable: bool,
    builtin_index: Option<u32>,
    decl_id: DeclId,
    decl_level: u32,
    decl_scope_depth: u32,
}

impl From<&Symbol> for SymbolUse {
    fn from(symbol: &Symbol) -> Self {
        SymbolUse {
            is_mutable: symbol.is_mutable,
            builtin_index: builtin_index(symbol),
            decl_id: symbol.decl_id,
            decl_level: symbol.function_level,
            decl_scope_depth: symbol.scope_depth,
        }
    }
}

/// Semantic analyzer that validates the AST and builds symbol tables
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    errors: Vec<CompilationError>,
    // One map per active scope, mirroring the symbol table's scope chain.
    // A present key shadows any outer type for that name; its value is
    // the known static type, or None if the type is unknown.
    type_env: Vec<HashMap<String, Option<StaticType>>>,
    loop_depth: u32,
    // Methods contributed by `impl` blocks, keyed by type name (a struct or
    // a builtin type) then method name.
    struct_methods: HashMap<String, HashMap<String, MethodSignature>>,
    resolutions: Resolutions,
    next_decl_id: u32,
    // One frame per level of function nesting, outermost (the script) first.
    function_frames: Vec<FunctionResolution>,
    // Top-level val/var DeclIds whose declaration statement hasn't been
    // resolved yet; a direct script-level read/write/postfix-op naming one
    // is a compile error.
    not_initialized_top_level: HashSet<DeclId>,
    // DeclId of the top-level val/var currently resolving its own
    // initializer, if any - a direct read of it there is "in its own
    // initializer", not "before its declaration".
    currently_initializing: Option<DeclId>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut symbol_table = SymbolTable::new();
        let mut type_env = vec![HashMap::new()];
        let mut next_decl_id = 0u32;

        // Namespaces (Math, File, ...) come from the method registry, the
        // single source of truth for what's callable as `Name.method(...)`
        // or constructible as `Name(...)`.
        for namespace in crate::common::method_registry::namespaces() {
            let decl_id = DeclId(next_decl_id);
            next_decl_id += 1;
            let symbol = Symbol::new(
                namespace.to_string(),
                SymbolKind::Namespace,
                false,
                0,
                SourceLocation::default(),
                decl_id,
                0,
            );
            let _ = symbol_table.define(symbol); // Ignore error since this is initial setup
        }

        // Runtime builtin values (args, ...) come from the same list the VM
        // uses to construct them.
        for (index, (name, static_type)) in crate::common::stdlib::BUILTIN_VALUES.iter().enumerate()
        {
            let decl_id = DeclId(next_decl_id);
            next_decl_id += 1;
            let symbol = Symbol::new(
                name.to_string(),
                SymbolKind::Builtin {
                    index: index as u32,
                },
                false,
                0,
                SourceLocation::default(),
                decl_id,
                0,
            );
            let _ = symbol_table.define(symbol); // Ignore error since this is initial setup
            type_env[0].insert(name.to_string(), Some(static_type.clone()));
        }

        SemanticAnalyzer {
            symbol_table,
            errors: Vec::new(),
            type_env,
            loop_depth: 0,
            struct_methods: HashMap::new(),
            resolutions: Resolutions::default(),
            next_decl_id,
            function_frames: vec![FunctionResolution::default()],
            not_initialized_top_level: HashSet::new(),
            currently_initializing: None,
        }
    }

    /// Analyze the AST and return the recorded name resolutions if successful
    pub fn analyze(&mut self, statements: &[Stmt]) -> CompilationResult<Resolutions> {
        // First: collect all top-level declarations
        self.collect_declarations(statements);

        // Then: resolve all references and validate
        self.resolve_statements(statements);

        if self.errors.is_empty() {
            Ok(std::mem::take(&mut self.resolutions))
        } else {
            Err(self.errors.clone())
        }
    }

    /// Current function-nesting level (0 = the script).
    fn function_level(&self) -> u32 {
        (self.function_frames.len() - 1) as u32
    }

    fn next_decl_id(&mut self) -> DeclId {
        let id = DeclId(self.next_decl_id);
        self.next_decl_id += 1;
        id
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
                    id,
                    location,
                    ..
                } => {
                    let arity = params.len() as u8;
                    self.declare_symbol(
                        *id,
                        name.clone(),
                        SymbolKind::Function { arity },
                        false,
                        *location,
                    );
                }
                Stmt::Struct {
                    name,
                    fields,
                    id,
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
                    self.declare_symbol(
                        *id,
                        name.clone(),
                        SymbolKind::Struct {
                            fields: fields.clone(),
                        },
                        false,
                        *location,
                    );
                }
                Stmt::Val {
                    name, id, location, ..
                }
                | Stmt::Var {
                    name, id, location, ..
                } => {
                    let is_mutable = matches!(stmt, Stmt::Var { .. });
                    let kind = if is_mutable {
                        SymbolKind::Variable
                    } else {
                        SymbolKind::Value
                    };
                    let decl_id =
                        self.declare_symbol(*id, name.clone(), kind, is_mutable, *location);
                    self.not_initialized_top_level.insert(decl_id);
                }
                _ => {}
            }
        }

        // Second pass, after all structs are declared: impl blocks.
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

    /// Defines a symbol in the current scope, allocating a fresh `DeclId`.
    fn define_symbol(
        &mut self,
        name: String,
        kind: SymbolKind,
        is_mutable: bool,
        location: SourceLocation,
    ) -> DeclId {
        let decl_id = self.next_decl_id();
        let depth = self.symbol_table.current_depth();
        let symbol = Symbol::new(
            name.clone(),
            kind,
            is_mutable,
            depth,
            location,
            decl_id,
            self.function_level(),
        );

        if let Err(err) = self.symbol_table.define(symbol) {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::DuplicateSymbol,
                err,
                location,
            ));
        }
        decl_id
    }

    /// Defines a symbol for a declaration node, recording the node's `DeclId`
    /// so codegen can later map it to a slot.
    fn declare_symbol(
        &mut self,
        id: NodeId,
        name: String,
        kind: SymbolKind,
        is_mutable: bool,
        location: SourceLocation,
    ) -> DeclId {
        let decl_id = self.define_symbol(name, kind, is_mutable, location);
        self.resolutions.record_decl(id, decl_id);
        decl_id
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
    fn define_type(&mut self, name: &str, ty: Option<StaticType>) {
        self.type_env
            .last_mut()
            .expect("global scope always present")
            .insert(name.to_string(), ty);
    }

    /// Update a name's static type in the scope where it was last
    /// defined, searching outward from the current scope. Falls back to
    /// defining it in the current scope if it isn't tracked yet.
    fn set_type(&mut self, name: &str, ty: Option<StaticType>) {
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
    fn lookup_type(&self, name: &str) -> Option<StaticType> {
        for scope in self.type_env.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return ty.clone();
            }
        }
        None
    }

    /// Infer the type of an expression based on its structure
    fn infer_expr_type(&self, expr: &Expr) -> Option<StaticType> {
        match expr {
            // Literal types
            Expr::Number { .. } => Some(StaticType::Number),
            Expr::String { .. } => Some(StaticType::String),
            Expr::StringInterpolation { .. } => Some(StaticType::String),
            Expr::Boolean { .. } => Some(StaticType::Boolean),
            Expr::ArrayLiteral { .. } => Some(StaticType::Array),
            Expr::MapLiteral { .. } => Some(StaticType::Map),
            Expr::SetLiteral { .. } => Some(StaticType::Set),
            Expr::Nil { .. } => Some(StaticType::Nil),

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
                    crate::common::method_registry::instance_return_type(object_type.name(), field)
                } else if let Expr::Variable { name, .. } = callee.as_ref() {
                    // A direct call to a known struct is a constructor;
                    // its result is statically an instance of that struct.
                    match self.symbol_table.resolve(name) {
                        Some(Symbol {
                            kind: SymbolKind::Struct { .. },
                            ..
                        }) => Some(StaticType::Struct(name.clone())),
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

                        if left_type == Some(StaticType::String)
                            || right_type == Some(StaticType::String)
                        {
                            Some(StaticType::String)
                        } else if left_type == Some(StaticType::Number)
                            && right_type == Some(StaticType::Number)
                        {
                            Some(StaticType::Number)
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
                        Some(StaticType::Number)
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
                        Some(StaticType::Boolean)
                    }
                    BinaryOp::BitwiseAnd
                    | BinaryOp::BitwiseOr
                    | BinaryOp::BitwiseXor
                    | BinaryOp::LeftShift
                    | BinaryOp::RightShift => {
                        // Bitwise operations return Number
                        Some(StaticType::Number)
                    }
                }
            }

            // Unary operations
            Expr::Unary { operator, .. } => {
                use crate::compiler::ast::UnaryOp;
                match operator {
                    UnaryOp::Negate => Some(StaticType::Number),
                    UnaryOp::Not => Some(StaticType::Boolean),
                    UnaryOp::BitwiseNot => Some(StaticType::Number),
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

    /// Resolves a symbol use into a `Res`, from the point of view of the
    /// current function, chaining an upvalue capture through enclosing
    /// functions as needed.
    fn compute_res(&mut self, use_: SymbolUse) -> Res {
        if let Some(index) = use_.builtin_index {
            return Res::Builtin(index);
        }
        if use_.decl_level == self.function_level() {
            return Res::Local(use_.decl_id);
        }
        if use_.decl_level == 0 && use_.decl_scope_depth == 0 {
            return Res::Global(use_.decl_id);
        }
        Res::Upvalue(self.resolve_upvalue_chain(use_.decl_id, use_.decl_level))
    }

    /// Chains an upvalue capture from `decl_level` up to the current
    /// function level, reusing an equal existing entry at each level
    /// instead of duplicating it. Returns the index in the current
    /// function's own upvalue array.
    fn resolve_upvalue_chain(&mut self, decl_id: DeclId, decl_level: u32) -> u32 {
        self.resolutions.mark_captured(decl_id);
        let mut index = 0u32;
        for level in (decl_level + 1)..=self.function_level() {
            let capture = if level == decl_level + 1 {
                Capture::Local(decl_id)
            } else {
                Capture::Upvalue(index)
            };
            index = self.add_upvalue(level, capture);
        }
        index
    }

    fn add_upvalue(&mut self, level: u32, capture: Capture) -> u32 {
        let upvalues = &mut self.function_frames[level as usize].upvalues;
        if let Some(pos) = upvalues.iter().position(|c| *c == capture) {
            return pos as u32;
        }
        upvalues.push(capture);
        (upvalues.len() - 1) as u32
    }

    /// Resolves a symbol use into a `Res` and records it under `id`.
    fn record_symbol_use(&mut self, id: NodeId, use_: SymbolUse) {
        let res = self.compute_res(use_);
        self.resolutions.record_use(id, res);
    }

    /// Helper method to check if a variable exists and is mutable, recording
    /// its resolution under `id` either way.
    fn check_variable_mutability(&mut self, id: NodeId, name: &str, location: SourceLocation) {
        let Some(symbol) = self.symbol_table.resolve(name) else {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::UndefinedSymbol,
                format!("Undefined variable '{}'", name),
                location,
            ));
            return;
        };
        let use_ = SymbolUse::from(symbol);
        self.check_top_level_forward_use(use_, name, location);

        if !use_.is_mutable {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::ImmutableAssignment,
                format!("Cannot modify immutable variable '{}'", name),
                location,
            ));
        }

        self.record_symbol_use(id, use_);
    }

    /// Compile error for a script-level (function level 0) direct use of a
    /// top-level val/var whose declaration statement hasn't resolved yet. A
    /// name shadowed by a block local never reaches here, since
    /// `decl_scope_depth` is then non-zero.
    fn check_top_level_forward_use(
        &mut self,
        use_: SymbolUse,
        name: &str,
        location: SourceLocation,
    ) {
        if self.function_level() != 0 || use_.decl_scope_depth != 0 {
            return;
        }
        let decl_id = use_.decl_id;
        if self.currently_initializing == Some(decl_id) {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::Other,
                format!("Cannot read '{}' in its own initializer", name),
                location,
            ));
        } else if self.not_initialized_top_level.contains(&decl_id) {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::Other,
                format!("Cannot use '{}' before its declaration", name),
                location,
            ));
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
                id,
                location,
            } => {
                self.resolve_variable_declaration(
                    *id,
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
                id,
                location,
            } => {
                self.resolve_variable_declaration(
                    *id,
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
                id,
                location,
            } => {
                self.resolve_function_declaration(*id, name, params, body, *location);
            }
            Stmt::Struct {
                name,
                fields,
                id,
                location,
            } => {
                // A top-level struct was already resolved in collect_declarations.
                if self.symbol_table.current_depth() != 0 {
                    self.errors.push(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::Other,
                        format!("Struct '{}' must be declared at the top level", name),
                        *location,
                    ));
                    self.declare_symbol(
                        *id,
                        name.clone(),
                        SymbolKind::Struct {
                            fields: fields.clone(),
                        },
                        false,
                        *location,
                    );
                }
            }
            Stmt::Impl {
                type_name,
                methods,
                location,
            } => {
                // Resolve method bodies here, at the impl's textual position,
                // so they see top-level val/var like other script-level code.
                if self.symbol_table.current_depth() != 0 {
                    self.errors.push(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::Other,
                        "'impl' blocks are only allowed at the top level".to_string(),
                        *location,
                    ));
                } else if self.is_struct_type(type_name)
                    || crate::common::method_registry::BUILTIN_TYPE_NAMES
                        .contains(&type_name.as_str())
                {
                    // An impl for an undefined type never registered its
                    // methods, so resolving bodies would only add follow-on
                    // errors on top of the undefined-type error.
                    for method in methods {
                        if let Stmt::Fn {
                            params,
                            body,
                            id,
                            location,
                            ..
                        } = method
                        {
                            self.resolve_function_body(
                                *id,
                                params,
                                body,
                                *location,
                                Some(type_name),
                            );
                        }
                    }
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
                id,
                location,
            } => {
                self.resolve_for_in_statement(*id, variable, collection, body, *location);
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
            Expr::Variable { name, id, location } => {
                self.resolve_variable(*id, name, *location);
            }
            Expr::Assign {
                name,
                value,
                id,
                location,
            } => {
                self.resolve_assignment(*id, name, value, *location);
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
                id,
                location,
            } => {
                self.resolve_call_expr(*id, callee, arguments, *location);
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
                id,
                location,
            } => {
                self.resolve_function_body(*id, params, body, *location, None);
            }
        }
    }

    // Statement resolution methods

    fn resolve_variable_declaration(
        &mut self,
        id: NodeId,
        name: &str,
        initializer: Option<&Expr>,
        kind: SymbolKind,
        is_mutable: bool,
        location: SourceLocation,
    ) {
        if self.symbol_table.current_depth() == 0 {
            // Top level: collect_declarations already declared this name and
            // recorded this node's DeclId - resolve the initializer,
            // tracking which decl is initializing so a self-read reports the
            // right message, then mark it initialized.
            let decl_id = self.resolutions.decl(id);
            let previous = self.currently_initializing.replace(decl_id);
            let inferred_type = initializer.map(|init| {
                self.resolve_expr(init);
                self.infer_expr_type(init)
            });
            self.currently_initializing = previous;
            self.define_type(name, inferred_type.flatten());
            self.not_initialized_top_level.remove(&decl_id);
            return;
        }

        // Resolve initializer first (if any), tracking its type (or
        // unknown) in the current scope - this shadows any outer type
        // recorded for the same name.
        let inferred_type = initializer.map(|init| {
            self.resolve_expr(init);
            self.infer_expr_type(init)
        });
        self.define_type(name, inferred_type.flatten());
        // Then define the variable in current scope
        self.declare_symbol(id, name.to_string(), kind, is_mutable, location);
    }

    fn resolve_function_declaration(
        &mut self,
        id: NodeId,
        name: &str,
        params: &[String],
        body: &[Stmt],
        location: SourceLocation,
    ) {
        // A nested function isn't hoisted by collect_declarations, so define it now, before resolving its body, so it can recurse.
        if self.symbol_table.current_depth() > 0 {
            let arity = params.len() as u8;
            self.declare_symbol(
                id,
                name.to_string(),
                SymbolKind::Function { arity },
                false,
                location,
            );
        }

        self.resolve_function_body(id, params, body, location, None);
    }

    /// Resolves a function's parameters and body in a fresh scope and
    /// records its `FunctionResolution` (params + captured upvalues) under
    /// `id`. Shared by named function declarations, lambda expressions, and
    /// impl methods. `self_type` names the struct a leading `self`
    /// parameter is typed as; it's `None` outside of an impl method.
    fn resolve_function_body(
        &mut self,
        id: NodeId,
        params: &[String],
        body: &[Stmt],
        location: SourceLocation,
        self_type: Option<&str>,
    ) {
        // Enter function scope
        self.enter_scope();
        self.function_frames.push(FunctionResolution::default());

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
                self_type.map(StaticType::from_name)
            } else {
                None
            };
            self.define_type(param, param_type);
            let decl_id =
                self.define_symbol(param.clone(), SymbolKind::Parameter, false, param_location);
            self.function_frames
                .last_mut()
                .expect("just pushed")
                .params
                .push(decl_id);
        }

        // Resolve function body
        for stmt in body {
            self.resolve_stmt(stmt);
        }

        self.loop_depth = saved_loop_depth;

        // Exit function scope
        self.exit_scope();
        let resolution = self.function_frames.pop().expect("just pushed");
        self.resolutions.record_function(id, resolution);
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
        id: NodeId,
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
        self.declare_symbol(id, variable.to_string(), SymbolKind::Value, false, location);

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

    fn resolve_variable(&mut self, id: NodeId, name: &str, location: SourceLocation) {
        let Some(symbol) = self.symbol_table.resolve(name) else {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::UndefinedSymbol,
                format!("Undefined variable '{}'", name),
                location,
            ));
            return;
        };

        if symbol.kind == SymbolKind::Namespace {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::Other,
                format!("'{}' is a namespace, not a value", name),
                location,
            ));
            return;
        }

        let use_ = SymbolUse::from(symbol);
        self.check_top_level_forward_use(use_, name, location);
        self.record_symbol_use(id, use_);
    }

    fn resolve_assignment(
        &mut self,
        id: NodeId,
        name: &str,
        value: &Expr,
        location: SourceLocation,
    ) {
        // Resolve the value being assigned
        self.resolve_expr(value);

        let Some(symbol) = self.symbol_table.resolve(name) else {
            self.errors.push(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::UndefinedSymbol,
                format!("Undefined variable '{}'", name),
                location,
            ));
            return;
        };

        let use_ = SymbolUse::from(symbol);
        self.check_top_level_forward_use(use_, name, location);

        if !use_.is_mutable {
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

        self.record_symbol_use(id, use_);
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

    fn resolve_call_expr(
        &mut self,
        id: NodeId,
        callee: &Expr,
        arguments: &[Expr],
        location: SourceLocation,
    ) {
        // Check if this is a method call: Call { callee: GetField { object, field }, arguments }
        if let Expr::GetField { object, field, .. } = callee {
            self.resolve_method_call(id, object, field, arguments, location);
        } else {
            self.resolve_function_call(id, callee, arguments, location);
        }
    }

    fn resolve_method_call(
        &mut self,
        id: NodeId,
        object: &Expr,
        method: &str,
        arguments: &[Expr],
        location: SourceLocation,
    ) {
        if let Expr::Variable { name, .. } = object {
            let is_namespace = matches!(
                self.symbol_table.resolve(name),
                Some(symbol) if symbol.kind == SymbolKind::Namespace
            );

            // A static method call, e.g. Math.abs(x). The namespace name
            // isn't a variable reference, so don't resolve it as one.
            if is_namespace && crate::common::method_registry::is_static_namespace(name) {
                for arg in arguments {
                    self.resolve_expr(arg);
                }
                if let Some(index) = self.validate_static_method(name, method, location) {
                    self.resolutions.record_native(id, index);
                }
                return;
            }

            // A static call on a struct's own name, e.g. Point.origin().
            // The receiver is the type itself, but codegen still loads it
            // as a value, so resolve it like any other variable use.
            if self.is_struct_type(name) {
                let struct_name = name.clone();
                self.resolve_expr(object);
                for arg in arguments {
                    self.resolve_expr(arg);
                }
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
                for arg in arguments {
                    self.resolve_expr(arg);
                }
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
        for arg in arguments {
            self.resolve_expr(arg);
        }

        // Instance method call - validate method if we can infer the object's type
        if let Some(object_type) = self.infer_expr_type(object) {
            self.validate_instance_method(object_type.name(), method, arguments.len(), location);
        }
    }

    fn resolve_function_call(
        &mut self,
        id: NodeId,
        callee: &Expr,
        arguments: &[Expr],
        location: SourceLocation,
    ) {
        if let Expr::Variable { name, .. } = callee {
            let (resolved, is_namespace) = match self.symbol_table.resolve(name) {
                Some(symbol) => (true, symbol.kind == SymbolKind::Namespace),
                None => (false, false),
            };

            // Constructor call on a namespace (e.g. File(path)), only when
            // nothing else shadows the namespace name.
            if is_namespace {
                if let Some(arity) = crate::common::method_registry::constructor_arity(name) {
                    self.validate_arity("Function", name, arity, arguments.len(), location);
                    if let Some(index) =
                        crate::common::method_registry::get_native_method_index(name, "new")
                    {
                        self.resolutions.record_native(id, index);
                    }
                    for arg in arguments {
                        self.resolve_expr(arg);
                    }
                    return;
                }
            } else if !resolved {
                // Nothing resolves this name - a native global function
                // (e.g. print) is the last resort.
                if let Some(index) =
                    crate::common::method_registry::get_native_method_index("", name)
                {
                    self.resolutions.record_native(id, index);
                    for arg in arguments {
                        self.resolve_expr(arg);
                    }
                    return;
                }
            }
        }

        // Regular call - resolve callee as normal
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
        if let Some(object_type) = self.infer_expr_type(object) {
            self.validate_struct_field(object_type.name(), field, location);
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
        if let Some(object_type) = self.infer_expr_type(object) {
            self.validate_struct_field(object_type.name(), field, location);
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
            Expr::Variable { name, id, .. } => {
                // Check if variable exists and is mutable
                self.check_variable_mutability(*id, name, location);
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
            Expr::Variable { name, id, .. } => {
                // Check if variable exists and is mutable
                self.check_variable_mutability(*id, name, location);
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

    /// Validate a static method call against the method registry, returning
    /// the registry index when it names an actual static method. A registry
    /// entry that exists but isn't a static method (e.g. a constructor)
    /// reports the same "not found" error, so a caller can record the
    /// native call directly from the returned index without a second,
    /// possibly-disagreeing lookup.
    fn validate_static_method(
        &mut self,
        namespace: &str,
        method: &str,
        location: SourceLocation,
    ) -> Option<usize> {
        let index = crate::common::method_registry::get_native_method_index(namespace, method)
            .filter(|_| crate::common::method_registry::is_static_method(namespace, method));
        if index.is_none() {
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
        index
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
                SymbolKind::Value
                | SymbolKind::Variable
                | SymbolKind::Parameter
                | SymbolKind::Builtin { .. } => {
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

/// The `stdlib::BUILTIN_VALUES` index of a symbol, if it's a runtime builtin.
fn builtin_index(symbol: &Symbol) -> Option<u32> {
    match symbol.kind {
        SymbolKind::Builtin { index } => Some(index),
        _ => None,
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
