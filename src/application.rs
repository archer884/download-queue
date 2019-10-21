use crate::{config::*, download::Download, error::Result, fmt::FormatDuration, job::Job};
use std::collections::{HashMap, HashSet};

pub struct Application {
    config: Config,
    command: Command,
}

impl Application {
    pub fn new(config: Config, command: Command) -> Self {
        Self { config, command }
    }

    pub fn run(self) -> Result<()> {
        use std::fs;
        use std::time::Instant;

        let queue = fs::read_to_string(&self.command.path)?;
        let segregated_queues = build_queues(queue.lines());

        print_job_stats(&segregated_queues);

        let start_time = Instant::now();

        for (_host, downloads) in segregated_queues.into_iter() {
            Job::new(downloads, self.command.no_wait).execute(&self.config.youtube_dl);
        }

        let elapsed = Instant::now() - start_time;
        let filename = self
            .command
            .path()
            .file_name()
            .expect("path must refer to a file")
            .to_string_lossy();

        println!("{} complete in: {}", filename, elapsed.format());
        Ok(())
    }
}

fn print_job_stats(job: &HashMap<String, HashSet<Download>>) {
    let host_count = job.keys().count();
    let total_count: usize = job.values().map(HashSet::len).sum();

    println!(
        "Downloading {} files from {} hosts.",
        total_count, host_count
    );
}

fn build_queues<'a>(
    items: impl IntoIterator<Item = &'a str>,
) -> HashMap<String, HashSet<Download>> {
    let mut segregated_queues: HashMap<_, HashSet<_>> = HashMap::new();

    let items = items
        .into_iter()
        .enumerate()
        .filter(|x| !x.1.starts_with('#'))
        .map(|(idx, url)| (url.trim(), Download::with_index(idx, url)));

    for (url, item) in items {
        match item {
            Err(e) => eprintln!("[Warn]: unable to parse url ({}):\n    {}", url, e),
            Ok(item) => {
                segregated_queues
                    .entry(item.host().to_owned())
                    .or_default()
                    .insert(item);
            }
        }
    }

    segregated_queues
}
