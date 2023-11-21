use crate::compiler::token::TokenType;
use crate::compiler::{Scanner, Token};

impl Scanner {
    pub(in crate::compiler) fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    //noinspection DuplicatedCode
    pub(in crate::compiler) fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            if self.is_after_end() {
                panic!("Scanner is after end of source.");
            }
            self.current += 1;
            return self.make_token(TokenType::Eof);
        }
        let c = self.advance();
        if Scanner::is_alpha(c) {
            return self.make_identifier();
        }
        if Scanner::is_digit(c) {
            return self.make_number();
        }

        return match c {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            ';' => return self.make_token(TokenType::Semicolon),
            '*' => return self.make_token(TokenType::Star),
            '!' => {
                return if self.matches('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                return if self.matches('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '<' => {
                return if self.matches('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '>' => {
                return if self.matches('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '/' => {
                return if self.matches('/') {
                    while self.peek_next() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    self.scan_token()
                } else {
                    return self.make_token(TokenType::Slash);
                }
            }
            '"' => return self.make_string(),
            _ => self.make_error_token("Unexpected character."),
        };
    }

    fn make_string(&mut self) -> Token {
        let mut placeholders: Vec<(usize, usize)> = Vec::new();
        let mut placeholder_start = None;
        loop {
            if self.is_at_end() {
                return self.make_error_token("Unterminated string.");
            }
            if self.peek() == '"' {
                break;
            }
            if self.peek() == '$' && self.peek_next() == '{' {
                placeholder_start = Some(self.current);
            }
            if self.peek() == '}' && placeholder_start.is_some() {
                placeholders.push((placeholder_start.unwrap(), self.current));
            }
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        self.advance();
        if placeholders.len() > 0 {
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
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
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

    fn advance(&mut self) -> char {
        self.current += 1;
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

    fn is_after_end(&self) -> bool {
        self.current > self.source.len()
    }

    fn make_identifier_type(&self) -> TokenType {
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

    fn make_error_token(&self, message: &str) -> Token {
        Token::new(TokenType::Error, self.start, message.len(), self.line)
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.start, self.current - self.start, self.line)
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::scanner::Token;
    use crate::compiler::token::TokenType;
    use crate::compiler::Scanner;

    #[test]
    fn can_scan_simple_statement() {
        let script = "var a = 1;".to_string();

        let mut scanner = super::Scanner::new(script);
        let x: Vec<Token> = collect_tokens(scanner);

        assert_eq!(x.len(), 6);

        assert_eq!(x[0].token_type, TokenType::Var);
        assert_eq!(x[0].start, 0);
        assert_eq!(x[0].length, 3);
        assert_eq!(x[0].line, 1);

        assert_eq!(x[1].token_type, TokenType::Identifier);
        assert_eq!(x[1].start, 4);
        assert_eq!(x[1].length, 1);
        assert_eq!(x[1].line, 1);

        assert_eq!(x[2].token_type, TokenType::Equal);
        assert_eq!(x[3].token_type, TokenType::Number);
        assert_eq!(x[4].token_type, TokenType::Semicolon);
        assert_eq!(x[5].token_type, TokenType::Eof);
    }

    #[test]
    fn can_scan_interpolated_string() {
        let script = "var a = \"This is an ${interpolated} string\";".to_string();

        let mut scanner = super::Scanner::new(script);
        let x: Vec<Token> = collect_tokens(scanner);

        assert_eq!(x.len(), 6);

        assert_eq!(x[0].token_type, TokenType::Var);
        assert_eq!(x[0].start, 0);
        assert_eq!(x[0].length, 3);
        assert_eq!(x[0].line, 1);

        assert_eq!(x[1].token_type, TokenType::Identifier);
        assert_eq!(x[1].start, 4);
        assert_eq!(x[1].length, 1);
        assert_eq!(x[1].line, 1);

        assert_eq!(x[2].token_type, TokenType::Equal);
        assert_eq!(x[3].token_type, TokenType::InterpolatedString);
        assert_eq!(x[4].token_type, TokenType::Semicolon);
        assert_eq!(x[5].token_type, TokenType::Eof);
    }

    fn collect_tokens(mut scanner: Scanner) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let token = scanner.scan_token();
            if token.token_type == TokenType::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }
}
