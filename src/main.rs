use std::fmt::Display;

use anyhow::{Context, Result};
use blog::Blog;
use clap::Parser;
use strum::IntoEnumIterator;

mod blog;
mod config;
mod theme;
mod traits;

use config::{CommonProjectConfig, ConfigError};
use traits::TryLoadConfig;

use crate::theme::Theme;

#[derive(Debug, Parser)]
struct Args {}

fn main() -> Result<()> {
    // Try to load config
    match CommonProjectConfig::try_load() {
        Ok(config) => cli_handle_project(config)?,
        Err(error) => {
            match error {
                ConfigError::TomlDeserializationError(_) => {
                    println!("Your Crablog.toml seems to be broken.");
                    println!("{:#?}", error);
                },
                _ => cli_handle_new_project()?,
            }
        },
    }

    Ok(())
}

#[derive(Debug, Clone, strum::EnumIter)]
enum NewProjectSelection {
    Blog,
    BlogAndTheme,
    Theme,
    Quit,
}

impl Display for NewProjectSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_name = match self {
            Self::Blog => "New Blog",
            Self::Theme => "New Theme",
            Self::BlogAndTheme => "New Blog with custom Theme",
            Self::Quit => "Quit",
        };
        write!(f, "{display_name}")
    }
}

fn cli_handle_new_project() -> Result<()> {
    println!("We couldn't find an existing project. Let's create one!");

    // Ask user for project kind
    let project_kind = {
        let project_kinds = NewProjectSelection::iter().collect::<Vec<_>>();
        let item = dialoguer::Select::new()
            .items(project_kinds.as_slice())
            .default(0)
            .interact()?;

        project_kinds
            .get(item)
            .context("Unable to read project kind")?
            .clone()
    };

    // Scaffold project
    match project_kind {
        NewProjectSelection::Blog => {
            let blog_name = dialoguer::Input::<String>::new()
                .with_prompt("Blog Name")
                .interact_text()?;
            Blog::scaffold(blog_name.into())?;
        }
        NewProjectSelection::Theme => {
            let theme_name = dialoguer::Input::<String>::new()
                .with_prompt("Theme Name")
                .interact_text()?;
            Theme::scaffold(theme_name.into())?;
        }
        NewProjectSelection::BlogAndTheme => {
            let blog_name = dialoguer::Input::<String>::new()
                .with_prompt("Blog Name")
                .interact_text()?;
            Blog::scaffold_with_theme(blog_name.into())?;
        }
        NewProjectSelection::Quit => (),
    }
    Ok(())
}

fn cli_handle_project(config: CommonProjectConfig) -> Result<()> {
    if let Some(theme) = config.to_theme() {}
    if let Some(blog) = config.to_blog() {}
    println!("{:#?}", config);
    Ok(())
}
