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

/// An enum variant and the location of its name.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: String,
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
    },
    /// `if cond { ... } else ...` in expression position. `then_branch` and
    /// a terminal `else_branch` are always `Stmt::Block`; an `else if`
    /// chains through `IfExprElse::If`. A branch's value is its last
    /// expression statement, else `nil`.
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Box<IfExprElse>,
        location: SourceLocation,
    },
}

/// The `else` clause of an if-expression.
#[derive(Debug, Clone, PartialEq)]
pub enum IfExprElse {
    /// `else if ...`; always wraps `Expr::If`.
    If(Expr),
    /// A terminal `else { ... }`; always wraps `Stmt::Block`.
    Block(Stmt),
}

/// Statement nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Val {
        name: String,
        initializer: Option<Expr>,
        id: NodeId,
        location: SourceLocation,
    },
    Var {
        name: String,
        initializer: Option<Expr>,
        id: NodeId,
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
        variable: String,
        collection: Expr,
        body: Box<Stmt>,
        id: NodeId,
        location: SourceLocation,
    },
    Break {
        location: SourceLocation,
    },
    Continue {
        location: SourceLocation,
    },
}
