use serde::Deserialize;
use std::{error::Error, fs, path::PathBuf};

static CONFIG_FILE_NAME: &str = "dotcc-config.json";

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub url: String,
    pub files: Vec<String>,
}

impl Configuration {
    pub fn new(content: &str) -> Result<Self, Box<dyn Error>> {
        let deserialized = serde_json::from_str::<Self>(content)?;
        Ok(deserialized)
    }
}

pub fn get_config() -> Result<Configuration, Box<dyn Error>> {
    let content = fs::read_to_string(CONFIG_FILE_NAME)?;
    let config = Configuration::new(&content)?;
    Ok(config)
}
