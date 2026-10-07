use crate::common::errors::{CompilationError, CompilationErrorKind, CompilationPhase};
use crate::common::SourceLocation;
use crate::compiler::ast::{
    BinaryOp, Expr, IfExprElse, InterpolationPart, MatchArm, MatchArmBody, MatchPattern, Pattern,
    Stmt, UnaryOp,
};
use crate::compiler::formatter::source_map::SourceMap;
use crate::compiler::{Comment, CommentKind, Trivia};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Gap {
    BetweenItems,
    AfterOpen,
    BeforeClose,
    Empty,
    Continuation,
}

impl Gap {
    fn keeps_leading(self) -> bool {
        matches!(self, Gap::BetweenItems | Gap::BeforeClose)
    }

    fn keeps_trailing(self) -> bool {
        matches!(self, Gap::BetweenItems | Gap::AfterOpen)
    }
}

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
    blank_lines: &'a [u32],
    comments: &'a [Comment],
    next_comment: usize,
    error: Option<CompilationError>,
}

impl<'a> Printer<'a> {
    pub(crate) fn new(map: &'a SourceMap, trivia: &'a Trivia) -> Self {
        Printer {
            out: String::new(),
            at_line_start: true,
            indent: 0,
            continued: false,
            map,
            blank_lines: &trivia.blank_lines,
            comments: &trivia.comments,
            next_comment: 0,
            error: None,
        }
    }

    pub(crate) fn program(mut self, stmts: &[Stmt]) -> Result<String, Vec<CompilationError>> {
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
        while self.next_comment < self.comments.len() {
            let comment = self.comments[self.next_comment].clone();
            self.record_unplaceable(&comment);
            self.next_comment += 1;
        }
        match self.error {
            Some(e) => Err(vec![e]),
            None => Ok(self.out),
        }
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

    fn blank(&mut self) {
        if self.out.is_empty() || self.out.ends_with("\n\n") {
            return;
        }
        self.out.push('\n');
    }

    fn has_blank_between(&self, a: u32, b: u32) -> bool {
        self.blank_lines.iter().any(|&line| line > a && line < b)
    }

    fn record_unplaceable(&mut self, comment: &Comment) {
        if self.error.is_none() {
            self.error = Some(CompilationError::new(
                CompilationPhase::Format,
                CompilationErrorKind::UnplaceableComment,
                format!("Cannot place the comment on line {}", comment.line),
                SourceLocation {
                    offset: 0,
                    line: comment.line,
                    column: comment.column,
                },
            ));
        }
    }

    /// The only place comments and blank lines are emitted. `prev`/`next`
    /// are the source lines of the last token printed and the next one to
    /// print; a synthetic break (`next <= prev`, the printer expanding a
    /// one-line body) never flushes a pending comment, since nothing in
    /// the source separates them.
    fn line_break(&mut self, prev: u32, next: u32, gap: Gap) {
        while self.next_comment < self.comments.len()
            && self.comments[self.next_comment].line < prev
        {
            let comment = self.comments[self.next_comment].clone();
            self.record_unplaceable(&comment);
            self.next_comment += 1;
        }

        if next <= prev {
            self.newline();
            return;
        }

        if let Some(comment) = self.comments.get(self.next_comment) {
            if comment.kind == CommentKind::Trailing && comment.line == prev {
                let text = comment.text.trim_end().to_string();
                if !self.at_line_start {
                    self.write(" ");
                }
                self.write(&text);
                self.next_comment += 1;
            }
        }

        self.newline();

        let mut last = prev;
        let mut first = true;
        while let Some(comment) = self.comments.get(self.next_comment) {
            if comment.line >= next {
                break;
            }
            if comment.kind == CommentKind::Trailing {
                let comment = comment.clone();
                self.record_unplaceable(&comment);
                self.next_comment += 1;
                continue;
            }
            let line = comment.line;
            if self.has_blank_between(last, line) && (!first || gap.keeps_leading()) {
                self.blank();
            }
            let text = comment.text.trim_end().to_string();
            self.write(&text);
            self.newline();
            last = line;
            first = false;
            self.next_comment += 1;
        }

        if self.has_blank_between(last, next)
            && gap.keeps_trailing()
            && (!first || gap.keeps_leading())
        {
            self.blank();
        }
    }

    fn has_comment_before(&self, line: u32) -> bool {
        self.comments
            .get(self.next_comment)
            .is_some_and(|c| c.line < line)
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
        print_item: impl FnMut(&mut Self, usize),
    ) {
        if spans.is_empty() && !self.has_comment_before(close_line) {
            self.write("{}");
            return;
        }
        self.write("{");
        self.braced_body(open_line, close_line, spans, print_item);
        self.write("}");
    }

    /// The line-broken items between an already-written `{` and `}`: one
    /// item per line, shared by `braced_lines` and a trailing block's own
    /// brace printing.
    fn braced_body(
        &mut self,
        open_line: u32,
        close_line: u32,
        spans: &[(u32, u32)],
        mut print_item: impl FnMut(&mut Self, usize),
    ) {
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
            let gap = if spans.is_empty() {
                Gap::Empty
            } else {
                Gap::BeforeClose
            };
            printer.line_break(prev, close_line, gap);
        });
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
            if self.has_comment_before(brackets.close_line) {
                self.nested(1, |printer| {
                    printer.line_break(brackets.open_line, brackets.close_line, Gap::Empty);
                });
            }
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
                self.nested(0, |printer| print_item(printer, i));
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
                printer.nested(0, |printer| print_item(printer, i));
                printer.write(",");
                prev = last;
            }
            printer.line_break(prev, brackets.close_line, Gap::BeforeClose);
        });
        self.write(brackets.close);
    }

    // --- Statements -----------------------------------------------------

    fn print_stmt(&mut self, stmt: &Stmt) {
        self.nested(0, |printer| printer.print_stmt_body(stmt));
    }

    fn write_pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Name(binding) => self.write(&binding.name),
            Pattern::Tuple(slots) => {
                self.write("(");
                for (i, slot) in slots.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.write(slot.as_ref().map_or("_", |b| b.name.as_str()));
                }
                self.write(")");
            }
        }
    }

    fn print_stmt_body(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Val {
                pattern,
                initializer,
                location,
            }
            | Stmt::Var {
                pattern,
                initializer,
                location,
            } => {
                let is_val = matches!(stmt, Stmt::Val { .. });
                self.write(if is_val { "val " } else { "var " });
                self.write_pattern(pattern);
                if let Some(init) = initializer {
                    self.write(" =");
                    self.write_space_or_continuation(location.line, self.map.first_line(init));
                    self.print_expr(init);
                }
            }
            Stmt::Expression { expr, .. } => self.print_expr(expr),
            Stmt::Return { value, .. } => match value {
                Some(value) => {
                    self.write("return ");
                    self.print_expr(value);
                }
                None => self.write("return"),
            },
            Stmt::Import {
                raw_path, alias, ..
            } => {
                self.write(&format!("use \"{}\"", raw_path));
                if let Some(alias) = alias {
                    self.write(&format!(" as {}", alias));
                }
            }
            Stmt::Export { declaration, .. } => {
                self.write("pub ");
                self.print_stmt_body(declaration);
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
                if self.map.is_expr_bodied_fn(location) {
                    let expr = match &body[0] {
                        Stmt::Expression { expr, .. } => expr,
                        _ => unreachable!("expression-bodied fn body is a single expression"),
                    };
                    self.write(" =");
                    let params_close_line = self.map.line(self.map.params_tokens(location).close);
                    self.write_space_or_continuation(params_close_line, self.map.first_line(expr));
                    self.print_expr(expr);
                } else {
                    self.write(" ");
                    self.print_body(location, body);
                }
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
                self.write("if ");
                self.nested(0, |printer| printer.print_condition(condition));
                self.write(" ");
                self.print_stmt(then_branch);
                if let Some(else_stmt) = else_branch {
                    self.write(" else ");
                    self.print_stmt(else_stmt);
                }
            }
            Stmt::While {
                condition, body, ..
            } => {
                self.write("while ");
                self.nested(0, |printer| printer.print_condition(condition));
                self.write(" ");
                self.print_stmt(body);
            }
            Stmt::ForIn {
                pattern,
                collection,
                body,
                ..
            } => {
                self.write("for ");
                self.write_pattern(pattern);
                self.write(" in ");
                self.nested(0, |printer| printer.print_condition(collection));
                self.write(" ");
                self.print_stmt(body);
            }
            Stmt::Struct {
                name,
                fields,
                location,
                ..
            } => {
                let names: Vec<(String, u32)> = fields
                    .iter()
                    .map(|field| (field.name.clone(), field.location.line))
                    .collect();
                self.print_named_braces("struct ", name, location, &names);
            }
            Stmt::Enum {
                name,
                variants,
                location,
                ..
            } => {
                let names: Vec<(String, u32)> = variants
                    .iter()
                    .map(|variant| {
                        let text = if variant.fields.is_empty() {
                            variant.name.clone()
                        } else {
                            format!("{}({})", variant.name, variant.fields.join(", "))
                        };
                        (text, variant.location.line)
                    })
                    .collect();
                self.print_named_braces("enum ", name, location, &names);
            }
            Stmt::Impl {
                module_name,
                type_name,
                methods,
                location,
                ..
            } => {
                self.write("impl ");
                if let Some(module_name) = module_name {
                    self.write(module_name);
                    self.write(".");
                }
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
        self.print_stmts_braced(open, close, statements);
    }

    /// A `{ ... }` body of statements between two already-located brace
    /// tokens: block statements and function bodies.
    fn print_stmts_braced(&mut self, open: usize, close: usize, statements: &[Stmt]) {
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

    /// A `keyword name { item, item, ... }` declaration: struct fields and
    /// enum variants, each printed as its bare name.
    fn print_named_braces(
        &mut self,
        keyword: &str,
        name: &str,
        location: &SourceLocation,
        items: &[(String, u32)],
    ) {
        self.write(keyword);
        self.write(name);
        self.write(" ");
        let (open, close) = self.map.braces_after(location);
        let spans: Vec<(u32, u32)> = items.iter().map(|&(_, line)| (line, line)).collect();
        self.braced_lines(
            self.map.line(open),
            self.map.line(close),
            &spans,
            |printer, i| {
                printer.write(&items[i].0);
            },
        );
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
        self.print_stmts_braced(fn_tokens.body_open, fn_tokens.body_close, body);
    }

    /// A trailing-block lambda's `{ params -> body }`, between its own
    /// already-located brace tokens. A block whose source `{`...`}` is on
    /// one line and whose body is a single statement prints on one line
    /// too; an empty body always collapses to `{}` (or `{ params -> }`),
    /// like `braced_lines`; anything else stays multi-line.
    fn print_trailing_block(
        &mut self,
        params: &[String],
        body: &[Stmt],
        open: usize,
        close: usize,
    ) {
        let open_line = self.map.line(open);
        let close_line = self.map.line(close);

        if body.is_empty() && !self.has_comment_before(close_line) {
            self.write("{");
            self.print_trailing_block_params(params);
            if !params.is_empty() {
                self.write(" ");
            }
            self.write("}");
            return;
        }

        if open_line == close_line {
            if let [stmt] = body {
                self.write("{");
                self.print_trailing_block_params(params);
                self.write(" ");
                self.print_stmt(stmt);
                self.write(" }");
                return;
            }
        }

        self.write("{");
        self.print_trailing_block_params(params);
        let spans: Vec<(u32, u32)> = body
            .iter()
            .map(|stmt| {
                (
                    self.map.stmt_first_line(stmt),
                    self.map.stmt_last_line(stmt),
                )
            })
            .collect();
        self.braced_body(open_line, close_line, &spans, |printer, i| {
            printer.print_stmt(&body[i]);
        });
        self.write("}");
    }

    fn print_trailing_block_params(&mut self, params: &[String]) {
        if !params.is_empty() {
            self.write(" ");
            self.write(&params.join(", "));
            self.write(" ->");
        }
    }

    // --- Expressions ------------------------------------------------

    /// Strips every layer of `(expr)` grouping around an if/while condition
    /// or for-in collection.
    fn print_condition(&mut self, expr: &Expr) {
        match expr {
            Expr::Grouping {
                expr: inner,
                location,
            } => {
                let close_line = self.map.line(self.map.at(location));
                if self.has_comment_before(close_line) || self.has_top_level_trailing_block(inner) {
                    self.print_expr(expr);
                } else {
                    self.print_condition(inner);
                }
            }
            _ => self.print_expr(expr),
        }
    }

    /// Whether `expr` contains a trailing-block call reachable without
    /// crossing its own `(...)` or `[...]`.
    fn has_top_level_trailing_block(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Call {
                callee, arguments, ..
            } => {
                let is_trailing_block = matches!(
                    arguments.last(),
                    Some(Expr::Function { location, .. }) if self.map.is_block_lambda(location)
                );
                is_trailing_block || self.has_top_level_trailing_block(callee)
            }
            Expr::GetField { object, .. } | Expr::SetField { object, .. } => {
                self.has_top_level_trailing_block(object)
            }
            Expr::Index { object, .. } => self.has_top_level_trailing_block(object),
            Expr::Binary { left, right, .. } => {
                self.has_top_level_trailing_block(left) || self.has_top_level_trailing_block(right)
            }
            Expr::Unary { operand, .. } => self.has_top_level_trailing_block(operand),
            Expr::Range { start, end, .. } => {
                self.has_top_level_trailing_block(start) || self.has_top_level_trailing_block(end)
            }
            Expr::Conditional {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                self.has_top_level_trailing_block(condition)
                    || self.has_top_level_trailing_block(then_expr)
                    || self.has_top_level_trailing_block(else_expr)
            }
            _ => false,
        }
    }

    fn print_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number { raw, .. } | Expr::Int { raw, .. } => self.write_raw(raw),
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

                let trailing_block = match arguments.last() {
                    Some(Expr::Function {
                        location: fn_location,
                        ..
                    }) if self.map.is_block_lambda(fn_location) => Some(fn_location),
                    _ => None,
                };
                let regular_args = match trailing_block {
                    Some(_) => &arguments[..arguments.len() - 1],
                    None => arguments.as_slice(),
                };

                if trailing_block.is_none() || !regular_args.is_empty() {
                    let open = self.map.at(location);
                    let close = self.map.partner(open);
                    let spans: Vec<(u32, u32)> = regular_args
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
                        printer.print_expr(&regular_args[i])
                    });
                }

                if let (Some(fn_location), Some(Expr::Function { params, body, .. })) =
                    (trailing_block, arguments.last())
                {
                    self.write(" ");
                    let open = self.map.at(fn_location);
                    let close = self.map.partner(open);
                    self.print_trailing_block(params, body, open, close);
                }
            }
            Expr::GetField {
                object,
                field,
                optional,
                location,
            } => {
                self.print_expr(object);
                let object_line = self.map.last_line(object);
                if location.line > object_line {
                    self.continue_line(object_line, location.line);
                }
                self.write(if *optional { "?." } else { "." });
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
                let object_line = self.map.last_line(object);
                if location.line > object_line {
                    self.continue_line(object_line, location.line);
                }
                self.write(".");
                self.write(field);
                self.write(" =");
                let dot_line = self.map.line(self.map.at(location));
                self.write_space_or_continuation(dot_line, self.map.first_line(value));
                self.print_expr(value);
            }
            Expr::CompoundAssignField {
                object,
                field,
                operator,
                value,
                location,
                ..
            } => {
                self.print_expr(object);
                let object_line = self.map.last_line(object);
                if location.line > object_line {
                    self.continue_line(object_line, location.line);
                }
                self.write(".");
                self.write(field);
                self.write(" ");
                self.write(&compound_op_text(operator));
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
                    self.nested(0, |printer| printer.print_expr(inner));
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
            Expr::CompoundAssignIndex {
                object,
                index,
                operator,
                value,
                ..
            } => {
                self.print_expr(object);
                self.write("[");
                self.print_expr(index);
                self.write("] ");
                self.write(&compound_op_text(operator));
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
            Expr::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.write("if ");
                self.nested(0, |printer| printer.print_condition(condition));
                self.write(" ");
                self.print_stmt(then_branch);
                self.write(" else ");
                match else_branch.as_ref() {
                    IfExprElse::If(expr) => self.print_expr(expr),
                    IfExprElse::Block(stmt) => self.print_stmt(stmt),
                }
            }
            Expr::Match {
                scrutinee, arms, ..
            } => {
                self.write("match ");
                self.nested(0, |printer| printer.print_condition(scrutinee));
                self.write(" ");
                self.print_match_arms(scrutinee, arms);
            }
        }
    }

    /// A match expression's `{ arm, arm, ... }`, one arm per line.
    fn print_match_arms(&mut self, scrutinee: &Expr, arms: &[MatchArm]) {
        let (open, close) = self.map.match_arms_braces(scrutinee);
        let spans: Vec<(u32, u32)> = arms
            .iter()
            .map(|arm| {
                let last_line = match &arm.body {
                    MatchArmBody::Expr(expr) => self.map.last_line(expr),
                    MatchArmBody::Block(stmt) => self.map.stmt_last_line(stmt),
                };
                (arm.location.line, last_line)
            })
            .collect();
        self.braced_lines(
            self.map.line(open),
            self.map.line(close),
            &spans,
            |printer, i| printer.print_match_arm(&arms[i]),
        );
    }

    fn print_match_pattern(&mut self, pattern: &MatchPattern) {
        match pattern {
            MatchPattern::Expr(expr) => self.print_expr(expr),
            MatchPattern::Wildcard(_) => self.write("_"),
            MatchPattern::Binding(binding) => self.write(&binding.name),
            MatchPattern::Rest { binding, .. } => {
                self.write("..");
                if let Some(binding) = binding {
                    self.write(&binding.name);
                }
            }
            MatchPattern::Variant { target, fields } => {
                self.print_expr(target);
                self.write("(");
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.print_match_pattern(field);
                }
                self.write(")");
            }
            MatchPattern::Array { elements, .. } => {
                self.write("[");
                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.print_match_pattern(element);
                }
                self.write("]");
            }
        }
    }

    fn print_match_arm(&mut self, arm: &MatchArm) {
        for (i, pattern) in arm.patterns.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.print_match_pattern(pattern);
        }
        if let Some(guard) = &arm.guard {
            self.write(" if ");
            self.print_expr(guard);
        }
        self.write(" -> ");
        match &arm.body {
            MatchArmBody::Expr(expr) => self.print_expr(expr),
            MatchArmBody::Block(stmt) => self.print_stmt(stmt),
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
        BinaryOp::NilCoalesce => "??",
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
