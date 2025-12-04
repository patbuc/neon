use crate::common::SourceLocation;

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
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
}

/// Unary operators
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

/// Parts of an interpolated string
#[derive(Debug, Clone, PartialEq)]
pub enum InterpolationPart {
    /// Literal string part: "Hello "
    Literal(String),
    /// Expression part: ${name}
    Expression(Box<Expr>),
}

/// Expression nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Numeric literal: 42, 3.14
    Number {
        value: f64,
        location: SourceLocation,
    },
    /// String literal: "hello"
    String {
        value: String,
        location: SourceLocation,
    },
    /// String interpolation: "Hello ${name}"
    StringInterpolation {
        parts: Vec<InterpolationPart>,
        location: SourceLocation,
    },
    /// Boolean literal: true, false
    Boolean {
        value: bool,
        location: SourceLocation,
    },
    /// Nil literal
    Nil {
        location: SourceLocation,
    },
    /// Variable reference: x
    Variable {
        name: String,
        location: SourceLocation,
    },
    /// Variable assignment: x = 5
    Assign {
        name: String,
        value: Box<Expr>,
        location: SourceLocation,
    },
    /// Binary operation: a + b, x == y
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
        location: SourceLocation,
    },
    /// Unary operation: -x, !flag
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
        location: SourceLocation,
    },
    /// Function call: foo(a, b)
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
        location: SourceLocation,
    },
    /// Field access: obj.field
    GetField {
        object: Box<Expr>,
        field: String,
        location: SourceLocation,
    },
    /// Field assignment: obj.field = value
    SetField {
        object: Box<Expr>,
        field: String,
        value: Box<Expr>,
        location: SourceLocation,
    },
    /// Grouping expression: (expr)
    Grouping {
        expr: Box<Expr>,
        location: SourceLocation,
    },
    /// Method call: obj.method(args)
    MethodCall {
        object: Box<Expr>,
        method: String,
        arguments: Vec<Expr>,
        location: SourceLocation,
    },
    /// Map literal: {"key": value, "key2": value2}
    MapLiteral {
        entries: Vec<(Expr, Expr)>,
        location: SourceLocation,
    },
    /// Array literal: [1, 2, 3]
    ArrayLiteral {
        elements: Vec<Expr>,
        location: SourceLocation,
    },
    /// Set literal: {1, 2, 3}
    SetLiteral {
        elements: Vec<Expr>,
        location: SourceLocation,
    },
    /// Index access: map["key"], array[0]
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
        location: SourceLocation,
    },
    /// Index assignment: map["key"] = value
    IndexAssign {
        object: Box<Expr>,
        index: Box<Expr>,
        value: Box<Expr>,
        location: SourceLocation,
    },
    /// Range expression: 1..10 (exclusive), 1..=10 (inclusive)
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        inclusive: bool,
        location: SourceLocation,
    },
}

/// Statement nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// Immutable value declaration: val x = 5
    Val {
        name: String,
        initializer: Option<Expr>,
        location: SourceLocation,
    },
    /// Mutable variable declaration: var x = 5
    Var {
        name: String,
        initializer: Option<Expr>,
        location: SourceLocation,
    },
    /// Function declaration: fn foo(a, b) { ... }
    Fn {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        location: SourceLocation,
    },
    /// Struct declaration: struct Point { x, y }
    Struct {
        name: String,
        fields: Vec<String>,
        location: SourceLocation,
    },
    /// Print statement: print expr
    Print {
        expr: Expr,
        location: SourceLocation,
    },
    /// Expression statement: expr;
    Expression {
        expr: Expr,
        location: SourceLocation,
    },
    /// Block statement: { ... }
    Block {
        statements: Vec<Stmt>,
        location: SourceLocation,
    },
    /// If statement: if (cond) then else
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
        location: SourceLocation,
    },
    /// While loop: while (cond) body
    While {
        condition: Expr,
        body: Box<Stmt>,
        location: SourceLocation,
    },
    /// Return statement: return expr
    Return {
        value: Expr,
        location: SourceLocation,
    },
    /// For-in loop: for (variable in collection) body
    ForIn {
        variable: String,
        collection: Expr,
        body: Box<Stmt>,
        location: SourceLocation,
    },
    /// Break statement: break
    Break {
        location: SourceLocation,
    },
    /// Continue statement: continue
    Continue {
        location: SourceLocation,
    },
}

impl Expr {
    /// Get the source location of this expression
    pub fn location(&self) -> &SourceLocation {
        match self {
            Expr::Number { location, .. }
            | Expr::String { location, .. }
            | Expr::StringInterpolation { location, .. }
            | Expr::Boolean { location, .. }
            | Expr::Nil { location }
            | Expr::Variable { location, .. }
            | Expr::Assign { location, .. }
            | Expr::Binary { location, .. }
            | Expr::Unary { location, .. }
            | Expr::Call { location, .. }
            | Expr::GetField { location, .. }
            | Expr::SetField { location, .. }
            | Expr::Grouping { location, .. }
            | Expr::MethodCall { location, .. }
            | Expr::MapLiteral { location, .. }
            | Expr::ArrayLiteral { location, .. }
            | Expr::SetLiteral { location, .. }
            | Expr::Index { location, .. }
            | Expr::IndexAssign { location, .. }
            | Expr::Range { location, .. } => location,
        }
    }
}

impl Stmt {
    /// Get the source location of this statement
    pub fn location(&self) -> &SourceLocation {
        match self {
            Stmt::Val { location, .. }
            | Stmt::Var { location, .. }
            | Stmt::Fn { location, .. }
            | Stmt::Struct { location, .. }
            | Stmt::Print { location, .. }
            | Stmt::Expression { location, .. }
            | Stmt::Block { location, .. }
            | Stmt::If { location, .. }
            | Stmt::While { location, .. }
            | Stmt::Return { location, .. }
            | Stmt::ForIn { location, .. }
            | Stmt::Break { location }
            | Stmt::Continue { location } => location,
        }
    }
}
