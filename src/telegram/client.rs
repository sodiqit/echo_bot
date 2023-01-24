use serde::de::DeserializeOwned;
use serde_json::json;
use ureq::Response;

use crate::logger::Logger;

use super::client_types::{ClientError, Message, Payload, RawUpdate, TelegramResponse};

pub trait TelegramClient {
    type T;
    type E;
    fn get_updates(&self, offset: u64) -> Result<Self::T, Self::E>;
    fn send(&self, chat_id: u64, payload: Payload) -> Result<Message, Self::E>;
    fn answer_callback_query(&self, id: &str, text: &str) -> Result<bool, Self::E>;
}

pub struct TelegramHttpClient<'a, 'b, L: Logger> {
    token: &'b str,
    logger: &'a L,
    base_url: &'b str,
}

impl<'a, 'b, L: Logger> TelegramHttpClient<'a, 'b, L> {
    pub fn new(token: &'b str, logger: &'a L) -> Self {
        Self {
            token,
            logger,
            base_url: "https://api.telegram.org",
        }
    }

    fn url(&self, method: &str) -> String {
        format!("{}/bot{}/{}", self.base_url, self.token, method)
    }

    fn parse<T: DeserializeOwned>(
        &self,
        response: Result<Response, ureq::Error>,
    ) -> Result<T, ClientError> {
        response
            .map_err(|e| ClientError::Http(Box::new(e)))?
            .into_json::<TelegramResponse<T>>()
            .map_err(ClientError::Serialize)?
            .into_result()
    }
}

impl<'a, 'b, L: Logger> TelegramClient for TelegramHttpClient<'a, 'b, L> {
    type T = Vec<RawUpdate>;
    type E = ClientError;

    fn get_updates(&self, offset: u64) -> Result<Self::T, Self::E> {
        let response = ureq::get(self.url("getUpdates").as_str())
            .query("offset", offset.to_string().as_str())
            .call();

        self.logger
            .log_info(format!("get updates with current offset: {}", offset).as_str());

        let response: Result<Self::T, Self::E> = self.parse(response);

        self.logger
            .log_debug(format!("get response from getUpdates: {:#?}", response).as_str());

        response
    }

    fn send(&self, chat_id: u64, payload: Payload) -> Result<Message, Self::E> {
        let body;
        let method;

        match payload {
            Payload::Text(text) => {
                body = ureq::json!({"chat_id": chat_id, "text": text});
                method = "sendMessage";
            }
            Payload::Video(file_id) => {
                body = ureq::json!({"chat_id": chat_id, "video": file_id});
                method = "sendVideo";
            }
            Payload::TextWithKeyboard(keyboard, text) => {
                body = json!({"chat_id": chat_id, "text": text, "reply_markup": {
                    "inline_keyboard": keyboard.into_json()
                }});
                method = "sendMessage";
            }
        }

        let response = ureq::post(self.url(method).as_str()).send_json(body);

        let response: Result<Message, ClientError> = self.parse(response);

        self.logger
            .log_debug(format!("get response from {}: {:#?}", method, response).as_str());

        response
    }

    fn answer_callback_query(&self, id: &str, text: &str) -> Result<bool, ClientError> {
        let response = ureq::post(self.url("answerCallbackQuery").as_str())
            .send_json(json!({"callback_query_id": id, "text": text}));

        self.logger
            .log_debug(format!("get response from answerCallbackQuery: {:#?}", response).as_str());

        self.parse(response)
    }
}
