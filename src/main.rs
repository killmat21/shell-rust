#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> io::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer)?;

        let command: &str = buffer.split_whitespace().next().unwrap();
        let error: String = String::from(command) + ": command not found\n";
        io::stderr().write_all(&error.as_bytes())?;
    }
    Ok(())
}
