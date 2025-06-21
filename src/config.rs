use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub programs: HashMap<String, Program>,
    pub listen: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Program {
    pub command: String,
    pub args: Option<Vec<String>>,
    pub autorestart: Option<bool>,
    pub logfile: Option<String>,
}

impl Config {
    pub fn load(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read config");
        toml::from_str(&content).expect("Invalid config format")
    }
}
