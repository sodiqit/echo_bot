pub mod config;
mod commands;
mod console;
pub mod logger;

use config::{BotMode, Config};
use logger::Logger;

pub fn run_bot<T: Logger>(config: Config, logger: &T) {
    match config.mode {
        BotMode::Console => {
            logger.log_info("start console bot");
            console::run_bot(&config, logger)
        },
        BotMode::Telegram => {
            logger.log_info("start telegram bot");
            todo!()
        }
    }
}

