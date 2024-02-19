// src/lib.rs

mod error_reporter;
mod expression;
mod parser;
mod scanner;
mod token;
mod token_type;

use scanner::Scanner;
use error_reporter::ErrorReporter;

pub use expression::{Expr, format_ast};
pub use token::Token;
pub use token_type::TokenType;

pub struct Miniscript {}

impl Miniscript {
    pub fn new() -> Miniscript {
        Self {}
    }

    pub fn run(&self, code: &str) -> i32 {
        // Placeholder for your language execution logic
        println!("Executing code: {}", code);

        let mut scanner = Scanner::new(code);
        scanner.scan_tokens();

        // For now, just print the tokens.
        for token in scanner.tokens {
            println!("Token: {:?}", token);
        }

        return if ErrorReporter::had_error() { 65 } else { 0 };
    }

    pub fn run_file(&self, path: &str) -> i32 {
        // Placeholder for your language execution logic
        println!("Loading: {}", path);

        // Load the contents of `path` into a string.  Throw an error if the path does not exist.
        let contents = std::fs::read_to_string(path).expect("Failed to read file.");
        println!("Contents: {}", contents);

        // Run the code in the file
        let result = self.run(&contents);
        println!("Result: {}", result);

        return result;
    }
}
