use std::error;
use std::fmt::{self, Display};
use std::io;
use std::result;
use url;

pub type Result<T> = result::Result<T, Error>;

type Message = &'static str;

#[derive(Debug)]
pub enum Error {
    Config(toml::de::Error),
    IO(io::Error),
    Other(Message),
    Url(url::ParseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Config(e) => writeln!(f, "Configuration unavailable: {}", e),
            Error::IO(e) => writeln!(f, "I/O error: {}", e),
            Error::Other(e) => f.write_str(e),
            Error::Url(e) => writeln!(f, "Unable to parse URL: {}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Config(e) => Some(e),
            Error::IO(e) => Some(e),
            Error::Url(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::Config(e)
    }
}

impl From<Message> for Error {
    fn from(message: Message) -> Self {
        Error::Other(message)
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::Url(e)
    }
}
