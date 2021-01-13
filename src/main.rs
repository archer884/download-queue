mod application;
mod download;
mod error;
mod fmt;
mod job;
mod opts;

use application::Application;
use opts::Opts;

type Result<T, E = error::Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    Application::new(Opts::parse()).run()
}
