#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Exit,
    Help,
    Repeat,
    Unknown,
}

impl Command {
    pub fn new(input: &str) -> Command {
        input.to_string().to_commands()
    }

    pub fn into_string(self) -> String {
        match self {
            Command::Help => "/help".to_string(),
            Command::Exit => "/exit".to_string(),
            Command::Repeat => "/repeat".to_string(),
            Command::Unknown => "".to_string(),
        }
    }
}

pub trait ToCommands {
    fn to_commands(&self) -> Command;
    fn is_command(&self) -> bool;
}

impl ToCommands for String {
    fn to_commands(&self) -> Command {
        match self.as_str() {
            "/help" => Command::Help,
            "/exit" => Command::Exit,
            "/repeat" => Command::Repeat,
            _ => Command::Unknown,
        }
    }

    fn is_command(&self) -> bool {
        self.starts_with('/')
    }
}
