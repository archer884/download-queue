use std::path::Path;

use clap::{crate_authors, crate_description, crate_version, Clap};

// FIXME: These wait time parameters are presently ignored.

#[derive(Clap, Clone, Debug)]
#[clap(author = crate_authors!(), version = crate_version!(), about = crate_description!())]
pub struct Opts {
    /// Path to a list of files to be downloaded.
    pub path: String,

    /// Path to youtube-dl binary.
    #[clap(short = 'y', long = "youtube-dl")]
    pub youtube_dl: Option<String>,

    /// Skip wait between downloads
    #[clap(short = 'f', long = "no-wait")]
    pub no_wait: bool,

    /// Minimum wait time
    pub min_wait: Option<u32>,

    /// Maximum wait time
    pub max_wait: Option<u32>,
}

impl Opts {
    pub fn parse() -> Self {
        Clap::parse()
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }
}
