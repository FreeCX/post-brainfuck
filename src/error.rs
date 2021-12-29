use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse(ParseError),
    Execute(ExecuteError),
}

#[derive(Debug)]
pub enum ParseError {
    BracketsNumber,
    NoLoopBegin,
}

#[derive(Debug)]
pub enum ExecuteError {
    OutOfMemory,
    NegativeMemoryIndex,
    ReadStdin,
}

impl error::Error for Error {}
impl error::Error for ParseError {}
impl error::Error for ExecuteError {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(ref e) => e.fmt(f),
            Error::Parse(ref e) => e.fmt(f),
            Error::Execute(ref e) => e.fmt(f),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::BracketsNumber => write!(f, "number of brackets does not match"),
            ParseError::NoLoopBegin => write!(f, "loop start not found"),
        }
    }
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecuteError::OutOfMemory => write!(f, "out of memory"),
            ExecuteError::NegativeMemoryIndex => write!(f, "cannot access negative memory index"),
            ExecuteError::ReadStdin => write!(f, "cannot read byte from stdin"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Error::Parse(error)
    }
}

impl From<ExecuteError> for Error {
    fn from(error: ExecuteError) -> Self {
        Error::Execute(error)
    }
}
