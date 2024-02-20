use std::fmt::{Debug, Display};

use crate::{token::Token, EvalResult, TokenType};

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
    Unary(Token, Box<Expr>),
}

pub fn is_truthy(value: EvalResult) -> bool {
    match value {
        EvalResult::Number(value) => value != 0.0,
        EvalResult::String(value) => value.len() > 0,
        EvalResult::Null => false,
    }
}

// pub fn is_falsy(value: EvalResult) -> bool {
//     !is_truthy(value)
// }

pub fn format_ast(expr: &Expr) -> String {
    match expr {
        Expr::Binary(left, operator, right) => format!("({:} {:} {:})", operator.lexeme, format_ast(left), format_ast(right)),
        Expr::Grouping(expr) => format!("(group {:})", format_ast(expr)),
        Expr::Literal(value) => (*value).lexeme.clone(),
        Expr::Unary(operator, expr) => format!("({:} {:})", operator.lexeme, format_ast(expr)),
    }
}

pub fn eval_ast(expr: &Expr) -> EvalResult {
    match expr {
        Expr::Binary(left, operator, right) => {
            let left = eval_ast(left);
            let right = eval_ast(right);

            match (&left, &right) {
                (EvalResult::Number(l), EvalResult::Number(r)) => {
                    match operator.token_type {
                        // TODO: Think about shortcutting when it's time.
                        TokenType::And => EvalResult::Number(if is_truthy(left) && is_truthy(right) { 1.0 } else { 0.0 }),
                        TokenType::Or => EvalResult::Number(if is_truthy(left) || is_truthy(right) { 1.0 } else { 0.0 }),
                        
                        TokenType::Plus => EvalResult::Number(l + r),
                        TokenType::Minus => EvalResult::Number(l - r),
                        TokenType::Star => EvalResult::Number(l * r),
                        TokenType::Slash => EvalResult::Number(l / r),
                        
                        TokenType::Greater => EvalResult::Number(if l > r { 1.0 } else { 0.0 }),
                        TokenType::GreaterEqual => EvalResult::Number(if l >= r { 1.0 } else { 0.0 }),
                        TokenType::Less => EvalResult::Number(if l < r { 1.0 } else { 0.0 }),
                        TokenType::LessEqual => EvalResult::Number(if l <= r { 1.0 } else { 0.0 }),
                        TokenType::BangEqual => EvalResult::Number(if l != r { 1.0 } else { 0.0 }),
                        TokenType::EqualEqual => EvalResult::Number(if l == r { 1.0 } else { 0.0 }),
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
            TokenType::Null => EvalResult::Null,
            TokenType::True => EvalResult::Number(1.0),
            TokenType::False => EvalResult::Number(0.0),
            _ => unreachable!(),
        }
        Expr::Unary(operator, expr) => {
            let expr = eval_ast(expr);
            match expr {
                EvalResult::Number(value) => match operator.token_type {
                    TokenType::Minus => EvalResult::Number(-value),
                    TokenType::Not => EvalResult::Number(if is_truthy(expr) { 0.0 } else { 1.0 }),
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
    use crate::{expression::eval_ast, parser::Parser, scanner::Scanner, EvalResult, Expr, Token, TokenType};

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

    #[test]
    fn test_eval_ast() {
        test_eval("1+1", EvalResult::Number(2.0));
        test_eval("1-1", EvalResult::Number(0.0));
        test_eval("1*1", EvalResult::Number(1.0));
        test_eval("1/1", EvalResult::Number(1.0));
        test_eval("1>1", EvalResult::Number(0.0));
        test_eval("1>=1", EvalResult::Number(1.0));
        test_eval("1<1", EvalResult::Number(0.0));
        test_eval("1<=1", EvalResult::Number(1.0));
        test_eval("1==1", EvalResult::Number(1.0));
        test_eval("1!=1", EvalResult::Number(0.0));
        test_eval("1 abd 1", EvalResult::Number(1.0));
        test_eval("1 or 1", EvalResult::Number(1.0));
        test_eval("1 and 0", EvalResult::Number(0.0));
        test_eval("1 or 0", EvalResult::Number(1.0));
        test_eval("-1", EvalResult::Number(-1.0));
        test_eval("not 1", EvalResult::Number(0.0));
        test_eval("true", EvalResult::Number(1.0));
        test_eval("false", EvalResult::Number(0.0));
        test_eval("1+2*3", EvalResult::Number(7.0));
        test_eval("(1+2)*3", EvalResult::Number(9.0));
        test_eval("1+2*3+4/5", EvalResult::Number(7.8));
        test_eval("1+2*3+4/5*6", EvalResult::Number(11.8));
    }

    fn test_eval(input: &str, expected: EvalResult) {
        let mut scanner = Scanner::new(input);
        scanner.scan_tokens();


        // // Print the tokens.
        // for token in &scanner.tokens {
        //     println!("{:?}", token);
        // }

        let mut parser = Parser::new(scanner.tokens);
        let expr = match parser.parse() {
            Some(expr) => expr,
            None => { panic!("Syntax error.") },
        };

        println!("{:}", expr);
        
        let result = eval_ast(&expr);
        assert_eq!(result, expected);
    }
}