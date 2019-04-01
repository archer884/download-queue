use crate::download::Download;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod waiter;

pub struct Job {
    downloads: Vec<Download>,
    no_wait: bool,
}

impl Job {
    pub fn new(downloads: impl IntoIterator<Item = Download>, no_wait: bool) -> Self {
        let mut downloads: Vec<_> = downloads.into_iter().collect();
        downloads.sort_by_key(|x| x.idx);

        Self { downloads, no_wait }
    }

    pub fn execute(self, path: impl AsRef<Path>) {
        use self::waiter::Waiter;

        let mut waiter = Waiter::new(33, 60);
        let formatter = ResultFormatter::new();

        for item in self.downloads {
            if !self.no_wait {
                waiter.wait();
            }

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
                        formatter.print_success(item.idx, url);
                    } else {
                        formatter.print_error(item.idx, url);
                    }
                }
            }
        }
    }
}

struct ResultFormatter {
    stdout: bool,
    stderr: bool,
}

impl ResultFormatter {
    fn new() -> Self {
        use atty::Stream;
        Self {
            stdout: atty::is(Stream::Stdout),
            stderr: atty::is(Stream::Stderr),
        }
    }

    fn print_success(&self, line: usize, url: &str) {
        if self.stdout {
            {
                let mut stream = StandardStream::stdout(ColorChoice::Always);
                let _ = stream.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
                let _ = stream.write(b"[Success]");
                let _ = stream.set_color(ColorSpec::new().set_fg(None));
            }
            println!(" #{} {}", line, url);
        } else {
            println!("#{} {}", line, url);
        }
    }

    fn print_error(&self, line: usize, url: &str) {
        if self.stderr {
            {
                let mut stream = StandardStream::stderr(ColorChoice::Always);
                let _ = stream.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
                let _ = stream.write(b"[Failure]");
                let _ = stream.set_color(ColorSpec::new().set_fg(None));
            }
            eprintln!(" #{} {}", line, url);
        } else {
            eprintln!("#{} {}", line, url);
        }
    }
}
