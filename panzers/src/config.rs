use toml;
use std::fs;

pub struct Config {
    pub predefined: PredefinedConfig,
}

pub struct PredefinedConfig {
    pub version: String,
}

impl Config {
    pub fn load_config(file_path: &str) -> Config {
        let content = fs::read_to_string(file_path).expect("Failed to read config file ❌");
        let table: toml::value::Table = toml::from_str(&content).expect("Failed to parse config ❌");

        let predefined = table.get("predefined").expect("No 'predefined' section in config file");

        let version = predefined.get("version").expect("No 'version' key in 'predefined' section")
            .as_str().expect("'version' value must be a string").to_string();

        Config {
            predefined: PredefinedConfig { version }
        }
    }
}
