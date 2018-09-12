use config::*;
use download::Download;
use error::*;
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
        use std::thread;

        let queue = fs::read_to_string(&self.command.path).map_err(Error::schedule)?;
        let segregated_queues = build_queues(queue.lines());

        format_job_stats(&segregated_queues);

        let mut jobs = Vec::new();
        for (host, download_set) in segregated_queues.into_iter() {
            jobs.push(thread::spawn(move || {
                // Download damned files here.
            }));
        }

        jobs.into_iter().for_each(|job| job.join().expect("wtaf?"));
        Ok(())
    }
}

fn format_job_stats(job: &HashMap<String, HashSet<Download>>) {
    let host_count = job.keys().count();
    let total_count = job.values().count();

    println!("Downloading {} files from {} hosts.", total_count, host_count);
}

fn build_queues<'a>(items: impl IntoIterator<Item = &'a str>) -> HashMap<String, HashSet<Download>> {
    let mut segregated_queues: HashMap<_, HashSet<_>> = HashMap::new();

    let items = items.into_iter().map(|url| (url.trim(), Download::from_url(url)));
    for (url, item) in items {
        match item {
            Err(e) => eprintln!("[Warn]: unable to parse url ({}):\n    {}", url, e),
            Ok(item) => {
                segregated_queues.entry(item.host().to_owned()).or_default().insert(item);
            }
        }
    }    

    segregated_queues
}
