use toml;
use std::fs;

pub fn load_config(file_path: &str) -> toml::value::Table {
    let content = fs::read_to_string(file_path).expect("There was an error while reading config file!");
    toml::from_str(&content).expect("Failed to parse string")
}