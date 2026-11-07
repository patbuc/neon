use crate::common::SourceLocation;
use crate::common::method_registry::NATIVE_METHODS;
use crate::common::stdlib::BUILTIN_VALUES;
use crate::compiler::ast::{
    Binding, Expr, IfExprElse, InterpolationPart, MatchArmBody, MatchPattern, NodeId, Pattern, Stmt,
};
use crate::compiler::json_string;
use crate::compiler::resolutions::{Capture, DeclId, DeclKind, Res, Resolutions};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SemanticType {
    Function,
    Parameter,
    Variable,
    Struct,
    Enum,
    EnumMember,
    Namespace,
}

impl SemanticType {
    pub(crate) fn name(self) -> &'static str {
        match self {
            SemanticType::Function => "function",
            SemanticType::Parameter => "parameter",
            SemanticType::Variable => "variable",
            SemanticType::Struct => "struct",
            SemanticType::Enum => "enum",
            SemanticType::EnumMember => "enumMember",
            SemanticType::Namespace => "namespace",
        }
    }

    fn of(kind: DeclKind) -> SemanticType {
        match kind {
            DeclKind::Value | DeclKind::Variable => SemanticType::Variable,
            DeclKind::Function => SemanticType::Function,
            DeclKind::Struct => SemanticType::Struct,
            DeclKind::Enum => SemanticType::Enum,
            DeclKind::Parameter => SemanticType::Parameter,
            DeclKind::Module => SemanticType::Namespace,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Modifiers {
    pub(crate) declaration: bool,
    pub(crate) readonly: bool,
    pub(crate) default_library: bool,
}

/// One named thing in the source. `line` and `column` are 1-based and
/// `column` and `length` count characters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SemanticToken {
    pub(crate) line: u32,
    pub(crate) column: u32,
    pub(crate) length: usize,
    pub(crate) kind: SemanticType,
    pub(crate) modifiers: Modifiers,
}

/// The tokens for the names in `ast` that `resolutions` resolved, sorted by
/// position.
pub(crate) fn compute(ast: &[Stmt], resolutions: &Resolutions) -> Vec<SemanticToken> {
    let mut walker = Walker {
        res: resolutions,
        functions: Vec::new(),
        tokens: Vec::new(),
    };
    walker.stmts(ast);
    let mut tokens = walker.tokens;
    tokens.sort_by_key(|token| (token.line, token.column));
    tokens.dedup_by_key(|token| (token.line, token.column));
    tokens
}

pub(crate) fn to_json(tokens: &[SemanticToken]) -> String {
    let items: Vec<String> = tokens
        .iter()
        .map(|token| {
            let m = token.modifiers;
            let modifiers: Vec<String> = [
                (m.declaration, "declaration"),
                (m.readonly, "readonly"),
                (m.default_library, "defaultLibrary"),
            ]
            .into_iter()
            .filter(|(on, _)| *on)
            .map(|(_, name)| json_string(name))
            .collect();
            format!(
                "{{\"line\":{},\"column\":{},\"length\":{},\"type\":{},\"modifiers\":[{}]}}",
                token.line,
                token.column,
                token.length,
                json_string(token.kind.name()),
                modifiers.join(",")
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}

struct Walker<'a> {
    res: &'a Resolutions,
    /// The functions enclosing the node being walked, outermost first.
    functions: Vec<NodeId>,
    tokens: Vec<SemanticToken>,
}

impl Walker<'_> {
    fn push(
        &mut self,
        location: SourceLocation,
        name: &str,
        kind: SemanticType,
        modifiers: Modifiers,
    ) {
        self.tokens.push(SemanticToken {
            line: location.line,
            column: location.column,
            length: name.chars().count(),
            kind,
            modifiers,
        });
    }

    fn plain(&mut self, location: SourceLocation, name: &str, kind: SemanticType) {
        self.push(location, name, kind, Modifiers::default());
    }

    fn declared(&mut self, location: SourceLocation, name: &str, kind: SemanticType) {
        let modifiers = Modifiers {
            declaration: true,
            ..Modifiers::default()
        };
        self.push(location, name, kind, modifiers);
    }

    fn decl_token(
        &mut self,
        decl: DeclId,
        location: SourceLocation,
        name: &str,
        declaration: bool,
    ) {
        let Some(kind) = self.res.decl_kind(decl) else {
            return;
        };
        let kind_type = SemanticType::of(kind);
        let modifiers = Modifiers {
            declaration,
            readonly: kind_type == SemanticType::Variable && self.res.is_immutable(decl),
            default_library: false,
        };
        self.push(location, name, kind_type, modifiers);
    }

    /// The declaration node `id` declares, as a token at `location`.
    fn declaration(&mut self, id: NodeId, location: SourceLocation, name: &str) {
        if let Some(decl) = self.res.try_decl(id) {
            self.decl_token(decl, location, name, true);
        }
    }

    fn binding(&mut self, binding: &Binding) {
        self.declaration(binding.id, binding.location, &binding.name);
    }

    fn pattern(&mut self, pattern: &Pattern) {
        for binding in pattern.bindings() {
            self.binding(binding);
        }
    }

    /// Follows an upvalue of the innermost function out through the
    /// functions that capture it, to the declaration it names.
    fn captured(&self, mut index: u32) -> Option<DeclId> {
        for function in self.functions.iter().rev() {
            let capture = self
                .res
                .try_function(*function)?
                .upvalues
                .get(index as usize)?;
            match capture {
                Capture::Local(decl) => return Some(*decl),
                Capture::Upvalue(outer) => index = *outer,
            }
        }
        None
    }

    fn variable(&mut self, id: NodeId, name: &str, location: SourceLocation) {
        match self.res.try_res(id) {
            Some(Res::Local(decl) | Res::Global(decl)) => {
                self.decl_token(decl, location, name, false);
            }
            Some(Res::Upvalue(index)) => {
                if let Some(decl) = self.captured(index) {
                    self.decl_token(decl, location, name, false);
                }
            }
            Some(Res::Builtin(index)) => {
                if let Some((builtin, _)) = BUILTIN_VALUES.get(index as usize) {
                    self.push(
                        location,
                        builtin,
                        SemanticType::Variable,
                        Modifiers {
                            default_library: true,
                            ..Modifiers::default()
                        },
                    );
                }
            }
            None => {}
        }
    }

    fn stmts(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.stmt(stmt);
        }
    }

    fn function(&mut self, id: NodeId, params: &[Binding], body: &[Stmt]) {
        self.functions.push(id);
        for param in params {
            self.binding(param);
        }
        self.stmts(body);
        self.functions.pop();
    }

    fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Val {
                pattern,
                initializer,
                ..
            }
            | Stmt::Var {
                pattern,
                initializer,
                ..
            } => {
                self.pattern(pattern);
                if let Some(initializer) = initializer {
                    self.expr(initializer);
                }
            }
            Stmt::Fn {
                name,
                params,
                body,
                id,
                location,
            } => {
                self.declaration(*id, *location, name);
                self.function(*id, params, body);
            }
            Stmt::Struct {
                name,
                fields,
                id,
                location,
            } => {
                self.declaration(*id, *location, name);
                for field in fields {
                    if let Some(default) = &field.default {
                        self.functions.push(field.default_id);
                        self.expr(default);
                        self.functions.pop();
                    }
                }
            }
            Stmt::Enum {
                name,
                variants,
                id,
                location,
            } => {
                self.declaration(*id, *location, name);
                for variant in variants {
                    self.declared(variant.location, &variant.name, SemanticType::EnumMember);
                }
            }
            Stmt::Impl { methods, .. } => {
                for method in methods {
                    if let Stmt::Fn {
                        params, body, id, ..
                    } = method
                    {
                        self.function(*id, params, body);
                    }
                }
            }
            Stmt::Expression { expr, .. } => self.expr(expr),
            Stmt::Block { statements, .. } => self.stmts(statements),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.expr(condition);
                self.stmt(then_branch);
                if let Some(else_branch) = else_branch {
                    self.stmt(else_branch);
                }
            }
            Stmt::While {
                condition, body, ..
            } => {
                self.expr(condition);
                self.stmt(body);
            }
            Stmt::Return { value, .. } => {
                if let Some(value) = value {
                    self.expr(value);
                }
            }
            Stmt::ForIn {
                pattern,
                collection,
                body,
                ..
            } => {
                self.pattern(pattern);
                self.expr(collection);
                self.stmt(body);
            }
            Stmt::Try {
                body,
                catch_binding,
                catch_body,
                ..
            } => {
                self.stmt(body);
                self.binding(catch_binding);
                self.stmt(catch_body);
            }
            Stmt::Throw { value, .. } => self.expr(value),
            Stmt::Export { declaration, .. } => self.stmt(declaration),
            Stmt::Import {
                alias: Some(alias),
                alias_location: Some(location),
                ..
            } => self.declared(*location, alias, SemanticType::Namespace),
            Stmt::Import { .. } | Stmt::Break { .. } | Stmt::Continue { .. } => {}
        }
    }

    fn match_pattern(&mut self, pattern: &MatchPattern) {
        match pattern {
            MatchPattern::Expr(expr) => self.expr(expr),
            MatchPattern::Binding(binding) => self.binding(binding),
            MatchPattern::Array { elements, .. } => {
                for element in elements {
                    self.match_pattern(element);
                }
            }
            MatchPattern::Variant { target, fields } => {
                self.expr(target);
                for field in fields {
                    self.match_pattern(field);
                }
            }
            MatchPattern::Rest { binding, .. } => {
                if let Some(binding) = binding {
                    self.binding(binding);
                }
            }
            MatchPattern::Wildcard(_) => {}
        }
    }

    /// The enum named by `object` in `Color.Red`, `Color.values()` or
    /// `utils.Color.Red`.
    fn enum_reference(&mut self, object: &Expr) {
        match object {
            Expr::Variable { name, location, .. } => {
                self.plain(*location, name, SemanticType::Enum);
            }
            Expr::GetField {
                object,
                field,
                field_location,
                ..
            } => {
                if let Expr::Variable { name, location, .. } = object.as_ref() {
                    self.plain(*location, name, SemanticType::Namespace);
                }
                self.plain(*field_location, field, SemanticType::Enum);
            }
            _ => {}
        }
    }

    /// Tokens for `object.field` when `object` names a module or an enum.
    /// Returns false for an ordinary field access.
    fn static_member(
        &mut self,
        object: &Expr,
        field: &str,
        field_location: SourceLocation,
    ) -> bool {
        match object {
            Expr::Variable { id, name, location } => {
                if self.res.enum_variant_access(*id).is_some() {
                    self.enum_reference(object);
                    self.plain(field_location, field, SemanticType::EnumMember);
                    return true;
                }
                let Some(kind) = self.res.module_member_kind(*id) else {
                    return false;
                };
                self.plain(*location, name, SemanticType::Namespace);
                self.plain(field_location, field, SemanticType::of(kind));
                true
            }
            Expr::GetField { object: inner, .. } => {
                let Expr::Variable { id, .. } = inner.as_ref() else {
                    return false;
                };
                if self.res.enum_variant_access(*id).is_none() {
                    return false;
                }
                self.enum_reference(object);
                self.plain(field_location, field, SemanticType::EnumMember);
                true
            }
            _ => false,
        }
    }

    fn call(&mut self, id: NodeId, callee: &Expr) {
        match callee {
            Expr::Variable {
                id: callee_id,
                name,
                location,
            } if self.res.try_res(*callee_id).is_none() => {
                let is_global_native = self
                    .res
                    .native(id)
                    .and_then(|index| NATIVE_METHODS.get(index))
                    .is_some_and(|(module, _, _)| module.is_empty());
                if is_global_native {
                    self.push(
                        *location,
                        name,
                        SemanticType::Function,
                        Modifiers {
                            default_library: true,
                            ..Modifiers::default()
                        },
                    );
                }
            }
            Expr::GetField {
                object,
                field,
                field_location,
                ..
            } => {
                if self.res.enum_construct(id).is_some() {
                    self.enum_reference(object);
                    self.plain(*field_location, field, SemanticType::EnumMember);
                } else if self.res.enum_values_access(id).is_some() {
                    self.enum_reference(object);
                } else if !self.static_member(object, field, *field_location) {
                    self.expr(object);
                }
            }
            _ => self.expr(callee),
        }
    }

    fn expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number { .. }
            | Expr::Int { .. }
            | Expr::String { .. }
            | Expr::Boolean { .. }
            | Expr::Nil { .. } => {}
            Expr::StringInterpolation { parts, .. } => {
                for part in parts {
                    if let InterpolationPart::Expression(expr) = part {
                        self.expr(expr);
                    }
                }
            }
            Expr::Variable { name, id, location } => self.variable(*id, name, *location),
            Expr::Assign {
                name,
                value,
                id,
                location,
            } => {
                self.variable(*id, name, *location);
                self.expr(value);
            }
            Expr::CompoundAssign {
                name,
                value,
                read_id,
                write_id,
                location,
                ..
            } => {
                self.variable(*read_id, name, *location);
                self.variable(*write_id, name, *location);
                self.expr(value);
            }
            Expr::Binary { left, right, .. } => {
                self.expr(left);
                self.expr(right);
            }
            Expr::Unary { operand, .. } => self.expr(operand),
            Expr::Call {
                callee,
                arguments,
                id,
                ..
            } => {
                self.call(*id, callee);
                for argument in arguments {
                    self.expr(argument);
                }
            }
            Expr::StructUpdate {
                callee,
                base,
                arguments,
                ..
            } => {
                self.expr(callee);
                self.expr(base);
                for argument in arguments {
                    self.expr(argument);
                }
            }
            Expr::GetField {
                object,
                field,
                field_location,
                ..
            } => {
                if !self.static_member(object, field, *field_location) {
                    self.expr(object);
                }
            }
            Expr::SetField {
                object,
                field,
                value,
                field_location,
                ..
            }
            | Expr::CompoundAssignField {
                object,
                field,
                value,
                field_location,
                ..
            } => {
                if !self.static_member(object, field, *field_location) {
                    self.expr(object);
                }
                self.expr(value);
            }
            Expr::Grouping { expr, .. } => self.expr(expr),
            Expr::MapLiteral { entries, .. } => {
                for (key, value) in entries {
                    self.expr(key);
                    self.expr(value);
                }
            }
            Expr::ArrayLiteral { elements, .. } | Expr::SetLiteral { elements, .. } => {
                for element in elements {
                    self.expr(element);
                }
            }
            Expr::Index { object, index, .. } => {
                self.expr(object);
                self.expr(index);
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
                self.expr(object);
                self.expr(index);
                self.expr(value);
            }
            Expr::Range { start, end, .. } => {
                self.expr(start);
                self.expr(end);
            }
            Expr::Conditional {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                self.expr(condition);
                self.expr(then_expr);
                self.expr(else_expr);
            }
            Expr::Function {
                params, body, id, ..
            } => self.function(*id, params, body),
            Expr::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.expr(condition);
                self.stmt(then_branch);
                match else_branch.as_ref() {
                    IfExprElse::If(expr) => self.expr(expr),
                    IfExprElse::Block(stmt) => self.stmt(stmt),
                }
            }
            Expr::Yield { value, .. } => {
                if let Some(value) = value {
                    self.expr(value);
                }
            }
            Expr::Match {
                scrutinee, arms, ..
            } => {
                self.expr(scrutinee);
                for arm in arms {
                    for pattern in &arm.patterns {
                        self.match_pattern(pattern);
                    }
                    if let Some(guard) = &arm.guard {
                        self.expr(guard);
                    }
                    match &arm.body {
                        MatchArmBody::Expr(expr) => self.expr(expr),
                        MatchArmBody::Block(stmt) => self.stmt(stmt),
                    }
                }
            }
        }
    }
}
