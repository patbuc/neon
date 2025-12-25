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
    Range,
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
            Precedence::Comparison => Precedence::Range,
            Precedence::Range => Precedence::Term,
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

    fn parse_comma_separated_list<T, F>(
        &mut self,
        closing_token: TokenType,
        max_count: Option<usize>,
        max_count_error: &str,
        mut parse_element: F,
    ) -> Option<Vec<T>>
    where
        F: FnMut(&mut Self) -> Option<T>,
    {
        let mut items = Vec::new();

        self.skip_new_lines();
        if !self.check(closing_token.clone()) {
            loop {
                // Check max count if specified
                if let Some(max) = max_count {
                    if items.len() >= max {
                        self.report_error_at_current(max_count_error.to_string());
                    }
                }

                // Parse element using provided closure
                items.push(parse_element(self)?);
                self.skip_new_lines();
                if !self.match_token(TokenType::Comma) {
                    break;
                }
                self.skip_new_lines();

                // Support trailing comma
                if self.check(closing_token.clone()) {
                    break;
                }
            }
        }
        self.skip_new_lines();
        Some(items)
    }

    fn parse_expression_list(
        &mut self,
        closing_token: TokenType,
        max_count: Option<usize>,
        max_count_error: &str,
    ) -> Option<Vec<Expr>> {
        self.parse_comma_separated_list(closing_token, max_count, max_count_error, |parser| {
            parser.expression(false)
        })
    }

    fn parse_parameter_list(&mut self) -> Option<Vec<String>> {
        self.parse_comma_separated_list(
            TokenType::RightParen,
            Some(crate::common::constants::MAX_FUNCTION_PARAMS),
            "Can't have more than 255 parameters.",
            |parser| {
                if !parser.consume(TokenType::Identifier, "Expect parameter name.") {
                    return None;
                }
                Some(parser.previous_token.token.clone())
            },
        )
    }

    fn parse_map_entry_list(&mut self) -> Option<Vec<(Expr, Expr)>> {
        self.parse_comma_separated_list(TokenType::RightBrace, None, "", |parser| {
            let key = parser.expression(false)?;
            if !parser.consume(TokenType::Colon, "Expect ':' after map key.") {
                return None;
            }
            let value = parser.expression(false)?;
            Some((key, value))
        })
    }

    fn parse_arguments(&mut self) -> Option<Vec<Expr>> {
        self.parse_expression_list(
            TokenType::RightParen,
            Some(crate::common::constants::MAX_CALL_ARGUMENTS),
            "Can't have more than 255 arguments.",
        )
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
                | TokenType::Return
                | TokenType::Import
                | TokenType::Export => return,
                _ => {}
            }
            self.advance();
        }
    }

    // ===== Declarations =====

    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_token(TokenType::Import) {
            self.import_declaration()
        } else if self.match_token(TokenType::Export) {
            self.export_declaration()
        } else if self.match_token(TokenType::Val) {
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

    fn parse_variable_declaration(
        &mut self,
        is_mutable: bool,
        require_terminator: bool,
    ) -> Option<Stmt> {
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

        if require_terminator {
            let decl_type = if is_mutable { "variable" } else { "value" };
            self.consume_either(
                TokenType::NewLine,
                TokenType::Eof,
                &format!("Expecting '\\n' or '\\0' after {} declaration.", decl_type),
            );
        }

        Some(if is_mutable {
            Stmt::Var {
                name,
                initializer,
                location,
            }
        } else {
            Stmt::Val {
                name,
                initializer,
                location,
            }
        })
    }

    fn val_declaration(&mut self) -> Option<Stmt> {
        self.parse_variable_declaration(false, true)
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        self.parse_variable_declaration(true, true)
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

        let params = self.parse_parameter_list()?;
        if !self.consume(TokenType::RightParen, "Expect ')' after parameters.") {
            return None;
        }

        if !self.consume(TokenType::LeftBrace, "Expect '{' before function body.") {
            return None;
        }

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

    fn import_declaration(&mut self) -> Option<Stmt> {
        let location = self.current_location();

        // Accept either an identifier (for relative paths like ./module) or a string literal
        let module_path = if self.check(TokenType::String) {
            self.advance();
            // Strip quotes from string literal
            let token_value = &self.previous_token.token;
            token_value[1..token_value.len() - 1].to_string()
        } else if self.check(TokenType::Identifier) {
            self.advance();
            self.previous_token.token.clone()
        } else {
            self.report_error_at_current(
                "Expect module path (identifier or string) after 'import'.".to_string(),
            );
            return None;
        };

        self.consume_either(
            TokenType::NewLine,
            TokenType::Eof,
            "Expecting '\\n' or '\\0' after import declaration.",
        );

        Some(Stmt::Import {
            module_path,
            location,
        })
    }

    fn export_declaration(&mut self) -> Option<Stmt> {
        let location = self.current_location();

        // Parse the declaration to be exported (fn, val, var, struct)
        let declaration = if self.match_token(TokenType::Fn) {
            Box::new(self.fn_declaration()?)
        } else if self.match_token(TokenType::Val) {
            Box::new(self.val_declaration()?)
        } else if self.match_token(TokenType::Var) {
            Box::new(self.var_declaration()?)
        } else if self.match_token(TokenType::Struct) {
            Box::new(self.struct_declaration()?)
        } else {
            self.report_error_at_current(
                "Expect declaration after 'export' (fn, val, var, or struct).".to_string(),
            );
            return None;
        };

        Some(Stmt::Export {
            declaration,
            location,
        })
    }

    // ===== Statements =====

    fn statement(&mut self) -> Option<Stmt> {
        if self.match_token(TokenType::LeftBrace) {
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
            // Check for 'else if' syntax
            if self.check(TokenType::If) {
                self.advance();
                Some(Box::new(self.if_statement()?))
            } else {
                Some(Box::new(self.statement()?))
            }
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
            self.parse_variable_declaration(false, false)?
        } else if self.match_token(TokenType::Var) {
            self.parse_variable_declaration(true, false)?
        } else {
            self.report_error_at_current(
                "Expecting 'val' or 'var' in for loop initializer.".to_string(),
            );
            return None;
        };

        if !self.consume(
            TokenType::Semicolon,
            "Expecting ';' after loop initializer.",
        ) {
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
        let body_location = *body.location();

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

        let mut expr = match self.previous_token.token_type {
            TokenType::Number => self.number(),
            TokenType::String => self.string(),
            TokenType::InterpolatedString => self.interpolated_string(),
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

        while precedence <= self.get_precedence(&self.current_token.token_type) {
            self.advance();
            expr = match self.previous_token.token_type {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Star
                | TokenType::Slash
                | TokenType::SlashSlash
                | TokenType::Percent
                | TokenType::EqualEqual
                | TokenType::BangEqual
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual
                | TokenType::AndAnd
                | TokenType::OrOr => self.binary(expr),
                TokenType::DotDot | TokenType::DotDotEqual => self.range(expr),
                TokenType::LeftParen => self.call(expr),
                TokenType::Dot => self.dot(expr),
                TokenType::LeftBracket => self.index(expr),
                TokenType::PlusPlus | TokenType::MinusMinus => self.postfix(expr),
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
            TokenType::LeftParen
            | TokenType::Dot
            | TokenType::LeftBracket
            | TokenType::PlusPlus
            | TokenType::MinusMinus => Precedence::Call,
            TokenType::Star | TokenType::Slash | TokenType::SlashSlash | TokenType::Percent => {
                Precedence::Factor
            }
            TokenType::Plus | TokenType::Minus => Precedence::Term,
            TokenType::DotDot | TokenType::DotDotEqual => Precedence::Range,
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

    fn interpolated_string(&mut self) -> Option<Expr> {
        use crate::compiler::ast::InterpolationPart;

        let token_value = &self.previous_token.token;
        let location = self.current_location();

        let content = &token_value[1..token_value.len() - 1];

        let mut parts = Vec::new();
        let mut current_literal = String::new();
        let mut chars = content.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '$' && chars.peek() == Some(&'{') {
                chars.next();

                if !current_literal.is_empty() {
                    parts.push(InterpolationPart::Literal(current_literal.clone()));
                    current_literal.clear();
                }

                let mut expr_str = String::new();
                let mut brace_depth = 1;

                for ch in chars.by_ref() {
                    if ch == '{' {
                        brace_depth += 1;
                        expr_str.push(ch);
                    } else if ch == '}' {
                        brace_depth -= 1;
                        if brace_depth == 0 {
                            break;
                        }
                        expr_str.push(ch);
                    } else {
                        expr_str.push(ch);
                    }
                }

                let mut expr_parser = Parser::new(&expr_str);
                expr_parser.advance();
                if let Some(expr) = expr_parser.expression(true) {
                    parts.push(InterpolationPart::Expression(Box::new(expr)));
                } else {
                    parts.push(InterpolationPart::Literal(String::new()));
                }
            } else {
                current_literal.push(ch);
            }
        }

        if !current_literal.is_empty() {
            parts.push(InterpolationPart::Literal(current_literal));
        }

        Some(Expr::StringInterpolation { parts, location })
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
            TokenType::SlashSlash => BinaryOp::FloorDivide,
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

    fn range(&mut self, start: Expr) -> Option<Expr> {
        let operator_type = self.previous_token.token_type.clone();
        let location = self.current_location();

        let inclusive = operator_type == TokenType::DotDotEqual;
        let precedence = self.get_precedence(&operator_type).next();
        let end = Box::new(self.parse_precedence(precedence, false)?);

        Some(Expr::Range {
            start: Box::new(start),
            end,
            inclusive,
            location,
        })
    }

    fn call(&mut self, callee: Expr) -> Option<Expr> {
        let location = self.current_location();
        let arguments = self.parse_arguments()?;

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
            let arguments = self.parse_arguments()?;

            if !self.consume(TokenType::RightParen, "Expect ')' after arguments.") {
                return None;
            }

            // Convert obj.method(args) to Call { callee: GetField { object: obj, field: method }, arguments }
            let get_field_expr = Expr::GetField {
                object: Box::new(object),
                field,
                location,
            };

            Some(Expr::Call {
                callee: Box::new(get_field_expr),
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

    fn parse_map_entries(&mut self, first_entry: (Expr, Expr)) -> Option<Vec<(Expr, Expr)>> {
        let mut entries = vec![first_entry];
        self.skip_new_lines();

        if self.match_token(TokenType::Comma) {
            self.skip_new_lines();
            if !self.check(TokenType::RightBrace) {
                let rest = self.parse_map_entry_list()?;
                entries.extend(rest);
            }
        }

        Some(entries)
    }

    fn parse_set_elements(&mut self, first_element: Expr) -> Option<Vec<Expr>> {
        let mut elements = vec![first_element];
        self.skip_new_lines();

        if self.match_token(TokenType::Comma) {
            self.skip_new_lines();
            if !self.check(TokenType::RightBrace) {
                let rest = self.parse_expression_list(TokenType::RightBrace, None, "")?;
                elements.extend(rest);
            }
        }

        Some(elements)
    }

    fn brace_literal(&mut self) -> Option<Expr> {
        let location = self.current_location();

        self.skip_new_lines();

        if self.check(TokenType::RightBrace) {
            self.advance();
            return Some(Expr::MapLiteral {
                entries: Vec::new(),
                location,
            });
        }

        let first_expr = self.expression(false)?;

        self.skip_new_lines();

        if self.match_token(TokenType::Colon) {
            // Map literal
            let first_value = self.expression(false)?;
            let entries = self.parse_map_entries((first_expr, first_value))?;

            if !self.consume(TokenType::RightBrace, "Expect '}' after map entries.") {
                return None;
            }

            Some(Expr::MapLiteral { entries, location })
        } else {
            // Set literal
            let elements = self.parse_set_elements(first_expr)?;

            if !self.consume(TokenType::RightBrace, "Expect '}' after set elements.") {
                return None;
            }

            Some(Expr::SetLiteral { elements, location })
        }
    }

    fn array_literal(&mut self) -> Option<Expr> {
        let location = self.current_location();

        let elements = self.parse_expression_list(TokenType::RightBracket, None, "")?;

        if !self.consume(TokenType::RightBracket, "Expect ']' after array elements.") {
            return None;
        }

        Some(Expr::ArrayLiteral { elements, location })
    }

    fn index(&mut self, object: Expr) -> Option<Expr> {
        let location = self.current_location();

        let index = Box::new(self.expression(false)?);

        if !self.consume(TokenType::RightBracket, "Expect ']' after index.") {
            return None;
        }

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

    fn postfix(&self, operand: Expr) -> Option<Expr> {
        let operator_type = self.previous_token.token_type.clone();
        let location = self.current_location();

        match operator_type {
            TokenType::PlusPlus => Some(Expr::PostfixIncrement {
                operand: Box::new(operand),
                location,
            }),
            TokenType::MinusMinus => Some(Expr::PostfixDecrement {
                operand: Box::new(operand),
                location,
            }),
            _ => None,
        }
    }
}
