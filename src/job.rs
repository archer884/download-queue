use download::Download;
use std::sync::mpsc::Sender;

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

    pub fn execute(self) {
        unimplemented!()
    }
}
