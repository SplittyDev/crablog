use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct BuildFile {
    virtual_path: PathBuf,
    content: String,
}

impl BuildFile {
    pub fn new(virtual_path: Cow<Path>, content: Cow<str>) -> Self {
        Self {
            virtual_path: virtual_path.into(),
            content: content.into(),
        }
    }
}
