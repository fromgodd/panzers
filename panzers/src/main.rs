mod shell;
mod config;
mod commands;
mod utils;

fn main() {
    let config = config::load_config("panzers.toml");
    let prompt = config.get("shell.prompt").unwrap_or(&toml::Value::String("$ ".to_string())).as_str().unwrap().to_string();
    shell::run_shell(prompt);
}