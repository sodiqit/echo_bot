use serde::de::DeserializeOwned;
use ureq::Response;

use crate::logger::Logger;

use super::client_types::{ClientError, RawUpdate, TelegramResponse};

pub trait TelegramClient {
    type T;
    type E;
    fn get_updates(&self, offset: u64) -> Result<Self::T, Self::E>;
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
            .to_result()
    }
}

impl<'a, 'b, L: Logger> TelegramClient for TelegramHttpClient<'a, 'b, L> {
    type T = Vec<RawUpdate>;
    type E = ClientError;

    fn get_updates(&self, offset: u64) -> Result<Self::T, Self::E> {
        let response = ureq::get(self.url("getUpdate").as_str())
            .query("offset", offset.to_string().as_str())
            .call();

        self.logger
            .log_info(format!("get updates with current offset {}", offset).as_str());

        self.parse(response)
    }
}
