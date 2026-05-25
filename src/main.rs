use std::{env, fs, io::{self, ErrorKind, Write}, os::unix::fs::PermissionsExt, path::Path, process::Command};
use io::Error;
use itertools::join;
use aho_corasick::AhoCorasick;

const BUILTIN_COMMANDS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

fn _sanitize_buffer(buffer: &String) -> (String, Vec<String>) {
    let mut command_with_args: Vec<String> = Vec::new();
    let mut in_quotes: bool = false;
    let mut in_double_quotes: bool = false;
    let mut current_quote: String = String::from("");
    for c in buffer.chars() {
        if c == '\"' && !in_quotes {
            in_double_quotes = !in_double_quotes;
        }
        else if c == '\'' && !in_double_quotes {
            in_quotes = !in_quotes;
        }
        else if in_double_quotes == false && in_quotes == false && c.is_whitespace() {
            if !current_quote.is_empty() {
                command_with_args.push(current_quote);
                current_quote = String::from("");
            }
        }
        else {
            current_quote.push(c);
        }
    }

    let command: String = match command_with_args.first() {
        Some(value) => {
            let command_value = value.to_string();
            command_with_args.remove(0);
            command_value
        },
        None => String::from(""),
    };
    (command, command_with_args)
}

fn _sanitize_command(buffer: &String) -> String {
    let patterns: &[&str; 2] = &["\"", "\'"];
    let replace_with: &[&str; 2] = &["", ""];

    let ac: AhoCorasick = AhoCorasick::new(patterns).unwrap();
    ac.replace_all(buffer, replace_with)
}


fn _find_executable_command_in_path(command: &String) -> Result<String, Error> {
    let path_env_var: String = env::var("PATH").unwrap();
    let paths = path_env_var.split(":");
    for path in paths {
        let entries = match fs::read_dir(path) {
            Ok(value) => value,
            Err(_) => continue
        };
        for entry in entries.into_iter() {
            let path = entry.unwrap().path();
            if path.is_file() && path.file_name().unwrap() == command.as_str() {
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

fn _execute_command(command: &String, args: Vec<String>) -> Result<(), Error> {
    let output = Command::new(command).args(args)
        .output()
        .expect("Failed to execute process");
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    Ok(())
}

fn _get_current_working_dir() -> Result<String, Error> {
    let path = env::current_dir()?;
    Ok(path.display().to_string())
}

fn _print_stderr(error: String) -> () {
    io::stderr().write_all(&error.as_bytes()).ok();
}

fn main() -> io::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer)?;
        let (command, args) = _sanitize_buffer(&buffer);

        match command.as_str() {
            "exit" => break,
            "pwd" => {
                let cwd: String = _get_current_working_dir().expect("");
                println!("{cwd}");
            },
            "cd" => {
                let path: String;
                if args.is_empty() || args[0] == "~" {
                    path = env::home_dir().unwrap().display().to_string();
                }
                else {
                    path = match args.first() {
                        Some(value) => value.to_string(),
                        None => String::from(""),
                    };
                }
                let root = Path::new(&path);
                if env::set_current_dir(&root).is_err() {
                    _print_stderr(format!("cd: {}: No such file or directory\n", path));
                }
            },
            "echo" => {
                let display_str: String = join(args, " ");
                println!("{display_str}");
            },
            "type" => {
                for arg in args{
                    if BUILTIN_COMMANDS.contains(&arg.as_str()) {
                        println!("{arg} is a shell builtin");
                    }
                    else {
                        match _find_executable_command_in_path(&arg){
                            Ok(path) => println!("{arg} is {path}"),
                            Err(_) => _print_stderr(format!("{arg}: not found\n")),
                        };
                    }
                }
            },
            _ => {
                match _find_executable_command_in_path(&command){
                    Ok(_) => _execute_command(&command.to_owned(), args)?,
                    Err(_) => {
                        _print_stderr(String::from(command) + ": command not found\n");
                    },
                };
            }
        }
    }
    Ok(())
}
