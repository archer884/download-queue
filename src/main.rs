#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate crossbeam;
extern crate dirs;
extern crate rand;
extern crate stopwatch;
extern crate structopt;
extern crate termcolor;
extern crate toml;
extern crate url;

mod application;
mod config;
mod download;
mod error;
mod fmt;
mod job;

use crate::{
    application::Application,
    config::*,
};

fn main() -> error::Result<()> {
    let command = Command::from_args();
    let config = Config::new(&command)?;

    Application::new(config, command).run()
}
