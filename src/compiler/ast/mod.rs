use crate::common::SourceLocation;

/// Reusable parameter struct for functions and methods
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParam {
    pub name: String,
    pub is_mutable: bool, // for mut self
}

/// Method definition within an impl block
#[derive(Debug, Clone, PartialEq)]
pub struct ImplMethod {
    pub name: String,
    pub params: Vec<FunctionParam>, // includes self/mut self as first param, or empty for static
    pub body: Vec<Stmt>,
    pub is_static: bool,   // true if no self parameter
    pub is_mutating: bool, // true if mut self
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
    FloorDivide,
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
    Literal(String),
    Expression(Box<Expr>),
}

/// Expression nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number {
        value: f64,
        location: SourceLocation,
    },
    String {
        value: String,
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
        location: SourceLocation,
    },
    Assign {
        name: String,
        value: Box<Expr>,
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
        location: SourceLocation,
    },
    GetField {
        object: Box<Expr>,
        field: String,
        location: SourceLocation,
    },
    SetField {
        object: Box<Expr>,
        field: String,
        value: Box<Expr>,
        location: SourceLocation,
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
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        inclusive: bool,
        location: SourceLocation,
    },
    PostfixIncrement {
        operand: Box<Expr>,
        location: SourceLocation,
    },
    PostfixDecrement {
        operand: Box<Expr>,
        location: SourceLocation,
    },
    Conditional {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
        location: SourceLocation,
    },
}

/// Statement nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Val {
        name: String,
        initializer: Option<Expr>,
        location: SourceLocation,
    },
    Var {
        name: String,
        initializer: Option<Expr>,
        location: SourceLocation,
    },
    Fn {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        location: SourceLocation,
    },
    Struct {
        name: String,
        fields: Vec<String>,
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
        value: Expr,
        location: SourceLocation,
    },
    ForIn {
        variable: String,
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
    Impl {
        struct_name: String,
        methods: Vec<ImplMethod>,
        location: SourceLocation,
    },
}

impl Expr {
    #[allow(dead_code)]
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
            | Expr::MapLiteral { location, .. }
            | Expr::ArrayLiteral { location, .. }
            | Expr::SetLiteral { location, .. }
            | Expr::Index { location, .. }
            | Expr::IndexAssign { location, .. }
            | Expr::Range { location, .. }
            | Expr::PostfixIncrement { location, .. }
            | Expr::PostfixDecrement { location, .. }
            | Expr::Conditional { location, .. } => location,
        }
    }
}

impl Stmt {
    pub fn location(&self) -> &SourceLocation {
        match self {
            Stmt::Val { location, .. }
            | Stmt::Var { location, .. }
            | Stmt::Fn { location, .. }
            | Stmt::Struct { location, .. }
            | Stmt::Expression { location, .. }
            | Stmt::Block { location, .. }
            | Stmt::If { location, .. }
            | Stmt::While { location, .. }
            | Stmt::Return { location, .. }
            | Stmt::ForIn { location, .. }
            | Stmt::Break { location }
            | Stmt::Continue { location }
            | Stmt::Impl { location, .. } => location,
        }
    }
}
