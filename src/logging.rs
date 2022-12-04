use std::fmt::Display;

use anyhow::Result;
use owo_colors::OwoColorize;

#[cfg(debug_assertions)]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Debug;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Warn;

struct ColoredLogLevel(log::Level);

impl Display for ColoredLogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self.0 {
            level if level == log::Level::Debug => level.to_string().blue().to_string(),
            level if level == log::Level::Error => level.to_string().red().to_string(),
            level if level == log::Level::Warn => level.to_string().yellow().to_string(),
            level if level == log::Level::Info => level.to_string().green().to_string(),
            level if level == log::Level::Trace => level.to_string().bright_white().to_string(),
            level => level.to_string(),
        };
        write!(f, "{str}")
    }
}

pub fn init() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let level = record.level().as_str();
            out.finish(format_args!(
                "[{timestamp} {level}{level_pad} {target}] {message}",
                timestamp = chrono::Local::now()
                    .format("%Y-%m-%dT%H:%M:%S")
                    .bright_black(),
                level = ColoredLogLevel(record.level()),
                level_pad = (0..(5 - level.len().min(5)))
                    .map(|_| ' ')
                    .collect::<String>(),
                target = record.target().purple(),
            ))
        })
        .level(LOG_LEVEL)
        .level_for("handlebars", log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
