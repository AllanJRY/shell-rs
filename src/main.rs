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
        let input = input.trim();

        let maybe_cmd = input
            .trim()
            .split_once(' ')
            .map(|(cmd, args)| (cmd.trim(), args.trim()));

        match maybe_cmd {
            Some(("exit", arg)) => {
                std::process::exit(arg.parse().expect("Invalid exit code"));
            }
            Some(("echo", arg)) => {
                println!("{arg}");
            }
            Some(("type", arg)) => match arg {
                "echo" | "exit" | "type" => println!("{arg} is a shell builtin"),
                _ => {
                    if let Some(ext_cmd) = path
                        .iter()
                        .find(|path| std::fs::metadata(format!("{path}/{arg}")).is_ok())
                    {
                        println!("{arg} is {ext_cmd}/{arg}");
                    } else {
                        println!("{arg} not found");
                    }
                }
            },
            _ => println!("{input}: command not found"),
        };
    }
}
