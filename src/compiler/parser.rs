use crate::common::constants::MAX_PARSER_RECURSION_DEPTH;
use crate::common::errors::{
    CompilationError, CompilationErrorKind, CompilationPhase, CompilationResult,
};
use crate::common::SourceLocation;
/// AST-building parser for the multi-pass compiler
/// This parser builds an Abstract Syntax Tree instead of emitting bytecode directly
use crate::compiler::ast::{
    BinaryOp, Binding, EnumVariant, Expr, IfExprElse, MatchArm, MatchArmBody, MatchPattern, NodeId,
    Pattern, Stmt, StructField, UnaryOp,
};
use crate::compiler::token::TokenType;
use crate::compiler::{Scanner, Token};
use std::collections::HashMap;

/// The value of a scanned number literal, decided by its spelling: a
/// decimal with no `.` and no exponent, or a hex/binary/octal literal, is
/// an int; anything else is a float.
enum NumberLiteral {
    Int(i64),
    Float(f64),
}

/// AST Parser that builds an Abstract Syntax Tree
pub struct Parser {
    scanner: Scanner,
    previous_token: Token,
    current_token: Token,
    errors: Vec<CompilationError>,
    panic_mode: bool,
    next_node_id: u32,
    /// Open `{`/`#{`, `(`, `[` counts, and open `${...}` interpolation
    /// count, tracked separately so a stray closer of one type can't be
    /// mistaken for closing another.
    nesting_depth: (usize, usize, usize, usize),
    /// Read from the scanner because an Error token can replace a string
    /// token.
    pending_interpolation_depth: usize,
    /// Rust call-stack recursion depth, bounded to avoid a stack overflow.
    recursion_depth: usize,
    /// Closing-brace location of each `fn` declaration/method and lambda
    /// body, keyed by its `NodeId`.
    end_locations: HashMap<NodeId, SourceLocation>,
    /// True while parsing an `if`/`while`/`for ... in` condition, where a
    /// `{` directly after it starts the statement's own body, not a
    /// trailing-block lambda.
    suppress_trailing_block: bool,
    /// True while parsing a match pattern's expression, where `Enum.Variant(`
    /// opens a list of sub-patterns instead of call arguments.
    in_match_pattern: bool,
    /// The sub-patterns `dot` parsed for a variant pattern, awaiting
    /// `match_pattern`.
    variant_pattern_fields: Option<Vec<MatchPattern>>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Precedence {
    None,
    Assignment,
    Ternary,
    NilCoalesce,
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
            Precedence::Ternary => Precedence::NilCoalesce,
            Precedence::NilCoalesce => Precedence::Or,
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
        Parser {
            scanner: Scanner::new(source),
            previous_token: Token::default(),
            current_token: Token::default(),
            errors: Vec::new(),
            panic_mode: false,
            next_node_id: 0,
            nesting_depth: (0, 0, 0, 0),
            pending_interpolation_depth: 0,
            recursion_depth: 0,
            end_locations: HashMap::new(),
            suppress_trailing_block: false,
            in_match_pattern: false,
            variant_pattern_fields: None,
        }
    }

    /// Runs `parse` with trailing-block lambdas disabled, for a condition
    /// or collection expression directly followed by the statement's own
    /// `{`.
    fn without_trailing_block<T>(
        &mut self,
        parse: impl FnOnce(&mut Self) -> Option<T>,
    ) -> Option<T> {
        let previous = self.suppress_trailing_block;
        self.suppress_trailing_block = true;
        let result = parse(self);
        self.suppress_trailing_block = previous;
        result
    }

    /// Runs `parse` with trailing-block lambdas re-enabled, for a
    /// sub-expression delimited by its own bracket or paren.
    fn with_trailing_block_allowed<T>(
        &mut self,
        parse: impl FnOnce(&mut Self) -> Option<T>,
    ) -> Option<T> {
        let previous = self.suppress_trailing_block;
        self.suppress_trailing_block = false;
        let result = parse(self);
        self.suppress_trailing_block = previous;
        result
    }

    /// True when the token right after a just-parsed `{ ... }` body, on the
    /// same line as its closing `}` (the current `previous_token`), would
    /// continue an expression: `{` (another trailing block), `.`/`?.` (a
    /// method call on its result), a binary operator, or `?`/`??`/`(`. Used
    /// right after parsing a condition/collection's `{` body (parsed with
    /// `without_trailing_block` stopped in front of it) to tell whether that
    /// `{` was actually the statement's own body or a trailing-block lambda
    /// belonging to the condition.
    fn continues_expression_after_block(&self) -> bool {
        self.current_token.line == self.previous_token.line
            && matches!(
                self.current_token.token_type,
                TokenType::LeftBrace
                    | TokenType::Dot
                    | TokenType::QuestionDot
                    | TokenType::LeftParen
                    | TokenType::Question
                    | TokenType::QuestionQuestion
                    | TokenType::Plus
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
                    | TokenType::GreaterGreater
                    | TokenType::DotDot
                    | TokenType::DotDotEqual
            )
    }

    /// Reports the "trailing block in a condition" error at `brace_location`.
    fn report_trailing_block_in_condition(&mut self, brace_location: SourceLocation) {
        self.report_error(
            CompilationErrorKind::ExpectedToken,
            brace_location,
            "Trailing block is not allowed in a condition; wrap the call in parentheses"
                .to_string(),
        );
    }

    fn next_id(&mut self) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;
        id
    }

    pub fn parse(&mut self) -> CompilationResult<Vec<Stmt>> {
        let mut statements = Vec::new();

        self.advance();

        while !self.match_token(TokenType::Eof) {
            let start_offset = self.current_token.offset;
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
            if self.panic_mode {
                if self.current_token.offset == start_offset {
                    self.advance();
                }
                self.synchronize(None);
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
        self.nesting_depth.3 = self.pending_interpolation_depth;
        let (braces, parens, brackets, _) = &mut self.nesting_depth;
        match self.previous_token.token_type {
            TokenType::LeftBrace | TokenType::HashLeftBrace => *braces += 1,
            TokenType::RightBrace => *braces = braces.saturating_sub(1),
            TokenType::LeftParen => *parens += 1,
            TokenType::RightParen => *parens = parens.saturating_sub(1),
            TokenType::LeftBracket => *brackets += 1,
            TokenType::RightBracket => *brackets = brackets.saturating_sub(1),
            _ => {}
        }
        loop {
            self.current_token = self.scanner.scan_token();
            let kind = match &self.current_token.token_type {
                TokenType::Error(kind) => *kind,
                _ => break,
            };
            self.report_error_at_current(kind, self.current_token.token.clone());
        }
        self.pending_interpolation_depth = self.scanner.interpolation_depth();
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
        self.report_error_at_current(CompilationErrorKind::ExpectedToken, message.to_string());
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
        self.report_error_at_current(CompilationErrorKind::ExpectedToken, message.to_string());
    }

    fn skip_new_lines(&mut self) {
        while self.check(TokenType::NewLine) {
            self.advance();
        }
    }

    fn parse_comma_separated_list<T, F>(
        &mut self,
        closing_token: TokenType,
        max_count: Option<(usize, CompilationErrorKind, &str)>,
        mut parse_element: F,
    ) -> Option<Vec<T>>
    where
        F: FnMut(&mut Self) -> Option<T>,
    {
        self.with_trailing_block_allowed(move |parser| {
            let mut items = Vec::new();

            parser.skip_new_lines();
            if !parser.check(closing_token.clone()) {
                loop {
                    // Check max count if specified
                    if let Some((max, kind, max_count_error)) = max_count {
                        if items.len() >= max {
                            parser.report_error_at_current(kind, max_count_error.to_string());
                        }
                    }

                    // Parse element using provided closure
                    items.push(parse_element(parser)?);
                    parser.skip_new_lines();
                    if !parser.match_token(TokenType::Comma) {
                        break;
                    }
                    parser.skip_new_lines();

                    // Support trailing comma
                    if parser.check(closing_token.clone()) {
                        break;
                    }
                }
            }
            parser.skip_new_lines();
            Some(items)
        })
    }

    fn parse_expression_list(
        &mut self,
        closing_token: TokenType,
        max_count: Option<(usize, CompilationErrorKind, &str)>,
    ) -> Option<Vec<Expr>> {
        self.parse_comma_separated_list(closing_token, max_count, |parser| parser.expression(false))
    }

    fn parse_parameter_list(&mut self) -> Option<Vec<String>> {
        self.parse_comma_separated_list(
            TokenType::RightParen,
            Some((
                crate::common::constants::MAX_FUNCTION_PARAMS,
                CompilationErrorKind::TooManyParameters,
                "Can't have more than 255 parameters.",
            )),
            |parser| {
                if !parser.consume(TokenType::Identifier, "Expect parameter name.") {
                    return None;
                }
                Some(parser.previous_token.token.clone())
            },
        )
    }

    fn parse_map_entry_list(&mut self) -> Option<Vec<(Expr, Expr)>> {
        self.parse_comma_separated_list(TokenType::RightBrace, None, |parser| {
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
            Some((
                crate::common::constants::MAX_CALL_ARGUMENTS,
                CompilationErrorKind::TooManyCallArguments,
                "Can't have more than 255 arguments.",
            )),
        )
    }

    fn current_location(&self) -> SourceLocation {
        SourceLocation {
            offset: self.previous_token.offset,
            line: self.previous_token.line,
            column: self.previous_token.column,
        }
    }

    /// Location of the end of the source, valid after `parse()` returns:
    /// its main loop only exits once the `Eof` token has been consumed.
    pub(crate) fn eof_location(&self) -> SourceLocation {
        self.current_location()
    }

    /// Valid after `parse()` returns.
    pub(crate) fn end_locations(&self) -> &HashMap<NodeId, SourceLocation> {
        &self.end_locations
    }

    /// Comments and blank lines dropped from the token stream. Valid
    /// after `parse()` returns.
    pub(crate) fn trivia(&self) -> &crate::compiler::Trivia {
        self.scanner.trivia()
    }

    fn current_token_location(&self) -> SourceLocation {
        SourceLocation {
            offset: self.current_token.offset,
            line: self.current_token.line,
            column: self.current_token.column,
        }
    }

    // ===== Error Handling =====

    /// Runs `parse` one recursion level deeper, or reports an error once
    /// `MAX_PARSER_RECURSION_DEPTH` is exceeded.
    fn nested<T>(&mut self, parse: impl FnOnce(&mut Self) -> Option<T>) -> Option<T> {
        self.recursion_depth += 1;
        let result = if self.recursion_depth > MAX_PARSER_RECURSION_DEPTH {
            self.report_error_at_current(
                CompilationErrorKind::NestingTooDeep,
                format!("Nesting too deep (limit is {MAX_PARSER_RECURSION_DEPTH})."),
            );
            None
        } else {
            parse(self)
        };
        self.recursion_depth -= 1;
        result
    }

    fn report_error_at_current(&mut self, kind: CompilationErrorKind, message: String) {
        let location = self.current_token_location();
        self.report_error(kind, location, message);
    }

    fn report_error(
        &mut self,
        kind: CompilationErrorKind,
        location: SourceLocation,
        message: String,
    ) {
        self.record_error(CompilationError::new(
            CompilationPhase::Parse,
            kind,
            message,
            location,
        ));
    }

    fn report_error_at_previous(&mut self, kind: CompilationErrorKind, message: String) {
        let location = self.current_location();
        self.report_error(kind, location, message);
    }

    /// Like `report_error`, but for an error a sub-parser already built.
    fn record_error(&mut self, error: CompilationError) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        self.errors.push(error);
    }

    /// Skips tokens until the start of the next statement, comparing the
    /// parser's `nesting_depth` against `block_depth` (the block being
    /// recovered, `None` at the top level) to tell a group opened before
    /// the error from the block itself.
    fn synchronize(&mut self, block_depth: Option<(usize, usize, usize, usize)>) {
        self.panic_mode = false;
        let mut local_depth: u32 = 0;
        loop {
            if self.previous_token.token_type == TokenType::Eof {
                return;
            }
            if let Some(depth @ (block_braces, _, _, block_interp)) = block_depth {
                let (braces, _, _, interp) = self.nesting_depth;
                if braces == block_braces && interp == block_interp {
                    if self.current_token.token_type == TokenType::RightBrace {
                        return;
                    }
                    // Unlike `fn`, these can only start a statement, never
                    // an expression, so seeing one means any paren/bracket
                    // the failed statement opened was abandoned, not that
                    // we're still inside it.
                    match self.current_token.token_type {
                        TokenType::Struct
                        | TokenType::Enum
                        | TokenType::Impl
                        | TokenType::Val
                        | TokenType::Var
                        | TokenType::For
                        | TokenType::While
                        | TokenType::Try
                        | TokenType::Throw
                        | TokenType::Use
                        | TokenType::Pub
                        | TokenType::Return => {
                            self.nesting_depth = depth;
                            return;
                        }
                        _ => {}
                    }
                }
            }
            let at_block_depth = match block_depth {
                Some(depth) => self.nesting_depth == depth,
                None => local_depth == 0 && self.nesting_depth.3 == 0,
            };
            if at_block_depth {
                if self.previous_token.token_type == TokenType::NewLine {
                    return;
                }
                match self.current_token.token_type {
                    TokenType::Fn
                    | TokenType::Struct
                    | TokenType::Enum
                    | TokenType::Impl
                    | TokenType::Val
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Try
                    | TokenType::Throw
                    | TokenType::Use
                    | TokenType::Pub
                    | TokenType::Return => return,
                    _ => {}
                }
            }
            if block_depth.is_none() {
                match self.current_token.token_type {
                    TokenType::LeftBrace | TokenType::HashLeftBrace => local_depth += 1,
                    TokenType::RightBrace => local_depth = local_depth.saturating_sub(1),
                    _ => {}
                }
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
        } else if self.check(TokenType::Fn) && !self.scanner.next_is_left_paren() {
            self.advance();
            self.fn_declaration()
        } else if self.match_token(TokenType::Struct) {
            self.struct_declaration()
        } else if self.match_token(TokenType::Enum) {
            self.enum_declaration()
        } else if self.match_token(TokenType::Impl) {
            self.impl_declaration()
        } else if self.match_token(TokenType::Use) {
            self.import_declaration()
        } else if self.match_token(TokenType::Pub) {
            self.export_declaration()
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
        let binding = Binding {
            name,
            id: self.next_id(),
            location,
        };

        let initializer = if self.match_token(TokenType::Equal) {
            Some(self.operand(Precedence::Assignment)?)
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
                pattern: Pattern::Name(binding),
                initializer,
                location,
            }
        } else {
            Stmt::Val {
                pattern: Pattern::Name(binding),
                initializer,
                location,
            }
        })
    }

    fn at_top_level(&self) -> bool {
        self.nesting_depth.0 == 0
    }

    fn import_declaration(&mut self) -> Option<Stmt> {
        let location = self.current_location();
        if !self.at_top_level() {
            self.report_error_at_previous(
                CompilationErrorKind::ImportNotTopLevel,
                "use is only allowed at the top level".to_string(),
            );
            return None;
        }
        if !self.consume(TokenType::String, "expected a string path after use") {
            return None;
        }
        let path = self.previous_token.token.clone();
        let raw_path = self.previous_token.raw.clone();
        let alias = if self.check(TokenType::Identifier) && self.current_token.token == "as" {
            self.advance();
            if !self.consume(TokenType::Identifier, "expected a name after as") {
                return None;
            }
            Some(self.previous_token.token.clone())
        } else {
            None
        };
        self.consume_statement_end("Expecting '\\n' or '\\0' after use declaration.");
        Some(Stmt::Import {
            path,
            raw_path,
            alias,
            id: self.next_id(),
            location,
        })
    }

    fn export_declaration(&mut self) -> Option<Stmt> {
        let location = self.current_location();
        if !self.at_top_level() {
            self.report_error_at_previous(
                CompilationErrorKind::ExportNotTopLevel,
                "pub is only allowed at the top level".to_string(),
            );
            return None;
        }
        let declaration = if self.match_token(TokenType::Val) || self.match_token(TokenType::Var) {
            if self.check(TokenType::LeftParen) {
                return self.export_destructuring_error();
            }
            if self.previous_token.token_type == TokenType::Val {
                self.val_declaration()
            } else {
                self.var_declaration()
            }
        } else if self.match_token(TokenType::Fn) {
            self.fn_declaration()
        } else if self.match_token(TokenType::Struct) {
            self.struct_declaration()
        } else if self.match_token(TokenType::Enum) {
            self.enum_declaration()
        } else {
            self.report_error_at_current(
                CompilationErrorKind::ExpectedToken,
                "pub must precede a fn, val, var, struct or enum".to_string(),
            );
            return None;
        }?;
        Some(Stmt::Export {
            declaration: Box::new(declaration),
            location,
        })
    }

    fn export_destructuring_error(&mut self) -> Option<Stmt> {
        self.report_error_at_current(
            CompilationErrorKind::ExpectedToken,
            "pub binds one name".to_string(),
        );
        None
    }

    fn val_declaration(&mut self) -> Option<Stmt> {
        if self.check(TokenType::LeftParen) {
            return self.tuple_declaration(false);
        }
        self.parse_variable_declaration(false, true)
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        if self.check(TokenType::LeftParen) {
            return self.tuple_declaration(true);
        }
        self.parse_variable_declaration(true, true)
    }

    /// `(a, _, c)`: two or more names in parens, `_` skipping a position
    /// without declaring anything. Shared by `val`/`var` tuple declarations
    /// and `for` tuple patterns.
    fn parse_tuple_pattern(&mut self) -> Option<Vec<Option<Binding>>> {
        if !self.consume(TokenType::LeftParen, "Expect '(' in tuple pattern.") {
            return None;
        }

        let mut slots = Vec::new();
        loop {
            if !self.consume(TokenType::Identifier, "Expecting a name in tuple pattern.") {
                return None;
            }
            let token = self.previous_token.token.clone();
            let location = self.current_location();
            slots.push(if token == "_" {
                None
            } else {
                Some(Binding {
                    name: token,
                    id: self.next_id(),
                    location,
                })
            });
            if !self.match_token(TokenType::Comma) {
                break;
            }
        }

        if !self.consume(TokenType::RightParen, "Expect ')' after tuple pattern.") {
            return None;
        }

        if slots.len() < 2 {
            self.report_error_at_previous(
                CompilationErrorKind::ExpectedToken,
                "Tuple pattern needs at least two names".to_string(),
            );
            return None;
        }

        Some(slots)
    }

    /// `(a, _, c) = expr` after a `val`/`var` keyword.
    fn tuple_declaration(&mut self, is_mutable: bool) -> Option<Stmt> {
        let location = self.current_location();
        let slots = self.parse_tuple_pattern()?;

        if !self.consume(TokenType::Equal, "Expect '=' after tuple pattern.") {
            return None;
        }

        let initializer = self.operand(Precedence::Assignment)?;

        let decl_type = if is_mutable { "variable" } else { "value" };
        self.consume_statement_end(&format!(
            "Expecting '\\n' or '\\0' after {} declaration.",
            decl_type
        ));

        let pattern = Pattern::Tuple(slots);
        Some(if is_mutable {
            Stmt::Var {
                pattern,
                initializer: Some(initializer),
                location,
            }
        } else {
            Stmt::Val {
                pattern,
                initializer: Some(initializer),
                location,
            }
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

        let params = self.parse_parameter_list()?;
        if !self.consume(TokenType::RightParen, "Expect ')' after parameters.") {
            return None;
        }

        if self.match_token(TokenType::Equal) {
            let expr_location = self.current_location();
            let expr = self.operand(Precedence::Assignment)?;
            let end_location = self.current_location();
            self.consume_statement_end("Expecting '\\n' or '\\0' after function body.");

            let id = self.next_id();
            self.end_locations.insert(id, end_location);
            return Some(Stmt::Fn {
                name,
                params,
                body: vec![Stmt::Expression {
                    expr,
                    location: expr_location,
                }],
                id,
                location,
            });
        }

        if !self.consume(TokenType::LeftBrace, "Expect '{' before function body.") {
            return None;
        }

        let body = self.parse_block_body()?;
        let end_location = self.current_location();
        self.consume_statement_end("Expecting '\\n' or '\\0' at end of block.");

        let id = self.next_id();
        self.end_locations.insert(id, end_location);
        Some(Stmt::Fn {
            name,
            params,
            body,
            id,
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
                fields.push(StructField {
                    name: self.previous_token.token.clone(),
                    location: self.current_location(),
                });
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
            id: self.next_id(),
            location,
        })
    }

    fn enum_declaration(&mut self) -> Option<Stmt> {
        if !self.consume(TokenType::Identifier, "Expect enum name.") {
            return None;
        }
        let name = self.previous_token.token.clone();
        let location = self.current_location();

        if !self.consume(TokenType::LeftBrace, "Expect '{' after enum name.") {
            return None;
        }

        let mut variants = Vec::new();
        self.skip_new_lines();

        if !self.check(TokenType::RightBrace) {
            loop {
                if !self.consume(TokenType::Identifier, "Expect variant name.") {
                    break;
                }
                let name = self.previous_token.token.clone();
                let location = self.current_location();
                let mut fields = Vec::new();
                if self.match_token(TokenType::LeftParen) {
                    fields = self.parse_comma_separated_list(
                        TokenType::RightParen,
                        Some((
                            crate::common::constants::MAX_FUNCTION_PARAMS,
                            CompilationErrorKind::TooManyParameters,
                            "Can't have more than 255 fields.",
                        )),
                        |parser| {
                            if !parser.consume(TokenType::Identifier, "Expect field name.") {
                                return None;
                            }
                            Some(parser.previous_token.token.clone())
                        },
                    )?;
                    if fields.is_empty() {
                        self.consume(TokenType::Identifier, "Expect field name.");
                        return None;
                    }
                    if !self.consume(TokenType::RightParen, "Expect ')' after variant fields.") {
                        return None;
                    }
                }
                variants.push(EnumVariant {
                    name,
                    fields,
                    location,
                });
                self.skip_new_lines();
                if self.check(TokenType::RightBrace) {
                    break;
                }
            }
        }

        if !self.consume(TokenType::RightBrace, "Expect '}' after enum variants.") {
            return None;
        }
        self.consume_statement_end("Expecting '\\n' or '\\0' after enum declaration.");

        Some(Stmt::Enum {
            name,
            variants,
            id: self.next_id(),
            location,
        })
    }

    fn impl_declaration(&mut self) -> Option<Stmt> {
        if !self.consume(TokenType::Identifier, "Expect type name.") {
            return None;
        }
        let mut module_name = None;
        let mut type_name = self.previous_token.token.clone();
        if self.match_token(TokenType::Dot) {
            if !self.consume(TokenType::Identifier, "Expect type name after '.'.") {
                return None;
            }
            module_name = Some(type_name);
            type_name = self.previous_token.token.clone();
        }
        let type_id = self.next_id();
        let location = self.current_location();

        if !self.consume(TokenType::LeftBrace, "Expect '{' after type name.") {
            return None;
        }

        let mut methods = Vec::new();
        self.skip_new_lines();

        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            if !self.consume(TokenType::Fn, "Expect method declaration.") {
                let mut depth = 0;
                loop {
                    if self.check(TokenType::Eof) {
                        break;
                    }
                    if self.check(TokenType::RightBrace) {
                        if depth == 0 {
                            break;
                        }
                        depth -= 1;
                    } else if self.check(TokenType::LeftBrace)
                        || self.check(TokenType::HashLeftBrace)
                    {
                        depth += 1;
                    }
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
            module_name,
            type_name,
            type_id,
            methods,
            location,
        })
    }

    // ===== Statements =====

    fn statement(&mut self) -> Option<Stmt> {
        self.nested(Self::statement_inner)
    }

    fn statement_inner(&mut self) -> Option<Stmt> {
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
        } else if self.match_token(TokenType::Try) {
            self.try_statement()
        } else if self.match_token(TokenType::Throw) {
            self.throw_statement()
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
        self.nested(Self::parse_block_body_inner)
    }

    fn parse_block_body_inner(&mut self) -> Option<Vec<Stmt>> {
        let mut statements = Vec::new();
        let block_depth = self.nesting_depth;
        self.skip_new_lines();

        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            let start_offset = self.current_token.offset;
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
            if self.panic_mode {
                if self.current_token.offset == start_offset {
                    self.advance();
                }
                self.synchronize(Some(block_depth));
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

        let condition = self.without_trailing_block(|parser| parser.expression(false))?;

        let then_branch = Box::new(self.require_block_body()?);
        let else_branch = if self.match_token(TokenType::Else) {
            Some(Box::new(self.require_else_body()?))
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

        let condition = self.without_trailing_block(|parser| parser.expression(false))?;

        let body = Box::new(self.require_block_body()?);

        Some(Stmt::While {
            condition,
            body,
            location,
        })
    }

    /// Reports "Expect '{' after condition" unless the current token starts
    /// a block, shared by `require_block_body` and `if_expr_block`.
    fn require_left_brace(&mut self) -> bool {
        if self.check(TokenType::LeftBrace) {
            true
        } else {
            self.report_error_at_current(
                CompilationErrorKind::ExpectedToken,
                "Expect '{' after condition".to_string(),
            );
            false
        }
    }

    /// Requires the next token to start a `{ ... }` block, as the body of an
    /// `if`, `while`, or `for ... in`.
    fn require_block_body(&mut self) -> Option<Stmt> {
        if !self.require_left_brace() {
            return None;
        }
        let brace_location = self.current_token_location();
        self.advance();
        let location = self.current_location();
        let statements = self.parse_block_body()?;
        if self.continues_expression_after_block() {
            self.report_trailing_block_in_condition(brace_location);
            return None;
        }
        if !self.check(TokenType::Else) {
            self.consume_statement_end("Expecting '\\n' or '\\0' at end of block.");
        }
        Some(Stmt::Block {
            statements,
            location,
        })
    }

    /// Requires an `else` to be followed by a block or another `if`.
    fn require_else_body(&mut self) -> Option<Stmt> {
        if self.check(TokenType::LeftBrace) || self.check(TokenType::If) {
            self.statement()
        } else {
            self.report_error_at_current(
                CompilationErrorKind::ExpectedToken,
                "Expect '{' or 'if' after 'else'".to_string(),
            );
            None
        }
    }

    fn for_statement(&mut self) -> Option<Stmt> {
        let location = self.current_location();

        let pattern = if self.check(TokenType::LeftParen) {
            Pattern::Tuple(self.parse_tuple_pattern()?)
        } else {
            if !self.consume(TokenType::Identifier, "Expecting identifier after 'for'.") {
                return None;
            }
            let name = self.previous_token.token.clone();
            let location = self.current_location();
            Pattern::Name(Binding {
                name,
                id: self.next_id(),
                location,
            })
        };

        let in_subject = if matches!(pattern, Pattern::Tuple(_)) {
            "tuple pattern"
        } else {
            "identifier"
        };
        if !self.consume(
            TokenType::In,
            &format!("Expecting 'in' after {} in for-in loop.", in_subject),
        ) {
            return None;
        }

        let collection = self.without_trailing_block(|parser| parser.expression(false))?;
        let body = Box::new(self.require_block_body()?);

        Some(Stmt::ForIn {
            pattern,
            collection,
            body,
            location,
        })
    }

    fn try_statement(&mut self) -> Option<Stmt> {
        let location = self.current_location();

        let body = Box::new(self.try_block("Expect '{' after 'try'.")?);
        if !self.consume(TokenType::Catch, "Expect 'catch' after try block.") {
            return None;
        }
        if !self.consume(TokenType::LeftParen, "Expect '(' after 'catch'.") {
            return None;
        }
        if !self.consume(TokenType::Identifier, "Expect catch parameter name.") {
            return None;
        }
        let catch_binding = Binding {
            name: self.previous_token.token.clone(),
            id: self.next_id(),
            location: self.current_location(),
        };
        if !self.consume(TokenType::RightParen, "Expect ')' after catch parameter.") {
            return None;
        }
        let catch_body = Box::new(self.try_block("Expect '{' after catch parameter.")?);
        self.consume_statement_end("Expecting '\\n' or '\\0' at end of block.");

        Some(Stmt::Try {
            body,
            catch_binding,
            catch_body,
            location,
        })
    }

    fn try_block(&mut self, message: &str) -> Option<Stmt> {
        if !self.consume(TokenType::LeftBrace, message) {
            return None;
        }
        let location = self.current_location();
        let statements = self.parse_block_body()?;
        Some(Stmt::Block {
            statements,
            location,
        })
    }

    fn throw_statement(&mut self) -> Option<Stmt> {
        let location = self.current_location();
        let value = self.expression(false)?;
        self.consume_statement_end("Expecting '\\n' or '\\0' at end of statement.");
        Some(Stmt::Throw { value, location })
    }

    fn return_statement(&mut self) -> Option<Stmt> {
        let location = self.current_location();
        if self.check(TokenType::NewLine)
            || self.check(TokenType::RightBrace)
            || self.check(TokenType::Eof)
        {
            self.consume_statement_end("Expecting '\\n' or '\\0' at end of statement.");
            return Some(Stmt::Return {
                value: None,
                location,
            });
        }
        let value = self.expression(false)?;
        self.consume_statement_end("Expecting '\\n' or '\\0' at end of statement.");
        Some(Stmt::Return {
            value: Some(value),
            location,
        })
    }

    fn yield_expression(&mut self) -> Option<Expr> {
        let location = self.current_location();
        if self.check(TokenType::NewLine)
            || self.check(TokenType::RightBrace)
            || self.check(TokenType::RightParen)
            || self.check(TokenType::RightBracket)
            || self.check(TokenType::Comma)
            || self.check(TokenType::Colon)
            || self.check(TokenType::Eof)
        {
            return Some(Expr::Yield {
                value: None,
                location,
            });
        }
        let value = self.expression(false)?;
        Some(Expr::Yield {
            value: Some(Box::new(value)),
            location,
        })
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

    /// Whether the current token can only start a new statement (`Eof`,
    /// `}`, or a statement keyword), never continue the expression in
    /// progress.
    fn is_statement_boundary(&self) -> bool {
        matches!(
            self.current_token.token_type,
            TokenType::Eof
                | TokenType::RightBrace
                | TokenType::Val
                | TokenType::Var
                | TokenType::Struct
                | TokenType::Enum
                | TokenType::Impl
                | TokenType::Use
                | TokenType::Pub
                | TokenType::For
                | TokenType::While
                | TokenType::Try
                | TokenType::Throw
                | TokenType::Return
                | TokenType::Break
                | TokenType::Continue
        )
    }

    fn operand(&mut self, precedence: Precedence) -> Option<Expr> {
        let operator_location = self.current_location();
        let had_newline = self.check(TokenType::NewLine);
        self.skip_new_lines();
        if had_newline && self.is_statement_boundary() {
            self.report_error(
                CompilationErrorKind::ExpectedExpression,
                operator_location,
                "Expect expression".to_string(),
            );
            return None;
        }
        self.parse_precedence(precedence, false)
    }

    fn parse_precedence(&mut self, precedence: Precedence, skip_new_lines: bool) -> Option<Expr> {
        self.nested(|parser| parser.parse_precedence_inner(precedence, skip_new_lines))
    }

    fn parse_precedence_inner(
        &mut self,
        precedence: Precedence,
        skip_new_lines: bool,
    ) -> Option<Expr> {
        if skip_new_lines {
            self.skip_new_lines();
        }

        if self.is_statement_boundary() {
            self.report_error_at_current(
                CompilationErrorKind::ExpectedExpression,
                "Expect expression".to_string(),
            );
            return None;
        }

        self.advance();

        let can_assign = precedence <= Precedence::Assignment;

        let mut expr = match self.previous_token.token_type {
            TokenType::Number => self.number(),
            TokenType::String => self.string(),
            TokenType::StringStart => self.interpolated_string(),
            TokenType::True | TokenType::False | TokenType::Nil => self.literal(),
            TokenType::LeftParen => self.grouping(),
            TokenType::Minus | TokenType::Bang | TokenType::Tilde => self.unary(),
            TokenType::Identifier => self.variable(can_assign),
            TokenType::LeftBrace => self.brace_literal(),
            TokenType::HashLeftBrace => self.set_literal(),
            TokenType::LeftBracket => self.array_literal(),
            TokenType::Fn => self.lambda(),
            TokenType::If => self.if_expression(),
            TokenType::Match => self.match_expression(),
            TokenType::Yield => self.yield_expression(),
            _ => {
                self.report_error_at_previous(
                    CompilationErrorKind::ExpectedExpression,
                    "Expect expression".to_string(),
                );
                return None;
            }
        }?;

        let mut has_trailing_block = false;
        loop {
            while precedence <= self.get_precedence(&self.current_token.token_type) {
                has_trailing_block = false;
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
                    | TokenType::QuestionQuestion
                    | TokenType::Ampersand
                    | TokenType::Pipe
                    | TokenType::Caret
                    | TokenType::LessLess
                    | TokenType::GreaterGreater => self.binary(expr),
                    TokenType::DotDot | TokenType::DotDotEqual => self.range(expr),
                    TokenType::LeftParen => self.call(expr),
                    TokenType::Dot => self.dot(expr, can_assign, false),
                    TokenType::QuestionDot => self.dot(expr, can_assign, true),
                    TokenType::LeftBracket => self.index(expr, can_assign),
                    TokenType::Question => self.ternary(expr),
                    _ => {
                        return Some(expr);
                    }
                }?;
            }

            if self.suppress_trailing_block
                || !self.check(TokenType::LeftBrace)
                || !matches!(
                    expr,
                    Expr::Call { .. } | Expr::Variable { .. } | Expr::GetField { .. }
                )
            {
                break;
            }
            if has_trailing_block {
                self.report_error_at_current(
                    CompilationErrorKind::ExpectedToken,
                    "A call accepts only one trailing block".to_string(),
                );
                return None;
            }
            self.advance();
            expr = self.trailing_block(expr)?;
            has_trailing_block = true;
        }

        if can_assign && (self.check(TokenType::Equal) || self.compound_assign_op().is_some()) {
            self.advance();
            self.report_error_at_previous(
                CompilationErrorKind::InvalidAssignmentTarget,
                "Invalid assignment target.".to_string(),
            );
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
            | TokenType::QuestionDot
            | TokenType::LeftBracket => Precedence::Call,
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
            TokenType::QuestionQuestion => Precedence::NilCoalesce,
            TokenType::Question => Precedence::Ternary,
            _ => Precedence::None,
        }
    }

    // ===== Primary Expressions =====

    fn number(&mut self) -> Option<Expr> {
        let raw = self.previous_token.token.clone();
        let value = self.parse_number_literal(&raw)?;
        let location = self.current_location();
        Some(match value {
            NumberLiteral::Int(value) => Expr::Int {
                value,
                raw,
                location,
            },
            NumberLiteral::Float(value) => Expr::Number {
                value,
                raw,
                location,
            },
        })
    }

    fn parse_number_literal(&mut self, s: &str) -> Option<NumberLiteral> {
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
                return match u64::from_str_radix(&clean[2..], radix)
                    .ok()
                    .and_then(|v| i64::try_from(v).ok())
                {
                    Some(int_value) => Some(NumberLiteral::Int(int_value)),
                    None => {
                        let location = self.current_location();
                        self.report_error(
                            CompilationErrorKind::NumberLiteralTooLarge,
                            location,
                            "Integer literal is too large".to_string(),
                        );
                        None
                    }
                };
            }
        }

        if !clean.contains('.') && !clean.contains('e') && !clean.contains('E') {
            return match clean.parse::<i64>() {
                Ok(int_value) => Some(NumberLiteral::Int(int_value)),
                Err(_) => {
                    let location = self.current_location();
                    self.report_error(
                        CompilationErrorKind::NumberLiteralTooLarge,
                        location,
                        "Integer literal is too large".to_string(),
                    );
                    None
                }
            };
        }

        // Parse as decimal floating point
        clean.parse::<f64>().ok().map(NumberLiteral::Float)
    }

    fn string(&self) -> Option<Expr> {
        let value = self.previous_token.token.clone();
        let raw = self.previous_token.raw.clone();
        let location = self.current_location();
        Some(Expr::String {
            value,
            raw,
            location,
        })
    }

    fn interpolated_string(&mut self) -> Option<Expr> {
        use crate::compiler::ast::InterpolationPart;

        let location = self.current_location();
        let mut parts = Vec::new();

        let start_text = self.previous_token.token.clone();
        let start_raw = self.previous_token.raw.clone();
        if !start_text.is_empty() {
            parts.push(InterpolationPart::Literal {
                value: start_text,
                raw: start_raw,
            });
        }

        loop {
            let expr = self.expression(true)?;
            parts.push(InterpolationPart::Expression(Box::new(expr)));

            if self.match_token(TokenType::StringMiddle) {
                let text = self.previous_token.token.clone();
                let raw = self.previous_token.raw.clone();
                if !text.is_empty() {
                    parts.push(InterpolationPart::Literal { value: text, raw });
                }
                continue;
            }
            if self.match_token(TokenType::StringEnd) {
                let text = self.previous_token.token.clone();
                let raw = self.previous_token.raw.clone();
                if !text.is_empty() {
                    parts.push(InterpolationPart::Literal { value: text, raw });
                }
                break;
            }
            self.report_error_at_current(
                CompilationErrorKind::ExpectedToken,
                "Expect '}' after interpolated expression.".to_string(),
            );
            return None;
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
        let expr = Box::new(self.with_trailing_block_allowed(|parser| parser.expression(true))?);
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
            let value = Box::new(self.operand(Precedence::Assignment)?);
            Some(Expr::Assign {
                name,
                value,
                id: self.next_id(),
                location,
            })
        } else if let Some(operator) = self.compound_assign_op().filter(|_| can_assign) {
            self.advance();
            let read_id = self.next_id();
            let value = Box::new(self.operand(Precedence::Assignment)?);
            Some(Expr::CompoundAssign {
                name,
                operator,
                value,
                read_id,
                write_id: self.next_id(),
                location,
            })
        } else {
            Some(Expr::Variable {
                name,
                id: self.next_id(),
                location,
            })
        }
    }

    fn compound_assign_op(&self) -> Option<BinaryOp> {
        match self.current_token.token_type {
            TokenType::PlusEqual => Some(BinaryOp::Add),
            TokenType::MinusEqual => Some(BinaryOp::Subtract),
            TokenType::StarEqual => Some(BinaryOp::Multiply),
            TokenType::StarStarEqual => Some(BinaryOp::Exponent),
            TokenType::SlashEqual => Some(BinaryOp::Divide),
            TokenType::PercentEqual => Some(BinaryOp::Modulo),
            _ => None,
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
        let right = Box::new(self.operand(precedence)?);

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
            TokenType::QuestionQuestion => BinaryOp::NilCoalesce,
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
        let end = Box::new(self.operand(precedence)?);

        Some(Expr::Range {
            start: Box::new(start),
            end,
            inclusive,
            location,
        })
    }

    fn ternary(&mut self, condition: Expr) -> Option<Expr> {
        let location = self.current_location();

        let then_expr = Box::new(self.operand(Precedence::Assignment)?);

        if !self.consume(
            TokenType::Colon,
            "Expect ':' after ternary then expression.",
        ) {
            return None;
        }

        let else_expr = Box::new(self.operand(Precedence::Assignment)?);

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
            id: self.next_id(),
            location,
        })
    }

    fn dot(&mut self, object: Expr, can_assign: bool, optional: bool) -> Option<Expr> {
        let location = self.current_location();

        if !self.consume(TokenType::Identifier, "Expect field name after '.'.") {
            return None;
        }
        let field = self.previous_token.token.clone();

        if self.in_match_pattern
            && !optional
            && matches!(object, Expr::Variable { .. })
            && self.match_token(TokenType::LeftParen)
        {
            self.in_match_pattern = false;
            let fields = self.parse_comma_separated_list(
                TokenType::RightParen,
                None,
                Self::match_array_element,
            )?;
            if !self.consume(
                TokenType::RightParen,
                "Expect ')' after variant pattern fields.",
            ) {
                return None;
            }
            self.variant_pattern_fields = Some(fields);
            return Some(Expr::GetField {
                object: Box::new(object),
                field,
                optional,
                location,
            });
        }

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
                optional,
                location,
            };

            Some(Expr::Call {
                callee: Box::new(get_field_expr),
                arguments,
                id: self.next_id(),
                location: method_location,
            })
        } else if !optional && can_assign && self.match_token(TokenType::Equal) {
            self.skip_new_lines();
            let value = Box::new(self.expression(false)?);
            Some(Expr::SetField {
                object: Box::new(object),
                field,
                value,
                location,
            })
        } else if let Some(operator) = self
            .compound_assign_op()
            .filter(|_| can_assign && !optional)
        {
            self.advance();
            let operator_location = self.current_location();
            self.skip_new_lines();
            let value = Box::new(self.expression(false)?);
            Some(Expr::CompoundAssignField {
                object: Box::new(object),
                field,
                operator,
                value,
                location,
                operator_location,
            })
        } else {
            Some(Expr::GetField {
                object: Box::new(object),
                field,
                optional,
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

        let elements = self.parse_expression_list(TokenType::RightBrace, None)?;

        if !self.consume(TokenType::RightBrace, "Expect '}' after set elements.") {
            return None;
        }

        Some(Expr::SetLiteral { elements, location })
    }

    fn array_literal(&mut self) -> Option<Expr> {
        let location = self.current_location();

        let elements = self.parse_expression_list(TokenType::RightBracket, None)?;

        if !self.consume(TokenType::RightBracket, "Expect ']' after array elements.") {
            return None;
        }

        Some(Expr::ArrayLiteral { elements, location })
    }

    /// Parses `fn(params) { body }` in expression position, e.g.
    /// `val double = fn(x) { return x * 2 }`. A statement beginning with
    /// `fn` is always the named declaration (`fn_declaration`); this
    /// parselet only runs where `fn` appears as a prefix inside an
    /// expression. The body is parsed with trailing blocks re-enabled: its
    /// own `{ ... }` already delimits it, so a condition's
    /// `without_trailing_block` must not leak into it.
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

        let body = self.with_trailing_block_allowed(Self::parse_block_body)?;
        let end_location = self.current_location();
        let id = self.next_id();
        self.end_locations.insert(id, end_location);
        Some(Expr::Function {
            params,
            body,
            id,
            location,
            implicit_it: false,
        })
    }

    /// Appends a trailing-block lambda, already past its opening `{`, as
    /// the last argument of `callee`: `f(args) { params -> body }`,
    /// `o.m { body }`, or a bare `f { body }` (a zero-argument call).
    fn trailing_block(&mut self, callee: Expr) -> Option<Expr> {
        let location = self.current_location();
        let lambda = self.block_lambda()?;

        let (call_callee, mut arguments, call_location) = match callee {
            Expr::Call {
                callee,
                arguments,
                location,
                ..
            } => (callee, arguments, location),
            other => (Box::new(other), Vec::new(), location),
        };
        arguments.push(lambda);

        Some(Expr::Call {
            callee: call_callee,
            arguments,
            id: self.next_id(),
            location: call_location,
        })
    }

    /// Parses a trailing block's body, already past its opening `{`: an
    /// optional `name, name -> ` parameter header, then statements up to
    /// the closing `}`. Produces the same `Expr::Function` node as `fn(...)
    /// { ... }`, so name resolution treats it identically.
    fn block_lambda(&mut self) -> Option<Expr> {
        let location = self.current_location();
        let params = self.parse_block_lambda_params()?;
        let implicit_it = params.is_empty();
        let body = self.parse_block_body()?;
        let end_location = self.current_location();
        let id = self.next_id();
        self.end_locations.insert(id, end_location);
        Some(Expr::Function {
            params,
            body,
            id,
            location,
            implicit_it,
        })
    }

    /// Parses the optional `name, name ->` header of a trailing block,
    /// looked ahead for with `looks_like_block_lambda_params` so a body
    /// that happens to start with an identifier isn't mistaken for one.
    fn parse_block_lambda_params(&mut self) -> Option<Vec<String>> {
        if !self
            .scanner
            .looks_like_block_lambda_params(self.current_token.offset)
        {
            return Some(Vec::new());
        }

        let mut params = Vec::new();
        loop {
            if !self.consume(TokenType::Identifier, "Expect parameter name.") {
                return None;
            }
            params.push(self.previous_token.token.clone());
            if !self.match_token(TokenType::Comma) {
                break;
            }
        }
        if !self.consume(TokenType::Arrow, "Expect '->' after block parameters.") {
            return None;
        }
        Some(params)
    }

    /// Parses `if cond { ... } else if cond { ... } else { ... }` in
    /// expression position; `else` is required. Reuses `Stmt::Block` for
    /// each branch so the formatter's brace-location lookups apply
    /// unchanged.
    fn if_expression(&mut self) -> Option<Expr> {
        let location = self.current_location();

        let condition = self.without_trailing_block(|parser| parser.expression(false))?;
        let brace_location = self.current_token_location();
        let then_branch = Box::new(self.if_expr_block()?);
        if self.continues_expression_after_block() {
            self.report_trailing_block_in_condition(brace_location);
            return None;
        }

        if !self.consume(TokenType::Else, "if expression requires else") {
            return None;
        }

        let else_branch = if self.match_token(TokenType::If) {
            Box::new(IfExprElse::If(self.if_expression()?))
        } else if self.check(TokenType::LeftBrace) {
            Box::new(IfExprElse::Block(self.if_expr_block()?))
        } else {
            self.report_error_at_current(
                CompilationErrorKind::ExpectedToken,
                "Expect '{' or 'if' after 'else'".to_string(),
            );
            return None;
        };

        Some(Expr::If {
            condition: Box::new(condition),
            then_branch,
            else_branch,
            location,
        })
    }

    /// Parses a `{ ... }` branch of an if-expression as a `Stmt::Block`,
    /// without requiring a statement terminator after the closing brace -
    /// the branch sits inside a larger expression, which may continue past it.
    fn if_expr_block(&mut self) -> Option<Stmt> {
        if !self.require_left_brace() {
            return None;
        }
        self.advance();
        let location = self.current_location();
        let statements = self.parse_block_body()?;
        Some(Stmt::Block {
            statements,
            location,
        })
    }

    /// Parses `match scrutinee { pattern, pattern -> body ... }`, valid in
    /// both statement and expression position; a bare statement reaches
    /// this through `expression_statement`, same as any other expression.
    fn match_expression(&mut self) -> Option<Expr> {
        let location = self.current_location();

        let scrutinee = self.without_trailing_block(|parser| parser.expression(false))?;
        if !self.consume(TokenType::LeftBrace, "Expect '{' after match expression.") {
            return None;
        }
        let arms = self.match_arms()?;
        if !self.consume(TokenType::RightBrace, "Expect '}' after match arms.") {
            return None;
        }

        Some(Expr::Match {
            scrutinee: Box::new(scrutinee),
            arms,
            location,
        })
    }

    fn match_arms(&mut self) -> Option<Vec<MatchArm>> {
        let mut arms = Vec::new();
        self.skip_new_lines();
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            arms.push(self.match_arm()?);
            self.skip_new_lines();
        }
        Some(arms)
    }

    fn match_arm(&mut self) -> Option<MatchArm> {
        let location = self.current_token_location();

        let mut patterns = vec![self.match_pattern()?];
        while self.match_token(TokenType::Comma) {
            patterns.push(self.match_pattern()?);
        }

        let guard = if self.match_token(TokenType::If) {
            Some(self.expression(false)?)
        } else {
            None
        };

        if !self.consume(TokenType::Arrow, "Expect '->' after match pattern.") {
            return None;
        }

        let body = if self.check(TokenType::LeftBrace) {
            self.advance();
            let block_location = self.current_location();
            let statements = self.parse_block_body()?;
            MatchArmBody::Block(Stmt::Block {
                statements,
                location: block_location,
            })
        } else {
            MatchArmBody::Expr(self.expression(false)?)
        };
        self.consume_statement_end("Expecting '\\n' or '\\0' after match arm.");

        Some(MatchArm {
            patterns,
            guard,
            body,
            location,
        })
    }

    /// An element of an array pattern: a pattern, `..` or `..name`.
    fn match_array_element(&mut self) -> Option<MatchPattern> {
        if !self.match_token(TokenType::DotDot) {
            return self.match_pattern();
        }
        let location = self.current_location();
        let binding = if self.check(TokenType::Identifier) {
            let name = self.current_token.token.clone();
            let binding_location = self.current_token_location();
            self.advance();
            if name == "_" {
                return Some(MatchPattern::Rest {
                    binding: None,
                    location,
                });
            }
            Some(Binding {
                name,
                id: self.next_id(),
                location: binding_location,
            })
        } else {
            None
        };
        Some(MatchPattern::Rest { binding, location })
    }

    /// A single pattern: `_`, a bare name, `[pattern, ...]`, or a literal,
    /// range, enum variant or `Enum.Variant(pattern, ...)`.
    fn match_pattern(&mut self) -> Option<MatchPattern> {
        if self.match_token(TokenType::LeftBracket) {
            let location = self.current_location();
            let elements = self.parse_comma_separated_list(
                TokenType::RightBracket,
                None,
                Self::match_array_element,
            )?;
            if !self.consume(TokenType::RightBracket, "Expect ']' after array pattern.") {
                return None;
            }
            return Some(MatchPattern::Array { elements, location });
        }
        if self.check(TokenType::Identifier) && self.current_token.token == "_" {
            let location = self.current_token_location();
            self.advance();
            return Some(MatchPattern::Wildcard(location));
        }
        self.in_match_pattern = true;
        let expr = self.expression(false);
        self.in_match_pattern = false;
        match expr? {
            Expr::Variable { name, id, location } => {
                Some(MatchPattern::Binding(Binding { name, id, location }))
            }
            target => match self.variant_pattern_fields.take() {
                Some(fields) => Some(MatchPattern::Variant { target, fields }),
                None => Some(MatchPattern::Expr(target)),
            },
        }
    }

    fn index(&mut self, object: Expr, can_assign: bool) -> Option<Expr> {
        let location = self.current_location();

        let index = Box::new(self.with_trailing_block_allowed(|parser| parser.expression(false))?);

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
        } else if let Some(operator) = self.compound_assign_op().filter(|_| can_assign) {
            self.advance();
            let operator_location = self.current_location();
            self.skip_new_lines();
            let value = Box::new(self.expression(false)?);
            Some(Expr::CompoundAssignIndex {
                object: Box::new(object),
                index,
                operator,
                value,
                location,
                operator_location,
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
