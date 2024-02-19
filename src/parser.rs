use crate::{error_reporter::ErrorReporter, Expr, Token, TokenType};

// Define a custom error that can be returned from a function.
#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
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

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn previous(&self) -> Token {
        self.tokens[self.current as usize - 1].clone()
    }

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

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(_err) => {
                None
            }
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
          let operator = self.previous();
          let right = self.comparison()?;
          expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
    
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_token(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
          let operator = self.previous();
          let right = self.term()?;
          expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
    
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
          let operator = self.previous();
          let right = self.factor()?;
          expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
    
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
          let operator = self.previous();
          let right = self.unary()?;
          expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
      
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal(self.previous()));
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal(self.previous()));
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal(self.previous()));
        }
    
        if self.match_token(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous()));
        }
    
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            Ok(Expr::Grouping(Box::new(expr)))
        } else {
            Err(ParseError::UnexpectedToken(self.previous()))
        }
    }

    fn consume(&self, token_type: TokenType, message: &str) -> Result<(), ParseError> {
        if self.check(token_type) {
            return Ok(());
        }

        self.error(self.peek(), message)
    }

    fn error(&self, token: Token, message: &str) -> Result<(), ParseError> {
        ErrorReporter::error_token(token.clone(), message);
        Err(ParseError::UnexpectedToken(token.clone()))?
    }

    // TODO: In MiniScript, it should be enough to just scan for the EOL.
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon {
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
    use crate::{parser::Parser, scanner::Scanner};

    #[test]
    fn test_parse_expressions() {
        test_parse_expression("-123 * (45.67)", "(* (- 123) (group 45.67))")
    }

    fn test_parse_expression(input: &str, expected_output: &str) {
        let mut scanner = Scanner::new(input);
        scanner.scan_tokens();

        let mut parser = Parser::new(scanner.tokens);
        let expr = match parser.parse() {
            Some(expr) => expr,
            None => { panic!("Syntax error.") },
        };

        let result = format!("{}", expr);

        assert_eq!(result, expected_output);
    }
}
