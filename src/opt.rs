use std::path::Path;
use structopt::StructOpt;

// FIXME: These wait time parameters are presently ignored.

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Path to a list of files to be downloaded.
    pub path: String,

    /// Path to youtube-dl binary.
    #[structopt(short = "y", long = "youtube-dl")]
    pub youtube_dl: Option<String>,

    /// Skip wait between downloads
    #[structopt(short = "f", long = "no-wait")]
    pub no_wait: bool,

    /// Minimum wait time
    pub min_wait: Option<u32>,

    /// Maximum wait time
    pub max_wait: Option<u32>,
}

impl Opt {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }
}
