use std::{borrow::Cow, fs::create_dir_all, path::Path};

use anyhow::Result;
use itertools::Itertools;
use walkdir::WalkDir;

use crate::{
    blog::{
        config::{BlogConfig, BlogMetadataConfig, BlogThemeConfig},
        Post,
    },
    config::CommonProjectConfig,
    engine::BuildEnvironment,
    theme::{config::ThemeConfig, Theme, ThemeBundle, ThemeSource},
    traits::{ToTheme, TryFromFile, TrySaveConfig},
};

#[derive(Debug)]
pub struct Blog {
    config: BlogConfig,
    theme: Theme,
    posts: Vec<Post>,
}

impl Blog {
    pub fn from_config(config: BlogConfig) -> Result<Self> {
        log::debug!("Loading theme");
        let theme = config.theme_config.source().to_theme()?;
        let post_path = Path::new("./posts");
        log::debug!("Loading posts from {:?}", post_path);
        let posts = Self::load_posts("posts");
        Ok(Self {
            config,
            theme,
            posts,
        })
    }

    fn load_posts(path: impl AsRef<Path>) -> Vec<Post> {
        WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter_map(|entry| Post::try_from_file(entry.into_path().into()).ok())
            .sorted_by(|a, b| b.metadata().created_at.cmp(&a.metadata().created_at))
            .collect()
    }

    /// Get union of requested and available theme features
    pub fn resolve_features(&self) -> Vec<String> {
        let features_available = self.theme.features();
        let features_requested = self.config.theme_config.features();
        let mut vec = Vec::with_capacity(features_available.len());
        for feature in features_available {
            if features_requested.contains(&feature) {
                vec.push(feature)
            }
        }
        vec
    }

    pub fn iter_posts(&self, env: BuildEnvironment) -> Box<dyn Iterator<Item = &Post> + '_> {
        match env {
            BuildEnvironment::Development => Box::new(self.posts.iter()),
            BuildEnvironment::Production => {
                Box::new(self.posts.iter().filter(|post| post.metadata().published))
            }
        }
    }

    pub fn config(&self) -> &BlogConfig {
        &self.config
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn theme_bundle(&self) -> &ThemeBundle {
        self.theme.bundle()
    }
}

// Scaffolding
impl Blog {
    fn scaffold_directory_structure(path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        create_dir_all(path)?;
        create_dir_all(path.join("posts"))?;

        Ok(())
    }

    fn scaffold_config(name: impl ToString, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        // Create default config
        let config = CommonProjectConfig::from(BlogConfig {
            name: name.to_string(),
            meta: BlogMetadataConfig {
                title: name.to_string(),
                ..Default::default()
            },
            ..Default::default()
        });

        // Serialize config
        config.try_save_to(path.into())?;

        Ok(())
    }

    fn scaffold_config_with_theme(name: impl ToString, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        // Create default blog config
        let blog_config = BlogConfig {
            name: name.to_string(),
            meta: BlogMetadataConfig {
                title: name.to_string(),
                ..Default::default()
            },
            theme_config: BlogThemeConfig {
                source: ThemeSource::default_local(),
                ..Default::default()
            },
            ..Default::default()
        };

        // Create default theme config
        let theme_config = ThemeConfig {
            name: name.to_string(),
            ..Default::default()
        };

        // Convert into common config
        let config: CommonProjectConfig = (blog_config, theme_config).into();

        // Serialize config
        config.try_save_to(path.into())?;

        Ok(())
    }

    pub fn scaffold(name: Cow<str>) -> Result<()> {
        // Normalize path
        let normalized_name = name.to_lowercase().replace([' ', '\t', '\r', '\n'], "_");
        let normalized_path = Path::new(&normalized_name);

        Self::scaffold_directory_structure(normalized_path)?;
        Self::scaffold_config(name, normalized_path)?;

        println!("Your new blog has been initialized at ./{normalized_name}!");

        Ok(())
    }

    pub fn scaffold_with_theme(name: Cow<str>) -> Result<()> {
        // Normalize path
        let normalized_name = name.to_lowercase().replace([' ', '\t', '\r', '\n'], "_");
        let normalized_path = Path::new(&normalized_name);

        Blog::scaffold_directory_structure(normalized_path)?;
        Theme::scaffold_directory_structure(normalized_path)?;
        Blog::scaffold_config_with_theme(name, normalized_path)?;

        println!("Your new blog has been initialized at ./{normalized_name}!");

        Ok(())
    }
}
