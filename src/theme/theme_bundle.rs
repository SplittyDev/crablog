use std::path::Path;

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::traits::TryFromFile;

use super::{theme_layout::LayoutKind, ThemeAsset, ThemeLayout, ThemeScript, ThemeStyle};

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
        Self::load_from_path(".")
    }

    pub fn get_layout(&self, kind: LayoutKind) -> Result<&ThemeLayout> {
        self.layouts
            .iter()
            .find(|layout| layout.kind == kind)
            .context("Unable to find index layout")
    }

    pub fn get_styles(&self) -> &Vec<ThemeStyle> {
        &self.styles
    }

    pub fn get_scripts(&self) -> &Vec<ThemeScript> {
        &self.scripts
    }

    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self> {
        log::debug!("Loading theme bundle from {:?}", path.as_ref());
        let layouts = Self::load(&path, "layouts", &EXT_LAYOUTS);
        let scripts = Self::load(&path, "scripts", &EXT_SCRIPTS);
        let styles = Self::load(&path, "styles", &EXT_STYLES);
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
        {
            let extensions = extensions
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>()
                .join(",");
            log::debug!("Loading {} files from {path:?}", extensions);
        }
        fn path_matches_extension(path: impl AsRef<Path>, exts: &[impl AsRef<str>]) -> bool {
            exts.into_iter().map(AsRef::as_ref).any(|ext| {
                path.as_ref()
                    .extension()
                    .map(std::ffi::OsStr::to_string_lossy)
                    .map(|fext| ext == fext)
                    .unwrap_or_default()
            })
        }
        WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| path_matches_extension(entry.path(), &extensions))
            .filter_map(|entry| T::try_from_file(entry.into_path().into()).ok())
            .collect()
    }
}
