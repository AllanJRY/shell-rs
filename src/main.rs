#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let path = std::env::var("PATH").unwrap_or_default();
    let path: Vec<&str> = path.split(':').collect();

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
                _ => {
                    if let Some(ext_cmd) = path.iter().find(|dir| dir.ends_with(arg)) {
                        println!("{} is {}", arg, ext_cmd);
                    } else {
                        println!("{} not found", arg.trim());
                    }
                }
            },
            _ => println!("{}: command not found", input.trim()),
        };
    }
}
