use serde::Serialize;
use serde_json::json;

#[derive(Serialize, Debug)]
pub struct InlineKeyboardMarkup {
    rows: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    pub fn new() -> Self {
        InlineKeyboardMarkup { rows: vec![] }
    }

    pub fn add(&mut self, row: Vec<InlineKeyboardButton>) {
        self.rows.push(row);
    }

    pub fn into_json(self) -> Vec<Vec<serde_json::Value>> {
        self.rows
            .into_iter()
            .map(|val| val.into_iter().map(|v| v.into_json()).collect())
            .collect()
    }
}

#[derive(Serialize, Debug)]
pub struct InlineKeyboardButton {
    text: String,
    callback_data: String,
}

impl InlineKeyboardButton {
    pub fn new(text: String, callback_data: String) -> Self {
        InlineKeyboardButton {
            text,
            callback_data,
        }
    }

    pub fn into_json(self) -> serde_json::Value {
        json!({
            "text": self.text,
            "callback_data": self.callback_data
        })
    }
}
