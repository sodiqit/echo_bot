use crate::commands::{Commands, ToCommands};

use super::client_types::RawUpdate;

#[derive(Debug)]
pub enum TelegramUpdate {
    Message {
        update_id: u64,
        chat_id: u64,
        content: MessageContent,
    },
    Ignore {
        update_id: u64,
    },
}

#[derive(Debug)]
pub enum MessageContent {
    Command(Commands, String),
    Video { file_id: String },
    Text(String),
}

pub trait ToTelegramUpdate {
    fn to_tg_update(self) -> TelegramUpdate;
}

impl ToTelegramUpdate for RawUpdate {
    fn to_tg_update(self) -> TelegramUpdate {
        let is_bot_message = match self.message.as_ref() {
            Some(msg) => match msg.from.as_ref() {
                Some(user) => user.is_bot,
                None => false,
            },
            None => false,
        };

        if is_bot_message {
            return TelegramUpdate::Ignore {
                update_id: self.update_id,
            };
        }

        if let Some(msg) = &self.message {
            if let Some(text) = &msg.text {
                if text.is_command() {
                    let command = text.to_commands();

                    match command {
                        Commands::Help => {
                            return TelegramUpdate::Message {
                                update_id: self.update_id,
                                chat_id: msg.chat.id,
                                content: MessageContent::Command(Commands::Help, text.to_owned()),
                            }
                        }
                        Commands::Repeat => {
                            return TelegramUpdate::Message {
                                update_id: self.update_id,
                                chat_id: msg.chat.id,
                                content: MessageContent::Command(Commands::Repeat, text.to_owned()),
                            }
                        }
                        Commands::Unknown => {
                            return TelegramUpdate::Message {
                                update_id: self.update_id,
                                chat_id: msg.chat.id,
                                content: MessageContent::Command(
                                    Commands::Unknown,
                                    text.to_owned(),
                                ),
                            }
                        }
                        _ => {}
                    }
                }

                return TelegramUpdate::Message {
                    update_id: self.update_id,
                    chat_id: msg.chat.id,
                    content: MessageContent::Text(text.clone()),
                };
            }

            if let Some(video) = msg.video.as_ref() {
                return TelegramUpdate::Message {
                    update_id: self.update_id,
                    chat_id: msg.chat.id,
                    content: MessageContent::Video {
                        file_id: video.file_id.clone(),
                    },
                };
            }
        }

        TelegramUpdate::Ignore {
            update_id: self.update_id,
        }
    }
}
