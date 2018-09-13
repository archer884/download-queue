use std::fmt::{self, Display};
use std::time::Duration;

pub struct DurationFormatter(Duration);

impl Display for DurationFormatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.0.as_secs() / 60 / 60;
        let minutes = self.0.as_secs() / 60 % 60;
        let seconds = self.0.as_secs() % 3600 % 60;

        write!(f, "{}:{:02}:{:02}", hours, minutes, seconds)
    }
}

pub trait FormatDuration {
    fn format(&self) -> DurationFormatter;
}

impl FormatDuration for Duration {
    fn format(&self) -> DurationFormatter {
        DurationFormatter(*self)
    }
}
