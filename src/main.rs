#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        match input.first().unwrap().as_str() {
            "exit" => {
                let code: i32 = input
                    .get(1)
                    .map(|code_str| code_str.parse().expect("Invalid exit code"))
                    .unwrap_or_default();
                std::process::exit(code);
            }
            _ => println!("{}: command not found", input.first().unwrap()),
        };
    }
}
