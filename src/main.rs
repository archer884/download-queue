#[macro_use]
extern crate serde_derive;

mod application;
mod config;
mod download;
mod error;
mod fmt;
mod job;

fn main() -> error::Result<()> {
    use crate::{
        application::Application,
        config::*,
    };

    let command = Command::from_args();
    let config = Config::new(&command)?;

    Application::new(config, command).run()
}
