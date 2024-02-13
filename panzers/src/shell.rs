use std::io;
use std::io::Write;
use crate::commands;

pub fn run_shell(prompt: String) {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        match input {
            "exit" => break,
            // "version" => commands::version::print_version(),
            _ => println!("Unknown command: {}", input),
        }
    }
}