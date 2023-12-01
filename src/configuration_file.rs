use std::fs;
use std::fs::File;
use std::path::Path;

use serde_derive::Deserialize;
use toml;

use crate::file_controller;

const FILE_PATH: &str = "./ngrust_config.toml";
const CONFIG_TEMPLATE: &str = "path = \"./ui/\"
welcome_message = \"Usage: ngrust --gc <COMPONENT_NAME> to generate a new component\"
";

#[derive(Deserialize)]
pub struct PathConfig {
    path: Option<String>,
    welcome_message: Option<String>,
}

pub fn read_config_file_content() -> PathConfig {
    let content = fs::read_to_string(FILE_PATH).unwrap();
    let config: PathConfig = toml::from_str(&content).unwrap();
    return config;
}

pub fn get_welcome_message() -> String {
    let config: PathConfig = read_config_file_content();
    match config.welcome_message {
        Some(message) => String::from(message),
        None => String::from("Welcome"),
    }
}

pub fn get_path() -> String {
    let config: PathConfig = read_config_file_content();
    match config.path {
        Some(path) => {
            let path = Path::new(&path);
            if !path.exists() {
                match fs::create_dir_all(path) {
                    Ok(_) => {
                        return path.display().to_string();
                    }
                    Err(error) => {
                        println!("Failed to create the path {}", error);
                    }
                }
            }
            return path.display().to_string();
        }
        None => String::from("./"),
    }
}

pub fn check_file_existence(file_path: &str) -> bool {
    let path = Path::new(file_path);
    if let Ok(metadata) = fs::metadata(path) {
        metadata.is_file()
    } else {
        false
    }
}

pub(crate) fn handle_configuration_file() {
    let configuration_file_exits = check_file_existence(FILE_PATH);
    if !configuration_file_exits {
        user_feedback("Initializing configuration...");
        create_configuration_file();
        user_feedback("Configuration file created with success")
    } else {
        user_feedback("Configuration file found")
    }
}

pub fn create_configuration_file() {
    let mut new_file = File::create(FILE_PATH)
        .expect("Failed to create the configuration file");
    file_controller::write_file(
        &mut new_file,
        CONFIG_TEMPLATE.to_string(),
    );
}

pub fn user_feedback(message: &str) {
    println!("{}", message);
}
