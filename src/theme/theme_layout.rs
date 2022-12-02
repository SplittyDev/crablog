use anyhow::Result;
use std::{
    borrow::Cow,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::traits::TryFromFile;

#[derive(Debug)]
pub struct ThemeLayout {
    path: PathBuf,
    source: String,
}

impl TryFromFile for ThemeLayout {
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
