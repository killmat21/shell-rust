pub mod utils {
    pub fn split_command_args(buffer: &String) -> (String, String) {
        let mut command: String = String::from("");
        let mut args_start_idx = 1;
        for (idx, character) in buffer.chars().enumerate() {
            if character.is_whitespace() {
                args_start_idx = idx;
                break
            }
            command.push(character);
        }
        (command, buffer[args_start_idx..].to_string())
    }
}
