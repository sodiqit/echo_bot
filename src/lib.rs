mod commands;
pub mod config;
mod console;
pub mod logger;
mod telegram;

use std::process;

use config::{BotMode, Config};
use logger::Logger;
use telegram::client_types::ClientError;

// I'd use
//
// fn run_bot<T: Logger>(config: Config, logger: &dyn Logger)
//
// here.
//
// The theoretical reason here is that we don't really want the whole thing to
// be _generic_ over a logger, logger is very much not a core detail, so we'd
// rather just pass a concrete logger object, and hide polymorphism in that
// object itself. The `dyn trait` achieves exactly that.
//
// The practical reason is that, if the function is generic, it generally slows
// down compilation. Generic function can be compiled to machine code only when
// you know the concrerte logger type which would be used. That is, a _library_
// generic function would be compiled only in the application. In contrast,
// non-generic functions can be compiled immediately, so a library can be
// _separaetly compiled_ from the application.
//
// This doesn't matter at all for such a small code base, but adds up in bigger
// ones.


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
                    // Feels better to bubble up an error with `?`, and let
                    // `main` to decide on the exit code and such. 
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
