use download::Download;
use rand::{self, Rng};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use std::process::Command;
use std::path::Path;

pub struct Job {
    log_sink: Sender<String>,
    downloads: Vec<Download>,
}

impl Job {
    pub fn new(log_sink: Sender<String>, downloads: impl IntoIterator<Item = Download>) -> Self {
        Self {
            log_sink,
            downloads: downloads.into_iter().collect(),
        }
    }

    pub fn execute(self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        let mut waiter = Waiter::new(33, 60);

        for item in self.downloads {
            waiter.wait();

            let url = item.url();
            match Command::new(path).args(&["--no-progress", url]).output() {
                Err(e) => {
                    eprintln!("Failed to execute process: {}", e);
                    return;
                }

                Ok(result) => {
                    if !result.status.success() {
                        eprintln!("Failed to download url:\n    {}", url);
                        let _ = self.log_sink.send(unsafe {
                            String::from_utf8_unchecked(result.stderr)
                        });
                    }
                }
            }
        }
    }
}

struct Waiter {
    min: u32,
    max: u32,
    first_time: bool,
}

impl Waiter {
    fn new(min: u32, max: u32) -> Self {
        Self { min, max, first_time: true }
    }

    fn wait(&mut self) {
        if self.first_time {
            self.first_time = false;
            return;
        }

        let seconds = rand::thread_rng().gen_range(self.min, self.max);
        let duration = Duration::from_secs(seconds.into());
        
        thread::sleep(duration);
    }
}
