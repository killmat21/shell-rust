#[allow(unused_imports)]
use std::io::{self, Write};
use itertools::join;
use aho_corasick::AhoCorasick;

fn _sanitize_user_input(buffer: &String) -> String {
    let patterns = &["\"", "\'"];
    let replace_with = &["", ""];

    let ac: AhoCorasick = AhoCorasick::new(patterns).unwrap();
    ac.replace_all(buffer, replace_with)
}

const ALLOWED_COMMANDS: [&str; 3] = ["exit", "echo", "type"];

fn main() -> io::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer)?;
        let buffer_sanitize: String = _sanitize_user_input(&buffer);

        let mut user_input = buffer_sanitize.split_whitespace();
        let command: &str = &user_input.next().unwrap();
        let mut args: Vec<&str> = user_input.collect();

        match command {
            "exit" => break,
            "echo" => {
                let display_str: String = join(&mut args, " ");
                println!("{display_str}");
            },
            "type" => {
                for arg in args{
                    if ALLOWED_COMMANDS.contains(&arg) {
                        println!("{arg} is a shell builtin");
                    }
                    else {
                        println!("{arg}: command not found");
                    }
                }
            },
            _ => {
                let error: String = String::from(command) + ": command not found\n";
                io::stderr().write_all(&error.as_bytes())?;
            }
        }
    }
    Ok(())
}
