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

        let maybe_cmd = input.trim().split_once(' ');

        match maybe_cmd {
            Some(("exit", arg)) => {
                std::process::exit(arg.parse().expect("Invalid exit code"));
            }
            Some(("echo", arg)) => {
                println!("{arg}");
            }
            Some(("type", arg)) => match arg {
                "echo" | "exit" | "type" => println!("{} is a shell builtin", arg.trim()),
                _ => println!("{} not found.", arg.trim()),
            },
            _ => println!("{}: command not found", input.trim()),
        };
    }
}
