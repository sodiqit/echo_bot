use serde::Deserialize;

use crate::commands::Command;

use super::keyboard::InlineKeyboardMarkup;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum TelegramResponse<T> {
    Success { result: T },
    Failure(TelegramApiError),
}

impl<T> TelegramResponse<T> {
    pub fn into_result(self) -> Result<T, ClientError> {
        match self {
            Self::Success { result } => Ok(result),
            Self::Failure(error) => Err(ClientError::Api(error)),
        }
    }
}

#[derive(Debug)]
pub enum ClientError {
    Http(Box<ureq::Error>),
    Serialize(std::io::Error),
    Api(TelegramApiError),
}

#[derive(Deserialize, Debug)]
pub struct TelegramApiError {
    error_code: u8,
    description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawUpdate {
    pub update_id: u64,
    pub message: Option<Message>,
    pub callback_query: Option<CallbackQuery>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub chat: Chat,
    pub from: Option<User>,
    pub video: Option<Video>,
    pub text: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CallbackQuery {
    pub id: String,
    pub message: Message,
    pub data: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub is_bot: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Chat {
    pub id: u64,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Video {
    pub file_id: String,
}

pub enum Payload<'a> {
    Text(&'a str),
    Video(&'a str),
    TextWithKeyboard(InlineKeyboardMarkup, &'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelegramCommand {
    pub command: Command,
    pub description: String,
}

impl TelegramCommand {
    pub fn new(command: Command, description: String) -> Self {
        Self {
            command,
            description,
        }
    }
}