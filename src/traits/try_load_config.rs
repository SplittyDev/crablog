use std::{fs::read_to_string, path::Path};

use serde::Deserialize;

use crate::config::{ConfigError, CONFIG_FILENAME};

pub trait TryLoadConfig {
    /// Try to read the config file in the current directory.
    fn try_load() -> Result<Self, ConfigError>
    where
        Self: Sized + for<'a> Deserialize<'a>,
    {
        let path = Path::new(CONFIG_FILENAME);
        log::debug!("Loading config from {:?}", path);
        let content = read_to_string(path)?;
        let config = toml::from_str::<Self>(&content)?;
        Ok(config)
    }
}

// Blanket implementation
impl<T> TryLoadConfig for T where T: Sized + for<'a> Deserialize<'a> {}
