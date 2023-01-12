use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs};
use validator::{Validate, ValidationError};

pub struct ConfigBuilder {
    file_path: Option<String>,
    file_content: Option<String>,
}

#[derive(Serialize, Validate, Deserialize, Debug)]
pub struct Config {
    mode: String,
}

impl ConfigBuilder {
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

    fn validate(&self, config: &Config) {
        let iterable_headers: HashMap<String, String> =
            serde_yaml::from_value(serde_yaml::to_value(config).unwrap()).unwrap();

        for header in &iterable_headers {
            println!("{:?}", header);
        }
    }
}
