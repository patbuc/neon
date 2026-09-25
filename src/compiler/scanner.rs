use crate::compiler::token::TokenType;
use crate::compiler::{Scanner, Token};

#[derive(Clone, Copy)]
struct InvalidDigit {
    matches: fn(char) -> bool,
    message: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    line: u32,
    column: u32,
    offset: usize,
}

/// One open `${...}` interpolation. `brace_depth` counts unclosed `{`/`#{`
/// opened since the `${`, so a nested brace expression's `}` doesn't end it.
#[derive(Debug)]
pub(in crate::compiler) struct Interpolation {
    brace_depth: usize,
    dollar: Position,
    quote: Position,
}

/// `chars` starts right after the `\`; the returned consumed count excludes it.
fn decode_escape(chars: &[char]) -> Option<(char, usize)> {
    match *chars.first()? {
        'n' => Some(('\n', 1)),
        't' => Some(('\t', 1)),
        'r' => Some(('\r', 1)),
        '\\' => Some(('\\', 1)),
        '"' => Some(('"', 1)),
        '$' => Some(('$', 1)),
        'u' => {
            if chars.get(1) != Some(&'{') {
                return None;
            }
            let mut end = 2;
            while end < 8 && chars.get(end).is_some_and(char::is_ascii_hexdigit) {
                end += 1;
            }
            if chars.get(end) != Some(&'}') {
                return None;
            }
            let hex: String = chars[2..end].iter().collect();
            if hex.is_empty() {
                return None;
            }
            let code_point = u32::from_str_radix(&hex, 16).ok()?;
            let decoded = char::from_u32(code_point)?;
            Some((decoded, end + 1))
        }
        _ => None,
    }
}

impl Scanner {
    pub(in crate::compiler) fn new(source: &str) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            start_line: 1,
            start_column: 1,
            previous_token_type: TokenType::NewLine,
            interpolations: Vec::new(),
        }
    }

    /// How many `${...}` interpolations are currently open.
    pub(in crate::compiler) fn interpolation_depth(&self) -> usize {
        self.interpolations.len()
    }

    //noinspection DuplicatedCode
    pub(in crate::compiler) fn scan_token(&mut self) -> Token {
        let mut c;
        loop {
            self.skip_whitespace();
            self.start = self.current;
            self.start_line = self.line;
            self.start_column = self.column;

            if self.is_at_end() {
                if !self.interpolations.is_empty() {
                    return self.make_interpolation_eof_error();
                }
                return self.make_eof_token();
            }
            c = self.advance();
            if c == '/' && self.matches('/') {
                // A comment runs to (not including) the newline.
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
                continue;
            }
            if !(self.previous_token_type == TokenType::NewLine && c == '\n') {
                break;
            }

            // A NewLine token was already emitted and this is another
            // newline right after it: a blank, whitespace-only, or
            // comment-only line. Count it without emitting a second token.
            self.line += 1;
            self.column = 1;
        }

        if Scanner::is_alpha(c) {
            return self.make_identifier();
        }
        if Scanner::is_digit(c) {
            return self.make_number();
        }

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => {
                if let Some(frame) = self.interpolations.last_mut() {
                    frame.brace_depth += 1;
                }
                self.make_token(TokenType::LeftBrace)
            }
            '}' => self.make_right_brace_or_string_continuation(),
            '[' => self.make_token(TokenType::LeftBracket),
            ']' => self.make_token(TokenType::RightBracket),
            ',' => self.make_token(TokenType::Comma),
            '.' => {
                if self.matches('.') {
                    if self.matches('=') {
                        self.make_token(TokenType::DotDotEqual)
                    } else {
                        self.make_token(TokenType::DotDot)
                    }
                } else {
                    self.make_token(TokenType::Dot)
                }
            }
            '-' => {
                if self.matches('-') {
                    self.make_token(TokenType::MinusMinus)
                } else {
                    self.make_token(TokenType::Minus)
                }
            }
            '+' => {
                if self.matches('+') {
                    self.make_token(TokenType::PlusPlus)
                } else {
                    self.make_token(TokenType::Plus)
                }
            }
            '%' => self.make_token(TokenType::Percent),
            ';' => self.make_token(TokenType::Semicolon),
            ':' => self.make_token(TokenType::Colon),
            '?' => self.make_token(TokenType::Question),
            '*' => {
                if self.matches('*') {
                    self.make_token(TokenType::StarStar)
                } else {
                    self.make_token(TokenType::Star)
                }
            }
            '!' => {
                if self.matches('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.matches('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.matches('<') {
                    self.make_token(TokenType::LessLess)
                } else if self.matches('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '>' => {
                if self.matches('>') {
                    self.make_token(TokenType::GreaterGreater)
                } else if self.matches('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '&' => {
                if self.matches('&') {
                    self.make_token(TokenType::AndAnd)
                } else {
                    self.make_token(TokenType::Ampersand)
                }
            }
            '|' => {
                if self.matches('|') {
                    self.make_token(TokenType::OrOr)
                } else {
                    self.make_token(TokenType::Pipe)
                }
            }
            '^' => self.make_token(TokenType::Caret),
            '~' => self.make_token(TokenType::Tilde),
            '/' => self.make_token(TokenType::Slash),
            '\n' => {
                let new_line = self.make_token(TokenType::NewLine);
                self.line += 1;
                self.column = 1;
                new_line
            }
            '"' => self.make_string(),
            '#' => {
                if self.matches('{') {
                    if let Some(frame) = self.interpolations.last_mut() {
                        frame.brace_depth += 1;
                    }
                    self.make_token(TokenType::HashLeftBrace)
                } else {
                    self.make_error_token("Unexpected character")
                }
            }
            _ => self.make_error_token("Unexpected character"),
        }
    }

    fn make_string(&mut self) -> Token {
        let quote = Position {
            line: self.start_line,
            column: self.start_column,
            offset: self.start,
        };
        self.scan_string_segment(true, quote)
    }

    fn make_right_brace_or_string_continuation(&mut self) -> Token {
        match self.interpolations.last_mut() {
            Some(frame) if frame.brace_depth > 0 => {
                frame.brace_depth -= 1;
                self.make_token(TokenType::RightBrace)
            }
            Some(_) => {
                let frame = self.interpolations.pop().unwrap();
                self.scan_string_segment(false, frame.quote)
            }
            None => self.make_token(TokenType::RightBrace),
        }
    }

    /// `quote` is the position of the segment's enclosing `"`.
    fn scan_string_segment(&mut self, is_fresh: bool, quote: Position) -> Token {
        let mut decoded = String::new();
        let mut invalid_escape: Option<(u32, u32, usize)> = None;

        loop {
            if self.is_at_end() {
                if !self.interpolations.is_empty() {
                    return self.make_interpolation_eof_error();
                }
                return self.make_error_token_at(
                    "Unterminated string",
                    quote.line,
                    quote.column,
                    quote.offset,
                );
            }
            if self.peek() == '"' {
                self.advance();
                if let Some(error) = self.invalid_escape_error(invalid_escape) {
                    return error;
                }
                let token_type = if is_fresh {
                    TokenType::String
                } else {
                    TokenType::StringEnd
                };
                return self.make_token_with_text(token_type, decoded);
            }
            if self.peek() == '\\' {
                let backslash_line = self.line;
                let backslash_column = self.column;
                let backslash_offset = self.current;
                self.advance();
                match decode_escape(&self.source[self.current..]) {
                    Some((decoded_char, consumed)) => {
                        decoded.push(decoded_char);
                        for _ in 0..consumed {
                            self.advance();
                        }
                    }
                    None => {
                        invalid_escape.get_or_insert((
                            backslash_line,
                            backslash_column,
                            backslash_offset,
                        ));
                    }
                }
                continue;
            }
            if self.peek() == '$' && self.peek_next() == '{' {
                let dollar = Position {
                    line: self.line,
                    column: self.column,
                    offset: self.current,
                };
                self.advance(); // '$'
                self.advance(); // '{'
                self.interpolations.push(Interpolation {
                    brace_depth: 0,
                    dollar,
                    quote,
                });
                if let Some(error) = self.invalid_escape_error(invalid_escape) {
                    return error;
                }
                let token_type = if is_fresh {
                    TokenType::StringStart
                } else {
                    TokenType::StringMiddle
                };
                return self.make_token_with_text(token_type, decoded);
            }
            let c = self.advance();
            decoded.push(c);
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            }
        }
    }

    fn invalid_escape_error(&mut self, invalid_escape: Option<(u32, u32, usize)>) -> Option<Token> {
        let (line, column, offset) = invalid_escape?;
        Some(self.make_error_token_at("Invalid escape sequence", line, column, offset))
    }

    /// Reports the innermost open interpolation as unclosed at EOF and
    /// clears the stack, so scanning resumes in ordinary token mode.
    fn make_interpolation_eof_error(&mut self) -> Token {
        let frame = self.interpolations.last().expect("stack checked non-empty");
        let dollar = frame.dollar;
        self.interpolations.clear();
        self.make_error_token_at(
            "Expect '}' after interpolated expression.",
            dollar.line,
            dollar.column,
            dollar.offset,
        )
    }

    fn make_identifier(&mut self) -> Token {
        loop {
            if !Scanner::is_alpha(self.peek()) && !Scanner::is_digit(self.peek()) {
                break;
            }
            self.advance();
        }
        self.make_token(self.make_identifier_type())
    }

    fn make_number(&mut self) -> Token {
        // Check if this starts with '0' followed by a base prefix
        // At this point, self.start points to the '0' and self.current is start+1
        if self.source[self.start] == '0' && !self.is_at_end() {
            let prefix = self.peek();
            match prefix {
                'x' | 'X' => return self.make_hex_number(),
                'b' | 'B' => return self.make_binary_number(),
                'o' | 'O' => return self.make_octal_number(),
                _ => {} // Continue with decimal parsing
            }
        }

        self.make_decimal_number()
    }

    fn make_hex_number(&mut self) -> Token {
        self.make_radix_number("Hexadecimal", Scanner::is_hex_digit, None)
    }

    fn make_binary_number(&mut self) -> Token {
        self.make_radix_number(
            "Binary",
            Scanner::is_binary_digit,
            Some(InvalidDigit {
                matches: |c| ('2'..='9').contains(&c),
                message: "Invalid digit in binary literal (only 0 and 1 allowed)",
            }),
        )
    }

    fn make_octal_number(&mut self) -> Token {
        self.make_radix_number(
            "Octal",
            Scanner::is_octal_digit,
            Some(InvalidDigit {
                matches: |c| c == '8' || c == '9',
                message: "Invalid digit in octal literal (only 0-7 allowed)",
            }),
        )
    }

    /// Scans a `0x`/`0b`/`0o` literal body after the base prefix letter.
    /// `invalid_digit` names digits that are never valid in this base (e.g.
    /// '8'/'9' for octal, '2'-'9' for binary), reported with a more specific
    /// message than the generic "requires at least one digit".
    fn make_radix_number(
        &mut self,
        label: &str,
        is_valid_digit: fn(char) -> bool,
        invalid_digit: Option<InvalidDigit>,
    ) -> Token {
        self.advance(); // consume base prefix letter

        let c = self.peek();
        if let Some(invalid_digit) = invalid_digit {
            if (invalid_digit.matches)(c) {
                return self.make_error_token(invalid_digit.message);
            }
        }
        if !is_valid_digit(c) {
            return self
                .make_error_token(&format!("{} literal requires at least one digit", label));
        }

        let mut has_digit = false;
        loop {
            let c = self.peek();
            if is_valid_digit(c) {
                has_digit = true;
                self.advance();
            } else if c == '_' {
                if !is_valid_digit(self.peek_next()) {
                    return self.make_error_token(&format!(
                        "Invalid underscore placement in {} literal",
                        label.to_lowercase()
                    ));
                }
                self.advance();
            } else if let Some(invalid_digit) = invalid_digit {
                if (invalid_digit.matches)(c) {
                    return self.make_error_token(invalid_digit.message);
                }
                break;
            } else {
                break;
            }
        }

        if !has_digit {
            return self
                .make_error_token(&format!("{} literal requires at least one digit", label));
        }

        self.make_token(TokenType::Number)
    }

    fn make_decimal_number(&mut self) -> Token {
        // Consume integer part with underscore support
        loop {
            let c = self.peek();
            if Scanner::is_digit(c) {
                self.advance();
            } else if c == '_' {
                if !Scanner::is_digit(self.peek_next()) {
                    return self.make_error_token("Invalid underscore placement in number literal");
                }
                self.advance();
            } else {
                break;
            }
        }

        // Handle decimal point
        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance(); // consume '.'

            // Consume fractional part with underscore support
            loop {
                let c = self.peek();
                if Scanner::is_digit(c) {
                    self.advance();
                } else if c == '_' {
                    if !Scanner::is_digit(self.peek_next()) {
                        return self
                            .make_error_token("Invalid underscore placement in number literal");
                    }
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.make_token(TokenType::Number)
    }

    fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        if self.current - self.start == start + length {
            let a = String::from_iter(self.source.iter().skip(self.start + start).take(length));
            if a == rest {
                return token_type;
            }
        }
        TokenType::Identifier
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_hex_digit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }

    fn is_binary_digit(c: char) -> bool {
        c == '0' || c == '1'
    }

    fn is_octal_digit(c: char) -> bool {
        ('0'..='7').contains(&c)
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;
        self.source[self.current - 1]
    }

    fn matches(&mut self, chr: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != chr {
            return false;
        }
        self.advance();
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_identifier_type(&self) -> TokenType {
        let chr = self.source[self.start];
        match chr {
            'b' => self.check_keyword(1, 4, "reak", TokenType::Break),
            'c' => self.check_keyword(1, 7, "ontinue", TokenType::Continue),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'i' => {
                if self.current - self.start > 1 {
                    return match self.source[self.start + 1] {
                        'f' => self.check_keyword(2, 0, "", TokenType::If),
                        'n' => self.check_keyword(2, 0, "", TokenType::In),
                        'm' => self.check_keyword(2, 2, "pl", TokenType::Impl),
                        _ => TokenType::Identifier,
                    };
                }
                TokenType::Identifier
            }
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => {
                if self.current - self.start > 1 {
                    return match self.source[self.start + 1] {
                        't' => self.check_keyword(2, 4, "ruct", TokenType::Struct),
                        _ => TokenType::Identifier,
                    };
                }
                TokenType::Identifier
            }
            'v' => {
                if self.current - self.start >= 3 && self.source[self.start + 1] == 'a' {
                    return match self.source[self.start + 2] {
                        'l' => self.check_keyword(3, 0, "", TokenType::Val),
                        'r' => self.check_keyword(3, 0, "", TokenType::Var),
                        _ => TokenType::Identifier,
                    };
                }
                TokenType::Identifier
            }
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            'f' => {
                if self.current - self.start > 1 {
                    return match self.source[self.start + 1] {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        'n' => self.check_keyword(2, 0, "", TokenType::Fn),
                        _ => TokenType::Identifier,
                    };
                }
                TokenType::Identifier
            }
            't' => {
                if self.current - self.start > 1 {
                    return match self.source[self.start + 1] {
                        'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    };
                }
                TokenType::Identifier
            }
            _ => TokenType::Identifier,
        }
    }

    fn make_error_token(&mut self, message: &str) -> Token {
        self.make_error_token_at(message, self.start_line, self.start_column, self.start)
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        let token_str = String::from_iter(&self.source[self.start..self.current]);
        self.make_token_with_text(token_type, token_str)
    }

    fn make_token_with_text(&mut self, token_type: TokenType, text: String) -> Token {
        self.previous_token_type = token_type.clone();
        Token::new(
            token_type,
            text,
            self.start_line,
            self.start_column,
            self.start,
        )
    }

    fn make_error_token_at(
        &mut self,
        message: &str,
        line: u32,
        column: u32,
        offset: usize,
    ) -> Token {
        self.previous_token_type = TokenType::Error;
        Token::new(
            TokenType::Error,
            String::from(message),
            line,
            column,
            offset,
        )
    }
    fn make_eof_token(&mut self) -> Token {
        self.previous_token_type = TokenType::Eof;
        Token::new(
            TokenType::Eof,
            String::new(),
            self.start_line,
            self.start_column,
            self.start,
        )
    }
}
