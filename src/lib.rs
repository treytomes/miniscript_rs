// src/lib.rs

mod environment;
mod error;
mod error_reporter;
mod error_stage;
mod eval_result;
mod expression;
mod parser;
mod scanner;
mod statement;
mod token;
mod token_type;

use environment::Environment;
use expression::eval_stmts;
use scanner::Scanner;
use error_reporter::ErrorReporter;

pub use error::Error;
pub use eval_result::EvalResult;
pub use expression::{Expr, format_ast};
pub use token::Token;
pub use token_type::TokenType;

use crate::parser::Parser;

pub struct Miniscript {
    pub globals: Environment,
    
    pub had_error: bool,
    pub had_runtime_error: bool,
}

impl Miniscript {
    pub fn new() -> Miniscript {
        Self {
            globals: Environment::new_root(),
            had_error: false,
            had_runtime_error: false,
        }
    }

    pub fn run(&mut self, code: &str) -> bool {
        let mut reporter = ErrorReporter::new();

        // // Placeholder for your language execution logic
        // println!("Executing code: {}", code);

        let mut scanner = Scanner::new(code);
        scanner.scan_tokens(&mut reporter);

        // Write the tokens to stdout.
        // for token in &scanner.tokens {
        //     println!("Token: {:?}", token);
        // }

        let mut parser = Parser::new(scanner.tokens);
        
        let stmts = match parser.parse(&mut reporter) {
            Ok(stmts) => stmts,
            _ => {
                /* Errors will be handled later. */
                Vec::new()
            },
        };

        // // Print all statements to stdout.
        // for stmt in &stmts {
        //     println!("Statement: {:?}", stmt);
        // }

        let result = match eval_stmts(&mut self.globals, &stmts, &mut reporter) {
            Ok(result) => result,
            _ => {
                /* Errors will be handled later. */
                EvalResult::Null
            }
        };
        if result != EvalResult::Null {
            println!("{}", result);
        }
        
        self.had_error = reporter.had_error();
        self.had_runtime_error = reporter.had_runtime_error();

        if reporter.had_error() || reporter.had_runtime_error() {
            reporter.dump();
        }
        reporter.had_error()
    }

    pub fn run_file(&mut self, path: &str) -> bool {
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
