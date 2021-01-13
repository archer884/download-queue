mod application;
mod download;
mod error;
mod fmt;
mod job;
mod opts;

use application::Application;
use opts::Opts;

fn main() -> error::Result<()> {
    Application::new(Opt::from_args()).run()
}
