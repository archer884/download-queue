use download::Download;
use rand::{self, Rng};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Job {
    downloads: Vec<Download>,
}

impl Job {
    pub fn new(downloads: impl IntoIterator<Item = Download>) -> Self {
        Self {
            downloads: downloads.into_iter().collect(),
        }
    }

    pub fn execute(self, path: impl AsRef<Path>, log: Arc<Mutex<impl Fn(&[u8]) + 'static>>) {
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

                        let log = log.lock().expect("Please don't get poisoned...");
                        log(&result.stderr);
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
        Self {
            min,
            max,
            first_time: true,
        }
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
