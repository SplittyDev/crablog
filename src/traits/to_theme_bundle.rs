use anyhow::Result;

use crate::theme::ThemeBundle;

pub trait ToThemeBundle {
    fn to_theme_bundle(&self) -> Result<ThemeBundle>;
}
