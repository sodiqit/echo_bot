mod commands;
pub mod config;
mod console;
pub mod logger;
mod telegram;

use std::process;

use config::{BotMode, Config};
use logger::Logger;
use telegram::client_types::ClientError;

pub fn run_bot<T: Logger>(config: Config, logger: &T) {
    match config.mode {
        BotMode::Console => {
            logger.log_info("start console bot");
            console::run_bot(&config, logger)
        }
        BotMode::Telegram => {
            logger.log_info("start telegram bot");
            match telegram::run_bot(&config, logger) {
                Ok(_) => {}
                Err(error) => {
                    match error {
                        ClientError::Api(e) => logger.log_error(format!("get error from api telegram: {:?}", e).as_str()),
                        ClientError::Http(e) => logger.log_error(format!("get error from http client: {}", e).as_str()),
                        ClientError::Serialize(e) => logger.log_error(format!("serialization error occurred : {}", e).as_str()),
                    }
                    process::exit(1);
                },
            }
        }
    }
}
