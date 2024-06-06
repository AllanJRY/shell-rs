#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let ext_cmds = ExtCmds::new();

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
                    if let Some(ext_cmd) = ext_cmds.try_find_cmd(arg) {
                        println!("{arg} is {ext_cmd}");
                    } else {
                        println!("{arg} not found");
                    }
                }
            },
            Some((cmd, arg)) => {
                if let Some(ext_cmd) = ext_cmds.try_find_cmd(cmd) {
                    // .status() inherit from the parent stdout, so no need to collect the ouput.
                    std::process::Command::new(ext_cmd)
                        .arg(arg)
                        .status()
                        .expect("unable to execute external command");
                } else {
                    println!("{input}: command not found");
                }
            }
            _ => println!("{input}: command not found"),
        };
    }
}

#[derive(Debug)]
struct ExtCmds {
    path_var: Vec<String>,
}

impl ExtCmds {
    fn new() -> Self {
        let path = std::env::var("PATH").unwrap_or_default();
        let path: Vec<String> = path.split(':').map(|path| path.to_string()).collect();
        Self { path_var: path }
    }

    fn try_find_cmd(&self, cmd_name: &str) -> Option<String> {
        self.path_var
            .iter()
            .find(|path| std::fs::metadata(format!("{path}/{cmd_name}")).is_ok())
            .map(|path| format!("{path}/{cmd_name}"))
    }
}
