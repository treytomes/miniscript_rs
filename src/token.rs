use std::fmt::Debug;

use crate::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,

    // TODO: I don't think we need `literal`.  `lexeme` will be parsed into it's actual value JIT.
    pub literal: Option<String>,
    
    pub line: i64,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Option<&str>, line: i64) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_string(),
            literal: match literal {
                Some(s) => Some(s.to_string()),
                None => None,
            },
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}