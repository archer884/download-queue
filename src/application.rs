use crate::{
    config::*,
    download::Download,
    error::{Error, Result},
    fmt::FormatDuration,
    job::Job,
};
use crossbeam;
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

        let queue = fs::read_to_string(&self.command.path).map_err(Error::schedule)?;
        let segregated_queues = build_queues(queue.lines());

        format_job_stats(&segregated_queues);

        let start_time = Instant::now();

        // FIXME: this represents an unbounded degree of concurrency. Such concurrency could prove
        // to be a problem if we connect to too many hosts at once; individual downloads could
        // then take too long and time out. To limit this possibility, it may be best to process
        // a maximum number of hosts at any given time. I have no idea how to do that.
        crossbeam::scope(|scope| {
            let mut jobs = Vec::new();
            for (_host, download_set) in segregated_queues.into_iter() {
                jobs.push(scope.spawn(|| Job::new(download_set).execute(&self.config.youtube_dl)));
            }
            jobs.into_iter().for_each(|job| job.join().expect("wtaf?"));
        });

        let elapsed = Instant::now() - start_time;
        println!("Jobs complete in: {}", elapsed.format());
        Ok(())
    }
}

fn format_job_stats(job: &HashMap<String, HashSet<Download>>) {
    let host_count = job.keys().count();
    let total_count: usize = job.values().map(|x| x.len()).sum();

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
