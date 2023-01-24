mod client;
pub mod client_types;
mod handler;
mod response_parser;
mod update_converter;
mod keyboard;

use std::{thread::sleep, time::Duration};

use self::{
    client::{TelegramClient, TelegramHttpClient},
    client_types::ClientError,
    handler::{Handler, TelegramHandler, TelegramState},
};
use crate::{config::Config, logger::Logger};

pub fn run_bot(config: &Config, logger: &impl Logger) -> Result<(), ClientError> {
    let token = config.bot_token.as_ref().unwrap();
    let client = TelegramHttpClient::new(token, logger);
    let mut state = TelegramState::new();
    let handler = TelegramHandler::new(logger, &client);

    loop {
        let offset = state.last_update_id.map_or(0, |v| v + 1);
        client
            .get_updates(offset)?
            .into_iter()
            .try_for_each(|update| handler.handle(config, &mut state, update))?;
        sleep(Duration::from_secs(1));
    }
}
