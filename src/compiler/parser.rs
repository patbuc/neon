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
        } else if self.match_token(TokenType::Return) {
            self.return_statement()
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
            TokenType::LeftBrace => self.map_literal(),
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

    fn map_literal(&mut self) -> Option<Expr> {
        let location = self.current_location();
        let mut entries = Vec::new();

        self.skip_new_lines();

        // Handle empty map: {}
        if self.check(TokenType::RightBrace) {
            self.advance();
            return Some(Expr::MapLiteral { entries, location });
        }

        // Parse key-value pairs
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

        if !self.consume(TokenType::RightBrace, "Expect '}' after map entries.") {
            return None;
        }

        Some(Expr::MapLiteral { entries, location })
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
