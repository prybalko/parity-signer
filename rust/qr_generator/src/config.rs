use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub chains: Vec<Chain>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chain {
    pub name: String,
    pub(crate) url: String
}


pub fn read_config() -> anyhow::Result<Config> {
    let config_toml = fs::read_to_string("config.toml")?;
    let config = toml::from_str::<Config>(config_toml.as_str())?;
    Ok(config)
}
