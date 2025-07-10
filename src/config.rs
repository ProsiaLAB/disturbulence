use std::fs;

use anyhow::Result;

use crate::modes::Config;

pub fn load_config(path: &str) -> Result<Config> {
    let toml_str = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&toml_str)?;
    Ok(config)
}
