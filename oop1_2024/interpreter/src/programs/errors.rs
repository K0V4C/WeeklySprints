use core::fmt;

use crate::cli::errors::InterpreterError;

#[derive(Clone)]
pub enum CommandError {
    Undefined(),

    EmptyString(),
    FileNotFound(String),

    NotAllowedArguments(),

    TouchFailedToCreateFile(String),

    RmFailedToDeleteFile(String),

    TruncateFailedToTruncateAFile(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undefined() => {
                write!(f, "Command object is not defined")
            }
            Self::EmptyString() => {
                write!(f, "Command got an empty string")
            }
            Self::FileNotFound(x) => {
                write!(f, "File not found: [{}]", x)
            },
            Self::NotAllowedArguments() => {
                write!(f, "Arguements were given to command!")
            },
            Self::TouchFailedToCreateFile(x) => {
                write!(f, "Command [Touch] failed to create file: {}", x)
            },
            Self::RmFailedToDeleteFile(x) => {
                write!(f, "Command [Rm] failed to delete file: {}", x)
            },
            Self::TruncateFailedToTruncateAFile(x) => {
                write!(f, "Command [Truncate] failed to truncate a file: {}", x)
            }
        }
    }
}

impl From<CommandError> for InterpreterError {
    fn from(value: CommandError) -> Self {
        match value {
            CommandError::Undefined() => {
                return InterpreterError::CommandError(format!("{}", CommandError::Undefined()))
            }

            CommandError::EmptyString() => {
                return InterpreterError::CommandError(format!(
                    "{}",
                    CommandError::EmptyString()
                ))
            }
            CommandError::FileNotFound(x) => {
                return InterpreterError::CommandError(format!(
                    "{}",
                    CommandError::FileNotFound(x)
                ))
            },
            CommandError::NotAllowedArguments() => {
                return InterpreterError::CommandError(
                    format!("{}", CommandError::NotAllowedArguments())
                )
            },
            CommandError::TouchFailedToCreateFile(x) => {
                return InterpreterError::CommandError(
                    format!("{}", CommandError::TouchFailedToCreateFile(x))
                )
            },
            CommandError::RmFailedToDeleteFile(x) => {
                return InterpreterError::CommandError(
                    format!("{}", CommandError::RmFailedToDeleteFile(x))
                )
            },
            CommandError::TruncateFailedToTruncateAFile(x) => {
                return InterpreterError::CommandError(
                    format!("{}", CommandError::TruncateFailedToTruncateAFile(x))
                )
            }
        };
    }
}
