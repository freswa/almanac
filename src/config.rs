use std::fs::File;
use std::io::Read;
use dirs;
use toml;

use errors::ConfigError;

const CONFIG_NAME: &str = "almanac.toml";

#[derive(Deserialize)]
pub struct Config {
    pub cals: Vec<String>,
}

impl Config {
    pub fn parse() -> Result<Config, ConfigError> {
        let config_path = match dirs::config_dir() {
            Some(path) => path,
            None => return Err(ConfigError::MissingPath),
        }.join(CONFIG_NAME);

        let mut file = File::open(&config_path)?;
        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str)?;

        Ok(toml::from_str(&toml_str)?)
    }
}
