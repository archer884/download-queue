#[macro_use]
extern crate serde_derive;

extern crate crossbeam;
extern crate dirs;
extern crate stopwatch;
extern crate structopt;
extern crate toml;
extern crate url;

mod application;
mod error;
mod config;
mod download;
mod job;

// Youtubed-dl location on macos
// /usr/local/bin/youtube-dl

fn main() -> error::Result<()> {
    use application::Application;
    use config::*;

    let command = Command::from_args();
    let config = Config::new(&command)?;

    Application::new(config, command).run()
}
