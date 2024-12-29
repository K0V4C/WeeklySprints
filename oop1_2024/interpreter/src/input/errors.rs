use std::fmt::{self, Display};

#[derive(Clone)]
pub enum ReaderError {
    ReadLineError(),
    UndefinedInputRedirection(),
    UndefinedOutputRedirection(),
    UnclosedQuotes(),
}

impl Display for ReaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadLineError() => {
                write!(f, "Error reading lines from file")
            }
            Self::UndefinedInputRedirection() => {
                write!(f, "Input redirection file not given")
            }
            Self::UndefinedOutputRedirection() => {
                write!(f, "Output redirection file not given")
            }
            Self::UnclosedQuotes() => {
                write!(f, "Quotes not close at the end of the line")
            }
        }
    }
}