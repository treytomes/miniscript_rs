use crate::{Token, TokenType};

static mut HAD_ERROR: bool = false;

pub struct ErrorReporter {
}

// Define a statif field on ErrorReporter for had_error.
impl ErrorReporter {
    pub fn error_token(token: Token, message: &str) {
        if token.token_type == TokenType::EOF {
            ErrorReporter::report(token.line, " at end", message);
        }

        ErrorReporter::report(token.line, format!(" at '{:}'", token.lexeme).as_str(), message)
    }

    pub fn error_line(line: i64, message: &str) {
        ErrorReporter::report(line, "", message)
    }

    pub fn report(line: i64, location: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, location, message);
        unsafe {
            HAD_ERROR = true;
        }
    }

    pub fn had_error() -> bool {
        unsafe {
            HAD_ERROR
        }
    }
}