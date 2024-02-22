use std::fmt::{Debug, Display};

use crate::{error_reporter::ErrorReporter, token::Token, Error, EvalResult, TokenType};

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
        _ => unreachable!("This shouldn't have happened."),
    }
}

pub fn is_falsy(value: EvalResult) -> bool {
    !is_truthy(value)
}

pub fn format_ast(expr: &Expr) -> String {
    match expr {
        Expr::Binary(left, operator, right) => format!("({:} {:} {:})", operator.lexeme, format_ast(left), format_ast(right)),
        Expr::Grouping(expr) => format!("(group {:})", format_ast(expr)),
        Expr::Literal(value) => (*value).lexeme.clone(),
        Expr::Unary(operator, expr) => format!("({:} {:})", operator.lexeme, format_ast(expr)),
    }
}

pub fn eval_ast(expr: &Expr, reporter: &mut ErrorReporter) -> Result<EvalResult, Error> {
    match expr {
        Expr::Binary(left, operator, right) => {
            let left = eval_ast(left, reporter)?;
            if operator.token_type == TokenType::And {
                if is_falsy(left) {
                    return Ok(EvalResult::Number(0.0));
                } else {
                    let right = eval_ast(right, reporter)?;
                    return Ok(EvalResult::Number(if is_truthy(right) { 1.0 } else { 0.0 }));
                }

            } else if operator.token_type == TokenType::Or {
                if is_truthy(left) {
                    return Ok(EvalResult::Number(1.0));
                } else {
                    let right = eval_ast(right, reporter)?;
                    return Ok(EvalResult::Number(if is_truthy(right) { 1.0 } else { 0.0 }));
                }
            }

            let right = eval_ast(right, reporter)?;

            match (&left, &right) {
                (EvalResult::Number(l), EvalResult::Number(r)) => match operator.token_type {
                    TokenType::Plus => Ok(EvalResult::Number(l + r)),
                    TokenType::Minus => Ok(EvalResult::Number(l - r)),
                    TokenType::Star => Ok(EvalResult::Number(l * r)),
                    TokenType::Slash => Ok(EvalResult::Number(l / r)),
                    
                    TokenType::Greater => Ok(EvalResult::Number(if l > r { 1.0 } else { 0.0 })),
                    TokenType::GreaterEqual => Ok(EvalResult::Number(if l >= r { 1.0 } else { 0.0 })),
                    TokenType::Less => Ok(EvalResult::Number(if l < r { 1.0 } else { 0.0 })),
                    TokenType::LessEqual => Ok(EvalResult::Number(if l <= r { 1.0 } else { 0.0 })),
                    TokenType::BangEqual => Ok(EvalResult::Number(if l != r { 1.0 } else { 0.0 })),
                    TokenType::EqualEqual => Ok(EvalResult::Number(if l == r { 1.0 } else { 0.0 })),
                    
                    _ => Err(reporter.runtime_error(operator.line, "Syntax error.")),
                },

                (EvalResult::String(l), EvalResult::String(r)) => match operator.token_type {
                    TokenType::Plus => Ok(EvalResult::String(format!("{}{}", l, r))),
                    TokenType::Minus => if l.ends_with(r) {  // If `l` ends with `r`, remove `r` from `l`.
                        Ok(EvalResult::String(l[..l.len() - r.len()].to_string()))
                    } else {
                        Ok(left)
                    }
                    
                    TokenType::Greater => Ok(EvalResult::Number(if l > r { 1.0 } else { 0.0 })),
                    TokenType::GreaterEqual => Ok(EvalResult::Number(if l >= r { 1.0 } else { 0.0 })),
                    TokenType::Less => Ok(EvalResult::Number(if l < r { 1.0 } else { 0.0 })),
                    TokenType::LessEqual => Ok(EvalResult::Number(if l <= r { 1.0 } else { 0.0 })),
                    TokenType::BangEqual => Ok(EvalResult::Number(if l != r { 1.0 } else { 0.0 })),
                    TokenType::EqualEqual => Ok(EvalResult::Number(if l == r { 1.0 } else { 0.0 })),

                    _ => Err(reporter.runtime_error(operator.line, "Syntax error.")),
                },

                (EvalResult::String(l), EvalResult::Number(r)) => match operator.token_type {
                    TokenType::Plus => Ok(EvalResult::String(format!("{}{}", l, r))),
                    TokenType::Minus => if l.ends_with(&r.to_string()) {  // If `l` ends with `r`, remove `r` from `l`.
                        Ok(EvalResult::String(l[..l.len() - r.to_string().len()].to_string()))
                    } else {
                        Ok(left)
                    },
                    TokenType::Star => {
                        // Repeat 'l' 'r' number of times.
                        let mut result = String::new();
                        for _ in 0..r.floor() as usize {
                            result.push_str(l);
                        }
                        Ok(EvalResult::String(result))
                    },
                    TokenType::Slash => {
                        // Calculate the length of `l`.  Divide that length by the ceiling value of `r`.  That number is the length of the substring of `l` to return.
                        let substring_length = ((l.len() as f64) / r.ceil()) as usize;
                        Ok(EvalResult::String(l[..substring_length].to_string()))
                    },
                    _ => Err(reporter.runtime_error(operator.line, "Syntax error.")),
                },

                (EvalResult::Number(l), EvalResult::String(r)) => match operator.token_type {
                    TokenType::Plus => Ok(EvalResult::String(format!("{}{}", l, r))),
                    _ => Err(reporter.runtime_error(operator.line, "Syntax error.")),
                },

                _ => Err(reporter.runtime_error(operator.line, "Syntax error.")),
            }
        },
        Expr::Grouping(expr) => eval_ast(expr, reporter),
        Expr::Literal(value) => match value.token_type {
            TokenType::Number => Ok(EvalResult::Number(value.lexeme.parse().unwrap())),
            TokenType::String => Ok(EvalResult::String(value.lexeme[1..value.lexeme.len() - 1].replace("\"\"", "\"").to_string())),
            TokenType::Null => Ok(EvalResult::Null),
            TokenType::True => Ok(EvalResult::Number(1.0)),
            TokenType::False => Ok(EvalResult::Number(0.0)),
            _ => Err(reporter.runtime_error(value.line, "Syntax error.")),
        },
        Expr::Unary(operator, expr) => {
            let expr = eval_ast(expr, reporter)?;
            match expr {
                EvalResult::Number(value) => match operator.token_type {
                    TokenType::Minus => Ok(EvalResult::Number(-value)),
                    TokenType::Not => Ok(EvalResult::Number(if is_truthy(expr) { 0.0 } else { 1.0 })),
                    _ => Err(reporter.runtime_error(operator.line, format!("Unknown operator: {:}", operator.lexeme).as_str())),
                },
                _ => Err(reporter.runtime_error(operator.line, format!("Expression type not allowed: {:}", expr).as_str())),
            }
        },
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
    use crate::{error_reporter::ErrorReporter, expression::eval_ast, parser::Parser, scanner::Scanner, EvalResult, Expr, Token, TokenType};

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
        test_eval("\"Hello!\"", EvalResult::String("Hello!".to_string()));
        test_eval("\"Hello\"\"World\"", EvalResult::String("Hello\"World".to_string()));
        test_eval("\"Hello\" + \" \" + \"World\"", EvalResult::String("Hello World".to_string()));
        test_eval("\"abcdefg\" - \"testing\"", EvalResult::String("abcdefg".to_string()));
        test_eval("\"abcdefg\" - \"efg\"", EvalResult::String("abcd".to_string()));
        test_eval("\"12345\" + 6", EvalResult::String("123456".to_string()));
        test_eval("6 + \"12345\"", EvalResult::String("612345".to_string()));
        test_eval("\"12345\" - 6", EvalResult::String("12345".to_string()));
        test_eval("\"12345\" - 45", EvalResult::String("123".to_string()));
        test_eval("\"12345678\" / 2", EvalResult::String("1234".to_string()));
        test_eval("\"12345678\" / 3", EvalResult::String("12".to_string()));
        test_eval("\"12345678\" / 4", EvalResult::String("12".to_string()));
        test_eval("\"12345678\" / 5", EvalResult::String("1".to_string()));
        test_eval("\"123\" * 3", EvalResult::String("123123123".to_string()));

        // Not sure if these are required cases.  They're weird.
        // test_eval("\"123\" * 3.1", EvalResult::String("123123123".to_string()));
        // test_eval("\"123\" * 3.6", EvalResult::String("1231231231".to_string()));
        // test_eval("\"123\" * 3.7", EvalResult::String("12312312312".to_string()));
    }

    fn test_eval(input: &str, expected: EvalResult) {
        let mut reporter = ErrorReporter::new();

        let mut scanner = Scanner::new(input);
        scanner.scan_tokens(&mut reporter);


        // // Print the tokens.
        // for token in &scanner.tokens {
        //     println!("{:?}", token);
        // }

        let mut parser = Parser::new(scanner.tokens);
        let expr = match parser.parse(&mut reporter) {
            Some(expr) => expr,
            None => { panic!("Syntax error.") },
        };

        // println!("{:}", expr);
        
        match eval_ast(&expr, &mut reporter) {
            Ok(result) => assert_eq!(result, expected),
            Err(err) => panic!("{}", err),
        }
    }
}