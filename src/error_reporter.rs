use crate::{Error, Token, TokenType};

pub struct ErrorReporter {
    had_errors: bool,
    errors: Vec<Error>,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self {
            had_errors: false,
            errors: Vec::new(),
        }
    }

    pub fn error_token(&mut self, token: Token, message: &str) -> Error {
        if token.token_type == TokenType::EOF {
            self.report(token.line, " at end", message);
        }

        self.report(token.line, format!(" at '{:}'", token.lexeme).as_str(), message)
    }

    pub fn error_line(&mut self, line: i64, message: &str) -> Error {
        self.report(line, "", message)
    }

    fn report(&mut self, line: i64, location: &str, message: &str) -> Error {
        // let already_errored = self.error_exists_for_line(line);
        let error = Error::new(line, location, message);
        
        // An error token will bubble to the top if there's an error and get printed in the REPL.
        // TODO: Is this what I really want though?
        // if !already_errored {
        //     println!("{:}", error);
        // }
        // println!("Had error: {:}", already_errored);
        
        self.errors.push(error);
        self.errors.last().unwrap().clone()
    }

    pub fn had_error(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn dump(&self) {
        println!("Begin dump.");
        let mut current_line = -1;
        for error in &self.errors {
            if error.line != current_line {
                println!("{:}", error);
                current_line = error.line;
            }
        }
        println!("End dump.");
    }
}