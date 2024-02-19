use miniscript::Miniscript; // This imports the library part of your project
use std::io::{self, Write};

const PROMPT: &str = "> ";

fn print_usage() {
    println!("Usage: miniscript [script]");
}

fn run_file() -> i32 {
    return Miniscript::new().run_file(&std::env::args().nth(1).unwrap());
}

fn run_prompt() {
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

        let output = Miniscript::new().run(&input);
        println!("{}", output);
    }
}

fn main() -> io::Result<()> {
    // Print the command-line arguments to the screen.
    println!("{:?}", std::env::args());

    if std::env::args().len() > 2 {
        print_usage();
    } else if std::env::args().len() == 2 {
        let result = run_file();
        if result != 0 {
            std::process::exit(result);
        }
    } else {
        run_prompt();
    }

    Ok(())
}
