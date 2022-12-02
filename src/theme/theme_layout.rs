use anyhow::{bail, Context, Result};
use std::{
    borrow::Cow,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::traits::TryFromFile;

#[derive(Debug, PartialEq, Eq)]
pub enum LayoutKind {
    Index,
    Post,
}

impl LayoutKind {
    pub fn from_file_name(path: impl AsRef<Path>) -> Result<Self> {
        let file_name_without_extension = path
            .as_ref()
            .file_name()
            .map(|str| str.to_string_lossy())
            .map(|str| str.chars().take_while(|&c| c != '.').collect::<String>())
            .context("Unable to obtain file name for layout")?;

        Ok(match file_name_without_extension.as_str() {
            "index" => Self::Index,
            "post" => Self::Post,
            _ => bail!("Unknown file name. Must match one of ['index', 'post']."),
        })
    }
}

#[derive(Debug)]
pub struct ThemeLayout {
    pub path: PathBuf,
    pub source: String,
    pub kind: LayoutKind,
}

impl TryFromFile for ThemeLayout {
    fn try_from_file(path: Cow<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let source = read_to_string(&path)?;
        let kind = LayoutKind::from_file_name(&path)?;
        Ok(Self {
            path: path.into(),
            source,
            kind,
        })
    }
}
