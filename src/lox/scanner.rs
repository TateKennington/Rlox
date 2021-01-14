use crate::lox;
use crate::lox::tokens::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            current: 0,
            start: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> &str {
        let c = self.source.get(self.current..self.current + 1).unwrap();
        self.current += 1;
        c
    }

    fn matches(&mut self, expected: &str) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.get(self.current..self.current + 1).unwrap() != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek_next(&self) -> &str {
        if self.current + 1 >= self.source.len() {
            return "\0";
        }
        self.source.get(self.current + 1..self.current + 2).unwrap()
    }

    fn peek(&self) -> &str {
        if self.is_at_end() {
            return "\0";
        }
        self.source.get(self.current..self.current + 1).unwrap()
    }

    fn string(&mut self) {
        while self.peek() != "\"" && !self.is_at_end() {
            if self.peek() == "\n" {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            lox::error(self.line, "Unterminated String Literal.")
        }

        self.advance();

        self.add_token(TokenType::String(String::from(
            self.source.get(self.start + 1..self.current - 1).unwrap(),
        )));
    }

    fn is_digit(c: &str) -> bool {
        let c = c.chars().next().unwrap();
        c >= '0' && c <= '9'
    }

    fn is_alpha(c: &str) -> bool {
        let c = c.chars().next().unwrap();
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }

    fn is_alphanum(c: &str) -> bool {
        Scanner::is_alpha(c) || Scanner::is_digit(c)
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek())
            || (self.peek() == "." && Scanner::is_digit(self.peek_next()))
        {
            self.advance();
        }
        let lexeme = String::from(self.source.get(self.start..self.current).unwrap());
        self.add_token(TokenType::Number(lexeme.parse().unwrap()))
    }

    fn identifier(&mut self) {
        while Scanner::is_alphanum(self.peek()) && !self.is_at_end() {
            if self.peek() == "\n" {
                self.line += 1
            }
            self.advance();
        }

        let lexeme: String = String::from(self.source.get(self.start..self.current).unwrap());
        match &lexeme[..] {
            "and" => self.add_token(TokenType::And),
            "class" => self.add_token(TokenType::Class),
            "else" => self.add_token(TokenType::Else),
            "false" => self.add_token(TokenType::False),
            "for" => self.add_token(TokenType::For),
            "fun" => self.add_token(TokenType::Fun),
            "if" => self.add_token(TokenType::If),
            "nil" => self.add_token(TokenType::Nil),
            "or" => self.add_token(TokenType::Or),
            "print" => self.add_token(TokenType::Print),
            "return" => self.add_token(TokenType::Return),
            "super" => self.add_token(TokenType::Super),
            "this" => self.add_token(TokenType::This),
            "true" => self.add_token(TokenType::True),
            "var" => self.add_token(TokenType::Var),
            "while" => self.add_token(TokenType::While),
            _ => self.add_token(TokenType::Identifier(lexeme)),
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            "(" => self.add_token(TokenType::LeftParen),
            ")" => self.add_token(TokenType::RightParen),
            "{" => self.add_token(TokenType::LeftBrace),
            "}" => self.add_token(TokenType::RightBrace),
            "," => self.add_token(TokenType::Comma),
            "." => self.add_token(TokenType::Dot),
            "-" => self.add_token(TokenType::Minus),
            "+" => self.add_token(TokenType::Plus),
            ";" => self.add_token(TokenType::Semicolon),
            "*" => self.add_token(TokenType::Star),
            "!" => {
                let token = if self.matches("=") {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token)
            }
            "=" => {
                let token = if self.matches("=") {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token)
            }
            "<" => {
                let token = if self.matches("=") {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token)
            }
            ">" => {
                let token = if self.matches("=") {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token)
            }
            "/" => {
                if self.matches("/") {
                    while self.peek() != "\n" && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            "\"" => self.string(),
            "\n" => self.line += 1,
            " " | "\r" | "\t" => {}
            _ => {
                if Scanner::is_digit(c) {
                    self.number();
                } else if Scanner::is_alpha(c) {
                    self.identifier();
                } else {
                    lox::error(self.line, "Unexpected Character.")
                }
            }
        };
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = String::from(self.source.get(self.start..self.current).unwrap());
        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::from(""), self.line));
        self.tokens
    }
}
