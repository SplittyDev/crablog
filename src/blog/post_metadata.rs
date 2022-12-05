use std::collections::HashMap;

use chrono::{Local, NaiveDateTime};
use once_cell::sync::Lazy;
use regex::Regex;

static RE_COMMENT: Lazy<Regex> = Lazy::new(|| {
    let re = r"(?im)^\[[/]{2}\]: # \((?P<key>.*?):\s+(?P<value>.*?)\)$";
    Regex::new(re).unwrap()
});

const DT_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Debug)]
pub struct PostMetadata {
    pub title: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
}

impl PostMetadata {
    pub fn safe_name(&self) -> String {
        self.title
            .trim()
            .to_lowercase()
            .replace(' ', "_")
            .replace('/', "")
    }
}

impl Default for PostMetadata {
    fn default() -> Self {
        Self {
            title: String::default(),
            published: false,
            created_at: Local::now().naive_local(),
        }
    }
}

impl PostMetadata {
    pub fn to_markdown(&self) -> String {
        fn to_markdown_comment(key: impl ToString, value: impl ToString) -> String {
            format!(
                "[//]: # ({key}: {value})\n",
                key = key.to_string(),
                value = value.to_string()
            )
        }
        let mut str = String::new();
        str.push_str(&to_markdown_comment("title", &self.title));
        str.push_str(&to_markdown_comment(
            "created_at",
            &self.created_at.format(DT_FORMAT),
        ));
        str.push_str(&to_markdown_comment("published", self.published));
        str
    }

    pub fn from_markdown(source: impl AsRef<str>) -> Self {
        let source = source.as_ref();

        // Extract metadata from source
        let map = source
            .lines()
            .filter_map(|line| {
                RE_COMMENT.captures(line).and_then(|captures| {
                    let key = captures.name("key");
                    let value = captures.name("value");
                    key.and_then(|key| value.map(|value| (key, value)))
                })
            })
            .fold(HashMap::new(), |mut accum, (key, value)| {
                accum.entry(key.as_str()).or_insert(value.as_str());
                accum
            });

        // Handle known metadata
        let title = map
            .get("title")
            .map(ToString::to_string)
            .unwrap_or_default();
        let published = map
            .get("published")
            .and_then(|value| value.parse().ok())
            .unwrap_or_default();
        let created_at = map
            .get("created_at")
            .and_then(|value| NaiveDateTime::parse_from_str(value, DT_FORMAT).ok())
            .unwrap_or_else(|| Local::now().naive_local());

        Self {
            title,
            published,
            created_at,
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDateTime;

    use super::PostMetadata;

    #[test]
    fn post_metadata_from_markdown() {
        let data = PostMetadata::from_markdown(
            r"
[//]: # (title: Hello world)
[//]: # (created_at: 1970-01-01 00:00:00)
[//]: # (published: true)
        ",
        );
        assert_eq!(data.title, "Hello world");
        assert_eq!(data.created_at, NaiveDateTime::default());
        assert_eq!(data.published, true);
    }

    #[test]
    fn post_metadata_to_markdown() {
        let data = PostMetadata {
            title: "Hello world".into(),
            published: false,
            created_at: NaiveDateTime::default(),
        };
        let markdown = data.to_markdown();
        let mut lines = markdown.lines();
        assert_eq!(lines.next(), Some("[//]: # (title: Hello world)"));
        assert_eq!(
            lines.next(),
            Some("[//]: # (created_at: 1970-01-01 00:00:00)")
        );
        assert_eq!(lines.next(), Some("[//]: # (published: false)"));
    }
}
