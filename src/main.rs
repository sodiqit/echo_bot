use std::process;

use echo_bot::config::ConfigBuilder;
use echo_bot::logger::{ConsoleLogger, Logger};
use echo_bot::{BotError, ClientError};

fn main() {
    let config = ConfigBuilder::new()
        .extract_path()
        .extract_config_body()
        .build();

    let logger = ConsoleLogger::new(config.log_level.clone());
    logger.log_info("logger with config success build");

    match echo_bot::run_bot(config, &logger) {
        Err(BotError::Console(err)) => {
            logger.log_error(format!("io error occurred: {}", err).as_str())
        }
        Err(BotError::Telegram(error)) => {
            match error {
                ClientError::Api(e) => {
                    logger.log_error(format!("get error from api telegram: {:?}", e).as_str())
                }
                ClientError::Http(e) => {
                    logger.log_error(format!("get error from http client: {}", e).as_str())
                }
                ClientError::Serialize(e) => {
                    logger.log_error(format!("serialization error occurred : {}", e).as_str())
                }
            }
            process::exit(1);
        }
        Ok(_) => {}
    }
}
