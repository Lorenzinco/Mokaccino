use toml;
use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config{
    pub username: String,
    pub server_addr:String,
    pub local_port: u16,
}

impl Config{
    pub fn new() -> Config{
        //read and parse username and port from config.toml
        let config_string = fs::read_to_string("config.toml").expect("Unable to read config.toml");
        let config: Config = toml::from_str(&config_string).expect("Unable to parse config.toml");
        config
    }
}