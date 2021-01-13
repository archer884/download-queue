use std::path::{Path, PathBuf};

use clap::{Clap, crate_authors, crate_description, crate_version};
use serde::{Deserialize, Serialize};

#[derive(Clap, Clone, Debug)]
#[clap(author = crate_authors!(), version = crate_version!(), about = crate_description!())]
pub struct Command {
    /// Path to a list of files to be downloaded.
    pub path: String,
    /// Path to the folder where downloads will be stored.
    ///
    /// Warning: not implemented.
    #[clap(short, long)]
    pub downloads: Option<String>,
    /// Path to youtube-dl binary.
    #[clap(short = 'y', long = "youtube-dl")]
    pub youtube_dl: Option<String>,
    /// Path to config file.
    #[clap(short = 'x', long = "config")]
    pub config: Option<String>,
    /// Skip wait between downloads
    #[clap(short = 'f', long = "no-wait")]
    pub no_wait: bool,
}

impl Command {
    pub fn parse() -> Self {
        Clap::parse()
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

/// Defines a set of default options.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// The youtube-dl location.
    #[serde(rename = "youtube-dl")]
    pub youtube_dl: String,
    /// The log file location.
    pub log: Option<String>,
    /// The minimum wait time.
    #[serde(rename = "minimum-wait")]
    pub min_wait: Option<u32>,
    /// The maximum wait time.
    #[serde(rename = "maximum-wait")]
    pub max_wait: Option<u32>,
}

impl Config {
    pub fn new(command: &Command) -> crate::Result<Self> {
        use std::fs;

        let path = command
            .config
            .as_ref()
            .map(AsRef::as_ref)
            .unwrap_or("~/.download-queue");

        let path = rehome(path)?;
        let content = fs::read_to_string(path)?;

        Ok(toml::from_str(&content)?)
    }
}

fn rehome(path: &str) -> crate::Result<PathBuf> {
    if path.starts_with("~/") {
        let mut full_path = dirs::home_dir().ok_or("Home directory not available")?;
        full_path.push(&path[2..]);
        Ok(full_path)
    } else {
        Ok(PathBuf::from(path))
    }
}
