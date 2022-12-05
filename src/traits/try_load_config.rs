use std::{fs::read_to_string, path::Path};

use serde::Deserialize;

use crate::config::{ConfigError, CONFIG_FILENAME};

pub trait TryLoadConfig {
    /// Try to read the config file in the current directory.
    fn try_load() -> Result<Self, ConfigError>
    where
        Self: Sized + for<'a> Deserialize<'a>,
    {
        Self::try_load_from(Path::new("."))
    }

    fn try_load_from(path: impl AsRef<Path>) -> Result<Self, ConfigError>
    where
        Self: Sized + for<'a> Deserialize<'a>,
    {
        let path = path.as_ref().join(CONFIG_FILENAME);
        let content = read_to_string(path)?;
        let config = toml::from_str::<Self>(&content)?;
        Ok(config)
    }
}

// Blanket implementation
impl<T> TryLoadConfig for T where T: Sized + for<'a> Deserialize<'a> {}
