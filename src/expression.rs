use std::fmt::{Debug, Display};

use crate::token::Token;

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
    Unary(Token, Box<Expr>),
}

pub fn format_ast(expr: &Expr) -> String {
    match expr {
        Expr::Binary(left, operator, right) =>
            format!("({:} {:} {:})", operator.lexeme, format_ast(left), format_ast(right)),
            Expr::Grouping(expr) =>
            format!("(group {:})", format_ast(expr)),
            Expr::Literal(value) => (*value).lexeme.clone(),
            Expr::Unary(operator, expr) =>
            format!("({:} {:})", operator.lexeme, format_ast(expr)),
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_ast(self))
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Expr, Token, TokenType};

    #[test]
    fn test_print_ast() {
        let expr = Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::Minus, "-", None, 1),
                Box::new(Expr::Literal(Token::new(TokenType::Number, "123", None, 1)))
            )),
            Token::new(TokenType::Star, "*", None, 1),
            Box::new(Expr::Grouping(
                Box::new(Expr::Literal(Token::new(TokenType::Number, "45.67", None, 1)))
            ))
        );

        // let result = format_ast(&expr);
        let result = format!("{}", expr);

        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}