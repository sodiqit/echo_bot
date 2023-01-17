use crate::commands::{Commands, ToCommands};
use crate::config::Config;

pub fn run_bot(config: &Config) {
    loop {
        let input = get_user_message();

        let response = respond_user(input, config);

        if let Some(answer) = response {
            println!("{}", answer);
        } else {
            break;
        }
    }
}

fn respond_user(input: String, config: &Config) -> Option<String> {
    if !input.is_command() {
        return Some(format!(
            "{}",
            construct_repeated_message(input.as_str(), config.default_repeat_number)
        ));
    }

    match input.to_commands() {
        Commands::Help => Some(config.help_msg.clone()),
        Commands::Exit => None,
        Commands::Repeat => todo!(),
        Commands::Unknown => Some(format!(
            "Unknown command: {}. Supported commands: /help, /repeat, /exit",
            input
        )),
    }
}

fn construct_repeated_message(input: &str, count: i32) -> String {
    (0..count).into_iter().fold(input.to_owned(), |mut acc, _| {
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
