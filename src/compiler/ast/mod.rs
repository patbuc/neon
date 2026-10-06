use crate::common::SourceLocation;

/// Stable identity for a name-use or name-declaration AST node, assigned by the parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

/// A struct field and the location of its name.
#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: String,
    pub location: SourceLocation,
}

/// An enum variant, its payload field names (empty for a unit variant), and
/// the location of its name.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<String>,
    pub location: SourceLocation,
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    // Comparison
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Logical
    And,
    Or,
    NilCoalesce,
    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
    BitwiseNot,
}

/// Parts of an interpolated string
#[derive(Debug, Clone, PartialEq)]
pub enum InterpolationPart {
    /// `value` is decoded text; `raw` is the exact source text of this
    /// segment, escapes undecoded.
    Literal {
        value: String,
        raw: String,
    },
    Expression(Box<Expr>),
}

/// Expression nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number {
        value: f64,
        /// Exact source spelling (e.g. `0xFF`, `1_000`, `1e3`).
        raw: String,
        location: SourceLocation,
    },
    /// A literal with no `.` and no exponent: decimal, hex, binary, or octal.
    Int {
        value: i64,
        /// Exact source spelling (e.g. `0xFF`, `1_000`).
        raw: String,
        location: SourceLocation,
    },
    String {
        value: String,
        /// Exact source text between the quotes, escapes undecoded.
        raw: String,
        location: SourceLocation,
    },
    StringInterpolation {
        parts: Vec<InterpolationPart>,
        location: SourceLocation,
    },
    Boolean {
        value: bool,
        location: SourceLocation,
    },
    Nil {
        location: SourceLocation,
    },
    Variable {
        name: String,
        id: NodeId,
        location: SourceLocation,
    },
    Assign {
        name: String,
        value: Box<Expr>,
        id: NodeId,
        location: SourceLocation,
    },
    /// `read_id` is the resolution of the implicit read of `x`; `write_id`
    /// is the resolution of the assignment target, matching `Assign::id`.
    CompoundAssign {
        name: String,
        operator: BinaryOp,
        value: Box<Expr>,
        read_id: NodeId,
        write_id: NodeId,
        location: SourceLocation,
    },
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
        location: SourceLocation,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
        location: SourceLocation,
    },
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
        id: NodeId,
        location: SourceLocation,
    },
    GetField {
        object: Box<Expr>,
        field: String,
        /// `true` for `a?.b`: yields nil without erroring if `object` is nil.
        optional: bool,
        location: SourceLocation,
    },
    SetField {
        object: Box<Expr>,
        field: String,
        value: Box<Expr>,
        location: SourceLocation,
    },
    /// `object.field op= value`; the VM's `Dup` opcode evaluates `object`
    /// once for both the read and the write. `operator_location` is the
    /// `op=` token, used to report arithmetic errors at the operator like
    /// the desugared long form does.
    CompoundAssignField {
        object: Box<Expr>,
        field: String,
        operator: BinaryOp,
        value: Box<Expr>,
        location: SourceLocation,
        operator_location: SourceLocation,
    },
    Grouping {
        expr: Box<Expr>,
        location: SourceLocation,
    },
    MapLiteral {
        entries: Vec<(Expr, Expr)>,
        location: SourceLocation,
    },
    ArrayLiteral {
        elements: Vec<Expr>,
        location: SourceLocation,
    },
    SetLiteral {
        elements: Vec<Expr>,
        location: SourceLocation,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
        location: SourceLocation,
    },
    IndexAssign {
        object: Box<Expr>,
        index: Box<Expr>,
        value: Box<Expr>,
        location: SourceLocation,
    },
    CompoundAssignIndex {
        object: Box<Expr>,
        index: Box<Expr>,
        operator: BinaryOp,
        value: Box<Expr>,
        location: SourceLocation,
        operator_location: SourceLocation,
    },
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        inclusive: bool,
        location: SourceLocation,
    },
    Conditional {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
        location: SourceLocation,
    },
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
        id: NodeId,
        location: SourceLocation,
        /// True for a trailing block parsed without a `name, name ->`
        /// header: the semantic pass gives it a synthesized `it` parameter
        /// when its body references `it` as a free variable.
        implicit_it: bool,
    },
    /// `if cond { ... } else ...` in expression position. `then_branch` and
    /// a terminal `else_branch` are always `Stmt::Block`; an `else if`
    /// chains through `IfExprElse::If`.
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Box<IfExprElse>,
        location: SourceLocation,
    },
    /// `match scrutinee { pattern, pattern -> body ... }`.
    Match {
        scrutinee: Box<Expr>,
        arms: Vec<MatchArm>,
        location: SourceLocation,
    },
}

/// One `pattern, pattern -> body` arm of a `match`.
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub patterns: Vec<MatchPattern>,
    /// `if cond` after the patterns; the arm is skipped when it is false.
    pub guard: Option<Expr>,
    pub body: MatchArmBody,
    pub location: SourceLocation,
}

/// A single pattern in a match arm.
#[derive(Debug, Clone, PartialEq)]
pub enum MatchPattern {
    /// A literal, `Enum.Variant` or range expression; matched with `==`,
    /// except an `Expr::Range`, which is matched by containment.
    Expr(Expr),
    Wildcard(SourceLocation),
    /// A bare name: matches anything and binds it, immutably, for the arm.
    Binding(Binding),
    /// `[p1, p2]`: an array of exactly that length whose elements match
    /// the sub-patterns. Any other value does not match. With a `Rest`
    /// element, any array at least that long (without the rest) matches.
    Array {
        elements: Vec<MatchPattern>,
        location: SourceLocation,
    },
    /// `Enum.Variant(p1, p2)`: that variant, whose fields match the
    /// sub-patterns by position. Any other value does not match. `target`
    /// is the `Enum.Variant` access, never a call.
    Variant {
        target: Expr,
        fields: Vec<MatchPattern>,
    },
    /// `..` or `..name` inside an array pattern: the elements between the
    /// ones before and after it. A name binds them as a new array.
    Rest {
        binding: Option<Binding>,
        location: SourceLocation,
    },
}

/// One step from a value to a part of it, as followed by an array or
/// variant pattern's bindings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathStep {
    /// The element at this index; negative counts from the end.
    Index(i64),
    /// The elements after the first `before` and before the last `after`.
    Rest { before: usize, after: usize },
    /// The field at this position of a variant pattern's variant; `variant`
    /// is the node id the semantic pass resolved `Enum.Variant` under.
    Field { variant: NodeId, index: usize },
}

impl PathStep {
    /// The step to each of an array pattern's `elements`.
    pub fn for_elements(elements: &[MatchPattern]) -> Vec<PathStep> {
        let rest_position = elements
            .iter()
            .position(|element| matches!(element, MatchPattern::Rest { .. }));
        (0..elements.len())
            .map(|index| match rest_position {
                Some(rest) if index == rest => PathStep::Rest {
                    before: rest,
                    after: elements.len() - rest - 1,
                },
                Some(rest) if index > rest => PathStep::Index(index as i64 - elements.len() as i64),
                _ => PathStep::Index(index as i64),
            })
            .collect()
    }
}

impl MatchPattern {
    /// Whether the pattern matches any value: `_`, a name, or a rest.
    pub fn is_irrefutable(&self) -> bool {
        matches!(
            self,
            MatchPattern::Wildcard(_) | MatchPattern::Binding(_) | MatchPattern::Rest { .. }
        )
    }

    /// Every name this pattern binds, in source order, each with the
    /// steps that lead from the matched value to what it binds.
    pub fn bindings(&self) -> Vec<(&Binding, Vec<PathStep>)> {
        let mut found = Vec::new();
        self.collect_bindings(&mut Vec::new(), &mut found);
        found
    }

    fn collect_bindings<'a>(
        &'a self,
        path: &mut Vec<PathStep>,
        found: &mut Vec<(&'a Binding, Vec<PathStep>)>,
    ) {
        match self {
            MatchPattern::Binding(binding) => found.push((binding, path.clone())),
            MatchPattern::Array { elements, .. } => {
                for (step, element) in PathStep::for_elements(elements).into_iter().zip(elements) {
                    path.push(step);
                    element.collect_bindings(path, found);
                    path.pop();
                }
            }
            MatchPattern::Variant { target, fields } => {
                let Expr::GetField { object, .. } = target else {
                    return;
                };
                let Expr::Variable { id, .. } = object.as_ref() else {
                    return;
                };
                for (index, field) in fields.iter().enumerate() {
                    path.push(PathStep::Field {
                        variant: *id,
                        index,
                    });
                    field.collect_bindings(path, found);
                    path.pop();
                }
            }
            MatchPattern::Rest { binding, .. } => {
                if let Some(binding) = binding {
                    found.push((binding, path.clone()));
                }
            }
            MatchPattern::Expr(_) | MatchPattern::Wildcard(_) => {}
        }
    }
}

/// The body of a match arm: `-> expr` or `-> { ... }`.
#[derive(Debug, Clone, PartialEq)]
pub enum MatchArmBody {
    Expr(Expr),
    /// Always wraps `Stmt::Block`.
    Block(Stmt),
}

/// The `else` clause of an if-expression.
#[derive(Debug, Clone, PartialEq)]
pub enum IfExprElse {
    /// `else if ...`; always wraps `Expr::If`.
    If(Expr),
    /// A terminal `else { ... }`; always wraps `Stmt::Block`.
    Block(Stmt),
}

/// A single declared name: `val x = ...`'s `x`, or one non-`_` slot of a
/// tuple pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    pub name: String,
    pub id: NodeId,
    pub location: SourceLocation,
}

/// What a `val`/`var`/`for` declares: a single name, or a tuple pattern
/// (`(a, _, c)`) destructuring an Array - `_` skips a position without
/// declaring anything, so that slot is `None`.
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Name(Binding),
    Tuple(Vec<Option<Binding>>),
}

impl Pattern {
    /// The names this pattern actually declares, in order - skipping `_`
    /// slots for a tuple pattern.
    pub fn bindings(&self) -> Vec<&Binding> {
        match self {
            Pattern::Name(binding) => vec![binding],
            Pattern::Tuple(slots) => slots.iter().filter_map(Option::as_ref).collect(),
        }
    }
}

/// Statement nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Val {
        pattern: Pattern,
        initializer: Option<Expr>,
        location: SourceLocation,
    },
    Var {
        pattern: Pattern,
        initializer: Option<Expr>,
        location: SourceLocation,
    },
    Fn {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        id: NodeId,
        location: SourceLocation,
    },
    Struct {
        name: String,
        fields: Vec<StructField>,
        id: NodeId,
        location: SourceLocation,
    },
    Enum {
        name: String,
        variants: Vec<EnumVariant>,
        id: NodeId,
        location: SourceLocation,
    },
    Impl {
        type_name: String,
        methods: Vec<Stmt>,
        location: SourceLocation,
    },
    Expression {
        expr: Expr,
        location: SourceLocation,
    },
    Block {
        statements: Vec<Stmt>,
        location: SourceLocation,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
        location: SourceLocation,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
        location: SourceLocation,
    },
    Return {
        value: Option<Expr>,
        location: SourceLocation,
    },
    ForIn {
        pattern: Pattern,
        collection: Expr,
        body: Box<Stmt>,
        location: SourceLocation,
    },
    Break {
        location: SourceLocation,
    },
    Continue {
        location: SourceLocation,
    },
    Export {
        declaration: Box<Stmt>,
        location: SourceLocation,
    },
    Import {
        path: String,
        alias: Option<String>,
        id: NodeId,
        location: SourceLocation,
    },
}
