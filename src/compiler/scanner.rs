use crate::compiler::token::TokenType;
use crate::compiler::{Scanner, Token};

impl Scanner {
    pub(in crate::compiler) fn new(source: &str) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            offset: 0,
            previous_token_type: TokenType::NewLine,
        }
    }

    //noinspection DuplicatedCode
    pub(in crate::compiler) fn scan_token(&mut self) -> Token {
        let mut c;
        loop {
            self.skip_whitespace();
            self.start = self.current;

            if self.is_at_end() {
                return self.make_eof_token();
            }
            c = self.advance();
            if !(self.previous_token_type == TokenType::NewLine && c == '\n') {
                break;
            }

            // increment line number if this is an empty line
            if self.previous() == '\n' && c == '\n' {
                self.line += 1;
            }
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
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
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
            '*' => self.make_token(TokenType::Star),
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
                if self.matches('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '>' => {
                if self.matches('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '&' => {
                if self.matches('&') {
                    self.make_token(TokenType::AndAnd)
                } else {
                    self.make_error_token("Expected '&&' operator")
                }
            }
            '|' => {
                if self.matches('|') {
                    self.make_token(TokenType::OrOr)
                } else {
                    self.make_error_token("Expected '||' operator")
                }
            }
            '/' => {
                if self.matches('/') {
                    // Check if this is a comment or integer division operator
                    // Comments have whitespace or newline after //
                    // Integer division has a non-whitespace character
                    let next_char = self.peek();
                    if next_char == ' ' || next_char == '\t' || next_char == '\n' || next_char == '\r' || self.is_at_end() {
                        // This is a comment
                        while self.peek_next() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                        if !self.is_at_end() {
                            self.advance();
                        }
                        self.line += 1;
                        self.column = 1;
                        self.scan_token()
                    } else {
                        // This is the integer division operator
                        self.make_token(TokenType::SlashSlash)
                    }
                } else {
                    self.make_token(TokenType::Slash)
                }
            }
            '\n' => {
                let new_line = self.make_token(TokenType::NewLine);
                self.line += 1;
                self.column = 1;
                new_line
            }
            '"' => self.make_string(),
            _ => self.make_error_token("Unexpected character"),
        }
    }

    fn make_string(&mut self) -> Token {
        let mut placeholders: Vec<(usize, usize)> = Vec::new();
        let mut placeholder_start = None;
        loop {
            if self.is_at_end() {
                return self.make_error_token("Unterminated string");
            }
            if self.peek() == '"' {
                break;
            }
            if self.peek() == '$' && self.peek_next() == '{' {
                placeholder_start = Some(self.current);
            }
            if self.peek() == '}' {
                if let Some(start) = placeholder_start {
                    placeholders.push((start, self.current));
                }
            }
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        self.advance();
        if !placeholders.is_empty() {
            return self.make_token(TokenType::InterpolatedString);
        }
        self.make_token(TokenType::String)
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
        loop {
            if !Scanner::is_digit(self.peek()) {
                break;
            }
            self.advance();
        }

        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance();
        }

        loop {
            if !Scanner::is_digit(self.peek()) {
                break;
            }
            self.advance();
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
                    self.column += 1;
                    self.offset += 1;
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
        self.offset += 1;
        self.source[self.current - 1]
    }

    fn matches(&mut self, chr: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != chr {
            return false;
        }
        self.current += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_identifier_type(&self) -> TokenType {
        let chr = self.source[self.start];
        match chr {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'b' => self.check_keyword(1, 4, "reak", TokenType::Break),
            'c' => {
                if self.current - self.start > 1 {
                    return match self.source[self.start + 1] {
                        'l' => self.check_keyword(2, 3, "ass", TokenType::Class),
                        'o' => self.check_keyword(2, 6, "ntinue", TokenType::Continue),
                        _ => TokenType::Identifier,
                    };
                }
                TokenType::Identifier
            }
            'e' => {
                if self.current - self.start > 1 {
                    return match self.source[self.start + 1] {
                        'l' => self.check_keyword(2, 2, "se", TokenType::Else),
                        'x' => self.check_keyword(2, 4, "port", TokenType::Export),
                        _ => TokenType::Identifier,
                    };
                }
                TokenType::Identifier
            }
            'i' => {
                if self.current - self.start > 1 {
                    return match self.source[self.start + 1] {
                        'f' => self.check_keyword(2, 0, "", TokenType::If),
                        'n' => self.check_keyword(2, 0, "", TokenType::In),
                        'm' => self.check_keyword(2, 4, "port", TokenType::Import),
                        _ => TokenType::Identifier,
                    };
                }
                TokenType::Identifier
            }
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => {
                if self.current - self.start > 1 {
                    return match self.source[self.start + 1] {
                        't' => self.check_keyword(2, 4, "ruct", TokenType::Struct),
                        'u' => self.check_keyword(2, 3, "per", TokenType::Super),
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
                        'n' => TokenType::Fn,
                        _ => TokenType::Identifier,
                    };
                }
                TokenType::Identifier
            }
            't' => {
                if self.current - self.start > 1 {
                    return match self.source[self.start + 1] {
                        'h' => self.check_keyword(2, 2, "is", TokenType::This),
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
        self.previous_token_type = TokenType::Error;
        Token::new(
            TokenType::Error,
            String::from(message),
            self.line,
            self.column,
            self.offset,
        )
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        self.previous_token_type = token_type.clone();
        let token_str = String::from_iter(&self.source[self.start..self.current]);
        let token_str_len = token_str.len() as u32;
        let token = Token::new(token_type, token_str, self.line, self.column, self.offset);
        self.column += token_str_len;
        token
    }
    fn make_eof_token(&mut self) -> Token {
        self.previous_token_type = TokenType::Eof;
        Token::new(TokenType::Eof, String::new(), self.line, self.column, self.offset)
    }

    fn previous(&self) -> char {
        if self.current < 2 {
            return '\n';
        }
        self.source[self.current - 2]
    }
}
