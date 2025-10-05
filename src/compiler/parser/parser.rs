use crate::common::opcodes::OpCode;
use crate::common::{Brick, Value};
use crate::compiler::parser::rules::{ParseRule, Precedence, PARSE_RULES};
use crate::compiler::token::TokenType;
use crate::compiler::{Parser, Scanner, Token};

use std::rc::Rc;
use std::str::FromStr;

use crate::{current_brick_mut, number, string};

#[cfg(feature = "disassemble")]
use tracing_attributes::instrument;

impl Parser {
    pub(in crate::compiler) fn new(source: String) -> Parser {
        Parser {
            scanner: Scanner::new(source),
            bricks: Vec::default(),
            previous_token: Token::default(),
            current_token: Token::default(),
            scope_depth: 0,
            had_error: false,
            panic_mode: false,
            compilation_errors: String::new(),
        }
    }

    pub(in crate::compiler) fn start(&mut self) {
        self.bricks.push(Brick::new(
            format!("Brick no. {}", self.bricks.len()).as_str(),
        ));
    }

    pub(in crate::compiler) fn end(&mut self) {
        self.emit_return();
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(in crate::compiler) fn advance(&mut self) {
        std::mem::swap(&mut self.previous_token, &mut self.current_token);
        loop {
            self.current_token = self.scanner.scan_token();
            if self.current_token.token_type != TokenType::Error {
                break;
            }
            self.report_error_at_current(self.current_token.token.clone().as_str());
        }
    }

    pub(in crate::compiler) fn match_token(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            return false;
        }
        self.advance();
        true
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(in crate::compiler) fn declaration(&mut self) {
        if self.match_token(TokenType::Val) {
            self.val_declaration();
        } else if self.match_token(TokenType::Var) {
            self.var_declaration();
        } else if self.match_token(TokenType::Fn) {
            self.fn_declaration();
        } else {
            self.statement();
        }

        if self.panic_mode {
            self.exit_panic_mode();
        }
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    fn val_declaration(&mut self) {
        let name = self.parse_value();

        if self.match_token(TokenType::Equal) {
            self.expression(false);
        } else {
            self.emit_op_code(OpCode::Nil);
        }

        self.consume_either(
            TokenType::NewLine,
            TokenType::Eof,
            "Expecting '\\n' or '\\0' after value declaration.",
        );

        self.define_value(name);
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    fn var_declaration(&mut self) {
        let name = self.parse_value();

        if self.match_token(TokenType::Equal) {
            self.expression(false);
        } else {
            self.emit_op_code(OpCode::Nil);
        }

        self.consume_either(
            TokenType::NewLine,
            TokenType::Eof,
            "Expecting '\\n' or '\\0' after value declaration.",
        );

        self.define_variable(name);
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    fn fn_declaration(&mut self) {
        let name = self.parse_value();

        self.consume(TokenType::LeftParen, "Expect '(' after function name.");

        // Parse parameters
        let mut arity = 0u8;
        let mut parameters = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if arity == 255 {
                    self.report_error_at_current("Can't have more than 255 parameters.");
                }

                self.consume(TokenType::Identifier, "Expect parameter name.");
                let param_name = self.previous_token.token.clone();
                parameters.push(param_name);
                arity += 1;

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters.");
        self.consume(TokenType::LeftBrace, "Expect '{' before function body.");

        // For recursion: define the function name with a placeholder (nil) before compiling the body
        // This allows the function to reference itself
        self.emit_op_code(OpCode::Nil);
        self.define_value(name.clone());

        // Create a new brick for the function
        self.bricks.push(Brick::new(&format!("function_{}", name)));

        // Compile function body in the new brick
        self.begin_scope();

        // Define parameters as local variables in the function scope
        // Parameters are already on the stack (pushed by the caller)
        for param in parameters {
            self.define_parameter(param);
        }

        self.block();
        self.end_scope();

        // Emit return at end of function
        self.emit_return();

        let function_value = Value::Object(Rc::new(crate::common::Object::Function(Rc::new(
            crate::common::ObjFunction {
                name: name.clone(),
                arity,
                brick: Rc::new(self.bricks.pop().unwrap()),
            },
        ))));

        // Now replace the nil placeholder with the actual function
        self.emit_constant(function_value);

        // Get the index of the function variable we defined earlier
        let (index, _is_mutable, is_global) = self.get_variable_index(&name);
        let index = index.unwrap();

        // Emit the appropriate Set opcode to update the placeholder
        if is_global {
            self.emit_op_code_variant(OpCode::SetGlobal, index);
        } else {
            self.emit_op_code_variant(OpCode::SetValue, index);
        }
        self.emit_op_code(OpCode::Pop); // Pop the function value from the stack
    }

    fn parse_value(&mut self) -> String {
        self.consume(TokenType::Identifier, "Expecting variable name.");
        self.previous_token.token.clone()
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    fn statement(&mut self) {
        if self.match_token(TokenType::Print) {
            self.print_statement();
        } else if self.match_token(TokenType::LeftBrace) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else if self.match_token(TokenType::If) {
            self.if_statement();
        } else if self.match_token(TokenType::While) {
            self.while_statement();
        } else if self.match_token(TokenType::Return) {
            self.return_statement();
        } else {
            self.expression_statement();
        }
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    fn expression(&mut self, skip_new_lines: bool) {
        self.parse_precedence(Precedence::Assignment, skip_new_lines);
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self, precedence)))]
    fn parse_precedence(&mut self, precedence: Precedence, skip_new_lines: bool) {
        if skip_new_lines {
            while self.check(TokenType::NewLine) {
                self.advance();
            }
        }

        self.advance();
        let rule = self.get_rule(self.previous_token.token_type.clone());

        if rule.prefix.is_none() {
            self.report_error_at_current("Expect expression");
            return;
        }
        rule.prefix.unwrap()(self);

        while precedence as u8
            <= self
                .get_rule(self.current_token.token_type.clone())
                .precedence as u8
        {
            self.advance();
            let infix_rule = self.get_rule(self.previous_token.token_type.clone()).infix;
            infix_rule.unwrap()(self);
        }

        if skip_new_lines {
            while self.check(TokenType::NewLine) {
                self.advance();
            }
        }
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(super) fn variable(&mut self) {
        let name = &*self.previous_token.token.clone();
        let (maybe_index, is_mutable, is_global) = self.get_variable_index(name);
        if maybe_index.is_none() {
            self.report_error_at_current(&format!("Undefined variable '{}'.", name));
            return;
        }

        let is_assignment = self.match_token(TokenType::Equal);
        if is_assignment && is_mutable {
            self.expression(false);
        } else if is_assignment {
            self.report_error_at_current("Can't assign to an immutable variable.");
            return;
        }

        let index = maybe_index.unwrap();
        if is_assignment {
            if is_global {
                self.emit_op_code_variant(OpCode::SetGlobal, index);
            } else {
                self.emit_op_code_variant(OpCode::SetVariable, index);
            }
        } else if is_global {
            self.emit_op_code_variant(OpCode::GetGlobal, index);
        } else {
            self.emit_op_code_variant(OpCode::GetVariable, index);
        }
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(super) fn number(&mut self) {
        let value = f64::from_str(&self.previous_token.token).unwrap();
        self.emit_constant(number!(value));
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(super) fn string(&mut self) {
        let value = &*self.previous_token.token;
        let string = &value[1..value.len() - 1];
        self.emit_string(string!(string));
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(super) fn grouping(&mut self) {
        self.expression(true);
        self.consume(TokenType::RightParen, "Expect end of expression");
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(super) fn binary(&mut self) {
        let operator_type = self.previous_token.token_type.clone();
        let rule = self.get_rule(operator_type.clone());
        self.parse_precedence(Precedence::from_u8(rule.precedence as u8 + 1), false);

        match operator_type {
            token_type if token_type == TokenType::BangEqual => {
                self.emit_op_codes(OpCode::Equal, OpCode::Not)
            }
            token_type if token_type == TokenType::EqualEqual => self.emit_op_code(OpCode::Equal),
            token_type if token_type == TokenType::Greater => self.emit_op_code(OpCode::Greater),
            token_type if token_type == TokenType::GreaterEqual => {
                self.emit_op_codes(OpCode::Less, OpCode::Not)
            }
            token_type if token_type == TokenType::Less => self.emit_op_code(OpCode::Less),
            token_type if token_type == TokenType::LessEqual => {
                self.emit_op_codes(OpCode::Greater, OpCode::Not)
            }
            token_type if token_type == TokenType::Plus => self.emit_op_code(OpCode::Add),
            token_type if token_type == TokenType::Minus => self.emit_op_code(OpCode::Subtract),
            token_type if token_type == TokenType::Star => self.emit_op_code(OpCode::Multiply),
            token_type if token_type == TokenType::Slash => self.emit_op_code(OpCode::Divide),
            _ => (), // Unreachable.
        }
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(super) fn literal(&mut self) {
        match self.previous_token.token_type {
            TokenType::False => self.emit_op_code(OpCode::False),
            TokenType::Nil => self.emit_op_code(OpCode::Nil),
            TokenType::True => self.emit_op_code(OpCode::True),
            _ => (), // Unreachable.
        }
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(super) fn unary(&mut self) {
        let operator_type = self.previous_token.token_type.clone();

        // Compile the operand.
        self.parse_precedence(Precedence::Unary, false);

        // Emit the operator instruction.
        match operator_type {
            TokenType::Bang => self.emit_op_code(OpCode::Not),
            TokenType::Minus => self.emit_op_code(OpCode::Negate),
            _ => (), // Unreachable.
        }
    }

    #[cfg_attr(feature = "disassemble", instrument(skip(self)))]
    pub(super) fn call(&mut self) {
        let arg_count = self.argument_list();
        self.emit_call(arg_count);
    }

    fn argument_list(&mut self) -> u8 {
        let mut arg_count = 0u8;
        if !self.check(TokenType::RightParen) {
            loop {
                self.expression(false);
                if arg_count == 255 {
                    self.report_error_at_current("Can't have more than 255 arguments.");
                }
                arg_count += 1;
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after arguments.");
        arg_count
    }

    pub(in crate::compiler) fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current_token.token_type == token_type {
            self.advance();
            return;
        }

        self.report_error_at_current(message);
    }

    pub(in crate::compiler) fn consume_either(
        &mut self,
        token_type_1: TokenType,
        token_type_2: TokenType,
        message: &str,
    ) {
        if self.current_token.token_type == token_type_1 {
            self.advance();
            return;
        } else if self.current_token.token_type == token_type_2 {
            self.advance();
            return;
        }

        self.report_error_at_current(message);
    }

    fn get_rule(&self, token_type: TokenType) -> &ParseRule {
        #[cfg(feature = "disassemble")]
        return self.get_rule_safe(token_type);
        #[cfg(not(feature = "disassemble"))]
        &(PARSE_RULES.get((token_type as u32) as usize).unwrap().1)
    }

    #[cfg(feature = "disassemble")]
    fn get_rule_safe(&self, token_type: TokenType) -> &ParseRule {
        let parse_rule = PARSE_RULES
            .get((token_type.clone() as u8) as usize)
            .unwrap();
        if parse_rule.0 != token_type {
            panic!("Parsing rules are out of sync with token types.");
        }
        &parse_rule.1
    }

    fn print_statement(&mut self) {
        self.expression(false);
        self.consume_either(
            TokenType::NewLine,
            TokenType::Eof,
            "Expecting '\\n' or '\\0' at end of statement.",
        );
        self.emit_op_code(OpCode::Print);
    }

    fn if_statement(&mut self) {
        self.consume(TokenType::LeftParen, "Expecting '(' after 'if'.");
        self.expression(false);
        self.consume(TokenType::RightParen, "Expecting ')' after condition.");

        let then_jump = self.emit_jump(OpCode::JumpIfFalse);
        self.statement();
        let else_jump = self.emit_jump(OpCode::Jump);
        self.patch_jump(then_jump);

        if self.match_token(TokenType::Else) {
            self.statement();
        }
        self.patch_jump(else_jump);
    }

    fn expression_statement(&mut self) {
        self.expression(false);
        self.consume_either(
            TokenType::NewLine,
            TokenType::Eof,
            "Expecting '\\n' or '\\0' at end of expression.",
        );
        self.emit_op_code(OpCode::Pop);
    }

    fn while_statement(&mut self) {
        let loop_start = current_brick_mut!(self.bricks).instruction_count() as u32;

        self.consume(TokenType::LeftParen, "Expecting '(' after 'while'.");
        self.expression(false);
        self.consume(TokenType::RightParen, "Expecting ')' after condition.");

        let exit_jump = self.emit_jump(OpCode::JumpIfFalse);
        self.emit_op_code(OpCode::Pop); // Pop the condition value for the true case
        self.statement();
        self.emit_loop(loop_start);

        self.patch_jump(exit_jump);
    }

    fn return_statement(&mut self) {
        // Parse the return value expression
        self.expression(false);
        self.consume_either(
            TokenType::NewLine,
            TokenType::Eof,
            "Expecting '\\n' or '\\0' at end of statement.",
        );
        self.emit_op_code(OpCode::Return);
    }

    fn report_error_at_current(&mut self, message: &str) {
        self.report_error(&self.current_token.clone(), message);
    }

    fn report_error(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }

        self.had_error = true;
        self.panic_mode = true;

        let mut error = format!("[{}:{}] Error", token.line, token.column);
        if token.token_type == TokenType::Eof {
            error += " at end";
        } else if token.token_type == TokenType::Error {
            // Nothing.
        } else {
            error += format!(" at {:?}", token.token).as_str();
        }
        error += format!(": {}", message).as_str();

        self.compilation_errors = error.to_string();
        eprintln!("{}", message);
    }

    fn exit_panic_mode(&mut self) {
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

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;
    }

    fn block(&mut self) {
        self.skip_new_lines();
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            self.declaration();
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.");
        if !self.check(TokenType::Else) {
            self.consume_either(
                TokenType::NewLine,
                TokenType::Eof,
                "Expecting '\\n' or '\\0' at end of block.",
            );
        }
    }

    fn skip_new_lines(&mut self) {
        while self.check(TokenType::NewLine) {
            self.advance();
        }
    }

    fn get_variable_index(&self, name: &str) -> (Option<u32>, bool, bool) {
        // Returns: (index, is_mutable, is_global)
        // is_global = true if variable is in a parent brick (not current brick)
        let current_brick_idx = self.bricks.len() - 1;

        for (brick_idx, brick) in self.bricks.iter().enumerate().rev() {
            let index = brick.get_variable_index(name);
            if index.0.is_some() {
                let is_global = brick_idx < current_brick_idx;
                return (index.0, index.1, is_global);
            }
        }
        (None, false, false)
    }
}
