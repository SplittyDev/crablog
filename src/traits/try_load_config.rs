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

    /// Try to read the config file in the specified directory.
    fn try_load_from(path: impl AsRef<Path>) -> Result<Self, ConfigError>
    where
        Self: Sized + for<'a> Deserialize<'a>,
    {
        let path = path.as_ref().join(CONFIG_FILENAME);
        let parent_path = path.parent().ok_or(ConfigError::ThemeParentNotFoundError)?;
        let content = read_to_string(&path)?;
        let mut config = toml::from_str::<Self>(&content)?;
        config.update_path(parent_path);
        Ok(config)
    }

    /// Patch the origin directory in the config
    fn update_path(&mut self, path: &Path);
}
