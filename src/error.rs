use std::error;
use std::fmt::{self, Display};
use std::result;
use url;

pub type Result<T> = result::Result<T, Error>;

type Message = &'static str;
type Cause = Box<error::Error>;

#[derive(Debug)]
pub enum Error {
    Config(Cause),
    Schedule(Cause),
    Url(Cause),
    Other(Message),
}

impl Error {
    pub fn config(e: impl error::Error + 'static) -> Self {
        Error::Config(Box::new(e))
    }

    pub fn schedule(e: impl error::Error + 'static) -> Self {
        Error::Schedule(Box::new(e))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match *self {
            Config(ref error) => writeln!(f, "Configuration unavailable: {}", error),    
            Schedule(ref error) => writeln!(f, "Invalid schedule: {}", error),
            Url(ref error) => writeln!(f, "Invalid url: {}", error),
            Other(message) => f.write_str(message),
        }
    }
}

impl From<Message> for Error {
    fn from(message: Message) -> Self {
        Error::Other(message)
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::Url(Box::new(e))
    }
}
