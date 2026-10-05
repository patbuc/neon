use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};

use crate::common::static_type::StaticType;
use crate::common::SourceLocation;
/// Semantic analyzer for the multi-pass compiler
/// Performs semantic analysis on the AST, building symbol tables and validating program semantics,
/// and resolves every name use to where it lives at runtime.
use crate::compiler::ast::{
    Binding, EnumVariant, Expr, IfExprElse, InterpolationPart, MatchArm, MatchArmBody,
    MatchPattern, NodeId, Pattern, Stmt, StructField, UnaryOp,
};
use crate::compiler::global_env::GlobalEnv;
use crate::compiler::resolutions::{
    Capture, DeclId, EnumValuesAccess, EnumVariantAccess, FunctionResolution, Res, Resolutions,
};
use crate::compiler::symbol_table::{Symbol, SymbolKind, SymbolTable};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// Comparable identity of a match pattern, for finding duplicates. A float
/// that is a whole number in `i64` range is stored as `Int`, so `1` and
/// `1.0` compare equal like `==` does while large ints stay distinct.
#[derive(Clone, PartialEq)]
enum MatchPatternKey {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Nil,
    Range(i64, i64, bool),
    Enum(Rc<str>, Rc<str>),
}

/// The value of a number-literal match pattern.
enum PatternNumber {
    Int(i64),
    Float(f64),
}

/// A method's call signature: the rule that decides whether it's callable
/// as `receiver.method(...)` or `Type.method(...)` is a single fact - does
/// its first parameter literally read `self`.
#[derive(Clone, Copy)]
pub(crate) struct MethodSignature {
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
    // DeclIds of the top-level val/var name(s) currently resolving their own
    // initializer, if any - a direct read of one of them there is "in its
    // own initializer", not "before its declaration". A tuple pattern can
    // declare more than one name from a single initializer.
    currently_initializing: HashSet<DeclId>,
    // DeclIds of block/function-body-local `fn` declarations whose own
    // `Stmt::Fn` hasn't been resolved yet. A read that resolves to one of
    // these can run before that `fn`'s slot is bound, so it needs a
    // runtime initialization check.
    pending_block_fns: HashSet<DeclId>,
    // Whether `TooManySymbols` has already been reported, so one program
    // with many overflowing names doesn't produce one error per name.
    too_many_symbols_reported: bool,
    // DeclIds below this were seeded by `new()` itself (namespaces, builtin
    // values) rather than declared by the program; `snapshot_env` excludes
    // them so a later REPL line can't redeclare a builtin, since a fresh
    // analyzer re-seeds them anyway.
    builtin_decl_count: u32,
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
            currently_initializing: HashSet::new(),
            pending_block_fns: HashSet::new(),
            too_many_symbols_reported: false,
            builtin_decl_count: next_decl_id,
        }
    }

    /// Seeds global scope with an earlier REPL line's declarations, so this
    /// line continues from where that line left off.
    pub(crate) fn seed(&mut self, env: &GlobalEnv) {
        self.symbol_table
            .seed_globals(env.globals.values().cloned());
        self.type_env[0].extend(env.types.clone());
        self.struct_methods = env.struct_methods.clone();
        self.resolutions
            .seed(env.symbols.clone(), env.immutable.clone());
        self.next_decl_id = self.next_decl_id.max(env.next_decl_id);
    }

    /// Builds the `GlobalEnv` a later REPL line compiles against: this
    /// line's final global scope, with `decl_slots` narrowed to the
    /// declarations that actually got a global. `previous_slot_count`
    /// keeps `slot_count` from shrinking below the seed env's.
    #[allow(clippy::expect_used)]
    pub(crate) fn snapshot_env(
        self,
        resolutions: Resolutions,
        decl_slots: HashMap<DeclId, u32>,
        previous_slot_count: u32,
    ) -> GlobalEnv {
        let builtin_decl_count = self.builtin_decl_count;
        let globals: HashMap<String, Symbol> = self
            .symbol_table
            .into_global_symbols()
            .into_iter()
            .filter(|(_, symbol)| symbol.decl_id.0 >= builtin_decl_count)
            .collect();
        let global_ids: HashSet<DeclId> = globals.values().map(|symbol| symbol.decl_id).collect();
        let decl_slots: HashMap<DeclId, u32> = decl_slots
            .into_iter()
            .filter(|(decl, _)| global_ids.contains(decl))
            .collect();
        let slot_count = decl_slots
            .values()
            .map(|&slot| slot + 1)
            .max()
            .unwrap_or(0)
            .max(previous_slot_count);
        let (symbols, immutable) = resolutions.into_symbols_and_immutable();
        let immutable = immutable
            .into_iter()
            .filter(|decl| global_ids.contains(decl))
            .collect();

        GlobalEnv {
            globals,
            types: self
                .type_env
                .into_iter()
                .next()
                .expect("type_env always starts with the global scope"),
            struct_methods: self.struct_methods,
            symbols,
            next_decl_id: self.next_decl_id,
            decl_slots,
            slot_count,
            immutable,
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
                    self.check_duplicate_fields(name, fields);
                    self.intern_name(name, *location);
                    for field in fields {
                        self.intern_name(&field.name, field.location);
                    }
                    if crate::common::method_registry::BUILTIN_TYPE_NAMES.contains(&name.as_str()) {
                        self.push_error(CompilationError::new(
                            CompilationPhase::Semantic,
                            CompilationErrorKind::ReservedStructName,
                            format!("Struct name '{}' is reserved for a builtin type", name),
                            *location,
                        ));
                        continue;
                    }
                    self.declare_symbol(
                        *id,
                        name.clone(),
                        SymbolKind::Struct {
                            fields: fields.iter().map(|f| f.name.clone()).collect(),
                        },
                        false,
                        *location,
                    );
                }
                Stmt::Enum {
                    name,
                    variants,
                    id,
                    location,
                } => {
                    self.check_enum_variants(name, variants);
                    self.intern_name(name, *location);
                    self.declare_symbol(
                        *id,
                        name.clone(),
                        SymbolKind::Enum {
                            variants: variants.clone(),
                        },
                        false,
                        *location,
                    );
                }
                Stmt::Val { pattern, .. } | Stmt::Var { pattern, .. } => {
                    let is_mutable = matches!(stmt, Stmt::Var { .. });
                    let kind = if is_mutable {
                        SymbolKind::Variable
                    } else {
                        SymbolKind::Value
                    };
                    for binding in pattern.bindings() {
                        let decl_id = self.declare_symbol(
                            binding.id,
                            binding.name.clone(),
                            kind.clone(),
                            is_mutable,
                            binding.location,
                        );
                        self.not_initialized_top_level.insert(decl_id);
                    }
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

    /// Interns a field, method, or type name, flagging the symbol table
    /// overflowing 65,536 distinct names as a compile error.
    fn intern_name(&mut self, name: &str, location: SourceLocation) {
        if self.resolutions.intern_symbol(name).is_none() && !self.too_many_symbols_reported {
            self.too_many_symbols_reported = true;
            self.push_error(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::TooManySymbols,
                "Too many distinct field, method and type names (limit 65536)".to_string(),
                location,
            ));
        }
    }

    fn check_duplicate_fields(&mut self, struct_name: &str, fields: &[StructField]) {
        let mut seen = HashSet::new();
        for field in fields {
            if !seen.insert(field.name.as_str()) {
                self.push_error(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::DuplicateField,
                    format!(
                        "Struct '{}' declares field '{}' more than once",
                        struct_name, field.name
                    ),
                    field.location,
                ));
            }
        }
    }

    fn check_enum_variants(&mut self, enum_name: &str, variants: &[EnumVariant]) {
        let mut seen = HashSet::new();
        for variant in variants {
            let mut seen_fields = HashSet::new();
            for field in &variant.fields {
                self.intern_name(field, variant.location);
                if !seen_fields.insert(field.as_str()) {
                    self.push_error(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::DuplicateField,
                        format!(
                            "Duplicate field '{}' in variant '{}.{}'",
                            field, enum_name, variant.name
                        ),
                        variant.location,
                    ));
                }
            }
            if !seen.insert(variant.name.as_str()) {
                self.push_error(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::DuplicateEnumVariant,
                    format!(
                        "Enum '{}' declares variant '{}' more than once",
                        enum_name, variant.name
                    ),
                    variant.location,
                ));
            }
        }
    }

    /// Register the methods an `impl` block contributes to a struct or a
    /// builtin type, flagging an impl for an undefined type, a method
    /// already defined for this type, one shadowing a field name, or one
    /// shadowing a native method (builtin types only).
    fn collect_impl_block(&mut self, type_name: &str, methods: &[Stmt], location: SourceLocation) {
        self.intern_name(type_name, location);
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
                Some(Symbol {
                    kind: SymbolKind::Enum { .. },
                    ..
                }) => {
                    self.push_error(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::ImplOnEnum,
                        format!("Cannot implement methods on enum '{}'", type_name),
                        location,
                    ));
                    return;
                }
                _ => {
                    self.push_error(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::UndefinedType,
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
            self.intern_name(name, *method_location);

            if crate::common::method_registry::is_valid_method(type_name, name) {
                self.push_error(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::NativeMethodConflict,
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
                self.push_error(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::StaticMethodOnBuiltinType,
                    format!(
                        "Method '{}' on {} must take self; static methods are only supported on structs",
                        name, type_name
                    ),
                    *method_location,
                ));
                continue;
            }

            if field_names.iter().any(|f| f == name) {
                self.push_error(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::MethodFieldConflict,
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
                self.push_error(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::DuplicateMethod,
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
            self.push_error(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::DuplicateSymbol,
                err,
                location,
            ));
        }
        if !is_mutable {
            self.resolutions.mark_immutable(decl_id);
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
    #[allow(clippy::expect_used)]
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
            Expr::Int { .. } => Some(StaticType::Number),
            Expr::String { .. } => Some(StaticType::String),
            Expr::StringInterpolation { .. } => Some(StaticType::String),
            Expr::Boolean { .. } => Some(StaticType::Boolean),
            Expr::ArrayLiteral { .. } => Some(StaticType::Array),
            Expr::MapLiteral { .. } => Some(StaticType::Map),
            Expr::SetLiteral { .. } => Some(StaticType::Set),
            Expr::Nil { .. } => Some(StaticType::Nil),
            Expr::Range { .. } => Some(StaticType::Range),

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
                        // addition. Only op_add's own outcomes are valid at
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
                    BinaryOp::NilCoalesce => None,
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

    fn push_error(&mut self, error: CompilationError) {
        if !self.errors.contains(&error) {
            self.errors.push(error);
        }
    }

    /// Resolves a symbol use into a `Res` and records it under `id`.
    fn record_symbol_use(&mut self, id: NodeId, use_: SymbolUse) {
        let res = self.compute_res(use_);
        self.resolutions.record_use(id, res);
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
        if self.currently_initializing.contains(&decl_id) {
            self.push_error(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::ReadInOwnInitializer,
                format!("Cannot read '{}' in its own initializer", name),
                location,
            ));
        } else if self.not_initialized_top_level.contains(&decl_id) {
            self.push_error(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::UseBeforeDeclaration,
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
                pattern,
                initializer,
                ..
            } => match (pattern, initializer.as_ref()) {
                (Pattern::Tuple(slots), Some(init)) => {
                    self.resolve_tuple_declaration(slots, init, false);
                }
                (Pattern::Name(binding), _) => {
                    self.resolve_variable_declaration(
                        binding.id,
                        &binding.name,
                        initializer.as_ref(),
                        SymbolKind::Value,
                        false,
                        binding.location,
                    );
                }
                (Pattern::Tuple(_), None) => {
                    unreachable!("tuple pattern always has an initializer")
                }
            },
            Stmt::Var {
                pattern,
                initializer,
                ..
            } => match (pattern, initializer.as_ref()) {
                (Pattern::Tuple(slots), Some(init)) => {
                    self.resolve_tuple_declaration(slots, init, true);
                }
                (Pattern::Name(binding), _) => {
                    self.resolve_variable_declaration(
                        binding.id,
                        &binding.name,
                        initializer.as_ref(),
                        SymbolKind::Variable,
                        true,
                        binding.location,
                    );
                }
                (Pattern::Tuple(_), None) => {
                    unreachable!("tuple pattern always has an initializer")
                }
            },
            Stmt::Fn {
                params,
                body,
                id,
                location,
                ..
            } => {
                self.resolve_function_declaration(*id, params, body, *location);
            }
            Stmt::Struct {
                name,
                fields,
                id,
                location,
            } => {
                // A top-level struct was already resolved in collect_declarations.
                if self.symbol_table.current_depth() != 0 {
                    self.push_error(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::StructNotTopLevel,
                        format!("Struct '{}' must be declared at the top level", name),
                        *location,
                    ));
                    self.check_duplicate_fields(name, fields);
                    self.declare_symbol(
                        *id,
                        name.clone(),
                        SymbolKind::Struct {
                            fields: fields.iter().map(|f| f.name.clone()).collect(),
                        },
                        false,
                        *location,
                    );
                }
            }
            Stmt::Enum {
                name,
                variants,
                id,
                location,
            } => {
                // A top-level enum was already resolved in collect_declarations.
                if self.symbol_table.current_depth() != 0 {
                    self.push_error(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::EnumNotTopLevel,
                        format!("Enum '{}' must be declared at the top level", name),
                        *location,
                    ));
                    self.check_enum_variants(name, variants);
                    self.declare_symbol(
                        *id,
                        name.clone(),
                        SymbolKind::Enum {
                            variants: variants.clone(),
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
                    self.push_error(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::ImplNotTopLevel,
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
                                false,
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
                if let Some(value) = value {
                    self.resolve_expr(value);
                }
            }
            Stmt::Break { location } => {
                self.validate_loop_control_statement("break", *location);
            }
            Stmt::Continue { location } => {
                self.validate_loop_control_statement("continue", *location);
            }
            Stmt::ForIn {
                pattern,
                collection,
                body,
                ..
            } => {
                self.resolve_for_in_statement(pattern, collection, body);
            }
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number { .. }
            | Expr::Int { .. }
            | Expr::String { .. }
            | Expr::Boolean { .. }
            | Expr::Nil { .. } => {
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
            Expr::CompoundAssign {
                name,
                value,
                read_id,
                write_id,
                location,
                ..
            } => {
                self.resolve_variable(*read_id, name, *location);
                self.resolve_expr(value);
                self.resolve_write(*write_id, name, *location);
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
                optional,
                location,
            } => {
                self.resolve_get_field(object, field, *optional, *location);
            }
            Expr::SetField {
                object,
                field,
                value,
                location,
            }
            | Expr::CompoundAssignField {
                object,
                field,
                value,
                location,
                ..
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
            }
            | Expr::CompoundAssignIndex {
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
                implicit_it,
            } => {
                self.resolve_function_body(*id, params, body, *location, None, *implicit_it);
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.resolve_expr(condition);
                self.resolve_stmt(then_branch);
                match else_branch.as_ref() {
                    IfExprElse::If(expr) => self.resolve_expr(expr),
                    IfExprElse::Block(stmt) => self.resolve_stmt(stmt),
                }
            }
            Expr::Match {
                scrutinee,
                arms,
                location,
            } => {
                self.resolve_expr(scrutinee);
                for arm in arms {
                    self.enter_scope();
                    self.declare_match_arm_bindings(arm);
                    for pattern in &arm.patterns {
                        self.resolve_match_pattern(pattern);
                    }
                    if let Some(guard) = &arm.guard {
                        self.resolve_expr(guard);
                    }
                    match &arm.body {
                        MatchArmBody::Expr(expr) => self.resolve_expr(expr),
                        MatchArmBody::Block(stmt) => self.resolve_stmt(stmt),
                    }
                    self.exit_scope();
                }
                self.check_match_exhaustiveness(arms, *location);
                self.check_match_unreachable_patterns(arms);
            }
        }
    }

    fn resolve_match_pattern(&mut self, pattern: &MatchPattern) {
        match pattern {
            MatchPattern::Expr(expr) => {
                self.resolve_expr(expr);
                if !self.is_valid_match_pattern(expr) {
                    self.push_error(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::InvalidMatchPattern,
                        "Invalid match pattern: expected a literal, an integer range, an enum variant, a name, an array pattern, or '_'".to_string(),
                        Self::match_pattern_location(expr),
                    ));
                } else if let Some(access) = self.match_pattern_enum_variant(expr) {
                    if !access.fields.is_empty() {
                        self.push_error(CompilationError::new(
                            CompilationPhase::Semantic,
                            CompilationErrorKind::InvalidMatchPattern,
                            format!(
                                "Invalid match pattern: payload variant {}.{} must be matched with its fields",
                                access.enum_name, access.variant_name
                            ),
                            Self::match_pattern_location(expr),
                        ));
                    }
                }
            }
            MatchPattern::Array { elements, .. } => {
                let mut rests = elements
                    .iter()
                    .filter(|element| matches!(element, MatchPattern::Rest { .. }));
                if let Some(second) = rests.nth(1) {
                    self.push_error(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::InvalidMatchPattern,
                        "Invalid match pattern: an array pattern can have only one rest ('..')"
                            .to_string(),
                        Self::pattern_location(second),
                    ));
                }
                for element in elements {
                    self.resolve_match_pattern(element);
                }
            }
            MatchPattern::Variant { target, fields } => {
                self.resolve_expr(target);
                self.resolve_variant_pattern(target, fields);
            }
            MatchPattern::Rest { binding, location } => {
                if binding.is_some() {
                    self.intern_name("slice", *location);
                    self.intern_name("drop", *location);
                }
            }
            MatchPattern::Wildcard(_) | MatchPattern::Binding(_) => {}
        }
    }

    fn resolve_variant_pattern(&mut self, target: &Expr, fields: &[MatchPattern]) {
        let location = Self::match_pattern_location(target);
        let Some(access) = self.match_pattern_enum_variant(target) else {
            if let Expr::GetField { object, .. } = target {
                if matches!(object.as_ref(), Expr::Variable { name, .. } if self.enum_variants(name).is_some())
                {
                    return;
                }
            }
            self.push_error(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::InvalidMatchPattern,
                "Invalid match pattern: only an enum variant can take a field list".to_string(),
                location,
            ));
            return;
        };
        let message = if access.fields.is_empty() {
            Some(format!(
                "Invalid match pattern: unit variant {}.{} takes no parentheses",
                access.enum_name, access.variant_name
            ))
        } else if access.fields.len() != fields.len() {
            Some(format!(
                "Invalid match pattern: {}.{} has {} fields but the pattern has {}",
                access.enum_name,
                access.variant_name,
                access.fields.len(),
                fields.len()
            ))
        } else {
            None
        };
        if let Some(message) = message {
            self.push_error(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::InvalidMatchPattern,
                message,
                location,
            ));
        }
        for field in fields {
            if let MatchPattern::Rest { location, .. } = field {
                self.push_error(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::InvalidMatchPattern,
                    "Invalid match pattern: a variant pattern cannot have a rest ('..')"
                        .to_string(),
                    *location,
                ));
            }
            self.resolve_match_pattern(field);
        }
    }

    fn pattern_location(pattern: &MatchPattern) -> SourceLocation {
        match pattern {
            MatchPattern::Wildcard(location)
            | MatchPattern::Array { location, .. }
            | MatchPattern::Rest { location, .. } => *location,
            MatchPattern::Binding(binding) => binding.location,
            MatchPattern::Expr(expr) | MatchPattern::Variant { target: expr, .. } => {
                Self::match_pattern_location(expr)
            }
        }
    }

    /// Declares the names an arm's patterns bind, as immutable values in
    /// the arm's scope. Alternatives must all bind the same names; the
    /// first alternative declares them.
    fn declare_match_arm_bindings(&mut self, arm: &MatchArm) {
        let Some(first_pattern) = arm.patterns.first() else {
            return;
        };
        let first = first_pattern.bindings();
        let mut first_names: Vec<&str> = first.iter().map(|(b, _)| b.name.as_str()).collect();
        first_names.sort_unstable();
        first_names.dedup();
        for pattern in &arm.patterns[1..] {
            let bindings = pattern.bindings();
            let mut names: Vec<&str> = Vec::new();
            for (binding, _) in &bindings {
                if names.contains(&binding.name.as_str()) {
                    self.push_error(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::DuplicateSymbol,
                        format!("Symbol '{}' already defined in this scope", binding.name),
                        binding.location,
                    ));
                }
                names.push(binding.name.as_str());
            }
            names.sort_unstable();
            names.dedup();
            if names != first_names {
                self.push_error(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::InvalidMatchPattern,
                    "Invalid match pattern: alternatives must bind the same names".to_string(),
                    Self::pattern_location(pattern),
                ));
            }
        }
        for (binding, _) in first {
            self.define_type(&binding.name, None);
            self.declare_symbol(
                binding.id,
                binding.name.clone(),
                SymbolKind::Value,
                false,
                binding.location,
            );
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
            let previous =
                std::mem::replace(&mut self.currently_initializing, HashSet::from([decl_id]));
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

    /// Resolves `val (a, _, c) = expr` / `var (...)`: the initializer, then
    /// each named slot (an element's type isn't tracked, unlike a plain
    /// `val`/`var`, since it's not the initializer's own type).
    fn resolve_tuple_declaration(
        &mut self,
        slots: &[Option<Binding>],
        initializer: &Expr,
        is_mutable: bool,
    ) {
        let bindings: Vec<&Binding> = slots.iter().filter_map(Option::as_ref).collect();

        if self.symbol_table.current_depth() == 0 {
            // Top level: collect_declarations already declared these names.
            let decl_ids: HashSet<DeclId> = bindings
                .iter()
                .map(|binding| self.resolutions.decl(binding.id))
                .collect();
            let previous = std::mem::replace(&mut self.currently_initializing, decl_ids.clone());
            self.resolve_expr(initializer);
            self.currently_initializing = previous;
            for binding in &bindings {
                self.define_type(&binding.name, None);
            }
            for decl_id in decl_ids {
                self.not_initialized_top_level.remove(&decl_id);
            }
            return;
        }

        self.resolve_expr(initializer);

        for binding in &bindings {
            self.define_type(&binding.name, None);
            let kind = if is_mutable {
                SymbolKind::Variable
            } else {
                SymbolKind::Value
            };
            self.declare_symbol(
                binding.id,
                binding.name.clone(),
                kind,
                is_mutable,
                binding.location,
            );
        }
    }

    fn resolve_function_declaration(
        &mut self,
        id: NodeId,
        params: &[String],
        body: &[Stmt],
        location: SourceLocation,
    ) {
        // Its own line has now been reached, so it's no longer pending.
        if self.symbol_table.current_depth() > 0 {
            let decl_id = self.resolutions.decl(id);
            self.pending_block_fns.remove(&decl_id);
        }

        self.resolve_function_body(id, params, body, location, None, false);
    }

    /// Pre-declares a statement list's own `fn` names before resolving any
    /// of its statements, so siblings can call each other regardless of order.
    fn predeclare_block_functions(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            if let Stmt::Fn {
                name,
                params,
                id,
                location,
                ..
            } = stmt
            {
                let arity = params.len() as u8;
                let decl_id = self.declare_symbol(
                    *id,
                    name.clone(),
                    SymbolKind::Function { arity },
                    false,
                    *location,
                );
                self.pending_block_fns.insert(decl_id);
            }
        }
    }

    /// Resolves a function's parameters and body in a fresh scope and
    /// records its `FunctionResolution` (params + captured upvalues) under
    /// `id`. Shared by named function declarations, lambda expressions, and
    /// impl methods. `self_type` names the struct a leading `self`
    /// parameter is typed as; it's `None` outside of an impl method.
    #[allow(clippy::expect_used)]
    fn resolve_function_body(
        &mut self,
        id: NodeId,
        params: &[String],
        body: &[Stmt],
        location: SourceLocation,
        self_type: Option<&str>,
        implicit_it: bool,
    ) {
        // Enter function scope
        self.enter_scope();
        self.function_frames.push(FunctionResolution::default());

        // A loop enclosing this declaration must not let break/continue
        // inside the function body see themselves as inside that loop.
        let saved_loop_depth = self.loop_depth;
        self.loop_depth = 0;

        // A trailing block without a `->` header gets a single `it`
        // parameter when its own body - not a nested block's - mentions it.
        if implicit_it && block_references_it(body) {
            let decl_id =
                self.define_symbol("it".to_string(), SymbolKind::Parameter, false, location);
            self.function_frames
                .last_mut()
                .expect("just pushed")
                .params
                .push(decl_id);
        }

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
        self.predeclare_block_functions(body);
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
        self.predeclare_block_functions(statements);
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

    fn resolve_for_in_statement(&mut self, pattern: &Pattern, collection: &Expr, body: &Stmt) {
        // Resolve the collection expression
        self.resolve_expr(collection);

        // Enter a new scope for the loop
        self.enter_scope();

        // Define the loop variable(s) as immutable (always val). Their type
        // is unknown and explicitly shadows any outer type of the same name.
        for binding in pattern.bindings() {
            self.define_type(&binding.name, None);
            self.declare_symbol(
                binding.id,
                binding.name.clone(),
                SymbolKind::Value,
                false,
                binding.location,
            );
        }

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
            self.push_error(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::LoopControlOutsideLoop,
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
            self.push_error(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::UndefinedVariable,
                format!("Undefined variable '{}'", name),
                location,
            ));
            return;
        };

        if symbol.kind == SymbolKind::Namespace {
            self.push_error(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::NamespaceAsValue,
                format!("'{}' is a namespace, not a value", name),
                location,
            ));
            return;
        }

        if matches!(symbol.kind, SymbolKind::Enum { .. }) {
            self.push_error(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::EnumAsValue,
                format!("'{}' is an enum, not a value", name),
                location,
            ));
            return;
        }

        let use_ = SymbolUse::from(symbol);
        self.check_top_level_forward_use(use_, name, location);
        if self.pending_block_fns.contains(&use_.decl_id) {
            self.resolutions.mark_checked(id);
        }
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
        self.resolve_write(id, name, location);
    }

    fn resolve_write(&mut self, id: NodeId, name: &str, location: SourceLocation) {
        let Some(symbol) = self.symbol_table.resolve(name) else {
            self.push_error(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::UndefinedVariable,
                format!("Undefined variable '{}'", name),
                location,
            ));
            return;
        };

        let use_ = SymbolUse::from(symbol);
        self.check_top_level_forward_use(use_, name, location);

        if !use_.is_mutable {
            self.push_error(CompilationError::new(
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
        if let Expr::GetField {
            object,
            field,
            optional,
            ..
        } = callee
        {
            self.resolve_method_call(id, object, field, *optional, arguments, location);
        } else {
            self.resolve_function_call(id, callee, arguments, location);
        }
    }

    fn resolve_method_call(
        &mut self,
        id: NodeId,
        object: &Expr,
        method: &str,
        optional: bool,
        arguments: &[Expr],
        location: SourceLocation,
    ) {
        self.intern_name(method, location);
        if let Expr::Variable { name, .. } = object {
            if optional && self.is_type_or_namespace_name(name) {
                for arg in arguments {
                    self.resolve_expr(arg);
                }
                self.push_optional_dot_on_type_error(location);
                return;
            }

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

            // A static call on an enum's own name, e.g. Color.values().
            if let Some(variants) = self.enum_variants(name) {
                for arg in arguments {
                    self.resolve_expr(arg);
                }
                if method == "values" {
                    if variants.iter().any(|v| !v.fields.is_empty()) {
                        self.push_error(CompilationError::new(
                            CompilationPhase::Semantic,
                            CompilationErrorKind::UnknownMethod,
                            format!("'values()' is not available on enum '{}'", name),
                            location,
                        ));
                        return;
                    }
                    self.validate_arity("Method", "values", 0, arguments.len(), location);
                    self.resolutions.record_enum_values_access(
                        id,
                        EnumValuesAccess {
                            enum_name: Rc::from(name.as_str()),
                            variants: variants.iter().map(|v| Rc::from(v.name.as_str())).collect(),
                        },
                    );
                } else if let Some(ordinal) = variants.iter().position(|v| v.name == method) {
                    let variant = &variants[ordinal];
                    if variant.fields.is_empty() {
                        self.push_error(CompilationError::new(
                            CompilationPhase::Semantic,
                            CompilationErrorKind::NotCallable,
                            format!("'{}' is not a payload variant", method),
                            location,
                        ));
                        return;
                    }
                    self.validate_arity(
                        "Function",
                        method,
                        variant.fields.len() as u8,
                        arguments.len(),
                        location,
                    );
                    self.resolutions
                        .record_enum_construct(id, EnumVariantAccess::new(name, variant, ordinal));
                } else {
                    let mut candidates: Vec<&str> = variants
                        .iter()
                        .filter(|v| !v.fields.is_empty())
                        .map(|v| v.name.as_str())
                        .collect();
                    if candidates.is_empty() {
                        candidates.push("values");
                    }
                    let error_message = unknown_method_error(name, method, &candidates, None);
                    self.push_error(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::UnknownMethod,
                        error_message,
                        location,
                    ));
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
                self.push_error(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::StaticCallOnBuiltinType,
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

        // Instance method call - validate method if we can infer the object's type.
        if let Some(object_type) = self.infer_expr_type(object) {
            if !(optional && object_type == StaticType::Nil) {
                self.validate_instance_method(
                    object_type.name(),
                    method,
                    arguments.len(),
                    location,
                );
            }
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

    fn resolve_get_field(
        &mut self,
        object: &Expr,
        field: &str,
        optional: bool,
        location: SourceLocation,
    ) {
        if let Expr::Variable { name, id, .. } = object {
            if optional && self.is_type_or_namespace_name(name) {
                self.push_optional_dot_on_type_error(location);
                return;
            }

            if let Some(variants) = self.enum_variants(name) {
                self.resolve_enum_variant_access(*id, name, &variants, field, location);
                return;
            }
        }

        self.intern_name(field, location);
        self.resolve_expr(object);
        if let Some(object_type) = self.infer_expr_type(object) {
            self.validate_struct_field(object_type.name(), field, location);
        }
    }

    /// Resolves `Color.Red`: `id` is the `Expr::Variable` node naming the
    /// enum.
    fn resolve_enum_variant_access(
        &mut self,
        id: NodeId,
        enum_name: &str,
        variants: &[EnumVariant],
        variant: &str,
        location: SourceLocation,
    ) {
        let Some(ordinal) = variants.iter().position(|v| v.name == variant) else {
            self.push_error(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::UnknownEnumVariant,
                format!("Enum '{}' has no variant named '{}'", enum_name, variant),
                location,
            ));
            return;
        };
        self.resolutions.record_enum_variant_access(
            id,
            EnumVariantAccess::new(enum_name, &variants[ordinal], ordinal),
        );
    }

    fn resolve_set_field(
        &mut self,
        object: &Expr,
        field: &str,
        value: &Expr,
        location: SourceLocation,
    ) {
        if let Expr::Variable { name, .. } = object {
            if self.enum_variants(name).is_some() {
                self.resolve_expr(value);
                self.push_error(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::ImmutableAssignment,
                    format!("Cannot assign to enum variant '{}.{}'", name, field),
                    location,
                ));
                return;
            }
        }

        self.intern_name(field, location);
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
            self.push_error(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::UnknownNamespaceMethod,
                format!(
                    "Static method '{}' not found in namespace '{}'",
                    method, namespace
                ),
                location,
            ));
        }
        index
    }

    /// True when `name` names a namespace (Math, File, ...), an enum, or a
    /// struct type itself - never nil, so `?.` on it is nonsensical.
    fn is_type_or_namespace_name(&self, name: &str) -> bool {
        matches!(
            self.symbol_table.resolve(name),
            Some(symbol) if symbol.kind == SymbolKind::Namespace
        ) || self.enum_variants(name).is_some()
            || self.is_struct_type(name)
    }

    fn push_optional_dot_on_type_error(&mut self, location: SourceLocation) {
        self.push_error(CompilationError::new(
            CompilationPhase::Semantic,
            CompilationErrorKind::OptionalDotOnType,
            "Cannot use '?.' on a type or namespace".to_string(),
            location,
        ));
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

    /// The variants of the enum named `name`, in declaration order, when
    /// `name` resolves (lexically - a local of the same name shadows it) to
    /// an enum.
    fn enum_variants(&self, name: &str) -> Option<Vec<EnumVariant>> {
        match self.symbol_table.resolve(name) {
            Some(Symbol {
                kind: SymbolKind::Enum { variants },
                ..
            }) => Some(variants.clone()),
            _ => None,
        }
    }

    /// If `pattern` is `Enum.Variant`, already resolved by `resolve_expr` as
    /// an enum-variant access, its recorded access.
    fn match_pattern_enum_variant(&self, expr: &Expr) -> Option<EnumVariantAccess> {
        let Expr::GetField { object, .. } = expr else {
            return None;
        };
        let Expr::Variable { id, .. } = object.as_ref() else {
            return None;
        };
        self.resolutions.enum_variant_access(*id).cloned()
    }

    /// The variant a pattern names, as `Enum.Variant` or `Enum.Variant(..)`,
    /// and whether it matches every value of that variant.
    fn match_pattern_variant(&self, pattern: &MatchPattern) -> Option<(EnumVariantAccess, bool)> {
        match pattern {
            MatchPattern::Expr(expr) => Some((self.match_pattern_enum_variant(expr)?, true)),
            MatchPattern::Variant { target, fields } => Some((
                self.match_pattern_enum_variant(target)?,
                fields.iter().all(MatchPattern::is_irrefutable),
            )),
            _ => None,
        }
    }

    /// The location of the first token of a match pattern expression.
    fn match_pattern_location(expr: &Expr) -> SourceLocation {
        match expr {
            Expr::Binary { left, .. } => Self::match_pattern_location(left),
            Expr::Call { callee, .. } => Self::match_pattern_location(callee),
            Expr::GetField { object, .. }
            | Expr::SetField { object, .. }
            | Expr::CompoundAssignField { object, .. }
            | Expr::Index { object, .. }
            | Expr::IndexAssign { object, .. }
            | Expr::CompoundAssignIndex { object, .. } => Self::match_pattern_location(object),
            Expr::Range { start, .. } => Self::match_pattern_location(start),
            Expr::Conditional { condition, .. } => Self::match_pattern_location(condition),
            Expr::Number { location, .. }
            | Expr::Int { location, .. }
            | Expr::String { location, .. }
            | Expr::StringInterpolation { location, .. }
            | Expr::Boolean { location, .. }
            | Expr::Nil { location }
            | Expr::Variable { location, .. }
            | Expr::Assign { location, .. }
            | Expr::CompoundAssign { location, .. }
            | Expr::Unary { location, .. }
            | Expr::Grouping { location, .. }
            | Expr::MapLiteral { location, .. }
            | Expr::ArrayLiteral { location, .. }
            | Expr::SetLiteral { location, .. }
            | Expr::Function { location, .. }
            | Expr::If { location, .. }
            | Expr::Match { location, .. } => *location,
        }
    }

    /// Whether `expr` is a pattern a match arm accepts: a number, string,
    /// bool or nil literal, `Enum.Variant`, or a range of integer literals.
    fn is_valid_match_pattern(&self, expr: &Expr) -> bool {
        match expr {
            Expr::String { .. } | Expr::Boolean { .. } | Expr::Nil { .. } => true,
            Expr::Range { start, end, .. } => {
                Self::match_pattern_int(start).is_some() && Self::match_pattern_int(end).is_some()
            }
            Expr::GetField { object, .. } => {
                matches!(object.as_ref(), Expr::Variable { name, .. } if self.enum_variants(name).is_some())
            }
            _ => Self::match_pattern_number(expr).is_some(),
        }
    }

    /// How a match pattern prints in an error message: its source spelling
    /// for a literal, negative number or range, `Enum.Variant` for an enum
    /// variant.
    fn match_pattern_display(expr: &Expr) -> String {
        match expr {
            Expr::Number { raw, .. } | Expr::Int { raw, .. } => raw.clone(),
            Expr::String { raw, .. } => format!("\"{}\"", raw),
            Expr::Boolean { value, .. } => value.to_string(),
            Expr::Nil { .. } => "nil".to_string(),
            Expr::Unary {
                operator: UnaryOp::Negate,
                operand,
                ..
            } => format!("-{}", Self::match_pattern_display(operand)),
            Expr::Range {
                start,
                end,
                inclusive,
                ..
            } => format!(
                "{}{}{}",
                Self::match_pattern_display(start),
                if *inclusive { "..=" } else { ".." },
                Self::match_pattern_display(end)
            ),
            Expr::GetField { object, field, .. } => {
                if let Expr::Variable { name, .. } = object.as_ref() {
                    format!("{}.{}", name, field)
                } else {
                    field.clone()
                }
            }
            _ => "<pattern>".to_string(),
        }
    }

    /// An enum match - one whose patterns include at least one
    /// `Enum.Variant` - must have every non-wildcard pattern belong to that
    /// enum, and (absent a wildcard) cover every one of its variants.
    #[allow(clippy::expect_used)]
    fn check_match_exhaustiveness(&mut self, arms: &[MatchArm], match_location: SourceLocation) {
        let mut enum_identity: Option<(Rc<str>, Vec<EnumVariant>)> = None;
        for arm in arms {
            for pattern in &arm.patterns {
                if let Some((access, _)) = self.match_pattern_variant(pattern) {
                    let variants = self
                        .enum_variants(&access.enum_name)
                        .expect("a resolved enum variant access names a declared enum");
                    enum_identity = Some((access.enum_name, variants));
                    break;
                }
            }
            if enum_identity.is_some() {
                break;
            }
        }

        let Some((enum_name, variants)) = enum_identity else {
            return;
        };

        let mut has_wildcard = false;
        let mut covered: Vec<String> = Vec::new();

        for arm in arms {
            for pattern in &arm.patterns {
                if pattern.is_irrefutable() {
                    has_wildcard |= arm.guard.is_none();
                    continue;
                }
                if let MatchPattern::Variant { .. } = pattern {
                    if let Some((access, matches_all)) = self.match_pattern_variant(pattern) {
                        if access.enum_name != enum_name {
                            self.push_error(CompilationError::new(
                                CompilationPhase::Semantic,
                                CompilationErrorKind::PatternNotInEnum,
                                format!(
                                    "Pattern {}.{} does not belong to enum {}",
                                    access.enum_name, access.variant_name, enum_name
                                ),
                                Self::pattern_location(pattern),
                            ));
                        } else if matches_all
                            && arm.guard.is_none()
                            && !covered.contains(&access.variant_name.to_string())
                        {
                            covered.push(access.variant_name.to_string());
                        }
                    }
                    continue;
                }
                let MatchPattern::Expr(expr) = pattern else {
                    continue;
                };
                match self.match_pattern_enum_variant(expr) {
                    Some(access) if access.enum_name == enum_name => {
                        if arm.guard.is_none()
                            && !covered.contains(&access.variant_name.to_string())
                        {
                            covered.push(access.variant_name.to_string());
                        }
                    }
                    _ if !self.is_valid_match_pattern(expr) => {}
                    _ => {
                        self.push_error(CompilationError::new(
                            CompilationPhase::Semantic,
                            CompilationErrorKind::PatternNotInEnum,
                            format!(
                                "Pattern {} does not belong to enum {}",
                                Self::match_pattern_display(expr),
                                enum_name
                            ),
                            Self::match_pattern_location(expr),
                        ));
                    }
                }
            }
        }

        if !has_wildcard {
            let missing: Vec<&str> = variants
                .iter()
                .filter(|v| !covered.contains(&v.name))
                .map(|v| v.name.as_str())
                .collect();
            if !missing.is_empty() {
                self.push_error(CompilationError::new(
                    CompilationPhase::Semantic,
                    CompilationErrorKind::NonExhaustiveMatch,
                    format!("match on {} is missing {}", enum_name, missing.join(", ")),
                    match_location,
                ));
            }
        }
    }

    /// The value of a number literal match pattern, optionally negated.
    fn match_pattern_number(expr: &Expr) -> Option<PatternNumber> {
        match expr {
            Expr::Int { value, .. } => Some(PatternNumber::Int(*value)),
            Expr::Number { value, .. } => Some(PatternNumber::Float(*value)),
            Expr::Unary {
                operator: UnaryOp::Negate,
                operand,
                ..
            } => match operand.as_ref() {
                Expr::Int { value, .. } => Some(PatternNumber::Int(-value)),
                Expr::Number { value, .. } => Some(PatternNumber::Float(-value)),
                _ => None,
            },
            _ => None,
        }
    }

    fn match_pattern_int(expr: &Expr) -> Option<i64> {
        match Self::match_pattern_number(expr)? {
            PatternNumber::Int(value) => Some(value),
            PatternNumber::Float(_) => None,
        }
    }

    /// A comparable identity for a match pattern expression, used to find
    /// duplicate patterns. `None` for an invalid pattern.
    fn match_pattern_key(&self, expr: &Expr) -> Option<MatchPatternKey> {
        match Self::match_pattern_number(expr) {
            Some(PatternNumber::Int(value)) => return Some(MatchPatternKey::Int(value)),
            Some(PatternNumber::Float(value)) => {
                let whole_in_range = value.fract() == 0.0
                    && (-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&value);
                return Some(if whole_in_range {
                    MatchPatternKey::Int(value as i64)
                } else {
                    MatchPatternKey::Float(value)
                });
            }
            None => {}
        }
        match expr {
            Expr::String { value, .. } => Some(MatchPatternKey::Str(value.clone())),
            Expr::Boolean { value, .. } => Some(MatchPatternKey::Bool(*value)),
            Expr::Nil { .. } => Some(MatchPatternKey::Nil),
            Expr::Range {
                start,
                end,
                inclusive,
                ..
            } => {
                let start = Self::match_pattern_int(start)?;
                let end = Self::match_pattern_int(end)?;
                Some(MatchPatternKey::Range(start, end, *inclusive))
            }
            _ => self
                .match_pattern_enum_variant(expr)
                .map(|access| MatchPatternKey::Enum(access.enum_name, access.variant_name)),
        }
    }

    /// Reports a pattern that repeats an earlier one or follows a `_`.
    fn check_match_unreachable_patterns(&mut self, arms: &[MatchArm]) {
        let mut seen: Vec<MatchPatternKey> = Vec::new();
        let mut seen_wildcard = false;
        for arm in arms {
            let seen_before_arm = seen.len();
            let mut arm_wildcard = seen_wildcard;
            for pattern in &arm.patterns {
                let key = match pattern {
                    MatchPattern::Expr(expr) => self.match_pattern_key(expr),
                    MatchPattern::Variant { .. } => {
                        self.match_pattern_variant(pattern)
                            .and_then(|(access, matches_all)| {
                                matches_all.then_some(MatchPatternKey::Enum(
                                    access.enum_name,
                                    access.variant_name,
                                ))
                            })
                    }
                    _ => None,
                };
                let location = Self::pattern_location(pattern);
                let duplicate = key.as_ref().is_some_and(|key| seen.contains(key));
                if arm_wildcard || duplicate {
                    self.push_error(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::UnreachablePattern,
                        "unreachable pattern".to_string(),
                        location,
                    ));
                } else if let Some(key) = key {
                    seen.push(key);
                }
                if pattern.is_irrefutable() {
                    arm_wildcard = true;
                }
            }
            if arm.guard.is_some() {
                seen.truncate(seen_before_arm);
            } else {
                seen_wildcard = arm_wildcard;
            }
        }
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
                    self.push_error(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::MethodNeedsInstance,
                        format!(
                            "Method '{}' needs an instance; call it on a {} value",
                            method, struct_name
                        ),
                        location,
                    ));
                }
                (MethodCallKind::Instance, false) => {
                    self.push_error(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::MethodIsStatic,
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

        let error_message = unknown_method_error(struct_name, method, &candidate_refs, None);

        self.push_error(CompilationError::new(
            CompilationPhase::Semantic,
            CompilationErrorKind::UnknownMethod,
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
        let renamed = renamed_method_suggestion(method, &candidate_refs);
        let error_message = unknown_method_error(object_type, method, &candidate_refs, renamed);

        self.push_error(CompilationError::new(
            CompilationPhase::Semantic,
            CompilationErrorKind::UnknownMethod,
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
            self.push_error(CompilationError::new(
                CompilationPhase::Semantic,
                CompilationErrorKind::UnknownField,
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
                SymbolKind::Namespace | SymbolKind::Enum { .. } => {
                    self.push_error(CompilationError::new(
                        CompilationPhase::Semantic,
                        CompilationErrorKind::NotCallable,
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
                CompilationErrorKind::TooManyArguments
            };
            self.push_error(CompilationError::new(
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

/// Builtin methods removed in favour of a unified name, so the unknown-method hint can point at it.
const RENAMED_METHODS: &[(&str, &str)] = &[
    ("len", "size"),
    ("length", "size"),
    ("includes", "contains"),
    ("has", "contains"),
];

/// The unified name `method` was renamed to, if any, provided it's among
/// `candidates`.
fn renamed_method_suggestion<'a>(method: &str, candidates: &[&'a str]) -> Option<&'a str> {
    RENAMED_METHODS
        .iter()
        .find(|(old, new)| *old == method && candidates.contains(new))
        .map(|(_, new)| *new)
}

/// Builds the "unknown method" error message for `type_name`, suggesting
/// `renamed` or else the closest match among `candidates` when one exists.
fn unknown_method_error(
    type_name: &str,
    method: &str,
    candidates: &[&str],
    renamed: Option<&str>,
) -> String {
    let suggestion = renamed
        .or_else(|| crate::common::string_similarity::find_closest_match(method, candidates));

    if let Some(suggestion) = suggestion {
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

/// True when `params`/`implicit_it` make a function own the name `it` in
/// its own scope, hiding any enclosing block's `it` from its body.
fn owns_it(params: &[String], implicit_it: bool) -> bool {
    implicit_it || params.iter().any(|param| param == "it")
}

/// Whether a trailing block's own body - not a nested block's or function's,
/// each of which binds its own `it` if it owns the name - mentions `it` as
/// a free variable. A statement that declares `it` itself (`val`/`var` or
/// `fn it`) shadows any outer `it` from that point on, so statements after
/// it in the same scope are not scanned. A `for it in ...` loop variable
/// only exists inside the loop's body, so it doesn't shadow anything here.
fn block_references_it(body: &[Stmt]) -> bool {
    for stmt in body {
        if stmt_references_it(stmt) {
            return true;
        }
        if stmt_declares_it(stmt) {
            return false;
        }
    }
    false
}

fn stmt_declares_it(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Val { pattern, .. } | Stmt::Var { pattern, .. } => {
            pattern.bindings().iter().any(|b| b.name == "it")
        }
        Stmt::Fn { name, .. } => name == "it",
        _ => false,
    }
}

fn stmt_references_it(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Val { initializer, .. } | Stmt::Var { initializer, .. } => {
            initializer.as_ref().is_some_and(expr_references_it)
        }
        Stmt::Fn { params, body, .. } => !owns_it(params, false) && block_references_it(body),
        Stmt::Struct { .. } | Stmt::Enum { .. } | Stmt::Break { .. } | Stmt::Continue { .. } => {
            false
        }
        Stmt::Impl { methods, .. } => methods.iter().any(stmt_references_it),
        Stmt::Expression { expr, .. } => expr_references_it(expr),
        Stmt::Block { statements, .. } => block_references_it(statements),
        Stmt::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            expr_references_it(condition)
                || stmt_references_it(then_branch)
                || else_branch.as_deref().is_some_and(stmt_references_it)
        }
        Stmt::While {
            condition, body, ..
        } => expr_references_it(condition) || stmt_references_it(body),
        Stmt::Return { value, .. } => value.as_ref().is_some_and(expr_references_it),
        Stmt::ForIn {
            pattern,
            collection,
            body,
            ..
        } => {
            let declares_it = pattern.bindings().iter().any(|b| b.name == "it");
            expr_references_it(collection) || (!declares_it && stmt_references_it(body))
        }
    }
}

fn pattern_references_it(pattern: &MatchPattern) -> bool {
    match pattern {
        MatchPattern::Expr(expr) => expr_references_it(expr),
        MatchPattern::Array { elements, .. } => elements.iter().any(pattern_references_it),
        MatchPattern::Variant { fields, .. } => fields.iter().any(pattern_references_it),
        MatchPattern::Wildcard(_) | MatchPattern::Binding(_) | MatchPattern::Rest { .. } => false,
    }
}

fn expr_references_it(expr: &Expr) -> bool {
    match expr {
        Expr::Variable { name, .. } => name == "it",
        Expr::Number { .. }
        | Expr::Int { .. }
        | Expr::String { .. }
        | Expr::Boolean { .. }
        | Expr::Nil { .. } => false,
        Expr::StringInterpolation { parts, .. } => parts.iter().any(|part| match part {
            InterpolationPart::Literal { .. } => false,
            InterpolationPart::Expression(expr) => expr_references_it(expr),
        }),
        Expr::Assign { name, value, .. } => name == "it" || expr_references_it(value),
        Expr::CompoundAssign { name, value, .. } => name == "it" || expr_references_it(value),
        Expr::Binary { left, right, .. } => expr_references_it(left) || expr_references_it(right),
        Expr::Unary { operand, .. } => expr_references_it(operand),
        Expr::Call {
            callee, arguments, ..
        } => expr_references_it(callee) || arguments.iter().any(expr_references_it),
        Expr::GetField { object, .. } => expr_references_it(object),
        Expr::SetField { object, value, .. } => {
            expr_references_it(object) || expr_references_it(value)
        }
        Expr::CompoundAssignField { object, value, .. } => {
            expr_references_it(object) || expr_references_it(value)
        }
        Expr::Grouping { expr, .. } => expr_references_it(expr),
        Expr::MapLiteral { entries, .. } => entries
            .iter()
            .any(|(key, value)| expr_references_it(key) || expr_references_it(value)),
        Expr::ArrayLiteral { elements, .. } | Expr::SetLiteral { elements, .. } => {
            elements.iter().any(expr_references_it)
        }
        Expr::Index { object, index, .. } => {
            expr_references_it(object) || expr_references_it(index)
        }
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        }
        | Expr::CompoundAssignIndex {
            object,
            index,
            value,
            ..
        } => expr_references_it(object) || expr_references_it(index) || expr_references_it(value),
        Expr::Range { start, end, .. } => expr_references_it(start) || expr_references_it(end),
        Expr::Conditional {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            expr_references_it(condition)
                || expr_references_it(then_expr)
                || expr_references_it(else_expr)
        }
        Expr::Function {
            params,
            body,
            implicit_it,
            ..
        } => !owns_it(params, *implicit_it) && block_references_it(body),
        Expr::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            expr_references_it(condition)
                || stmt_references_it(then_branch)
                || match else_branch.as_ref() {
                    IfExprElse::If(expr) => expr_references_it(expr),
                    IfExprElse::Block(stmt) => stmt_references_it(stmt),
                }
        }
        Expr::Match {
            scrutinee, arms, ..
        } => {
            expr_references_it(scrutinee)
                || arms.iter().any(|arm| {
                    arm.patterns.iter().any(pattern_references_it)
                        || arm.guard.as_ref().is_some_and(expr_references_it)
                        || match &arm.body {
                            MatchArmBody::Expr(expr) => expr_references_it(expr),
                            MatchArmBody::Block(stmt) => stmt_references_it(stmt),
                        }
                })
        }
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
