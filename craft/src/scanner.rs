use crate::error;
use crate::token::Token;
use crate::token_type::TokenType;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut map = HashMap::new();
        map.insert("and".to_string(), TokenType::AND);
        map.insert("class".to_string(), TokenType::CLASS);
        map.insert("else".to_string(), TokenType::ELSE);
        map.insert("false".to_string(), TokenType::FALSE);
        map.insert("for".to_string(), TokenType::FOR);
        map.insert("fun".to_string(), TokenType::FUN);
        map.insert("if".to_string(), TokenType::IF);
        map.insert("nil".to_string(), TokenType::NIL);
        map.insert("or".to_string(), TokenType::OR);
        map.insert("print".to_string(), TokenType::PRINT);
        map.insert("return".to_string(), TokenType::RETURN);
        map.insert("super".to_string(), TokenType::SUPER);
        map.insert("this".to_string(), TokenType::THIS);
        map.insert("true".to_string(), TokenType::TRUE);
        map.insert("var".to_string(), TokenType::VAR);
        map.insert("while".to_string(), TokenType::WHILE);
        map
    };
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), self.line));

        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '/' => self.add_token(TokenType::SLASH),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.check_and_use_next('=') {
                    self.add_token(TokenType::BANG_EQUAL)
                } else {
                    self.add_token(TokenType::BANG)
                }
            }
            '=' => {
                if self.check_and_use_next('=') {
                    self.add_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.add_token(TokenType::EQUAL)
                }
            }
            '<' => {
                if self.check_and_use_next('=') {
                    self.add_token(TokenType::LESS_EQUAL)
                } else {
                    self.add_token(TokenType::LESS)
                }
            }
            '>' => {
                if self.check_and_use_next('=') {
                    self.add_token(TokenType::GREATER_EQUAL)
                } else {
                    self.add_token(TokenType::GREATER)
                }
            }
            ' ' | '\r' | '\t' => (), // Ignore whitespace.
            '\n' => self.line += 1,
            '"' => self.string(),
            c => {
                if self.is_digit(c) {
                    self.number();
                } else if c.is_ascii_alphabetic() {
                    self.identifier();
                } else {
                    error(self.line, &format!("Unexpected character {c}"))
                }
            }
        }
    }

    fn add_token(&mut self, variant: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(variant, text, self.line));
    }

    fn advance(&mut self) -> char {
        let value = self.source.chars().nth(self.current);

        if let Some(c) = value {
            self.current += 1;
            return c;
        } else {
            panic!(
                "Currenly at {} (char {}), length is {}, tried to advance",
                self.current,
                self.source.chars().nth(self.current).unwrap_or('\0'),
                self.source.len()
            );
        }
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek2(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn check_and_use_next(&mut self, arg: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != arg {
            return false;
        }

        self.current += 1;
        true
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        println!("peek: {}", self.peek());

        if self.is_at_end() || self.peek() != '"' {
            error(self.line, "Unterminated string literal");
            return;
        }

        self.advance(); // The closing ".

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::STRING(value))
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == ' ' && self.is_digit(self.peek2()) {
            self.advance();
        }

        while self.is_digit(self.peek()) {
            self.advance();
        }

        self.add_token(TokenType::NUMBER(
            self.source[self.start..self.current].parse().unwrap(),
        ));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() && !self.peek().is_whitespace() {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let token_type = KEYWORDS
            .get(&text)
            .cloned()
            .unwrap_or(TokenType::IDENTIFIER(text));

        self.add_token(token_type);
    }
}
