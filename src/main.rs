use miniscript::Miniscript; // This imports the library part of your project
use std::io::{self, Write};

const PROMPT: &str = "> ";

struct MiniCmd {
    interpreter: Miniscript,
}

impl MiniCmd {
    pub fn new() -> Self {
        Self { interpreter: Miniscript::new() }
    }

    pub fn print_usage(&self) {
        println!("Usage: miniscript [script]");
    }

    pub fn run_file(&mut self) -> i32 {
        let filename = std::env::args().nth(1).unwrap();
        let result = self.interpreter.run_file(&filename);
        if result {
            0
        } else {
            if self.interpreter.had_runtime_error {
                70
            } else { // if miniscript.had_error {
                65
            }
        }
    }

    pub fn run_prompt(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
    
        loop {
            print!("{}", PROMPT);
            stdout.flush().unwrap(); // Ensure the prompt is written out
    
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
    
            // Break if the user enters Ctrl+D (EOF).
            if input.is_empty() {
                break;
            }
    
            let _output = self.interpreter.run(&input);
            // println!("{}", output);
        }
    }
}

fn main() -> io::Result<()> {
    let mut minicmd = MiniCmd::new();

    // Print the command-line arguments to the screen.
    println!("{:?}", std::env::args());

    if std::env::args().len() > 2 {
        minicmd.print_usage();
    } else if std::env::args().len() == 2 {
        let result = minicmd.run_file();
        if result != 0 {
            std::process::exit(result);
        }
    } else {
        minicmd.run_prompt();
    }

    Ok(())
}
