use std::{error::Error, fmt::{self, Display, Formatter}};

use crate::{error_reporter::ErrorReporter, statement::Stmt, Expr, Token, TokenType};

// Define a custom error that can be returned from a function.
#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match self {
            ParseError::UnexpectedToken(_) => "Unexpected token",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => write!(f, "Unexpected token: {:?}", token),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,

    current: i64,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn peek(&self) -> Token {
        self.tokens[self.current as usize].clone()
    }

    fn peek_next(&self) -> Token {
        self.tokens[(self.current + 1) as usize].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return token_type == TokenType::EOF;
        }

        self.peek().token_type == token_type
    }

    fn previous(&self) -> Token {
        self.tokens[self.current as usize - 1].clone()
    }

    // Return the current token; move the pointer to the next token.
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous()
    }

    // The `match_token` function will take in one or more TokenTypes as parameters, then validate that the next token matches of of these token types.
    // If it does, it will consume the token and return true. Otherwise, it will return false.
    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(*t) {
                self.advance();
                return true;
            }
        }

        false
    }

    pub fn parse(&mut self, reporter: &mut ErrorReporter) -> Result<Vec<Stmt>, ParseError> {
        self.program(reporter)
    }

    fn program(&mut self, reporter: &mut ErrorReporter) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();

        while !self.is_at_end() {
            // let stmt = self.statement(reporter)?;
            // stmts.push(stmt);

            match self.statement(reporter) {
                Ok(stmt) => {
                    stmts.push(stmt);
                },
                Err(_e) => {
                    // panic!("{:?}", e);
                    self.synchronize();
                }
            }
        }

        Ok(stmts)
    }

    fn statement(&mut self, reporter: &mut ErrorReporter) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_stmt(reporter);
        // } else if self.peek().token_type == TokenType::Identifier { //} (&[TokenType::Identifier]) {
        //     if self.peek_next().token_type == TokenType::Equal {
        //         return self.assignment_stmt(reporter);
        //     }
        }

        self.expr_stmt(reporter)
    }

    fn print_stmt(&mut self, reporter: &mut ErrorReporter) -> Result<Stmt, ParseError> {
        let expr = self.expression(reporter)?;
        self.end_of_stmt(reporter)?;
        Ok(Stmt::Print(expr))
    }

    // fn assignment_stmt(&mut self, reporter: &mut ErrorReporter) -> Result<Stmt, ParseError> {
    //     if !self.match_token(&[TokenType::Identifier]) {
    //         return Err(self.error(self.peek(), "Expected identifier.", reporter));
    //     }
    //     let name = self.previous();

    //     self.consume(TokenType::Equal, "Expected '=' after identifier.", reporter)?;
        
    //     let expr = self.expression(reporter)?;
    //     self.end_of_stmt(reporter)?;
    //     return Ok(Stmt::Assignment(name.lexeme.clone(), expr));
    // }

    fn expr_stmt(&mut self, reporter: &mut ErrorReporter) -> Result<Stmt, ParseError> {
        let expr = self.expression(reporter)?;
        self.end_of_stmt(reporter)?;
        Ok(Stmt::Expression(expr))
    }

    fn end_of_stmt(&mut self, reporter: &mut ErrorReporter) -> Result<(), ParseError> {
        let mut eos_count = 0;
        while self.match_token(&[TokenType::SemiColon, TokenType::NewLine, TokenType::EOF]) {
            eos_count += 1;
            if self.is_at_end() {
                break;
            }
        }
        if eos_count > 0 {
            Ok(())
        } else {
            Err(self.error(self.peek(), "Expected ';', EOL, or EOF.", reporter))
        }
    }

    fn expression(&mut self, reporter: &mut ErrorReporter) -> Result<Expr, ParseError> {
        self.assignment(reporter)
    }

    fn assignment(&mut self, reporter: &mut ErrorReporter) -> Result<Expr, ParseError> {
        let mut expr = self.logical(reporter)?;

        // This will group the expressions from right-to-left, allowing constructs like `a=b=2`.
        if self.match_token(&[TokenType::Equal]) {
            let operator = self.previous();
            let right = self.assignment(reporter)?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        // This will group the expressions from left-to-right.
        // while self.match_token(&[TokenType::Equal]) {
        //   let operator = self.previous();
        //   let right = self.logical(reporter)?;
        //   expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        // }
    
        Ok(expr)
    }

    fn logical(&mut self, reporter: &mut ErrorReporter) -> Result<Expr, ParseError> {
        let mut expr = self.equality(reporter)?;

        while self.match_token(&[TokenType::And, TokenType::Or]) {
          let operator = self.previous();
          let right = self.equality(reporter)?;
          expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
    
        Ok(expr)
    }

    fn equality(&mut self, reporter: &mut ErrorReporter) -> Result<Expr, ParseError> {
        let mut expr = self.comparison(reporter)?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
          let operator = self.previous();
          let right = self.comparison(reporter)?;
          expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
    
        Ok(expr)
    }

    fn comparison(&mut self, reporter: &mut ErrorReporter) -> Result<Expr, ParseError> {
        let mut expr = self.term(reporter)?;

        while self.match_token(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
          let operator = self.previous();
          let right = self.term(reporter)?;
          expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
    
        Ok(expr)
    }

    fn term(&mut self, reporter: &mut ErrorReporter) -> Result<Expr, ParseError> {
        let mut expr = self.factor(reporter)?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
          let operator = self.previous();
          let right = self.factor(reporter)?;
          expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
    
        Ok(expr)
    }

    fn factor(&mut self, reporter: &mut ErrorReporter) -> Result<Expr, ParseError> {
        let mut expr = self.unary(reporter)?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
          let operator = self.previous();
          let right = self.unary(reporter)?;
          expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self, reporter: &mut ErrorReporter) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::Not, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary(reporter)?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
      
        self.primary(reporter)
    }

    fn primary(&mut self, reporter: &mut ErrorReporter) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::False, TokenType::True, TokenType::Null, TokenType::Number, TokenType::String, TokenType::Identifier]) {
            return Ok(Expr::Literal(self.previous()));
        }
    
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression(reporter)?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.", reporter)?;
            Ok(Expr::Grouping(Box::new(expr)))
        } else {
            Err(ParseError::UnexpectedToken(self.peek()))
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str, reporter: &mut ErrorReporter) -> Result<(), ParseError> {
        if self.check(token_type) {
            self.advance();
            return Ok(());
        }

        Err(self.error(self.peek(), message, reporter))
    }

    fn error(&self, token: Token, message: &str, reporter: &mut ErrorReporter) -> ParseError {
        reporter.error_token(token.clone(), message);
        ParseError::UnexpectedToken(token.clone())
    }

    // TODO: In MiniScript, it should be enough to just scan for the EOL.
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon || self.previous().token_type == TokenType::NewLine || self.is_at_end() {
                return;
            }

            match self.peek().token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{error_reporter::ErrorReporter, parser::Parser, scanner::Scanner};

    #[test]
    fn test_parse_expressions() {
        test_parse_expression("-123 * (45.67)", "(* (- 123) (group 45.67))")
    }

    fn test_parse_expression(input: &str, expected_output: &str) {
        let mut reporter = ErrorReporter::new();

        let mut scanner = Scanner::new(input);
        scanner.scan_tokens(&mut reporter);

        let mut parser = Parser::new(scanner.tokens);
        let prgm = match parser.parse(&mut reporter) {
            Ok(prgm) => prgm,
            Err(err) => {
                println!("ParseError: {:?}", err);
                println!("\"{:?}\": {:?}", input, expected_output);
                panic!("Syntax error.");
            },
        };

        let result = format!("{:}", prgm[0]); // Assuming we are only running a single line.

        assert_eq!(result, expected_output);
    }
}
