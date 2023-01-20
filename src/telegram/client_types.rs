use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum TelegramResponse<T> {
    Success { result: T },
    Failure(TelegramApiError),
}

impl<T> TelegramResponse<T> {
    pub fn to_result(self) -> Result<T, ClientError> {
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

#[derive(Deserialize, Debug)]
pub struct RawUpdate {
    update_id: u64,
}
