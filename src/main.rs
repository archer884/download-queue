#[macro_use]
extern crate serde_derive;

extern crate dirs;
extern crate structopt;
extern crate toml;

mod error;
mod config;

// Youtubed-dl location on macos
// /usr/local/bin/youtube-dl

fn main() -> error::Result<()> {
    use config::*;

    let command = Command {
        path: String::from("meh"),
        downloads: None,
        youtube_dl: None,
        config: None,
    };

    let config = Config::new(&command)?;

    println!("{:?}", config);
    Ok(())
}
