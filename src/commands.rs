pub enum Commands {
    Help,
    Repeat,
    Exit,
    Unknown,
}

pub trait ToCommands {
    fn to_commands(&self) -> Commands;
    fn is_command(&self) -> bool;
}

impl ToCommands for String {
    fn to_commands(&self) -> Commands {
        match self.as_str() {
            "/help" => Commands::Help,
            "/exit" => Commands::Exit,
            "/repeat" => Commands::Repeat,
            _ => Commands::Unknown,
        }
    }

    fn is_command(&self) -> bool {
        self.starts_with("/")
    }
}