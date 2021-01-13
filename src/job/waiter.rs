use std::{thread, u32};

use rand::{
    self,
    distributions::{DistIter, Distribution, Uniform},
    prelude::ThreadRng,
};

type WaitTimeGen = DistIter<Uniform<u32>, ThreadRng, u32>;

pub struct Waiter {
    first_time: bool,
    wait_time_gen: WaitTimeGen,
}

impl Waiter {
    pub fn new(min: u32, max: u32) -> Self {
        Self {
            first_time: true,
            wait_time_gen: Uniform::from(min..max).sample_iter(rand::thread_rng()),
        }
    }

    pub fn wait(&mut self) {
        use std::time::Duration;

        if self.first_time {
            self.first_time = false;
            return;
        }

        thread::sleep(Duration::from_secs(u64::from(
            self.wait_time_gen.next().unwrap_or_default(),
        )));
    }
}
