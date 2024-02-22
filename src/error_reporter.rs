use crate::{error_stage::ErrorStage, Error, Token, TokenType};

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

    pub fn runtime_error(&mut self, line: i64, message: &str) -> Error {
        self.report(line, "", message, ErrorStage::Runtime)
    }

    pub fn error_token(&mut self, token: Token, message: &str) -> Error {
        if token.token_type == TokenType::EOF {
            self.report(token.line, " at end", message, ErrorStage::Compile);
        }

        self.report(token.line, format!(" at '{:}'", token.lexeme).as_str(), message, ErrorStage::Compile)
    }

    pub fn error_line(&mut self, line: i64, message: &str) -> Error {
        self.report(line, "", message, ErrorStage::Compile)
    }

    fn report(&mut self, line: i64, location: &str, message: &str, stage: ErrorStage) -> Error {
        // let already_errored = self.error_exists_for_line(line);
        let error = Error::new(line, location, message, stage);
        
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
        // Were there any Compile errors?
        for error in &self.errors {
            if error.stage == ErrorStage::Compile {
                return true;
            }
        }
        false
    }

    pub fn had_runtime_error(&self) -> bool {
        for error in &self.errors {
            if error.stage == ErrorStage::Runtime {
                return true;
            }
        }
        false
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