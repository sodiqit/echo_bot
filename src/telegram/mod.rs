mod client;
pub mod client_types;
mod handler;
mod keyboard;
mod response_parser;
mod update_converter;

use std::{thread::sleep, time::Duration};

use self::{
    client::{TelegramClient, TelegramHttpClient},
    client_types::{ClientError, TelegramCommand},
    handler::{Handler, TelegramHandler, TelegramState},
};
use crate::{commands::Command, config::Config, logger::Logger};

pub fn run_bot(config: &Config, logger: &impl Logger) -> Result<(), ClientError> {
    let token = config.bot_token.as_ref().unwrap();
    let client = TelegramHttpClient::new(token, logger);
    let mut state = TelegramState::new();
    let handler = TelegramHandler::new(logger, &client);

    client.set_commands(vec![
        TelegramCommand::new(
            Command::new("/help"),
            "provide help message about bot".to_string(),
        ),
        TelegramCommand::new(
            Command::new("/repeat"),
            "provide menu for choose repeat number".to_string(),
        ),
    ])?;

    loop {
        communicate(&mut state, &client, &handler, config)?;
        sleep(Duration::from_secs(1));
    }
}

fn communicate<T: TelegramClient, H: Handler<T>>(
    state: &mut TelegramState,
    client: &T,
    handler: &H,
    config: &Config,
) -> Result<(), T::E> {
    let offset = state.last_update_id.map_or(0, |v| v + 1);
    client
        .get_updates(offset)?
        .into_iter()
        .try_for_each(|update| handler.handle(config, state, update))
}
