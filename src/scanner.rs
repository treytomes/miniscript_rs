use crate::{error_reporter::ErrorReporter, token::Token, token_type::TokenType};

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,

    start: i64,
    current: i64,
    line: i64,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "", self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i64
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // Match the single-character operators.

            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            
            // Match the potential multi-character operators.
            '=' => if self.match_char('=') {
                self.add_token(TokenType::EqualEqual)
            } else {
                self.add_token(TokenType::Equal)
            }
            '<' => if self.match_char('=') {
                self.add_token(TokenType::LessEqual)
            } else {
                self.add_token(TokenType::Less)
            },
            '>' => if self.match_char('=') {
                self.add_token(TokenType::GreaterEqual)
            } else {
                self.add_token(TokenType::Greater)
            },

            // Match division symbol or line-comments.
            '/' => if self.match_char('/') {
                // `peek`-ing the newline is important.  The newline character will be consumed in the next cycle to increment the line counter.
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            } else {
                self.add_token(TokenType::Slash)
            },

            // Match strings.
            '"' => self.string(),

            // Match numbers.
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.number(),

            '.' => if self.peek().is_ascii_digit() {
                self.number()
            } else {
                self.add_token(TokenType::Dot)
            },

            '!' => if self.peek() == '=' {
                self.advance();
                self.add_token(TokenType::BangEqual);
            } else {
                ErrorReporter::error_line(self.line, format!("Unexpected character: {}", c).as_str());
            }

            // Match identifiers and keywords.
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),

            // Ignore the whitespace.
            ' ' | '\r' | '\t' => {}

            // Increment the line count on a newline.
            '\n' => self.line += 1,

            // Report an error on any other character.
            _ => {
                ErrorReporter::error_line(self.line, format!("Unexpected character: {}", c).as_str());
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = &self.source[self.start as usize..self.current as usize];
        let token_type = match text {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "null" => TokenType::Null,
            "not" => TokenType::Not,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_token(token_type);
    }

    fn string(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                '\n' => self.line += 1,
                '"' => if self.peek_next() == '"' {
                    self.advance();
                } else {
                    break;
                },
                _ => {},
            }

            self.advance();
          }
      
          if self.is_at_end() {
            ErrorReporter::error_line(self.line, "Unterminated string.");
            return;
          }
      
          // The closing ".
          self.advance();

          self.add_token(TokenType::String);
    }

    fn number(&mut self) {
        if self.peek() != '.' {
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(TokenType::Number);
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as i64 {
            return '\0';
        }
        self.source.chars().nth((self.current + 1) as usize).unwrap()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(Token::new(token_type, text, self.line));
    }
}
