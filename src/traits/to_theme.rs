use anyhow::Result;

use crate::theme::Theme;

pub trait ToTheme {
    fn to_theme(&self) -> Result<Theme>;
}
