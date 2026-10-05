use std::collections::HashMap;

use crate::common::SourceLocation;
use crate::compiler::ast::{Expr, IfExprElse, Stmt};
use crate::compiler::token::TokenType;
use crate::compiler::Scanner;

/// Every real token of a source, re-scanned so the formatter can look up
/// where an AST node starts and ends.
pub(crate) struct SourceMap {
    kinds: Vec<TokenType>,
    lines: Vec<u32>,
    end_lines: Vec<u32>,
    by_offset: HashMap<usize, usize>,
    partners: Vec<Option<usize>>,
}

/// The two token positions that bracket a function's parameter list,
/// shared by named declarations and lambdas.
pub(crate) struct ParamsTokens {
    pub(crate) open: usize,
    pub(crate) close: usize,
}

/// Params and body braces of a function or lambda. For an expression-bodied
/// `fn name(params) = expr`, `body_open`/`body_close` both point at the `=`
/// token; callers that need the expression's real extent use `last_token`
/// on the body expression instead.
pub(crate) struct FnTokens {
    pub(crate) params_open: usize,
    pub(crate) params_close: usize,
    pub(crate) body_open: usize,
    pub(crate) body_close: usize,
}

impl SourceMap {
    #[allow(clippy::expect_used)]
    pub(crate) fn new(source: &str) -> Self {
        let mut scanner = Scanner::new(source);
        let mut kinds = Vec::new();
        let mut lines = Vec::new();
        let mut end_lines = Vec::new();
        let mut by_offset = HashMap::new();
        let mut partners: Vec<Option<usize>> = Vec::new();
        let mut open_stack: Vec<usize> = Vec::new();

        loop {
            let token = scanner.scan_token();
            if token.token_type == TokenType::Eof {
                break;
            }
            if token.token_type == TokenType::NewLine {
                continue;
            }

            let index = kinds.len();
            by_offset.insert(token.offset, index);
            lines.push(token.line);
            end_lines.push(token.line + token.raw.matches('\n').count() as u32);
            partners.push(None);

            match token.token_type {
                TokenType::LeftParen
                | TokenType::LeftBracket
                | TokenType::LeftBrace
                | TokenType::HashLeftBrace
                | TokenType::StringStart => open_stack.push(index),
                TokenType::RightParen
                | TokenType::RightBracket
                | TokenType::RightBrace
                | TokenType::StringEnd => {
                    let open = open_stack.pop().expect("closer without a matching opener");
                    partners[open] = Some(index);
                    partners[index] = Some(open);
                }
                _ => {}
            }

            kinds.push(token.token_type);
        }

        SourceMap {
            kinds,
            lines,
            end_lines,
            by_offset,
            partners,
        }
    }

    #[allow(clippy::expect_used)]
    pub(crate) fn at(&self, location: &SourceLocation) -> usize {
        *self
            .by_offset
            .get(&location.offset)
            .expect("AST locations are token starts")
    }

    pub(crate) fn line(&self, token: usize) -> u32 {
        self.lines[token]
    }

    pub(crate) fn end_line(&self, token: usize) -> u32 {
        self.end_lines[token]
    }

    #[allow(clippy::expect_used)]
    pub(crate) fn partner(&self, token: usize) -> usize {
        self.partners[token].expect("token has a matching bracket")
    }

    pub(crate) fn first_token(&self, expr: &Expr) -> usize {
        match expr {
            Expr::Number { location, .. }
            | Expr::Int { location, .. }
            | Expr::String { location, .. }
            | Expr::StringInterpolation { location, .. }
            | Expr::Boolean { location, .. }
            | Expr::Nil { location }
            | Expr::Variable { location, .. }
            | Expr::Assign { location, .. }
            | Expr::CompoundAssign { location, .. }
            | Expr::Unary { location, .. }
            | Expr::MapLiteral { location, .. }
            | Expr::ArrayLiteral { location, .. }
            | Expr::SetLiteral { location, .. }
            | Expr::Function { location, .. }
            | Expr::If { location, .. } => self.at(location),
            Expr::Binary { left, .. } => self.first_token(left),
            Expr::Range { start, .. } => self.first_token(start),
            Expr::Call { callee, .. } => self.first_token(callee),
            Expr::GetField { object, .. }
            | Expr::SetField { object, .. }
            | Expr::CompoundAssignField { object, .. }
            | Expr::Index { object, .. }
            | Expr::IndexAssign { object, .. }
            | Expr::CompoundAssignIndex { object, .. } => self.first_token(object),
            Expr::Conditional { condition, .. } => self.first_token(condition),
            Expr::Grouping { location, .. } => self.partner(self.at(location)),
        }
    }

    pub(crate) fn last_token(&self, expr: &Expr) -> usize {
        match expr {
            Expr::Number { location, .. }
            | Expr::Int { location, .. }
            | Expr::String { location, .. }
            | Expr::Boolean { location, .. }
            | Expr::Nil { location }
            | Expr::Variable { location, .. }
            | Expr::Grouping { location, .. } => self.at(location),
            Expr::StringInterpolation { location, .. }
            | Expr::Call { location, .. }
            | Expr::MapLiteral { location, .. }
            | Expr::ArrayLiteral { location, .. }
            | Expr::SetLiteral { location, .. }
            | Expr::Index { location, .. } => self.partner(self.at(location)),
            Expr::GetField { location, .. } => self.at(location) + 1,
            Expr::Assign { value, .. }
            | Expr::CompoundAssign { value, .. }
            | Expr::SetField { value, .. }
            | Expr::CompoundAssignField { value, .. }
            | Expr::IndexAssign { value, .. }
            | Expr::CompoundAssignIndex { value, .. } => self.last_token(value),
            Expr::Binary { right, .. } => self.last_token(right),
            Expr::Range { end, .. } => self.last_token(end),
            Expr::Unary { operand, .. } => self.last_token(operand),
            Expr::Conditional { else_expr, .. } => self.last_token(else_expr),
            Expr::Function { location, .. } => self.fn_tokens(location).body_close,
            Expr::If { else_branch, .. } => match else_branch.as_ref() {
                IfExprElse::If(expr) => self.last_token(expr),
                IfExprElse::Block(stmt) => self.stmt_last_token(stmt),
            },
        }
    }

    pub(crate) fn first_line(&self, expr: &Expr) -> u32 {
        self.line(self.first_token(expr))
    }

    pub(crate) fn last_line(&self, expr: &Expr) -> u32 {
        self.end_line(self.last_token(expr))
    }

    /// Params of a function or lambda, as a `(` `)` token pair.
    pub(crate) fn params_tokens(&self, location: &SourceLocation) -> ParamsTokens {
        let open = self.at(location) + 1;
        let close = self.partner(open);
        ParamsTokens { open, close }
    }

    pub(crate) fn fn_tokens(&self, location: &SourceLocation) -> FnTokens {
        let params = self.params_tokens(location);
        let body_open = params.close + 1;
        let body_close = if self.kinds[body_open] == TokenType::LeftBrace {
            self.partner(body_open)
        } else {
            body_open
        };
        FnTokens {
            params_open: params.open,
            params_close: params.close,
            body_open,
            body_close,
        }
    }

    /// True for a `fn name(params) = expr` declaration rather than a braced body.
    pub(crate) fn is_expr_bodied_fn(&self, location: &SourceLocation) -> bool {
        let params_close = self.params_tokens(location).close;
        self.kinds[params_close + 1] == TokenType::Equal
    }

    /// Source lines of the parameter names, in order.
    pub(crate) fn param_lines(&self, fn_tokens: &FnTokens) -> Vec<u32> {
        (fn_tokens.params_open + 1..fn_tokens.params_close)
            .filter(|&token| self.kinds[token] == TokenType::Identifier)
            .map(|token| self.lines[token])
            .collect()
    }

    /// The `{` `}` pair following a struct/enum/impl name token.
    pub(crate) fn braces_after(&self, location: &SourceLocation) -> (usize, usize) {
        let open = self.at(location) + 1;
        (open, self.partner(open))
    }

    pub(crate) fn stmt_first_token(&self, stmt: &Stmt) -> usize {
        match stmt {
            Stmt::Expression { expr, .. } => self.first_token(expr),
            other => self.at(stmt_location(other)),
        }
    }

    pub(crate) fn stmt_last_token(&self, stmt: &Stmt) -> usize {
        match stmt {
            Stmt::Val {
                initializer: Some(expr),
                ..
            }
            | Stmt::Var {
                initializer: Some(expr),
                ..
            } => self.last_token(expr),
            Stmt::Val { location, .. } | Stmt::Var { location, .. } => self.at(location),
            Stmt::Fn { body, location, .. } => {
                if self.is_expr_bodied_fn(location) {
                    self.stmt_last_token(&body[0])
                } else {
                    self.fn_tokens(location).body_close
                }
            }
            Stmt::Struct { location, .. }
            | Stmt::Enum { location, .. }
            | Stmt::Impl { location, .. } => self.braces_after(location).1,
            Stmt::Expression { expr, .. } => self.last_token(expr),
            Stmt::Block { location, .. } => self.partner(self.at(location)),
            Stmt::If {
                then_branch,
                else_branch,
                ..
            } => match else_branch {
                Some(else_stmt) => self.stmt_last_token(else_stmt),
                None => self.stmt_last_token(then_branch),
            },
            Stmt::While { body, .. } | Stmt::ForIn { body, .. } => self.stmt_last_token(body),
            Stmt::Return { value, location } => match value {
                Some(value) => self.last_token(value),
                None => self.at(location),
            },
            Stmt::Break { location } | Stmt::Continue { location } => self.at(location),
        }
    }

    pub(crate) fn stmt_first_line(&self, stmt: &Stmt) -> u32 {
        self.line(self.stmt_first_token(stmt))
    }

    pub(crate) fn stmt_last_line(&self, stmt: &Stmt) -> u32 {
        self.end_line(self.stmt_last_token(stmt))
    }
}

fn stmt_location(stmt: &Stmt) -> &SourceLocation {
    match stmt {
        Stmt::Val { location, .. }
        | Stmt::Var { location, .. }
        | Stmt::Fn { location, .. }
        | Stmt::Struct { location, .. }
        | Stmt::Enum { location, .. }
        | Stmt::Impl { location, .. }
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
