use std::fmt;
use std::convert::From;
use std::io;
use std::error::Error as StdError;

use nix;

/// An error arising from terminal operations.
#[derive(Debug)]
pub struct Error {
    description: &'static str,
    cause: Option<Box<StdError>>,
}

impl Error {
    /// Creates a new `Error` with the given description.
    pub fn new(desc: &'static str) -> Error {
        Error {
            description: desc,
            cause: None,
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.description
    }

    fn cause(&self) -> Option<&StdError> {
        if let Some(ref err) = self.cause {
            Some(&**err)
        } else {
            None
        }
    }
}

impl From<nix::Error> for Error {
    fn from(err: nix::Error) -> Self {
        Error::new(err.errno().desc())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error {
            description: "internal io error",
            cause: Some(Box::new(err)),
        }
    }
}

impl From<io::CharsError> for Error {
    fn from(err: io::CharsError) -> Self {
        Error {
            description: "utf8 translation error",
            cause: Some(Box::new(err)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

