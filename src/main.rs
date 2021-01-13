mod application;
mod config;
mod download;
mod error;
mod fmt;
mod job;

use application::Application;
use config::{Command, Config};

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let command = Command::parse();
    let config = Config::new(&command)?;
    Application::new(config, command).run()
}
