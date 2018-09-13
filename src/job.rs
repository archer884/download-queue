use chrono::Local;
use download::Download;
use rand::{self, Rng};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::thread;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct Job {
    downloads: Vec<Download>,
}

impl Job {
    pub fn new(downloads: impl IntoIterator<Item = Download>) -> Self {
        let mut downloads: Vec<_> = downloads.into_iter().collect();
        downloads.sort_by_key(|x| x.idx);

        Self { downloads }
    }

    pub fn execute(self, path: impl AsRef<Path>) {
        let mut waiter = Waiter::new(33, 60);

        for item in self.downloads {
            waiter.wait();

            let url = item.url();
            match Command::new(path.as_ref())
                .args(&["--no-progress", url])
                .output()
            {
                Err(e) => {
                    eprintln!("Failed to execute process: {}", e);
                    return;
                }

                Ok(result) => {
                    if result.status.success() {
                        print_success(url);
                    } else {
                        print_error(item.idx, url);
                    }
                }
            }
        }
    }
}

fn print_success(url: &str) {
    {
        let mut stream = StandardStream::stderr(ColorChoice::Always);
        let _ = stream.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
        let _ = stream.write(b"[Success]");
        let _ = stream.set_color(ColorSpec::new().set_fg(None));
    }

    println!(" {} {}", Local::now().format("%F %T"), url);
}

fn print_error(line: usize, url: &str) {
    {
        let mut stream = StandardStream::stderr(ColorChoice::Always);
        let _ = stream.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
        let _ = stream.write(b"[Failure]");
        let _ = stream.set_color(ColorSpec::new().set_fg(None));
    }

    println!(" {} (line {}) {}", Local::now().format("%F %T"), line, url);
}

struct Waiter {
    min: u32,
    max: u32,
    first_time: bool,
}

impl Waiter {
    fn new(min: u32, max: u32) -> Self {
        Self {
            min,
            max,
            first_time: true,
        }
    }

    fn wait(&mut self) {
        use std::time::Duration;

        if self.first_time {
            self.first_time = false;
            return;
        }

        let seconds = rand::thread_rng().gen_range(self.min, self.max);
        let duration = Duration::from_secs(seconds.into());

        thread::sleep(duration);
    }
}
