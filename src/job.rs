use std::{path::Path, process::Command};

mod fmt;
mod waiter;

use crate::download::Download;

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
        use self::{fmt::ResultFormatter, waiter::Waiter};

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
