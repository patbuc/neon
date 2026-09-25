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
    Ternary,
    Or,
    And,
    Equality,
    Comparison,
    BitwiseOr,  // |
    BitwiseXor, // ^
    BitwiseAnd, // &
    Shift,      // << >>
    Range,
    Term,
    Factor,
    Unary,
    Exponent, // **
    Call,
    Primary,
}

impl Precedence {
    fn next(self) -> Precedence {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Ternary,
            Precedence::Ternary => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::BitwiseOr,
            Precedence::BitwiseOr => Precedence::BitwiseXor,
            Precedence::BitwiseXor => Precedence::BitwiseAnd,
            Precedence::BitwiseAnd => Precedence::Shift,
            Precedence::Shift => Precedence::Range,
            Precedence::Range => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Exponent,
            Precedence::Exponent => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary,
        }
    }
}

impl Parser {
    pub fn new(source: &str) -> Self {
        Parser::new_at(source, 1, 1, 0)
    }

    /// Like `new`, but starts counting position at the given line/column/offset.
    fn new_at(source: &str, line: u32, column: u32, offset: usize) -> Self {
        Parser {
            scanner: Scanner::new_at(source, line, column, offset),
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

    fn consume_statement_end(&mut self, message: &str) {
        if self.check(TokenType::RightBrace) {
            return;
        }
        if self.check(TokenType::NewLine) || self.check(TokenType::Eof) {
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
            parser.skip_new_lines();
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
        let location = SourceLocation {
            offset: self.current_token.offset,
            line: self.current_token.line,
            column: self.current_token.column,
        };
        self.report_error(location, message);
    }

    fn report_error(&mut self, location: SourceLocation, message: String) {
        self.record_error(CompilationError::new(
            CompilationPhase::Parse,
            CompilationErrorKind::UnexpectedToken,
            message,
            location,
        ));
    }

    fn report_error_at_previous(&mut self, message: String) {
        let location = self.current_location();
        self.report_error(location, message);
    }

    /// Like `report_error`, but for an error a sub-parser already built.
    fn record_error(&mut self, error: CompilationError) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        self.errors.push(error);
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
                TokenType::Fn
                | TokenType::Struct
                | TokenType::Impl
                | TokenType::Val
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
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
        } else if self.match_token(TokenType::Impl) {
            self.impl_declaration()
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
            self.skip_new_lines();
            Some(self.expression(false)?)
        } else {
            None
        };

        if require_terminator {
            let decl_type = if is_mutable { "variable" } else { "value" };
            self.consume_statement_end(&format!(
                "Expecting '\\n' or '\\0' after {} declaration.",
                decl_type
            ));
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
        self.consume_statement_end("Expecting '\\n' or '\\0' after struct declaration.");

        Some(Stmt::Struct {
            name,
            fields,
            location,
        })
    }

    fn impl_declaration(&mut self) -> Option<Stmt> {
        if !self.consume(TokenType::Identifier, "Expect type name.") {
            return None;
        }
        let type_name = self.previous_token.token.clone();
        let location = self.current_location();

        if !self.consume(TokenType::LeftBrace, "Expect '{' after type name.") {
            return None;
        }

        let mut methods = Vec::new();
        self.skip_new_lines();

        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            if !self.consume(TokenType::Fn, "Expect method declaration.") {
                while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
                    self.advance();
                }
                break;
            }
            let method = self.fn_declaration()?;
            methods.push(method);
            self.skip_new_lines();
        }

        if !self.consume(TokenType::RightBrace, "Expect '}' after impl body.") {
            return None;
        }
        self.consume_statement_end("Expecting '\\n' or '\\0' after impl declaration.");

        Some(Stmt::Impl {
            type_name,
            methods,
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
        self.consume_statement_end("Expecting '\\n' or '\\0' at end of expression.");
        Some(Stmt::Expression { expr, location })
    }

    /// Parses the statements of a `{ ... }` body up to and including the
    /// closing brace, without requiring a statement terminator after it.
    /// Shared by block statements, function declarations and lambda
    /// expressions, whose surrounding context decides what may follow.
    fn parse_block_body(&mut self) -> Option<Vec<Stmt>> {
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

        Some(statements)
    }

    fn block_statements(&mut self) -> Option<Vec<Stmt>> {
        let statements = self.parse_block_body()?;

        if !self.check(TokenType::Else) {
            self.consume_statement_end("Expecting '\\n' or '\\0' at end of block.");
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
        let increment = self.expression(false)?;

        if !self.consume(TokenType::RightParen, "Expecting ')' after for clauses.") {
            return None;
        }

        // Parse loop body
        let body = self.statement()?;

        Some(Stmt::For {
            initializer: Box::new(init),
            condition,
            increment,
            body: Box::new(body),
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
        self.consume_statement_end("Expecting '\\n' or '\\0' at end of statement.");
        Some(Stmt::Return { value, location })
    }

    fn break_statement(&mut self) -> Option<Stmt> {
        let location = self.current_location();
        self.consume_statement_end("Expecting '\\n' or '\\0' after 'break'.");
        Some(Stmt::Break { location })
    }

    fn continue_statement(&mut self) -> Option<Stmt> {
        let location = self.current_location();
        self.consume_statement_end("Expecting '\\n' or '\\0' after 'continue'.");
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

        let can_assign = precedence <= Precedence::Assignment;

        let mut expr = match self.previous_token.token_type {
            TokenType::Number => self.number(),
            TokenType::String => self.string(),
            TokenType::InterpolatedString => self.interpolated_string(),
            TokenType::True | TokenType::False | TokenType::Nil => self.literal(),
            TokenType::LeftParen => self.grouping(),
            TokenType::Minus | TokenType::Bang | TokenType::Tilde => self.unary(),
            TokenType::Identifier => self.variable(can_assign),
            TokenType::LeftBrace => self.brace_literal(),
            TokenType::HashLeftBrace => self.set_literal(),
            TokenType::LeftBracket => self.array_literal(),
            TokenType::Fn => self.lambda(),
            _ => {
                self.report_error_at_previous("Expect expression".to_string());
                return None;
            }
        }?;

        while precedence <= self.get_precedence(&self.current_token.token_type) {
            self.advance();
            expr = match self.previous_token.token_type {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Star
                | TokenType::StarStar
                | TokenType::Slash
                | TokenType::Percent
                | TokenType::EqualEqual
                | TokenType::BangEqual
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual
                | TokenType::AndAnd
                | TokenType::OrOr
                | TokenType::Ampersand
                | TokenType::Pipe
                | TokenType::Caret
                | TokenType::LessLess
                | TokenType::GreaterGreater => self.binary(expr),
                TokenType::DotDot | TokenType::DotDotEqual => self.range(expr),
                TokenType::LeftParen => self.call(expr),
                TokenType::Dot => self.dot(expr, can_assign),
                TokenType::LeftBracket => self.index(expr, can_assign),
                TokenType::PlusPlus | TokenType::MinusMinus => self.postfix(expr),
                TokenType::Question => self.ternary(expr),
                _ => {
                    return Some(expr);
                }
            }?;
        }

        if can_assign && self.match_token(TokenType::Equal) {
            self.report_error_at_previous("Invalid assignment target.".to_string());
            return None;
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
            TokenType::StarStar => Precedence::Exponent,
            TokenType::Star | TokenType::Slash | TokenType::Percent => Precedence::Factor,
            TokenType::Plus | TokenType::Minus => Precedence::Term,
            TokenType::LessLess | TokenType::GreaterGreater => Precedence::Shift,
            TokenType::DotDot | TokenType::DotDotEqual => Precedence::Range,
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => Precedence::Comparison,
            TokenType::EqualEqual | TokenType::BangEqual => Precedence::Equality,
            TokenType::Ampersand => Precedence::BitwiseAnd,
            TokenType::Caret => Precedence::BitwiseXor,
            TokenType::Pipe => Precedence::BitwiseOr,
            TokenType::AndAnd => Precedence::And,
            TokenType::OrOr => Precedence::Or,
            TokenType::Question => Precedence::Ternary,
            _ => Precedence::None,
        }
    }

    // ===== Primary Expressions =====

    fn number(&mut self) -> Option<Expr> {
        let token_str = self.previous_token.token.clone();
        let value = self.parse_number_literal(&token_str)?;
        let location = self.current_location();
        Some(Expr::Number { value, location })
    }

    fn parse_number_literal(&mut self, s: &str) -> Option<f64> {
        // Remove all underscores for parsing
        let clean: String = s.chars().filter(|c| *c != '_').collect();

        if clean.len() >= 2 {
            let radix = match &clean[..2] {
                "0x" | "0X" => Some(16),
                "0b" | "0B" => Some(2),
                "0o" | "0O" => Some(8),
                _ => None,
            };
            if let Some(radix) = radix {
                return match u64::from_str_radix(&clean[2..], radix) {
                    Ok(int_value) => Some(int_value as f64),
                    Err(_) => {
                        let location = self.current_location();
                        self.report_error(location, "Number literal too large".to_string());
                        None
                    }
                };
            }
        }

        // Parse as decimal (with potential floating point)
        clean.parse::<f64>().ok()
    }

    fn string(&self) -> Option<Expr> {
        let value = self.previous_token.token.clone();
        let location = self.current_location();
        Some(Expr::String { value, location })
    }

    fn interpolated_string(&mut self) -> Option<Expr> {
        use crate::compiler::ast::InterpolationPart;
        use crate::compiler::scanner::decode_escape;

        let token_value = self.previous_token.token.clone();
        let location = self.current_location();

        let content = &token_value[1..token_value.len() - 1];
        let chars: Vec<char> = content.chars().collect();

        let mut parts = Vec::new();
        let mut current_literal = String::new();

        // Right after the opening quote.
        let mut line = location.line;
        let mut column = location.column + 1;
        let mut offset = location.offset + 1;

        let mut i = 0;
        while i < chars.len() {
            let ch = chars[i];

            if ch == '\\' {
                let (decoded_char, consumed) = decode_escape(&chars[i + 1..])
                    .expect("scanner already validated this escape sequence");
                current_literal.push(decoded_char);
                Self::advance_text_position(&mut line, &mut column, &mut offset, ch);
                i += 1;
                for _ in 0..consumed {
                    Self::advance_text_position(&mut line, &mut column, &mut offset, chars[i]);
                    i += 1;
                }
                continue;
            }

            Self::advance_text_position(&mut line, &mut column, &mut offset, ch);

            if ch == '$' && chars.get(i + 1) == Some(&'{') {
                i += 1;
                Self::advance_text_position(&mut line, &mut column, &mut offset, '{');
                i += 1;

                if !current_literal.is_empty() {
                    parts.push(InterpolationPart::Literal(current_literal.clone()));
                    current_literal.clear();
                }

                let (expr_line, expr_column, expr_offset) = (line, column, offset);

                let mut expr_str = String::new();
                let mut brace_depth = 1;

                while i < chars.len() {
                    let ch = chars[i];
                    i += 1;
                    Self::advance_text_position(&mut line, &mut column, &mut offset, ch);
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

                if brace_depth != 0 {
                    let end_of_string = SourceLocation {
                        offset,
                        line,
                        column,
                    };
                    self.report_error(
                        end_of_string,
                        "Expect '}' after interpolated expression.".to_string(),
                    );
                    return None;
                }

                let mut expr_parser =
                    Parser::new_at(&expr_str, expr_line, expr_column, expr_offset);
                expr_parser.advance();
                let expr = expr_parser.expression(true);
                let ends_cleanly = expr_parser
                    .consume(TokenType::Eof, "Expect '}' after interpolated expression.");

                match expr {
                    Some(expr) if ends_cleanly => {
                        parts.push(InterpolationPart::Expression(Box::new(expr)));
                    }
                    _ => {
                        if let Some(error) = expr_parser.errors.into_iter().next() {
                            self.record_error(error);
                        }
                        return None;
                    }
                }
            } else {
                current_literal.push(ch);
                i += 1;
            }
        }

        if !current_literal.is_empty() {
            parts.push(InterpolationPart::Literal(current_literal));
        }

        Some(Expr::StringInterpolation { parts, location })
    }

    fn advance_text_position(line: &mut u32, column: &mut u32, offset: &mut usize, ch: char) {
        *offset += 1;
        if ch == '\n' {
            *line += 1;
            *column = 1;
        } else {
            *column += 1;
        }
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

    fn variable(&mut self, can_assign: bool) -> Option<Expr> {
        let name = self.previous_token.token.clone();
        let location = self.current_location();

        if can_assign && self.match_token(TokenType::Equal) {
            self.skip_new_lines();
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

        // For right-associative operators (like **), use same precedence level
        // For left-associative operators, use next precedence level
        let precedence = if operator_type == TokenType::StarStar {
            self.get_precedence(&operator_type)
        } else {
            self.get_precedence(&operator_type).next()
        };
        self.skip_new_lines();
        let right = Box::new(self.parse_precedence(precedence, false)?);

        let operator = match operator_type {
            TokenType::Plus => BinaryOp::Add,
            TokenType::Minus => BinaryOp::Subtract,
            TokenType::Star => BinaryOp::Multiply,
            TokenType::StarStar => BinaryOp::Exponent,
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
            TokenType::Ampersand => BinaryOp::BitwiseAnd,
            TokenType::Pipe => BinaryOp::BitwiseOr,
            TokenType::Caret => BinaryOp::BitwiseXor,
            TokenType::LessLess => BinaryOp::LeftShift,
            TokenType::GreaterGreater => BinaryOp::RightShift,
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

        let operand = Box::new(self.parse_precedence(Precedence::Exponent, false)?);

        let operator = match operator_type {
            TokenType::Minus => UnaryOp::Negate,
            TokenType::Bang => UnaryOp::Not,
            TokenType::Tilde => UnaryOp::BitwiseNot,
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
        self.skip_new_lines();
        let end = Box::new(self.parse_precedence(precedence, false)?);

        Some(Expr::Range {
            start: Box::new(start),
            end,
            inclusive,
            location,
        })
    }

    fn ternary(&mut self, condition: Expr) -> Option<Expr> {
        let location = self.current_location();

        self.skip_new_lines();
        let then_expr = Box::new(self.expression(false)?);

        if !self.consume(
            TokenType::Colon,
            "Expect ':' after ternary then expression.",
        ) {
            return None;
        }

        self.skip_new_lines();
        let else_expr = Box::new(self.expression(false)?);

        Some(Expr::Conditional {
            condition: Box::new(condition),
            then_expr,
            else_expr,
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

    fn dot(&mut self, object: Expr, can_assign: bool) -> Option<Expr> {
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
        } else if can_assign && self.match_token(TokenType::Equal) {
            self.skip_new_lines();
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

        let entries = self.parse_map_entry_list()?;

        if !self.consume(TokenType::RightBrace, "Expect '}' after map entries.") {
            return None;
        }

        Some(Expr::MapLiteral { entries, location })
    }

    fn set_literal(&mut self) -> Option<Expr> {
        let location = self.current_location();

        let elements = self.parse_expression_list(TokenType::RightBrace, None, "")?;

        if !self.consume(TokenType::RightBrace, "Expect '}' after set elements.") {
            return None;
        }

        Some(Expr::SetLiteral { elements, location })
    }

    fn array_literal(&mut self) -> Option<Expr> {
        let location = self.current_location();

        let elements = self.parse_expression_list(TokenType::RightBracket, None, "")?;

        if !self.consume(TokenType::RightBracket, "Expect ']' after array elements.") {
            return None;
        }

        Some(Expr::ArrayLiteral { elements, location })
    }

    /// Parses `fn(params) { body }` in expression position, e.g.
    /// `val double = fn(x) { return x * 2 }`. A statement beginning with
    /// `fn` is always the named declaration (`fn_declaration`); this
    /// parselet only runs where `fn` appears as a prefix inside an
    /// expression.
    fn lambda(&mut self) -> Option<Expr> {
        let location = self.current_location();

        if !self.consume(TokenType::LeftParen, "Expect '(' after 'fn'.") {
            return None;
        }

        let params = self.parse_parameter_list()?;
        if !self.consume(TokenType::RightParen, "Expect ')' after parameters.") {
            return None;
        }

        if !self.consume(TokenType::LeftBrace, "Expect '{' before function body.") {
            return None;
        }

        let body = self.parse_block_body()?;
        Some(Expr::Function {
            params,
            body,
            location,
        })
    }

    fn index(&mut self, object: Expr, can_assign: bool) -> Option<Expr> {
        let location = self.current_location();

        let index = Box::new(self.expression(false)?);

        if !self.consume(TokenType::RightBracket, "Expect ']' after index.") {
            return None;
        }

        if can_assign && self.match_token(TokenType::Equal) {
            self.skip_new_lines();
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
