use crate::common::SourceLocation;
use crate::compiler::ast::{BinaryOp, Expr, InterpolationPart, Stmt, UnaryOp};
use crate::compiler::formatter::source_map::SourceMap;

/// The source-line gap a `line_break` sits in, so blank-line and comment
/// rules (added once the printer understands them) can tell an edge of a
/// bracketed list or block from a gap between two of its items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Gap {
    BetweenItems,
    AfterOpen,
    BeforeClose,
    Empty,
    Continuation,
}

/// The open/close text and source lines of a bracketed construct (call
/// args, params, array/map/set literals). A small struct instead of four
/// parameters, to stay under clippy's argument-count limit.
struct Brackets {
    open: &'static str,
    close: &'static str,
    open_line: u32,
    close_line: u32,
}

pub(crate) struct Printer<'a> {
    out: String,
    at_line_start: bool,
    indent: usize,
    continued: bool,
    map: &'a SourceMap,
}

impl<'a> Printer<'a> {
    pub(crate) fn new(map: &'a SourceMap) -> Self {
        Printer {
            out: String::new(),
            at_line_start: true,
            indent: 0,
            continued: false,
            map,
        }
    }

    pub(crate) fn program(mut self, stmts: &[Stmt]) -> String {
        let mut prev = 0;
        for (i, stmt) in stmts.iter().enumerate() {
            let gap = if i == 0 {
                Gap::AfterOpen
            } else {
                Gap::BetweenItems
            };
            self.line_break(prev, self.map.stmt_first_line(stmt), gap);
            self.print_stmt(stmt);
            prev = self.map.stmt_last_line(stmt);
        }
        let gap = if stmts.is_empty() {
            Gap::Empty
        } else {
            Gap::BeforeClose
        };
        self.line_break(prev, u32::MAX, gap);
        self.out
    }

    // --- Primitives -------------------------------------------------

    fn write(&mut self, s: &str) {
        if self.at_line_start {
            self.out.push_str(&"    ".repeat(self.indent));
            self.at_line_start = false;
        }
        self.out.push_str(s);
    }

    fn write_raw(&mut self, raw: &str) {
        let normalized = raw.replace("\r\n", "\n");
        self.write(&normalized);
    }

    fn newline(&mut self) {
        if self.out.is_empty() {
            return;
        }
        self.out.push('\n');
        self.at_line_start = true;
    }

    /// Comments and blank lines are threaded in here in later units; for
    /// now every call just starts a new line.
    fn line_break(&mut self, _prev: u32, _next: u32, _gap: Gap) {
        self.newline();
    }

    fn nested(&mut self, extra: usize, f: impl FnOnce(&mut Self)) {
        let saved_indent = self.indent;
        let saved_continued = self.continued;
        self.indent += extra;
        self.continued = false;
        f(self);
        self.indent = saved_indent;
        self.continued = saved_continued;
    }

    /// A continuation after an operator: the first break bumps the indent
    /// one level and every later break on the same statement stays there.
    fn continue_line(&mut self, prev: u32, next: u32) {
        if !self.continued {
            self.indent += 1;
            self.continued = true;
        }
        self.line_break(prev, next, Gap::Continuation);
    }

    // --- Composites ---------------------------------------------------

    /// A `{ ... }` body with one item per line: statement bodies, struct
    /// fields, enum variants, impl methods.
    fn braced_lines(
        &mut self,
        open_line: u32,
        close_line: u32,
        spans: &[(u32, u32)],
        mut print_item: impl FnMut(&mut Self, usize),
    ) {
        if spans.is_empty() {
            self.write("{}");
            return;
        }
        self.write("{");
        self.nested(1, |printer| {
            let mut prev = open_line;
            for (i, &(first, last)) in spans.iter().enumerate() {
                let gap = if i == 0 {
                    Gap::AfterOpen
                } else {
                    Gap::BetweenItems
                };
                printer.line_break(prev, first, gap);
                print_item(printer, i);
                prev = last;
            }
            printer.line_break(prev, close_line, Gap::BeforeClose);
        });
        self.write("}");
    }

    /// A bracketed, comma-separated list: call args, params, array/map/set
    /// literals. Broken (one item per line, trailing comma) iff the author
    /// broke it anywhere in the source; otherwise joined inline with `, `.
    fn bracket_list(
        &mut self,
        brackets: Brackets,
        spans: &[(u32, u32)],
        mut print_item: impl FnMut(&mut Self, usize),
    ) {
        self.write(brackets.open);
        if spans.is_empty() {
            self.write(brackets.close);
            return;
        }

        let broken = spans[0].0 > brackets.open_line
            || spans.windows(2).any(|pair| pair[1].0 > pair[0].1)
            || brackets.close_line > spans[spans.len() - 1].1;

        if !broken {
            for i in 0..spans.len() {
                if i > 0 {
                    self.write(", ");
                }
                print_item(self, i);
            }
            self.write(brackets.close);
            return;
        }

        self.nested(1, |printer| {
            let mut prev = brackets.open_line;
            for (i, &(first, last)) in spans.iter().enumerate() {
                let gap = if i == 0 {
                    Gap::AfterOpen
                } else {
                    Gap::BetweenItems
                };
                printer.line_break(prev, first, gap);
                print_item(printer, i);
                printer.write(",");
                prev = last;
            }
            printer.line_break(prev, brackets.close_line, Gap::BeforeClose);
        });
        self.write(brackets.close);
    }

    // --- Statements -----------------------------------------------------

    fn print_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Val {
                name,
                initializer,
                location,
                ..
            }
            | Stmt::Var {
                name,
                initializer,
                location,
                ..
            } => {
                self.write(if matches!(stmt, Stmt::Val { .. }) {
                    "val "
                } else {
                    "var "
                });
                self.write(name);
                if let Some(init) = initializer {
                    self.write(" =");
                    self.write_space_or_continuation(location.line, self.map.first_line(init));
                    self.print_expr(init);
                }
            }
            Stmt::Expression { expr, .. } => self.print_expr(expr),
            Stmt::Return { value, .. } => {
                self.write("return ");
                self.print_expr(value);
            }
            Stmt::Break { .. } => self.write("break"),
            Stmt::Continue { .. } => self.write("continue"),
            Stmt::Fn {
                name,
                params,
                body,
                location,
                ..
            } => {
                self.write("fn ");
                self.write(name);
                self.print_params(location, params);
                self.write(" ");
                self.print_body(location, body);
            }
            Stmt::Block {
                statements,
                location,
            } => self.print_block(location, statements),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.write("if (");
                self.print_expr(condition);
                self.write(") ");
                self.print_stmt(then_branch);
                if let Some(else_stmt) = else_branch {
                    if matches!(then_branch.as_ref(), Stmt::Block { .. }) {
                        self.write(" else ");
                    } else {
                        let then_last_line = self.map.stmt_last_line(then_branch);
                        let else_line = self.map.line(self.map.stmt_last_token(then_branch) + 1);
                        self.line_break(then_last_line, else_line, Gap::Continuation);
                        self.write("else ");
                    }
                    self.print_stmt(else_stmt);
                }
            }
            Stmt::While {
                condition, body, ..
            } => {
                self.write("while (");
                self.print_expr(condition);
                self.write(") ");
                self.print_stmt(body);
            }
            Stmt::For {
                initializer,
                condition,
                increment,
                body,
                ..
            } => {
                self.write("for (");
                self.print_stmt(initializer);
                self.write("; ");
                self.print_expr(condition);
                self.write("; ");
                self.print_expr(increment);
                self.write(") ");
                self.print_stmt(body);
            }
            Stmt::ForIn {
                variable,
                collection,
                body,
                ..
            } => {
                self.write("for (");
                self.write(variable);
                self.write(" in ");
                self.print_expr(collection);
                self.write(") ");
                self.print_stmt(body);
            }
            Stmt::Struct {
                name,
                fields,
                location,
                ..
            } => {
                self.write("struct ");
                self.write(name);
                self.write(" ");
                let (open, close) = self.map.braces_after(location);
                let spans: Vec<(u32, u32)> = fields
                    .iter()
                    .map(|field| (field.location.line, field.location.line))
                    .collect();
                self.braced_lines(
                    self.map.line(open),
                    self.map.line(close),
                    &spans,
                    |printer, i| {
                        printer.write(&fields[i].name);
                    },
                );
            }
            Stmt::Enum {
                name,
                variants,
                location,
                ..
            } => {
                self.write("enum ");
                self.write(name);
                self.write(" ");
                let (open, close) = self.map.braces_after(location);
                let spans: Vec<(u32, u32)> = variants
                    .iter()
                    .map(|variant| (variant.location.line, variant.location.line))
                    .collect();
                self.braced_lines(
                    self.map.line(open),
                    self.map.line(close),
                    &spans,
                    |printer, i| {
                        printer.write(&variants[i].name);
                    },
                );
            }
            Stmt::Impl {
                type_name,
                methods,
                location,
            } => {
                self.write("impl ");
                self.write(type_name);
                self.write(" ");
                let (open, close) = self.map.braces_after(location);
                let spans: Vec<(u32, u32)> = methods
                    .iter()
                    .map(|method| {
                        (
                            self.map.stmt_first_line(method),
                            self.map.stmt_last_line(method),
                        )
                    })
                    .collect();
                self.braced_lines(
                    self.map.line(open),
                    self.map.line(close),
                    &spans,
                    |printer, i| {
                        printer.print_stmt(&methods[i]);
                    },
                );
            }
        }
    }

    fn print_block(&mut self, location: &SourceLocation, statements: &[Stmt]) {
        let open = self.map.at(location);
        let close = self.map.partner(open);
        let spans: Vec<(u32, u32)> = statements
            .iter()
            .map(|stmt| {
                (
                    self.map.stmt_first_line(stmt),
                    self.map.stmt_last_line(stmt),
                )
            })
            .collect();
        let open_line = self.map.line(open);
        let close_line = self.map.line(close);
        self.braced_lines(open_line, close_line, &spans, |printer, i| {
            printer.print_stmt(&statements[i])
        });
    }

    fn print_params(&mut self, location: &SourceLocation, params: &[String]) {
        let fn_tokens = self.map.fn_tokens(location);
        let spans: Vec<(u32, u32)> = self
            .map
            .param_lines(&fn_tokens)
            .into_iter()
            .map(|line| (line, line))
            .collect();
        let brackets = Brackets {
            open: "(",
            close: ")",
            open_line: self.map.line(fn_tokens.params_open),
            close_line: self.map.line(fn_tokens.params_close),
        };
        self.bracket_list(brackets, &spans, |printer, i| printer.write(&params[i]));
    }

    fn print_body(&mut self, location: &SourceLocation, body: &[Stmt]) {
        let fn_tokens = self.map.fn_tokens(location);
        let spans: Vec<(u32, u32)> = body
            .iter()
            .map(|stmt| {
                (
                    self.map.stmt_first_line(stmt),
                    self.map.stmt_last_line(stmt),
                )
            })
            .collect();
        let open_line = self.map.line(fn_tokens.body_open);
        let close_line = self.map.line(fn_tokens.body_close);
        self.braced_lines(open_line, close_line, &spans, |printer, i| {
            printer.print_stmt(&body[i])
        });
    }

    // --- Expressions ------------------------------------------------

    fn print_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number { raw, .. } => self.write_raw(raw),
            Expr::String { raw, .. } => {
                self.write("\"");
                self.write_raw(raw);
                self.write("\"");
            }
            Expr::StringInterpolation { parts, .. } => {
                self.write("\"");
                for part in parts {
                    match part {
                        InterpolationPart::Literal { raw, .. } => self.write_raw(raw),
                        InterpolationPart::Expression(inner) => {
                            self.write("${");
                            self.print_expr(inner);
                            self.write("}");
                        }
                    }
                }
                self.write("\"");
            }
            Expr::Boolean { value, .. } => self.write(if *value { "true" } else { "false" }),
            Expr::Nil { .. } => self.write("nil"),
            Expr::Variable { name, .. } => self.write(name),
            Expr::Assign {
                name,
                value,
                location,
                ..
            } => {
                self.write(name);
                self.write(" =");
                self.write_space_or_continuation(location.line, self.map.first_line(value));
                self.print_expr(value);
            }
            Expr::CompoundAssign {
                name,
                operator,
                value,
                location,
                ..
            } => {
                self.write(name);
                self.write(" ");
                self.write(&compound_op_text(operator));
                self.write_space_or_continuation(location.line, self.map.first_line(value));
                self.print_expr(value);
            }
            Expr::Binary {
                left,
                operator,
                right,
                location,
                ..
            } => {
                self.print_expr(left);
                self.write(" ");
                self.write(binary_op_text(operator));
                self.write_space_or_continuation(location.line, self.map.first_line(right));
                self.print_expr(right);
            }
            Expr::Unary {
                operator, operand, ..
            } => {
                self.write(unary_op_text(operator));
                if matches!(operator, UnaryOp::Negate)
                    && matches!(
                        operand.as_ref(),
                        Expr::Unary {
                            operator: UnaryOp::Negate,
                            ..
                        }
                    )
                {
                    self.write(" ");
                }
                self.print_expr(operand);
            }
            Expr::Call {
                callee,
                arguments,
                location,
                ..
            } => {
                self.print_expr(callee);
                let open = self.map.at(location);
                let close = self.map.partner(open);
                let spans: Vec<(u32, u32)> = arguments
                    .iter()
                    .map(|arg| (self.map.first_line(arg), self.map.last_line(arg)))
                    .collect();
                let brackets = Brackets {
                    open: "(",
                    close: ")",
                    open_line: self.map.line(open),
                    close_line: self.map.line(close),
                };
                self.bracket_list(brackets, &spans, |printer, i| {
                    printer.print_expr(&arguments[i])
                });
            }
            Expr::GetField { object, field, .. } => {
                self.print_expr(object);
                self.write(".");
                self.write(field);
            }
            Expr::SetField {
                object,
                field,
                value,
                location,
                ..
            } => {
                self.print_expr(object);
                self.write(".");
                self.write(field);
                self.write(" =");
                let dot_line = self.map.line(self.map.at(location));
                self.write_space_or_continuation(dot_line, self.map.first_line(value));
                self.print_expr(value);
            }
            Expr::Grouping {
                expr: inner,
                location,
                ..
            } => {
                let close = self.map.at(location);
                let open = self.map.partner(close);
                let open_line = self.map.line(open);
                let close_line = self.map.line(close);
                let inner_first = self.map.first_line(inner);
                let inner_last = self.map.last_line(inner);
                let broken = inner_first > open_line || close_line > inner_last;
                self.write("(");
                if broken {
                    self.nested(1, |printer| {
                        printer.line_break(open_line, inner_first, Gap::AfterOpen);
                        printer.print_expr(inner);
                        printer.line_break(inner_last, close_line, Gap::BeforeClose);
                    });
                } else {
                    self.print_expr(inner);
                }
                self.write(")");
            }
            Expr::MapLiteral {
                entries, location, ..
            } => {
                let open = self.map.at(location);
                let close = self.map.partner(open);
                let spans: Vec<(u32, u32)> = entries
                    .iter()
                    .map(|(key, value)| (self.map.first_line(key), self.map.last_line(value)))
                    .collect();
                let brackets = Brackets {
                    open: "{",
                    close: "}",
                    open_line: self.map.line(open),
                    close_line: self.map.line(close),
                };
                self.bracket_list(brackets, &spans, |printer, i| {
                    let (key, value) = &entries[i];
                    printer.print_map_entry(key, value);
                });
            }
            Expr::ArrayLiteral {
                elements, location, ..
            } => {
                self.print_bracketed_elements("[", "]", elements, location);
            }
            Expr::SetLiteral {
                elements, location, ..
            } => {
                self.print_bracketed_elements("#{", "}", elements, location);
            }
            Expr::Index { object, index, .. } => {
                self.print_expr(object);
                self.write("[");
                self.print_expr(index);
                self.write("]");
            }
            Expr::IndexAssign {
                object,
                index,
                value,
                ..
            } => {
                self.print_expr(object);
                self.write("[");
                self.print_expr(index);
                self.write("] =");
                let index_last_line = self.map.last_line(index);
                self.write_space_or_continuation(index_last_line, self.map.first_line(value));
                self.print_expr(value);
            }
            Expr::Range {
                start,
                end,
                inclusive,
                location,
                ..
            } => {
                self.print_expr(start);
                self.write(if *inclusive { "..=" } else { ".." });
                let end_first_line = self.map.first_line(end);
                if end_first_line > location.line {
                    self.continue_line(location.line, end_first_line);
                }
                self.print_expr(end);
            }
            Expr::PostfixIncrement { operand, .. } => {
                self.print_expr(operand);
                self.write("++");
            }
            Expr::PostfixDecrement { operand, .. } => {
                self.print_expr(operand);
                self.write("--");
            }
            Expr::Conditional {
                condition,
                then_expr,
                else_expr,
                location,
                ..
            } => {
                self.print_expr(condition);
                self.write(" ?");
                self.write_space_or_continuation(location.line, self.map.first_line(then_expr));
                self.print_expr(then_expr);
                let then_last_line = self.map.last_line(then_expr);
                self.write(" :");
                self.write_space_or_continuation(then_last_line, self.map.first_line(else_expr));
                self.print_expr(else_expr);
            }
            Expr::Function {
                params,
                body,
                location,
                ..
            } => {
                self.write("fn");
                self.print_params(location, params);
                self.write(" ");
                self.print_body(location, body);
            }
        }
    }

    fn print_map_entry(&mut self, key: &Expr, value: &Expr) {
        self.print_expr(key);
        self.write(":");
        let key_last_line = self.map.last_line(key);
        self.write_space_or_continuation(key_last_line, self.map.first_line(value));
        self.print_expr(value);
    }

    fn print_bracketed_elements(
        &mut self,
        open: &'static str,
        close: &'static str,
        elements: &[Expr],
        location: &SourceLocation,
    ) {
        let open_token = self.map.at(location);
        let close_token = self.map.partner(open_token);
        let spans: Vec<(u32, u32)> = elements
            .iter()
            .map(|element| (self.map.first_line(element), self.map.last_line(element)))
            .collect();
        let brackets = Brackets {
            open,
            close,
            open_line: self.map.line(open_token),
            close_line: self.map.line(close_token),
        };
        self.bracket_list(brackets, &spans, |printer, i| {
            printer.print_expr(&elements[i])
        });
    }

    /// Writes a single space, unless the value/operand starts on a later
    /// source line than `prev_line`, in which case it continues on a new
    /// indented line instead.
    fn write_space_or_continuation(&mut self, prev_line: u32, next_line: u32) {
        if next_line > prev_line {
            self.continue_line(prev_line, next_line);
        } else {
            self.write(" ");
        }
    }
}

fn binary_op_text(op: &BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "+",
        BinaryOp::Subtract => "-",
        BinaryOp::Multiply => "*",
        BinaryOp::Divide => "/",
        BinaryOp::Modulo => "%",
        BinaryOp::Exponent => "**",
        BinaryOp::Equal => "==",
        BinaryOp::NotEqual => "!=",
        BinaryOp::Greater => ">",
        BinaryOp::GreaterEqual => ">=",
        BinaryOp::Less => "<",
        BinaryOp::LessEqual => "<=",
        BinaryOp::And => "&&",
        BinaryOp::Or => "||",
        BinaryOp::BitwiseAnd => "&",
        BinaryOp::BitwiseOr => "|",
        BinaryOp::BitwiseXor => "^",
        BinaryOp::LeftShift => "<<",
        BinaryOp::RightShift => ">>",
    }
}

fn compound_op_text(op: &BinaryOp) -> String {
    format!("{}=", binary_op_text(op))
}

fn unary_op_text(op: &UnaryOp) -> &'static str {
    match op {
        UnaryOp::Negate => "-",
        UnaryOp::Not => "!",
        UnaryOp::BitwiseNot => "~",
    }
}
