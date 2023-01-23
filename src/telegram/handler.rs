use std::collections::HashMap;

use crate::{commands::Commands, config::Config, logger::Logger};

use super::{
    client::TelegramClient,
    client_types::{Payload, RawUpdate},
    update_converter::{MessageContent, TelegramUpdate, ToTelegramUpdate},
};

pub trait Handler<T: TelegramClient> {
    fn handle(
        &self,
        config: &Config,
        state: &mut TelegramState,
        raw_update: RawUpdate,
    ) -> Result<(), T::E>;
}

pub struct TelegramState {
    pub last_update_id: Option<u64>,
    pub repeat_numbers: HashMap<u64, u8>,
}

impl TelegramState {
    pub fn new() -> Self {
        TelegramState {
            last_update_id: None,
            repeat_numbers: HashMap::new(),
        }
    }
}

pub struct TelegramHandler<'a, 'b, L: Logger, T: TelegramClient> {
    logger: &'a L,
    client: &'b T,
}

impl<'a, 'b, L: Logger, T: TelegramClient> TelegramHandler<'a, 'b, L, T> {
    pub fn new(logger: &'a L, client: &'b T) -> Self {
        Self { logger, client }
    }

    fn handle_text_message(
        &self,
        state: &mut TelegramState,
        config: &Config,
        message: String,
        chat_id: u64,
    ) -> Result<(), T::E> {
        let repeat_number = state
            .repeat_numbers
            .get(&chat_id)
            .map_or(config.default_repeat_number, |v| v.to_owned());

        (0..repeat_number).try_for_each(|_| {
            self.client
                .send(chat_id, Payload::Text(&message))
                .map(|_| ())
        })?;

        Ok(())
    }

    fn handle_command_message(
        &self,
        config: &Config,
        command: Commands,
        initial_msg: String,
        chat_id: u64,
    ) -> Result<(), T::E> {
        match command {
            Commands::Help => {
                self.client.send(chat_id, Payload::Text(&config.help_msg))?;
            }
            Commands::Repeat => {
                self.client
                    .send(chat_id, Payload::Text(&config.repeat_msg))?;
            }
            Commands::Unknown => {
                let msg = format!("get unknown command: {}", initial_msg);
                self.logger.log_warn(&msg);
                self.client.send(chat_id, Payload::Text(&msg))?;
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_video_message(
        &self,
        state: &mut TelegramState,
        config: &Config,
        file_id: String,
        chat_id: u64,
    ) -> Result<(), T::E> {
        let repeat_number = state
            .repeat_numbers
            .get(&chat_id)
            .map_or(config.default_repeat_number, |v| v.to_owned());

        (0..repeat_number).try_for_each(|_| {
            self.client
                .send(chat_id, Payload::Video(&file_id))
                .map(|_| ())
        })?;

        Ok(())
    }
}

impl<'a, 'b, L: Logger, T: TelegramClient> Handler<T> for TelegramHandler<'a, 'b, L, T> {
    fn handle(
        &self,
        config: &Config,
        state: &mut TelegramState,
        raw_update: RawUpdate,
    ) -> Result<(), T::E> {
        let update = raw_update.to_tg_update();

        match update {
            TelegramUpdate::Message {
                update_id,
                chat_id,
                content,
            } => {
                self.logger
                    .log_info(format!("Handle update: {}", update_id).as_str());

                match content {
                    MessageContent::Text(msg) => {
                        self.handle_text_message(state, config, msg, chat_id)?;
                    }
                    MessageContent::Command(command, initial_msg) => {
                        self.handle_command_message(config, command, initial_msg, chat_id)?;
                    }
                    MessageContent::Video { file_id } => {
                        self.handle_video_message(state, config, file_id, chat_id)?;
                    }
                }

                state.last_update_id = Some(update_id);
            }
            TelegramUpdate::Ignore { update_id } => {
                state.last_update_id = Some(update_id);
            }
        }

        Ok(())
    }
}
