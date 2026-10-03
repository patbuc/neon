use std::collections::HashMap;

use crate::common::SourceLocation;
use crate::compiler::ast::{Expr, Stmt};
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

/// The four token positions that bracket a function's params and body,
/// shared by named declarations and lambdas.
pub(crate) struct FnTokens {
    pub(crate) params_open: usize,
    pub(crate) params_close: usize,
    pub(crate) body_open: usize,
    pub(crate) body_close: usize,
}

impl SourceMap {
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

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn kind(&self, token: usize) -> &TokenType {
        &self.kinds[token]
    }

    pub(crate) fn partner(&self, token: usize) -> usize {
        self.partners[token].expect("token has a matching bracket")
    }

    pub(crate) fn first_token(&self, expr: &Expr) -> usize {
        match expr {
            Expr::Number { location, .. }
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
            | Expr::Function { location, .. } => self.at(location),
            Expr::Binary { left, .. } => self.first_token(left),
            Expr::Range { start, .. } => self.first_token(start),
            Expr::Call { callee, .. } => self.first_token(callee),
            Expr::GetField { object, .. }
            | Expr::SetField { object, .. }
            | Expr::Index { object, .. }
            | Expr::IndexAssign { object, .. } => self.first_token(object),
            Expr::PostfixIncrement { operand, .. } | Expr::PostfixDecrement { operand, .. } => {
                self.first_token(operand)
            }
            Expr::Conditional { condition, .. } => self.first_token(condition),
            Expr::Grouping { location, .. } => self.partner(self.at(location)),
        }
    }

    pub(crate) fn last_token(&self, expr: &Expr) -> usize {
        match expr {
            Expr::Number { location, .. }
            | Expr::String { location, .. }
            | Expr::Boolean { location, .. }
            | Expr::Nil { location }
            | Expr::Variable { location, .. }
            | Expr::PostfixIncrement { location, .. }
            | Expr::PostfixDecrement { location, .. }
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
            | Expr::IndexAssign { value, .. } => self.last_token(value),
            Expr::Binary { right, .. } => self.last_token(right),
            Expr::Range { end, .. } => self.last_token(end),
            Expr::Unary { operand, .. } => self.last_token(operand),
            Expr::Conditional { else_expr, .. } => self.last_token(else_expr),
            Expr::Function { location, .. } => self.fn_tokens(location).body_close,
        }
    }

    pub(crate) fn first_line(&self, expr: &Expr) -> u32 {
        self.line(self.first_token(expr))
    }

    pub(crate) fn last_line(&self, expr: &Expr) -> u32 {
        self.end_line(self.last_token(expr))
    }

    /// Params and body braces of a function or lambda.
    pub(crate) fn fn_tokens(&self, location: &SourceLocation) -> FnTokens {
        let params_open = self.at(location) + 1;
        let params_close = self.partner(params_open);
        let body_open = params_close + 1;
        let body_close = self.partner(body_open);
        FnTokens {
            params_open,
            params_close,
            body_open,
            body_close,
        }
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
            Stmt::Fn { location, .. } => self.fn_tokens(location).body_close,
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
            Stmt::While { body, .. } | Stmt::ForIn { body, .. } | Stmt::For { body, .. } => {
                self.stmt_last_token(body)
            }
            Stmt::Return { value, .. } => self.last_token(value),
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
        | Stmt::For { location, .. }
        | Stmt::Break { location }
        | Stmt::Continue { location } => location,
    }
}
