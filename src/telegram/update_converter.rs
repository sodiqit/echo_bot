use crate::commands::{Command, IsCommand};

use super::client_types::RawUpdate;

#[derive(Debug)]
pub enum TelegramUpdate {
    Message {
        update_id: u64,
        chat_id: u64,
        content: MessageContent,
    },
    CallbackQuery {
        update_id: u64,
        chat_id: u64,
        content: CallbackData,
    },
    Ignore {
        update_id: u64,
    },
}

#[derive(Debug)]
pub struct CallbackData {
    pub id: String,
    pub data: String,
}

#[derive(Debug)]
pub enum MessageContent {
    Command(Command, String),
    Video { file_id: String },
    Text(String),
}

impl From<RawUpdate> for TelegramUpdate {
    fn from(value: RawUpdate) -> Self {
        let is_bot_message = match value.message.as_ref() {
            Some(msg) => match msg.from.as_ref() {
                Some(user) => user.is_bot,
                None => false,
            },
            None => false,
        };

        if is_bot_message {
            return TelegramUpdate::Ignore {
                update_id: value.update_id,
            };
        }

        if let Some(msg) = &value.message {
            if let Some(text) = &msg.text {
                if text.is_command() {
                    let command = text.parse::<Command>().unwrap();

                    match command {
                        Command::Help => {
                            return TelegramUpdate::Message {
                                update_id: value.update_id,
                                chat_id: msg.chat.id,
                                content: MessageContent::Command(Command::Help, text.to_owned()),
                            }
                        }
                        Command::Repeat => {
                            return TelegramUpdate::Message {
                                update_id: value.update_id,
                                chat_id: msg.chat.id,
                                content: MessageContent::Command(Command::Repeat, text.to_owned()),
                            }
                        }
                        Command::Unknown => {
                            return TelegramUpdate::Message {
                                update_id: value.update_id,
                                chat_id: msg.chat.id,
                                content: MessageContent::Command(Command::Unknown, text.to_owned()),
                            }
                        }
                        _ => {}
                    }
                }

                return TelegramUpdate::Message {
                    update_id: value.update_id,
                    chat_id: msg.chat.id,
                    content: MessageContent::Text(text.clone()),
                };
            }

            if let Some(video) = msg.video.as_ref() {
                return TelegramUpdate::Message {
                    update_id: value.update_id,
                    chat_id: msg.chat.id,
                    content: MessageContent::Video {
                        file_id: video.file_id.clone(),
                    },
                };
            }
        }

        if let Some(query) = &value.callback_query {
            return TelegramUpdate::CallbackQuery {
                update_id: value.update_id,
                chat_id: query.message.chat.id,
                content: CallbackData {
                    id: query.id.clone(),
                    data: query.data.clone(),
                },
            };
        }

        TelegramUpdate::Ignore {
            update_id: value.update_id,
        }
    }
}
