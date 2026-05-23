use std::{fs, io::{self, ErrorKind, Write}, os::unix::fs::PermissionsExt, process::Command};
use io::Error;
use itertools::join;
use aho_corasick::AhoCorasick;

const ALLOWED_COMMANDS: [&str; 4] = ["exit", "echo", "type", "pwd"];

fn _sanitize_user_input(buffer: &String) -> String {
    let patterns: &[&str; 2] = &["\"", "\'"];
    let replace_with: &[&str; 2] = &["", ""];

    let ac: AhoCorasick = AhoCorasick::new(patterns).unwrap();
    ac.replace_all(buffer, replace_with)
}

fn _find_executable_command_in_path(command: &str) -> Result<String, Error> {
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
                let md = match fs::metadata(&utf8_path) {
                    Ok(metadata) => metadata,
                    Err(_) => continue,
                };
                let is_executable = md.permissions().mode() & 0o111 != 0;
                if is_executable {
                    return Ok(utf8_path);
                }
            }
        }
    }
    Err(Error::new(ErrorKind::NotFound, "No executable found in PATH environment variable"))
}

fn _execute_command(command: &String, args: Vec<&str>) -> Result<(), Error> {
    let output = Command::new(command).args(args)
        .output()
        .expect("Failed to execute process");
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    Ok(())
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
            "pwd" => {
                let pwd_env_var: String = std::env::var("PWD").unwrap();
                println!("{pwd_env_var}");
            },
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
                        match _find_executable_command_in_path(arg){
                            Ok(path) => println!("{arg} is {path}"),
                            Err(_) => println!("{arg}: not found"),
                        };
                    }
                }
            },
            _ => {
                match _find_executable_command_in_path(command){
                    Ok(_) => _execute_command(&command.to_owned(), args)?,
                    Err(_) => {
                        let error: String = String::from(command) + ": command not found\n";
                        io::stderr().write_all(&error.as_bytes())?;
                    },
                };
            }
        }
    }
    Ok(())
}
