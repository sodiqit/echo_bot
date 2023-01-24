use std::collections::HashMap;

use crate::{commands::Command, config::Config, logger::Logger};

use super::{
    client::TelegramClient,
    client_types::{Payload, RawUpdate},
    keyboard::{InlineKeyboardButton, InlineKeyboardMarkup},
    update_converter::{CallbackData, MessageContent, TelegramUpdate, ToTelegramUpdate},
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

    pub fn set_repeat_number(&mut self, chat_id: u64, number: u8) {
        self.repeat_numbers.insert(chat_id, number);
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
        state: &mut TelegramState,
        config: &Config,
        command: Command,
        initial_msg: String,
        chat_id: u64,
    ) -> Result<(), T::E> {
        match command {
            Command::Help => {
                self.client.send(chat_id, Payload::Text(&config.help_msg))?;
            }
            Command::Repeat => {
                let repeat_number = state
                    .repeat_numbers
                    .get(&chat_id)
                    .map_or(config.default_repeat_number, |x| x.to_owned());
                let msg = format!(
                    "{}\nCurrent repeat number is {}",
                    config.repeat_msg, repeat_number
                );
                self.client.send(
                    chat_id,
                    Payload::TextWithKeyboard(self.construct_inline_keyboard(), &msg),
                )?;
            }
            Command::Unknown => {
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

    fn handle_callback_query(
        &self,
        state: &mut TelegramState,
        chat_id: u64,
        content: CallbackData,
    ) -> Result<(), T::E> {
        let repeat_number = content.data.parse::<u8>().unwrap(); // this error never not be throw because buttons construct on bot side

        state.set_repeat_number(chat_id, repeat_number);

        let answer = format!("Repeats number was changed to {}", repeat_number);

        self.logger.log_info(
            format!(
                "Set repeat number: {} for this chat: {}",
                repeat_number, chat_id
            )
            .as_str(),
        );

        self.client.answer_callback_query(&content.id, &answer)?;

        Ok(())
    }

    fn construct_inline_keyboard(&self) -> InlineKeyboardMarkup {
        let mut markup = InlineKeyboardMarkup::new();
        let mut buttons = vec![];

        (1..6).for_each(|i| {
            let button = InlineKeyboardButton::new(i.to_string(), i.to_string());

            buttons.push(button);
        });

        markup.add(buttons);

        markup
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

        self.logger
            .log_debug(format!("Receive update: {:#?}", &update).as_str());

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
                        self.handle_command_message(state, config, command, initial_msg, chat_id)?;
                    }
                    MessageContent::Video { file_id } => {
                        self.handle_video_message(state, config, file_id, chat_id)?;
                    }
                }

                state.last_update_id = Some(update_id);
            }
            TelegramUpdate::CallbackQuery {
                update_id,
                chat_id,
                content,
            } => {
                self.handle_callback_query(state, chat_id, content)?;
                state.last_update_id = Some(update_id);
            }
            TelegramUpdate::Ignore { update_id } => {
                state.last_update_id = Some(update_id);
            }
        }

        Ok(())
    }
}
