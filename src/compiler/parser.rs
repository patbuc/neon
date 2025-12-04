use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};
use crate::common::SourceLocation;
/// AST-building parser for the multi-pass compiler
/// This parser builds an Abstract Syntax Tree instead of emitting bytecode directly
use crate::compiler::ast::{BinaryOp, Expr, Stmt, UnaryOp};
use crate::compiler::token::TokenType;
use crate::compiler::{Scanner, Token};

/// AST Parser that builds an Abstract Syntax Tree
pub struct Parser {
    scanner: Scanner,
    previous_token: Token,
    current_token: Token,
    errors: Vec<CompilationError>,
    panic_mode: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    fn next(self) -> Precedence {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary,
        }
    }
}

impl Parser {
    pub fn new(source: &str) -> Self {
        Parser {
            scanner: Scanner::new(source),
            previous_token: Token::default(),
            current_token: Token::default(),
            errors: Vec::new(),
            panic_mode: false,
        }
    }

    /// Parse the source code into an AST
    pub fn parse(&mut self) -> CompilationResult<Vec<Stmt>> {
        let mut statements = Vec::new();

        self.advance();

        while !self.match_token(TokenType::Eof) {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
            if self.panic_mode {
                self.synchronize();
            }
        }

        if self.errors.is_empty() {
            Ok(statements)
        } else {
            Err(self.errors.clone())
        }
    }

    // ===== Token Management =====

    fn advance(&mut self) {
        std::mem::swap(&mut self.previous_token, &mut self.current_token);
        loop {
            self.current_token = self.scanner.scan_token();
            if self.current_token.token_type != TokenType::Error {
                break;
            }
            self.report_error_at_current(self.current_token.token.clone());
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            return false;
        }
        self.advance();
        true
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> bool {
        if self.current_token.token_type == token_type {
            self.advance();
            return true;
        }
        self.report_error_at_current(message.to_string());
        false
    }

    fn consume_either(&mut self, token_type_1: TokenType, token_type_2: TokenType, message: &str) {
        if self.current_token.token_type == token_type_1
            || self.current_token.token_type == token_type_2
        {
            self.advance();
            return;
        }
        self.report_error_at_current(message.to_string());
    }

    fn skip_new_lines(&mut self) {
        while self.check(TokenType::NewLine) {
            self.advance();
        }
    }

    fn current_location(&self) -> SourceLocation {
        SourceLocation {
            offset: self.previous_token.offset,
            line: self.previous_token.line,
            column: self.previous_token.column,
        }
    }

    // ===== Error Handling =====

    fn report_error_at_current(&mut self, message: String) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        let location = SourceLocation {
            offset: self.current_token.offset,
            line: self.current_token.line,
            column: self.current_token.column,
        };

        self.errors.push(CompilationError::new(
            CompilationPhase::Parse,
            CompilationErrorKind::UnexpectedToken,
            message,
            location,
        ));
    }

    fn synchronize(&mut self) {
        self.panic_mode = false;
        loop {
            if self.previous_token.token_type == TokenType::NewLine
                || self.previous_token.token_type == TokenType::Eof
            {
                return;
            }
            match self.current_token.token_type {
                TokenType::Class
                | TokenType::Fn
                | TokenType::Struct
                | TokenType::Val
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
            self.advance();
        }
    }

    // ===== Declarations =====

    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_token(TokenType::Val) {
            self.val_declaration()
        } else if self.match_token(TokenType::Var) {
            self.var_declaration()
        } else if self.match_token(TokenType::Fn) {
            self.fn_declaration()
        } else if self.match_token(TokenType::Struct) {
            self.struct_declaration()
        } else {
            self.statement()
        }
    }

    fn val_declaration(&mut self) -> Option<Stmt> {
        if !self.consume(TokenType::Identifier, "Expecting variable name.") {
            return None;
        }
        let name = self.previous_token.token.clone();
        let location = self.current_location();

        let initializer = if self.match_token(TokenType::Equal) {
            self.expression(false)
        } else {
            None
        };

        self.consume_either(
            TokenType::NewLine,
            TokenType::Eof,
            "Expecting '\\n' or '\\0' after value declaration.",
        );

        Some(Stmt::Val {
            name,
            initializer,
            location,
        })
    }

    /// Parses a val (immutable variable) declaration without requiring a newline terminator.
    ///
    /// This helper method is used in for loop initialization clauses where the declaration
    /// is followed by a semicolon instead of a newline.
    ///
    /// Syntax: `val identifier [= expression]`
    fn val_declaration_no_terminator(&mut self) -> Option<Stmt> {
        if !self.consume(TokenType::Identifier, "Expecting variable name.") {
            return None;
        }
        let name = self.previous_token.token.clone();
        let location = self.current_location();

        let initializer = if self.match_token(TokenType::Equal) {
            self.expression(false)
        } else {
            None
        };

        Some(Stmt::Val {
            name,
            initializer,
            location,
        })
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        if !self.consume(TokenType::Identifier, "Expecting variable name.") {
            return None;
        }
        let name = self.previous_token.token.clone();
        let location = self.current_location();

        let initializer = if self.match_token(TokenType::Equal) {
            self.expression(false)
        } else {
            None
        };

        self.consume_either(
            TokenType::NewLine,
            TokenType::Eof,
            "Expecting '\\n' or '\\0' after variable declaration.",
        );

        Some(Stmt::Var {
            name,
            initializer,
            location,
        })
    }

    /// Parses a var (mutable variable) declaration without requiring a newline terminator.
    ///
    /// This helper method is used in for loop initialization clauses where the declaration
    /// is followed by a semicolon instead of a newline.
    ///
    /// Syntax: `var identifier [= expression]`
    fn var_declaration_no_terminator(&mut self) -> Option<Stmt> {
        if !self.consume(TokenType::Identifier, "Expecting variable name.") {
            return None;
        }
        let name = self.previous_token.token.clone();
        let location = self.current_location();

        let initializer = if self.match_token(TokenType::Equal) {
            self.expression(false)
        } else {
            None
        };

        Some(Stmt::Var {
            name,
            initializer,
            location,
        })
    }

    fn fn_declaration(&mut self) -> Option<Stmt> {
        if !self.consume(TokenType::Identifier, "Expect function name.") {
            return None;
        }
        let name = self.previous_token.token.clone();
        let location = self.current_location();

        if !self.consume(TokenType::LeftParen, "Expect '(' after function name.") {
            return None;
        }

        // Parse parameters
        let mut params = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if params.len() >= crate::common::constants::MAX_FUNCTION_PARAMS {
                    self.report_error_at_current(
                        "Can't have more than 255 parameters.".to_string(),
                    );
                }

                if !self.consume(TokenType::Identifier, "Expect parameter name.") {
                    return None;
                }
                params.push(self.previous_token.token.clone());

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        if !self.consume(TokenType::RightParen, "Expect ')' after parameters.") {
            return None;
        }

        if !self.consume(TokenType::LeftBrace, "Expect '{' before function body.") {
            return None;
        }

        // Parse function body
        let body = self.block_statements()?;

        Some(Stmt::Fn {
            name,
            params,
            body,
            location,
        })
    }

    fn struct_declaration(&mut self) -> Option<Stmt> {
        if !self.consume(TokenType::Identifier, "Expect struct name.") {
            return None;
        }
        let name = self.previous_token.token.clone();
        let location = self.current_location();

        if !self.consume(TokenType::LeftBrace, "Expect '{' after struct name.") {
            return None;
        }

        // Parse field list
        let mut fields = Vec::new();
        self.skip_new_lines();

        if !self.check(TokenType::RightBrace) {
            loop {
                if !self.consume(TokenType::Identifier, "Expect field name.") {
                    break;
                }
                fields.push(self.previous_token.token.clone());
                self.skip_new_lines();
                if self.check(TokenType::RightBrace) {
                    break;
                }
            }
        }

        if !self.consume(TokenType::RightBrace, "Expect '}' after struct fields.") {
            return None;
        }
        self.consume_either(
            TokenType::NewLine,
            TokenType::Eof,
            "Expecting '\\n' or '\\0' after struct declaration.",
        );

        Some(Stmt::Struct {
            name,
            fields,
            location,
        })
    }

    // ===== Statements =====

    fn statement(&mut self) -> Option<Stmt> {
        if self.match_token(TokenType::Print) {
            self.print_statement()
        } else if self.match_token(TokenType::LeftBrace) {
            let location = self.current_location();
            let statements = self.block_statements()?;
            Some(Stmt::Block {
                statements,
                location,
            })
        } else if self.match_token(TokenType::If) {
            self.if_statement()
        } else if self.match_token(TokenType::While) {
            self.while_statement()
        } else if self.match_token(TokenType::For) {
            self.for_statement()
        } else if self.match_token(TokenType::Return) {
            self.return_statement()
        } else if self.match_token(TokenType::Break) {
            self.break_statement()
        } else if self.match_token(TokenType::Continue) {
            self.continue_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Option<Stmt> {
        let location = self.current_location();
        let expr = self.expression(false)?;
        self.consume_either(
            TokenType::NewLine,
            TokenType::Eof,
            "Expecting '\\n' or '\\0' at end of statement.",
        );
        Some(Stmt::Print { expr, location })
    }

    fn expression_statement(&mut self) -> Option<Stmt> {
        let location = self.current_location();
        let expr = self.expression(false)?;
        self.consume_either(
            TokenType::NewLine,
            TokenType::Eof,
            "Expecting '\\n' or '\\0' at end of expression.",
        );
        Some(Stmt::Expression { expr, location })
    }

    fn block_statements(&mut self) -> Option<Vec<Stmt>> {
        let mut statements = Vec::new();
        self.skip_new_lines();

        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }

        if !self.consume(TokenType::RightBrace, "Expect '}' after block.") {
            return None;
        }

        if !self.check(TokenType::Else) {
            self.consume_either(
                TokenType::NewLine,
                TokenType::Eof,
                "Expecting '\\n' or '\\0' at end of block.",
            );
        }

        Some(statements)
    }

    fn if_statement(&mut self) -> Option<Stmt> {
        let location = self.current_location();

        if !self.consume(TokenType::LeftParen, "Expecting '(' after 'if'.") {
            return None;
        }

        let condition = self.expression(false)?;

        if !self.consume(TokenType::RightParen, "Expecting ')' after condition.") {
            return None;
        }

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token(TokenType::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Some(Stmt::If {
            condition,
            then_branch,
            else_branch,
            location,
        })
    }

    fn while_statement(&mut self) -> Option<Stmt> {
        let location = self.current_location();

        if !self.consume(TokenType::LeftParen, "Expecting '(' after 'while'.") {
            return None;
        }

        let condition = self.expression(false)?;

        if !self.consume(TokenType::RightParen, "Expecting ')' after condition.") {
            return None;
        }

        let body = Box::new(self.statement()?);

        Some(Stmt::While {
            condition,
            body,
            location,
        })
    }

    /// Parses for loops: supports both C-style and for-in loops
    ///
    /// # Syntax Options
    /// 1. C-style for loop:
    ///    ```neon
    ///    for (val|var identifier = expression; condition; increment) statement
    ///    ```
    ///
    /// 2. For-in loop:
    ///    ```neon
    ///    for (identifier in collection) statement
    ///    ```
    ///
    /// # C-style For Loop Desugaring
    /// The C-style for loop is transformed into a while loop:
    ///
    /// ```neon
    /// for (val i = 0; i < 10; i = i + 1) {
    ///     print i
    /// }
    /// ```
    ///
    /// Becomes:
    ///
    /// ```neon
    /// {
    ///     val i = 0
    ///     while (i < 10) {
    ///         print i
    ///         i = i + 1
    ///     }
    /// }
    /// ```
    ///
    /// # For-in Loop Structure
    /// For-in loops iterate over collections (arrays, maps, sets):
    ///
    /// ```neon
    /// for (item in collection) {
    ///     print item
    /// }
    /// ```
    ///
    /// - The loop variable is always immutable (implicit val)
    /// - Maps iterate over keys (use map[key] to access values)
    /// - Arrays iterate over elements
    /// - Sets iterate over elements (converted to array internally)
    fn for_statement(&mut self) -> Option<Stmt> {
        let location = self.current_location();

        if !self.consume(TokenType::LeftParen, "Expecting '(' after 'for'.") {
            return None;
        }

        // Look ahead to determine if this is a for-in loop or C-style for loop
        // For-in: for (identifier in collection)
        // C-style: for (val/var identifier = ...)

        // Check if we have an identifier followed by 'in' keyword
        if self.check(TokenType::Identifier) {
            // Save the current position in case we need to backtrack
            let identifier = self.current_token.token.clone();
            self.advance(); // consume identifier

            // Check for 'in' keyword
            if self.match_token(TokenType::In) {
                // This is a for-in loop
                return self.for_in_loop(identifier, location);
            } else {
                // This is not a for-in loop, report error
                // User wrote: for (identifier ...
                // Expected either: for (identifier in ...) or for (val/var identifier ...)
                self.report_error_at_current(
                    "Expecting 'in' after identifier in for-in loop, or 'val'/'var' for C-style for loop.".to_string()
                );
                return None;
            }
        }

        // Not a for-in loop, parse as C-style for loop
        // Parse init clause - must be val or var declaration
        let init = if self.match_token(TokenType::Val) {
            self.val_declaration_no_terminator()?
        } else if self.match_token(TokenType::Var) {
            self.var_declaration_no_terminator()?
        } else {
            self.report_error_at_current("Expecting 'val' or 'var' in for loop initializer.".to_string());
            return None;
        };

        if !self.consume(TokenType::Semicolon, "Expecting ';' after loop initializer.") {
            return None;
        }

        // Parse condition expression
        let condition = self.expression(false)?;

        if !self.consume(TokenType::Semicolon, "Expecting ';' after loop condition.") {
            return None;
        }

        // Parse increment - any expression is allowed
        let increment_expr = self.expression(false)?;
        let increment_location = self.current_location();
        let increment = Stmt::Expression {
            expr: increment_expr,
            location: increment_location,
        };

        if !self.consume(TokenType::RightParen, "Expecting ')' after for clauses.") {
            return None;
        }

        // Parse loop body
        let body = self.statement()?;
        let body_location = body.location().clone();

        // Desugar to: Block { init, While { condition, Block { body, increment } } }
        let while_body = Stmt::Block {
            statements: vec![body, increment],
            location: body_location,
        };

        let while_loop = Stmt::While {
            condition,
            body: Box::new(while_body),
            location: body_location,
        };

        Some(Stmt::Block {
            statements: vec![init, while_loop],
            location,
        })
    }

    /// Parses a for-in loop after detecting 'identifier in' pattern
    ///
    /// # Syntax
    /// ```neon
    /// for (variable in collection) statement
    /// ```
    ///
    /// # Behavior
    /// - Loop variable is always immutable (implicit val)
    /// - Arrays: iterate over elements
    /// - Maps: iterate over keys
    /// - Sets: iterate over elements
    fn for_in_loop(&mut self, variable: String, location: SourceLocation) -> Option<Stmt> {
        // Parse collection expression
        let collection = self.expression(false)?;

        if !self.consume(TokenType::RightParen, "Expecting ')' after for-in clauses.") {
            return None;
        }

        // Parse loop body
        let body = Box::new(self.statement()?);

        Some(Stmt::ForIn {
            variable,
            collection,
            body,
            location,
        })
    }

    fn return_statement(&mut self) -> Option<Stmt> {
        let location = self.current_location();
        let value = self.expression(false)?;
        self.consume_either(
            TokenType::NewLine,
            TokenType::Eof,
            "Expecting '\\n' or '\\0' at end of statement.",
        );
        Some(Stmt::Return { value, location })
    }

    fn break_statement(&mut self) -> Option<Stmt> {
        let location = self.current_location();
        self.consume_either(
            TokenType::NewLine,
            TokenType::Eof,
            "Expecting '\\n' or '\\0' after 'break'.",
        );
        Some(Stmt::Break { location })
    }

    fn continue_statement(&mut self) -> Option<Stmt> {
        let location = self.current_location();
        self.consume_either(
            TokenType::NewLine,
            TokenType::Eof,
            "Expecting '\\n' or '\\0' after 'continue'.",
        );
        Some(Stmt::Continue { location })
    }

    // ===== Expressions =====

    fn expression(&mut self, skip_new_lines: bool) -> Option<Expr> {
        self.parse_precedence(Precedence::Assignment, skip_new_lines)
    }

    fn parse_precedence(&mut self, precedence: Precedence, skip_new_lines: bool) -> Option<Expr> {
        if skip_new_lines {
            self.skip_new_lines();
        }

        self.advance();

        // Get prefix expression
        let mut expr = match self.previous_token.token_type {
            TokenType::Number => self.number(),
            TokenType::String => self.string(),
            TokenType::True | TokenType::False | TokenType::Nil => self.literal(),
            TokenType::LeftParen => self.grouping(),
            TokenType::Minus | TokenType::Bang => self.unary(),
            TokenType::Identifier => self.variable(),
            TokenType::LeftBrace => self.brace_literal(),
            TokenType::LeftBracket => self.array_literal(),
            _ => {
                self.report_error_at_current("Expect expression".to_string());
                return None;
            }
        }?;

        // Parse infix expressions
        while precedence <= self.get_precedence(&self.current_token.token_type) {
            self.advance();
            expr = match self.previous_token.token_type {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Star
                | TokenType::Slash
                | TokenType::Percent
                | TokenType::EqualEqual
                | TokenType::BangEqual
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual
                | TokenType::AndAnd
                | TokenType::OrOr => self.binary(expr),
                TokenType::LeftParen => self.call(expr),
                TokenType::Dot => self.dot(expr),
                TokenType::LeftBracket => self.index(expr),
                _ => {
                    return Some(expr);
                }
            }?;
        }

        if skip_new_lines {
            self.skip_new_lines();
        }

        Some(expr)
    }

    fn get_precedence(&self, token_type: &TokenType) -> Precedence {
        match token_type {
            TokenType::LeftParen | TokenType::Dot | TokenType::LeftBracket => Precedence::Call,
            TokenType::Star | TokenType::Slash | TokenType::Percent => Precedence::Factor,
            TokenType::Plus | TokenType::Minus => Precedence::Term,
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => Precedence::Comparison,
            TokenType::EqualEqual | TokenType::BangEqual => Precedence::Equality,
            TokenType::AndAnd => Precedence::And,
            TokenType::OrOr => Precedence::Or,
            _ => Precedence::None,
        }
    }

    // ===== Primary Expressions =====

    fn number(&self) -> Option<Expr> {
        let value = self.previous_token.token.parse::<f64>().ok()?;
        let location = self.current_location();
        Some(Expr::Number { value, location })
    }

    fn string(&self) -> Option<Expr> {
        let token_value = &self.previous_token.token;
        let value = token_value[1..token_value.len() - 1].to_string();
        let location = self.current_location();
        Some(Expr::String { value, location })
    }

    fn literal(&self) -> Option<Expr> {
        let location = self.current_location();
        match self.previous_token.token_type {
            TokenType::True => Some(Expr::Boolean {
                value: true,
                location,
            }),
            TokenType::False => Some(Expr::Boolean {
                value: false,
                location,
            }),
            TokenType::Nil => Some(Expr::Nil { location }),
            _ => None,
        }
    }

    fn grouping(&mut self) -> Option<Expr> {
        let expr = Box::new(self.expression(true)?);
        if !self.consume(TokenType::RightParen, "Expect ')' after expression") {
            return None;
        }
        let location = self.current_location();
        Some(Expr::Grouping { expr, location })
    }

    fn variable(&mut self) -> Option<Expr> {
        let name = self.previous_token.token.clone();
        let location = self.current_location();

        // Check for assignment
        if self.match_token(TokenType::Equal) {
            let value = Box::new(self.expression(false)?);
            Some(Expr::Assign {
                name,
                value,
                location,
            })
        } else {
            Some(Expr::Variable { name, location })
        }
    }

    // ===== Binary & Unary =====

    fn binary(&mut self, left: Expr) -> Option<Expr> {
        let operator_type = self.previous_token.token_type.clone();
        let location = self.current_location();

        let precedence = self.get_precedence(&operator_type).next();
        let right = Box::new(self.parse_precedence(precedence, false)?);

        let operator = match operator_type {
            TokenType::Plus => BinaryOp::Add,
            TokenType::Minus => BinaryOp::Subtract,
            TokenType::Star => BinaryOp::Multiply,
            TokenType::Slash => BinaryOp::Divide,
            TokenType::Percent => BinaryOp::Modulo,
            TokenType::EqualEqual => BinaryOp::Equal,
            TokenType::BangEqual => BinaryOp::NotEqual,
            TokenType::Greater => BinaryOp::Greater,
            TokenType::GreaterEqual => BinaryOp::GreaterEqual,
            TokenType::Less => BinaryOp::Less,
            TokenType::LessEqual => BinaryOp::LessEqual,
            TokenType::AndAnd => BinaryOp::And,
            TokenType::OrOr => BinaryOp::Or,
            _ => return None,
        };

        Some(Expr::Binary {
            left: Box::new(left),
            operator,
            right,
            location,
        })
    }

    fn unary(&mut self) -> Option<Expr> {
        let operator_type = self.previous_token.token_type.clone();
        let location = self.current_location();

        let operand = Box::new(self.parse_precedence(Precedence::Unary, false)?);

        let operator = match operator_type {
            TokenType::Minus => UnaryOp::Negate,
            TokenType::Bang => UnaryOp::Not,
            _ => return None,
        };

        Some(Expr::Unary {
            operator,
            operand,
            location,
        })
    }

    fn call(&mut self, callee: Expr) -> Option<Expr> {
        let location = self.current_location();
        let mut arguments = Vec::new();

        self.skip_new_lines();
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= crate::common::constants::MAX_CALL_ARGUMENTS {
                    self.report_error_at_current("Can't have more than 255 arguments.".to_string());
                }
                arguments.push(self.expression(false)?);
                if !self.match_token(TokenType::Comma) {
                    break;
                }
                self.skip_new_lines();
            }
        }
        self.skip_new_lines();

        if !self.consume(TokenType::RightParen, "Expect ')' after arguments.") {
            return None;
        }

        Some(Expr::Call {
            callee: Box::new(callee),
            arguments,
            location,
        })
    }

    fn dot(&mut self, object: Expr) -> Option<Expr> {
        let location = self.current_location();

        if !self.consume(TokenType::Identifier, "Expect field name after '.'.") {
            return None;
        }
        let field = self.previous_token.token.clone();

        // Check if this is a method call: obj.method(args)
        if self.check(TokenType::LeftParen) {
            self.advance(); // consume '('
            let method_location = self.current_location();
            let mut arguments = Vec::new();

            self.skip_new_lines();
            if !self.check(TokenType::RightParen) {
                loop {
                    if arguments.len() >= crate::common::constants::MAX_CALL_ARGUMENTS {
                        self.report_error_at_current("Can't have more than 255 arguments.".to_string());
                    }
                    arguments.push(self.expression(false)?);
                    if !self.match_token(TokenType::Comma) {
                        break;
                    }
                    self.skip_new_lines();
                }
            }
            self.skip_new_lines();

            if !self.consume(TokenType::RightParen, "Expect ')' after arguments.") {
                return None;
            }

            Some(Expr::MethodCall {
                object: Box::new(object),
                method: field,
                arguments,
                location: method_location,
            })
        } else if self.match_token(TokenType::Equal) {
            let value = Box::new(self.expression(false)?);
            Some(Expr::SetField {
                object: Box::new(object),
                field,
                value,
                location,
            })
        } else {
            Some(Expr::GetField {
                object: Box::new(object),
                field,
                location,
            })
        }
    }

    fn brace_literal(&mut self) -> Option<Expr> {
        let location = self.current_location();

        self.skip_new_lines();

        // Handle empty braces: {} - treat as empty map (consistent with Python, JavaScript, etc.)
        if self.check(TokenType::RightBrace) {
            self.advance();
            return Some(Expr::MapLiteral {
                entries: Vec::new(),
                location,
            });
        }

        // Parse first expression to determine if this is a set or map
        let first_expr = self.expression(false)?;

        self.skip_new_lines();

        // Check if next token is a colon (map) or comma/closing brace (set)
        if self.match_token(TokenType::Colon) {
            // This is a map literal: {key: value, ...}
            let mut entries = Vec::new();

            // Parse first value
            let first_value = self.expression(false)?;
            entries.push((first_expr, first_value));

            self.skip_new_lines();

            // Parse remaining key-value pairs
            if self.match_token(TokenType::Comma) {
                self.skip_new_lines();

                // Allow trailing comma
                if !self.check(TokenType::RightBrace) {
                    loop {
                        // Parse key
                        let key = self.expression(false)?;

                        // Expect colon
                        if !self.consume(TokenType::Colon, "Expect ':' after map key.") {
                            return None;
                        }

                        // Parse value
                        let value = self.expression(false)?;

                        entries.push((key, value));

                        self.skip_new_lines();

                        // Check for comma or end of map
                        if !self.match_token(TokenType::Comma) {
                            break;
                        }

                        self.skip_new_lines();

                        // Allow trailing comma
                        if self.check(TokenType::RightBrace) {
                            break;
                        }
                    }
                }
            }

            if !self.consume(TokenType::RightBrace, "Expect '}' after map entries.") {
                return None;
            }

            Some(Expr::MapLiteral { entries, location })
        } else {
            // This is a set literal: {elem1, elem2, ...}
            let mut elements = Vec::new();
            elements.push(first_expr);

            self.skip_new_lines();

            // Parse remaining elements
            if self.match_token(TokenType::Comma) {
                self.skip_new_lines();

                // Allow trailing comma
                if !self.check(TokenType::RightBrace) {
                    loop {
                        // Parse element
                        let element = self.expression(false)?;
                        elements.push(element);

                        self.skip_new_lines();

                        // Check for comma or end of set
                        if !self.match_token(TokenType::Comma) {
                            break;
                        }

                        self.skip_new_lines();

                        // Allow trailing comma
                        if self.check(TokenType::RightBrace) {
                            break;
                        }
                    }
                }
            }

            if !self.consume(TokenType::RightBrace, "Expect '}' after set elements.") {
                return None;
            }

            Some(Expr::SetLiteral { elements, location })
        }
    }

    fn array_literal(&mut self) -> Option<Expr> {
        let location = self.current_location();
        let mut elements = Vec::new();

        self.skip_new_lines();

        // Handle empty array: []
        if self.check(TokenType::RightBracket) {
            self.advance();
            return Some(Expr::ArrayLiteral { elements, location });
        }

        // Parse comma-separated elements
        loop {
            // Parse element expression
            let element = self.expression(false)?;
            elements.push(element);

            self.skip_new_lines();

            // Check for comma or end of array
            if !self.match_token(TokenType::Comma) {
                break;
            }

            self.skip_new_lines();

            // Allow trailing comma
            if self.check(TokenType::RightBracket) {
                break;
            }
        }

        if !self.consume(TokenType::RightBracket, "Expect ']' after array elements.") {
            return None;
        }

        Some(Expr::ArrayLiteral { elements, location })
    }

    fn index(&mut self, object: Expr) -> Option<Expr> {
        let location = self.current_location();

        // Parse index expression
        let index = Box::new(self.expression(false)?);

        if !self.consume(TokenType::RightBracket, "Expect ']' after index.") {
            return None;
        }

        // Check if this is an index assignment: expr[index] = value
        if self.match_token(TokenType::Equal) {
            let value = Box::new(self.expression(false)?);
            Some(Expr::IndexAssign {
                object: Box::new(object),
                index,
                value,
                location,
            })
        } else {
            Some(Expr::Index {
                object: Box::new(object),
                index,
                location,
            })
        }
    }
}
