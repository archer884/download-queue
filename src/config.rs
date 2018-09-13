use error::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Command {
    /// Path to a list of files to be downloaded.
    pub path: String,
    /// Path to the folder where downloads will be stored.
    ///
    /// Warning: not implemented.
    #[structopt(short = "d", long = "downloads")]
    pub downloads: Option<String>,
    /// Path to youtube-dl binary.
    #[structopt(short = "y", long = "youtube-dl")]
    pub youtube_dl: Option<String>,
    /// Path to config file.
    #[structopt(short = "x", long = "config")]
    pub config: Option<String>,
}

impl Command {
    pub fn from_args() -> Self {
        StructOpt::from_args()
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
    pub fn new(command: &Command) -> Result<Self> {
        use std::fs;
        use toml;

        let path = command
            .config
            .as_ref()
            .map(AsRef::as_ref)
            .unwrap_or("~/.download-queue");

        let path = rehome(path)?;
        let content = fs::read_to_string(path).map_err(Error::config)?;

        toml::from_str(&content).map_err(Error::config)
    }

    pub fn log(&self) -> PathBuf {
        match self.log {
            None => rehome("~/Documents/download-log.txt").expect("FIXME: quit being lazy"),
            Some(ref path) => rehome(path).expect("FIXME: seriously, though"),
        }
    }
}

fn rehome(path: &str) -> Result<PathBuf> {
    use dirs;

    if path.starts_with("~/") {
        let mut full_path = dirs::home_dir().ok_or("Home directory not available")?;
        full_path.push(&path[2..]);
        Ok(full_path)
    } else {
        Ok(PathBuf::from(path))
    }
}
