use std::path::Path;

use anyhow::Result;
use walkdir::WalkDir;

use crate::traits::TryFromFile;

use super::{ThemeAsset, ThemeLayout, ThemeScript, ThemeStyle};

const EXT_LAYOUTS: [&str; 3] = ["html", "hbs", "handlebars"];
const EXT_SCRIPTS: [&str; 1] = ["js"];
const EXT_STYLES: [&str; 1] = ["css"];

#[derive(Debug)]
pub struct ThemeBundle {
    assets: Vec<ThemeAsset>,
    layouts: Vec<ThemeLayout>,
    scripts: Vec<ThemeScript>,
    styles: Vec<ThemeStyle>,
}

impl ThemeBundle {
    pub fn load_local() -> Result<Self> {
        let local_path = Path::new(".");
        let layouts = Self::load(local_path, "layouts", &EXT_LAYOUTS);
        let scripts = Self::load(local_path, "scripts", &EXT_SCRIPTS);
        let styles = Self::load(local_path, "styles", &EXT_STYLES);
        Ok(Self {
            layouts,
            scripts,
            styles,
            assets: Default::default(),
        })
    }

    fn load<T>(
        base_path: impl AsRef<Path>,
        dir: impl AsRef<str>,
        extensions: &[impl AsRef<str>],
    ) -> Vec<T>
    where
        T: TryFromFile,
    {
        let path = base_path.as_ref().join(dir.as_ref());
        fn path_matches_extension(path: impl AsRef<Path>, exts: &[impl AsRef<str>]) -> bool {
            exts.into_iter()
                .any(|ext| path.as_ref().ends_with(format!(".{}", ext.as_ref())))
        }
        WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| path_matches_extension(entry.path(), &extensions))
            .filter_map(|entry| T::try_from_file(entry.into_path()).ok())
            .collect()
    }
}
