use crate::ast::*;
use crate::token::{Token, TokenType};

/// Parser converts a sequence of tokens produced by the scanner / lexer into a syntax tree (AST).
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.match_token(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.comparison();
            expr = Expr::Binary(Box::new(Binary::new(expr, operator, right)));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.match_token(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.term();
            expr = Expr::Binary(Box::new(Binary::new(expr, operator, right)));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.match_token(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.factor();
            expr = Expr::Binary(Box::new(Binary::new(expr, operator, right)));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.match_token(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary();
            expr = Expr::Binary(Box::new(Binary::new(expr, operator, right)));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary();
            return Expr::Unary(Box::new(Unary::new(operator, right)));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        let token = self.peek().clone();

        match &token.variant {
            TokenType::FALSE => {
                self.advance();
                return Expr::Literal(Box::new(Literal::new(Object::Boolean(false))));
            }
            TokenType::TRUE => {
                self.advance();
                return Expr::Literal(Box::new(Literal::new(Object::Boolean(true))));
            }
            TokenType::NIL => {
                self.advance();
                return Expr::Literal(Box::new(Literal::new(Object::Nil)));
            }
            TokenType::NUMBER(num) => {
                self.advance();
                return Expr::Literal(Box::new(Literal::new(Object::Number(*num))));
            }
            TokenType::STRING(str) => {
                self.advance();
                return Expr::Literal(Box::new(Literal::new(Object::String(str.clone()))));
            }
            TokenType::LEFT_PAREN => {
                self.advance();
                let expr: Expr = self.expression();
                self.consume(
                    TokenType::RIGHT_PAREN,
                    "Expected ')' after expression, to match '('",
                );
                return Expr::Grouping(Box::new(Grouping::new(expr)));
            }
            _ => {
                panic!("Unexpected token: {:?}", token);
            }
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.check(token_type) {
            self.advance();
            return;
        }

        panic!("{}", message);
    }

    fn match_token(&mut self, vec: Vec<TokenType>) -> bool {
        for token_type in vec {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().variant == token_type
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.tokens.len();
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.current - 1];
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }
}
