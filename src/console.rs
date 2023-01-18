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
        return Some(construct_repeated_message(input.as_str(), state, config));
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
        }
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
    let error = "Try again input number".to_string();
    let count: i32 = match input.parse() {
        Ok(res) => {
            if res == 0 {
                return error;
            }
            res
        }
        Err(_) => {
            return error;
        }
    };

    state.repeat_number = Some(count);
    state.is_await_repeat_number = false;

    format!("Repeat message count currently is: {}", count)
}

#[cfg(test)]
mod tests {
    use crate::config::{BotMode, ConfigBuilder};

    use super::*;

    #[test]
    fn should_success_return_help_msg() {
        let mut state = State::new();
        let input = "/help".to_string();
        let config = ConfigBuilder::build_default(BotMode::Console);

        let response = respond_user(input, &mut state, &config);
        assert_eq!(response, Some("help msg".to_string()));
    }

    #[test]
    fn should_return_none_if_provide_exit_command() {
        let mut state = State::new();
        let input = "/exit".to_string();
        let config = ConfigBuilder::build_default(BotMode::Console);

        let response = respond_user(input, &mut state, &config);
        assert_eq!(response, None);
    }

    #[test]
    fn should_return_error_if_provided_unknown_command() {
        let mut state = State::new();
        let input = "/unknown".to_string();
        let input_clone = input.clone();
        let config = ConfigBuilder::build_default(BotMode::Console);

        let response = respond_user(input, &mut state, &config);
        assert_eq!(
            response,
            Some(format!(
                "Unknown command: {}. Supported commands: /help, /repeat, /exit",
                input_clone
            ))
        );
    }

    #[test]
    fn should_success_repeat_message_with_default_repeat_count() {
        let mut state = State::new();
        let input = "test".to_string();
        let config = ConfigBuilder::build_default(BotMode::Console);

        let response = respond_user(input, &mut state, &config);
        assert_eq!(response, Some("test".to_string()));
    }

    #[test]
    fn should_return_error_if_provided_invalid_number() {
        let mut state = State::new();
        let input = "/repeat".to_string();
        let config = ConfigBuilder::build_default(BotMode::Console);

        respond_user(input, &mut state, &config);

        let response1 = respond_user("0".to_string(), &mut state, &config);
        assert_eq!(response1, Some("Try again input number".to_string()));
        assert_eq!(state.repeat_number, None);

        let response2 = respond_user("txt".to_string(), &mut state, &config);
        assert_eq!(response2, Some("Try again input number".to_string()));
        assert_eq!(state.repeat_number, None);
    }

    #[test]
    fn should_success_change_repeat_number() {
        let mut state = State::new();
        let input = "/repeat".to_string();
        let config = ConfigBuilder::build_default(BotMode::Console);

        let response1 = respond_user(input, &mut state, &config);
        assert_eq!(
            response1,
            Some(format!(
                "Currently repeat set: {}. {}",
                config.default_repeat_number, config.repeat_msg
            ))
        );

        let response2 = respond_user("3".to_string(), &mut state, &config);
        assert_eq!(
            response2,
            Some("Repeat message count currently is: 3".to_string())
        );
        assert_eq!(state.repeat_number, Some(3));
    }

    #[test]
    fn should_success_repeat_message_after_change_number() {
        let mut state = State::new();
        let input = "/repeat".to_string();
        let config = ConfigBuilder::build_default(BotMode::Console);

        respond_user(input, &mut state, &config);
        respond_user("2".to_string(), &mut state, &config);
        let response = respond_user("test".to_string(), &mut state, &config);
        assert_eq!(response, Some("test\ntest".to_string()));
    }
}
