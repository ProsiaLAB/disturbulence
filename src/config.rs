use serde::Deserialize;
use std::fs;

use crate::modes::VerticalShearInstabilityConfig;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ConfigFile {
    VerticalShearInstability {
        vertical_shear_instability: VerticalShearInstabilityConfig,
    },
    // Add other modes here...
}

pub fn load_config(path: &str) -> Result<ConfigFile, Box<dyn std::error::Error>> {
    let toml_str = fs::read_to_string(path)?;
    let config: ConfigFile = toml::from_str(&toml_str)?;
    Ok(config)
}
