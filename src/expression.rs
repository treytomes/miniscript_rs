use std::fmt::{Debug, Display};

use crate::{token::Token, EvalResult, TokenType};

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

pub fn eval_ast(expr: &Expr) -> EvalResult {
    match expr {
        Expr::Binary(left, operator, right) => {
            let left = eval_ast(left);
            let right = eval_ast(right);

            match (left, right) {
                (EvalResult::Number(left), EvalResult::Number(right)) => {
                    match operator.token_type {
                        TokenType::Plus => EvalResult::Number(left + right),
                        TokenType::Minus => EvalResult::Number(left - right),
                        TokenType::Star => EvalResult::Number(left * right),
                        TokenType::Slash => EvalResult::Number(left / right),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        Expr::Grouping(expr) => eval_ast(expr),
        Expr::Literal(value) => match value.token_type {
            TokenType::Number => EvalResult::Number(value.lexeme.parse().unwrap()),
            TokenType::String => EvalResult::String(value.lexeme.clone()),
            _ => unreachable!(),
        }
        Expr::Unary(operator, expr) => {
            let expr = eval_ast(expr);
            match expr {
                EvalResult::Number(value) => match operator.token_type {
                    TokenType::Minus => EvalResult::Number(-value),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
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
                Token::new(TokenType::Minus, "-", 1),
                Box::new(Expr::Literal(Token::new(TokenType::Number, "123", 1)))
            )),
            Token::new(TokenType::Star, "*", 1),
            Box::new(Expr::Grouping(
                Box::new(Expr::Literal(Token::new(TokenType::Number, "45.67", 1)))
            ))
        );

        // let result = format_ast(&expr);
        let result = format!("{}", expr);

        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}