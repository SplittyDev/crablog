use std::{borrow::Cow, fs::create_dir_all, path::Path};

use anyhow::Result;

use crate::config::ThemeConfig;

use super::ThemeBundle;

#[derive(Debug)]
pub struct Theme {
    config: ThemeConfig,
    bundle: ThemeBundle,
}

impl Theme {
    pub fn from_config(config: ThemeConfig) -> Result<Self> {
        Ok(Self {
            config,
            bundle: ThemeBundle::load_local()?,
        })
    }

    pub fn scaffold_directory_structure(path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        create_dir_all(path)?;
        create_dir_all(path.join("layouts"))?;
        create_dir_all(path.join("styles"))?;
        create_dir_all(path.join("scripts"))?;

        Ok(())
    }

    pub fn scaffold(_name: Cow<str>) -> Result<()> {
        todo!()
    }
}
