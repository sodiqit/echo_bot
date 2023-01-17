use crate::commands::{Commands, ToCommands};
use crate::config::Config;

struct State {
    pub repeat_number: Option<i32>,
    pub is_await_repeat_number: bool,
}

impl State {
    fn new() -> Self {
        State {
            repeat_number: None,
            is_await_repeat_number: false,
        }
    }
}

pub fn run_bot(config: &Config) {
    let mut state = State::new();

    loop {
        let input = get_user_message();

        let response = respond_user(input, &mut state, config);

        if let Some(answer) = response {
            println!("{}", answer);
        } else {
            break;
        }
    }
}

fn respond_user(input: String, state: &mut State, config: &Config) -> Option<String> {
    let command = input.to_commands();

    if let Commands::Exit = command {
        return None;
    }

    if state.is_await_repeat_number {
        return Some(extract_repeat_count(input, state));
    }

    if !input.is_command() {
        return Some(format!(
            "{}",
            construct_repeated_message(input.as_str(), state, config)
        ));
    }

    match command {
        Commands::Help => Some(config.help_msg.clone()),
        Commands::Repeat => {
            state.is_await_repeat_number = true;
            Some(format!(
                "Currently repeat set: {}. {}",
                state.repeat_number.unwrap_or(config.default_repeat_number),
                config.repeat_msg
            ))
        },
        Commands::Unknown => Some(format!(
            "Unknown command: {}. Supported commands: /help, /repeat, /exit",
            input
        )),
        Commands::Exit => unreachable!(),
    }
}

fn construct_repeated_message(input: &str, state: &mut State, config: &Config) -> String {
    let count = state.repeat_number.unwrap_or(config.default_repeat_number);

    (0..count - 1)
        .into_iter()
        .fold(input.to_owned(), |mut acc, _| {
            acc.push_str(format!("\n{}", input).as_str());
            acc
        })
}

fn get_user_message() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    input
}

fn extract_repeat_count(input: String, state: &mut State) -> String {
    let count: i32 = match input.parse() {
        Ok(res) => res,
        Err(_) => {
            return format!("Try again input number");
        }
    };

    state.repeat_number = Some(count);
    state.is_await_repeat_number = false;

    format!("Repeat message count currently is: {}", count)
}
