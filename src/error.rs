use std::error;
use std::fmt::{self, Display};
use std::result;

pub type Result<T> = result::Result<T, Error>;

type Message = &'static str;

#[derive(Debug)]
pub enum Error {
    ConfigUnavailable(Box<error::Error>),
    Other(Message),
}

impl Error {
    pub fn config(e: impl error::Error + 'static) -> Self {
        Error::ConfigUnavailable(Box::new(e))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match *self {
            ConfigUnavailable(ref error) => writeln!(f, "Configuration unavailable\n{}", error),    
            Other(message) => f.write_str(message),
        }
    }
}

impl From<Message> for Error {
    fn from(message: Message) -> Self {
        Error::Other(message)
    }
}
