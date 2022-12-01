use std::{borrow::Cow, fs::write, path::Path};

use serde::Serialize;

use crate::config::{CONFIG_FILENAME, ConfigError};

pub trait TrySaveConfig {
    /// Try to write the config file to the current directory.
    fn try_save(&self) -> Result<(), ConfigError>
    where
        Self: Serialize,
    {
        let path = Path::new(CONFIG_FILENAME).canonicalize()?;
        let toml = toml::to_string_pretty(self)?;
        write(path, toml)?;
        Ok(())
    }

    /// Try to write the config file to the specified directory.
    fn try_save_to(&self, path: Cow<Path>) -> Result<(), ConfigError>
    where
        Self: Serialize,
    {
        let path = path
            .ends_with(CONFIG_FILENAME)
            .then(|| path.to_path_buf())
            .unwrap_or_else(|| path.join(CONFIG_FILENAME));
        let toml = toml::to_string_pretty(self)?;
        write(path, toml)?;
        Ok(())
    }
}

// Blanket implementation
impl<T> TrySaveConfig for T where T: Serialize {}
