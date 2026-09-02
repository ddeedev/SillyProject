use serde::de::DeserializeOwned;
use std::path::Path;
use crate::space::SpaceContext;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse TOML configuration: {0}")]
    Parse(#[from] toml::de::Error),
}

pub trait Config: Sized + DeserializeOwned {
    fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    fn init() -> Result<Self, ConfigError>;
}

impl Config for SpaceContext {
    fn init() -> Result<Self, ConfigError> {
        Self::from_file("../../../assets/mock/space/first.toml")
    }
}
