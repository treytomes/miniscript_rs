use std::fmt::Debug;

use crate::token_type::TokenType;

#[derive(Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i64,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: i64) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_string(),
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.lexeme.trim().len() > 0 {
            write!(f, "{:?} {}", self.token_type, self.lexeme)
        } else {
            write!(f, "{:?}", self.token_type)
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", self)
    }
}