use anyhow::{Context, Result};
use std::{
    borrow::Cow,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::traits::TryFromFile;

#[derive(Debug)]
pub struct ThemeStyle {
    path: PathBuf,
    source: String,
}

impl ThemeStyle {
    pub fn file_name(&self) -> Result<String> {
        self.path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .context("Unable to get filename")
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

impl TryFromFile for ThemeStyle {
    fn try_from_file(path: Cow<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let source = read_to_string(&path)?;
        Ok(Self {
            path: path.into(),
            source,
        })
    }
}
