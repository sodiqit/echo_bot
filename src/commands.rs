use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Exit,
    Help,
    Repeat,
    Unknown,
}

impl Command {
    pub fn new(input: &str) -> Command {
        input.parse::<Command>().unwrap()
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

pub trait IsCommand {
    fn is_command(&self) -> bool;
}

impl IsCommand for String {
    fn is_command(&self) -> bool {
        self.starts_with('/')
    }
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = match s {
            "/help" => Command::Help,
            "/exit" => Command::Exit,
            "/repeat" => Command::Repeat,
            _ => Command::Unknown,
        };

        Ok(command)
    }
}