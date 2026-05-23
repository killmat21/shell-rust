use std::{fs, io::{self, Write}};
use itertools::join;
use aho_corasick::AhoCorasick;

const ALLOWED_COMMANDS: [&str; 3] = ["exit", "echo", "type"];

fn _sanitize_user_input(buffer: &String) -> String {
    let patterns: &[&str; 2] = &["\"", "\'"];
    let replace_with: &[&str; 2] = &["", ""];

    let ac: AhoCorasick = AhoCorasick::new(patterns).unwrap();
    ac.replace_all(buffer, replace_with)
}

fn _find_executable_command_in_path(command: &str) -> () {
    let path_env_var: String = std::env::var("PATH").unwrap();
    let paths = path_env_var.split(":");
    for path in paths {
        let entries = match fs::read_dir(path) {
            Ok(value) => value,
            Err(_) => continue
        };
        for entry in entries.into_iter() {
            let path = entry.unwrap().path();
            if path.is_file() && path.file_name().unwrap() == command {
                let utf8_path = path.into_os_string().into_string().unwrap();
                println!("{command} is {utf8_path}");
                return;
            }
        }
    }
    println!("{command}: not found");
}

fn main() -> io::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer)?;
        let buffer_sanitize: String = _sanitize_user_input(&buffer);

        let mut user_input = buffer_sanitize.split_whitespace();
        let command: &str = match &user_input.next() {
            Some(val) => val,
            None => continue
        };
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
                        _find_executable_command_in_path(arg);
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
