use std::fmt::Display;

pub enum InterpreterError {
    CommandError(String),

    ColideInRedirectionAndPipe(),
    FileNotFound(String),
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandError(s) => write!(f, "{}", s),

            Self::ColideInRedirectionAndPipe() => {
                write!(f, "Both input redirection and pipe were given")
            }
            Self::FileNotFound(file) => write!(f, "File was not found: {}", file),
        }
    }
}
