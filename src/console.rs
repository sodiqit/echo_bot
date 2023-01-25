use crate::commands::{Command, ToCommands};
use crate::config::Config;
use crate::logger::Logger;

struct State {
    pub repeat_number: Option<u8>,
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

pub fn run_bot<T: Logger>(config: &Config, logger: &T) {
    let mut state = State::new();

    loop {
        let input = get_user_message();

        let Some(response) = respond_user(input, &mut state, config, logger) else { break }; // fancy new syntax.
        println!("{answer}");
    }
}

fn respond_user<T: Logger>(
    input: String,
    state: &mut State,
    config: &Config,
    logger: &T,
) -> Option<String> {
    let command = input.to_commands();

    if let Command::Exit = command {
        return None;
    }

    if state.is_await_repeat_number {
        return Some(extract_repeat_count(input, state, logger));
    }

    if !input.is_command() {
        return Some(construct_repeated_message(
            input.as_str(),
            state,
            config,
            logger,
        ));
    }

    logger.log_debug(format!("handle user command: {:?}", command).as_str());

    match command {
        Command::Help => Some(config.help_msg.clone()),
        Command::Repeat => {
            state.is_await_repeat_number = true;
            Some(format!(
                "Currently repeat set: {}. {}",
                state.repeat_number.unwrap_or(config.default_repeat_number),
                config.repeat_msg
            ))
        }
        Command::Unknown => {
            let response = format!(
                "Unknown command: {}. Supported commands: /help, /repeat, /exit",
                input
            );
            logger.log_warn(&response);
            Some(response)
        }
        Command::Exit => unreachable!(),
    }
}

fn construct_repeated_message<T: Logger>(
    input: &str,
    state: &mut State,
    config: &Config,
    logger: &T,
) -> String {
    logger.log_info(format!("respond to user input: {}", input).as_str());
    let count = state.repeat_number.unwrap_or(config.default_repeat_number);


    // Don't really love this loop, for two reasons:
    //  - folds in general are hard to read
    //  - it does a lot of intermediate allocations
    (0..count - 1)
        .into_iter()
        .fold(input.to_owned(), |mut acc, _| {
            acc.push_str(format!("\n{}", input).as_str());
            acc
        });
    // I'd probably try to keep that super simple:
    let mut result = input.to_string();
    for _ in 0..count - 1 {
        result.push_str("\n");
        result.push_str(input);
    }
    result
}

fn get_user_message() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap(); // Again, better to `?` this error.
    input = input.trim().to_string();

    input
}

fn extract_repeat_count<T: Logger>(input: &str, state: &mut State, logger: &T) -> String {
    let error = "Try again input number".to_string();
    let count: u8 = match input.parse() {
        Ok(res) => {
            if res == 0 {
                logger.log_warn("input number can't be zero");
                return error;
            }
            res
        }
        Err(e) => {
            logger.log_warn(format!("failed parsing: {:?}", e).as_str());
            return error;
        }
    };

    state.repeat_number = Some(count);
    state.is_await_repeat_number = false;

    format!("Repeat message count currently is: {}", count)
}

#[cfg(test)]
mod tests {
    use crate::{
        config::{BotMode, ConfigBuilder},
        logger::LogLevel,
    };

    use super::*;

    #[derive(Default)]
    struct MockLogger {}

    impl Logger for MockLogger {
        fn log(&self, _log_level: LogLevel, _msg: &str) {}
    }

    #[test]
    fn should_success_return_help_msg() {
        let mut state = State::new();
        let logger = MockLogger::default();
        let input = "/help".to_string();
        let config = ConfigBuilder::build_default(BotMode::Console);

        let response = respond_user(input, &mut state, &config, &logger);
        assert_eq!(response, Some("help msg".to_string()));
    }

    #[test]
    fn should_return_none_if_provide_exit_command() {
        let mut state = State::new();
        let logger = MockLogger::default();
        let input = "/exit".to_string();
        let config = ConfigBuilder::build_default(BotMode::Console);

        let response = respond_user(input, &mut state, &config, &logger);
        assert_eq!(response, None);
    }

    #[test]
    fn should_return_error_if_provided_unknown_command() {
        let mut state = State::new();
        let logger = MockLogger::default();
        let input = "/unknown".to_string();
        let input_clone = input.clone();
        let config = ConfigBuilder::build_default(BotMode::Console);

        let response = respond_user(input, &mut state, &config, &logger);
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
        let logger = MockLogger::default();
        let input = "test".to_string();
        let config = ConfigBuilder::build_default(BotMode::Console);

        let response = respond_user(input, &mut state, &config, &logger);
        assert_eq!(response, Some("test".to_string()));
    }

    #[test]
    fn should_return_error_if_provided_invalid_number() {
        let mut state = State::new();
        let logger = MockLogger::default();
        let input = "/repeat".to_string();
        let config = ConfigBuilder::build_default(BotMode::Console);

        respond_user(input, &mut state, &config, &logger);

        let response1 = respond_user("0".to_string(), &mut state, &config, &logger);
        assert_eq!(response1, Some("Try again input number".to_string()));
        assert_eq!(state.repeat_number, None);

        let response2 = respond_user("txt".to_string(), &mut state, &config, &logger);
        assert_eq!(response2, Some("Try again input number".to_string()));
        assert_eq!(state.repeat_number, None);
    }

    #[test]
    fn should_success_change_repeat_number() {
        let mut state = State::new();
        let logger = MockLogger::default();
        let input = "/repeat".to_string();
        let config = ConfigBuilder::build_default(BotMode::Console);

        let response1 = respond_user(input, &mut state, &config, &logger);
        assert_eq!(
            response1,
            Some(format!(
                "Currently repeat set: {}. {}",
                config.default_repeat_number, config.repeat_msg
            ))
        );

        let response2 = respond_user("3".to_string(), &mut state, &config, &logger);
        assert_eq!(
            response2,
            Some("Repeat message count currently is: 3".to_string())
        );
        assert_eq!(state.repeat_number, Some(3));
    }

    #[test]
    fn should_success_repeat_message_after_change_number() {
        let mut state = State::new();
        let logger = MockLogger::default();
        let input = "/repeat".to_string();
        let config = ConfigBuilder::build_default(BotMode::Console);

        respond_user(input, &mut state, &config, &logger);
        respond_user("2".to_string(), &mut state, &config, &logger);
        let response = respond_user("test".to_string(), &mut state, &config, &logger);
        assert_eq!(response, Some("test\ntest".to_string()));
    }
}
