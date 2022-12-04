mod blog;
mod config;
mod engine;
mod logging;
mod theme;
mod traits;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fmt::Display;
use strum::IntoEnumIterator;

use crate::{
    blog::Blog,
    config::{CommonProjectConfig, ConfigError},
    engine::{BuildEngine, BuildEnvironment},
    theme::Theme,
    traits::TryLoadConfig,
};

#[derive(Debug, Subcommand)]
enum Command {
    New,
    Dev,
    Build,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    // Setup logging
    logging::init()?;

    // Try to load config
    log::debug!("Detecting presence of Crablog.toml");
    match CommonProjectConfig::try_load() {
        Ok(config) => {
            log::debug!("Configuration file is present and valid");
            cli_handle_project(config)?
        }
        Err(error) => match error {
            ConfigError::TomlDeserializationError(error) => {
                log::error!("Found malformed config");
                println!("Your Crablog.toml seems to be broken.");
                println!("Reason: {:#}", error);
            }
            _ => {
                log::debug!("Configuration file not found");
                cli_handle_new_project()?
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
            Self::Quit => "Nevermind",
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
    if let Some(blog) = config.to_blog() {
        let mut engine = BuildEngine::new(BuildEnvironment::Production, blog);
        engine.build()?;
    }
    Ok(())
}
