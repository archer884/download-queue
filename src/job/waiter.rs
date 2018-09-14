use rand::{self, Rng};
use std::thread;

pub struct Waiter {
    min: u32,
    max: u32,
    first_time: bool,
}

impl Waiter {
    pub fn new(min: u32, max: u32) -> Self {
        Self {
            min,
            max,
            first_time: true,
        }
    }

    pub fn wait(&mut self) {
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
