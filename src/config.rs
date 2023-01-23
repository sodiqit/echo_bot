use serde::{Deserialize, Serialize};
use std::{env, fs};
use crate::logger::LogLevel;

pub struct ConfigBuilder {
    file_path: Option<String>,
    file_content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BotMode {
    #[serde(rename="telegram")]
    Telegram,
    #[serde(rename="console")]
    Console,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub mode: BotMode,
    pub help_msg: String,
    pub repeat_msg: String,
    pub default_repeat_number: u8,
    pub log_level: LogLevel,
    pub bot_token: Option<String> //TODO: add validator for Telegram mode 
}

impl ConfigBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ConfigBuilder {
            file_path: Some("config.yaml".to_string()),
            file_content: None,
        }
    }

    pub fn extract_path(mut self) -> Self {
        let path = env::args().nth(1);

        if let Some(file_path) = path {
            self.file_path = Some(file_path);
        }

        self
    }

    pub fn extract_config_body(mut self) -> Self {
        let mut path = env::current_dir().expect("Cannot get current directory");
        path.push(self.file_path.as_ref().unwrap());

        let content = fs::read_to_string(path).unwrap_or_else(|_| {
            panic!(
                "Cannot read file with current path: {}",
                self.file_path.as_ref().unwrap()
            )
        });

        self.file_content = Some(content);

        self
    }

    pub fn build(self) -> Config {
        let config: Config = serde_yaml::from_str(self.file_content.as_ref().unwrap())
            .map_err(|e| format!("Parse config failed: {}", e))
            .unwrap();

        config
    }

    pub fn build_default(mode: BotMode) -> Config {
        Config {
            mode,
            help_msg: "help msg".to_string(),
            repeat_msg: "repeat msg".to_string(),
            default_repeat_number: 1,
            bot_token: Some("test".to_string()),
            log_level: LogLevel::Debug,
        }
    }
}
