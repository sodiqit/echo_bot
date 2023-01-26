mod commands;
pub mod config;
mod console;
pub mod logger;
mod telegram;

use config::{BotMode, Config};
use logger::Logger;
pub use telegram::client_types::ClientError;

pub enum BotError {
    Console(std::io::Error),
    Telegram(ClientError),
}

pub fn run_bot(config: Config, logger: &dyn Logger) -> Result<(), BotError> {
    match config.mode {
        BotMode::Console => {
            logger.log_info("start console bot");
            console::run_bot(&config, logger).map_err(BotError::Console)
        }
        BotMode::Telegram => {
            logger.log_info("start telegram bot");
            telegram::run_bot(&config, logger).map_err(BotError::Telegram)
        }
    }
}
