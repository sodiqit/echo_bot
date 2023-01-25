mod commands;
pub mod config;
mod console;
pub mod logger;
mod telegram;

use std::process;

use config::{BotMode, Config};
use logger::Logger;
use telegram::client_types::ClientError;

pub fn run_bot(config: Config, logger: &dyn Logger) {
    match config.mode {
        BotMode::Console => {
            logger.log_info("start console bot");
            match console::run_bot(&config, logger) {
                Ok(_) => {}
                Err(e) => logger.log_error(format!("io error occurred: {}", e).as_str())
            }
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
