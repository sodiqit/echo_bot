pub mod config;
mod commands;
mod console;

use config::{BotMode, Config};

pub fn run_bot(config: Config) {
    match config.mode {
        BotMode::Console => console::run_bot(&config),
        BotMode::Telegram => todo!(),
    }
}

