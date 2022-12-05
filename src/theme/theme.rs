use std::{
    borrow::Cow,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use anyhow::Result;

use super::{config::ThemeConfig, ThemeBundle};

#[derive(Debug)]
pub struct Theme {
    config: ThemeConfig,
    bundle: ThemeBundle,
}

impl Theme {
    pub fn from_config(config: ThemeConfig, path: PathBuf) -> Result<Self> {
        let bundle = ThemeBundle::load_from_path(&path)?;
        Ok(Self { config, bundle })
    }

    pub fn bundle(&self) -> &ThemeBundle {
        &self.bundle
    }

    pub fn features(&self) -> Vec<String> {
        self.config.features.clone()
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
