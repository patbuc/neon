#[derive(Debug)]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

#[derive(Debug)]
pub(crate) struct Token {
    token_type: TokenType,
    start: usize,
    length: usize,
    line: u32,
}

impl Token {
    fn new(token_type: TokenType, start: usize, length: usize, line: u32) -> Token {
        Token {
            token_type,
            start,
            length,
            line,
        }
    }
}

pub(crate) struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub(crate) fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            if self.is_after_end() {
                return None;
            }
            self.current += 1;
            return Option::from(self.make_token(TokenType::Eof));
        }

        let c = self.advance();
        if Scanner::is_alpha(c) {
            return self.make_identifier();
        }
        if Scanner::is_digit(c) {
            return Option::from(self.make_number());
        }
        match c {
            '(' => return Option::from(self.make_token(TokenType::LeftParen)),
            ')' => return Option::from(self.make_token(TokenType::RightParen)),
            '{' => return Option::from(self.make_token(TokenType::LeftBrace)),
            '}' => return Option::from(self.make_token(TokenType::RightBrace)),
            ',' => return Option::from(self.make_token(TokenType::Comma)),
            '.' => return Option::from(self.make_token(TokenType::Dot)),
            '-' => return Option::from(self.make_token(TokenType::Minus)),
            '+' => return Option::from(self.make_token(TokenType::Plus)),
            ';' => return Option::from(self.make_token(TokenType::Semicolon)),
            '*' => return Option::from(self.make_token(TokenType::Star)),
            '!' => {
                if self.matches('=') {
                    return Option::from(self.make_token(TokenType::BangEqual));
                } else {
                    return Option::from(self.make_token(TokenType::Bang));
                }
            }
            '=' => {
                if self.matches('=') {
                    return Option::from(self.make_token(TokenType::EqualEqual));
                } else {
                    return Option::from(self.make_token(TokenType::Equal));
                }
            }
            '<' => {
                if self.matches('=') {
                    return Option::from(self.make_token(TokenType::LessEqual));
                } else {
                    return Option::from(self.make_token(TokenType::Less));
                }
            }
            '>' => {
                if self.matches('=') {
                    return Option::from(self.make_token(TokenType::GreaterEqual));
                } else {
                    return Option::from(self.make_token(TokenType::Greater));
                }
            }
            '/' => {
                if self.matches('/') {
                    while self.peek_next() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    return self.next();
                } else {
                    return Option::from(self.make_token(TokenType::Slash));
                }
            }
            '"' => return self.make_string(),
            _ => Option::from(self.error_token("Unexpected character.")),
        }
    }
}

impl Scanner {
    fn make_string(&mut self) -> Option<Token> {
        loop {
            if self.is_at_end() {
                return Option::from(self.error_token("Unterminated string."));
            }
            if self.peek() == '"' {
                break;
            }
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        self.advance();
        return Option::from(self.make_token(TokenType::String));
    }

    fn make_identifier(&mut self) -> Option<Token> {
        loop {
            if !Scanner::is_alpha(self.peek()) && !Scanner::is_digit(self.peek()) {
                break;
            }
            self.advance();
        }

        return Option::from(self.make_token(self.identifier_type()));
    }

    fn make_number(&mut self) -> Token {
        loop {
            if !Scanner::is_digit(self.peek()) {
                break;
            }
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            // Consume the ".".
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

    fn identifier_type(&self) -> TokenType {
        let chr = self.source[self.start];
        return match chr {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            'f' => {
                if self.current - self.start > 1 {
                    return match self.source[self.start + 1] {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    };
                }
                TokenType::Identifier
            }
            't' => {
                if self.current - self.start > 1 {
                    return match self.source[self.start + 1] {
                        'h' => self.check_keyword(2, 2, "is", TokenType::This),
                        'r' => self.check_keyword(2, 1, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    };
                }
                TokenType::Identifier
            }
            _ => TokenType::Identifier,
        };
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
        return TokenType::Identifier;
    }

    fn is_alpha(c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_digit(c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source[self.current + 1];
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source[self.current];
    }

    fn skip_whitespace(&mut self) {
        while let c = self.peek() {
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }
    }
    fn matches(&mut self, chr: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != chr {
            return false;
        }

        self.current += 1;
        return true;
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn has_more(&self) -> bool {
        self.current < self.source.len()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_after_end(&self) -> bool {
        self.current > self.source.len()
    }

    fn error_token(&self, message: &str) -> Token {
        Token::new(TokenType::Error, self.start, message.len(), self.line)
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.start, self.current - self.start, self.line)
    }
}
