use std::fmt::Display;

pub use crate::token_type::TokenType;

#[derive(Clone, Debug)]
pub struct Token {
    pub variant: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(variant: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            variant,
            lexeme,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lexeme = self.lexeme.clone();
        write!(f, "{:?} {}", self.variant, lexeme)
    }
}
