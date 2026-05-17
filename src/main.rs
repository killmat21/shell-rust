#[allow(unused_imports)]
use std::io::{self, Write};
use itertools::join;

fn main() -> io::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer)?;

        let mut user_input = buffer.split_whitespace();
        let command: &str = &user_input.next().unwrap();
        let args: String = join(&mut user_input, " ").replace("\"", "").replace("\'", "");

        match command {
            "exit" => break,
            "echo" => println!("{args}"),
            _ => {
                let error: String = String::from(command) + ": command not found\n";
                io::stderr().write_all(&error.as_bytes())?;
            }
        }
    }
    Ok(())
}
