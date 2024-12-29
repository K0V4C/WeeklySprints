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

    WcBothOptionsListed(),
    OptionsNotDefined(),

    TooManyArguments(),
    
    HeadCountNotGiven(),
    HeadCountTooLarge(),
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
            }
            Self::NotAllowedArguments() => {
                write!(f, "Arguements were given to command!")
            }
            Self::TouchFailedToCreateFile(x) => {
                write!(f, "Command [Touch] failed to create file: {}", x)
            }
            Self::RmFailedToDeleteFile(x) => {
                write!(f, "Command [Rm] failed to delete file: {}", x)
            }
            Self::TruncateFailedToTruncateAFile(x) => {
                write!(f, "Command [Truncate] failed to truncate a file: {}", x)
            }
            Self::WcBothOptionsListed() => {
                write!(f, "Command [Wc] ecnoutered an error, both -wc were listed")
            }
            Self::OptionsNotDefined() => {
                write!(f, "Command ecnoutered an error, options were not given")
            },
            Self::TooManyArguments() => {
                write!(f, "Command ecnoutered an error, too many options wer given")
            },          
            Self::HeadCountNotGiven() => {
                write!(f, "Command [Head] ecnoutered an error, count not given")
            },
            Self::HeadCountTooLarge() => {
                write!(f, "Command [Head] ecnoutered an error, count too large")
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
                return InterpreterError::CommandError(format!("{}", CommandError::EmptyString()))
            }
            CommandError::FileNotFound(x) => {
                return InterpreterError::CommandError(format!("{}", CommandError::FileNotFound(x)))
            }
            CommandError::NotAllowedArguments() => {
                return InterpreterError::CommandError(format!(
                    "{}",
                    CommandError::NotAllowedArguments()
                ))
            }
            CommandError::TouchFailedToCreateFile(x) => {
                return InterpreterError::CommandError(format!(
                    "{}",
                    CommandError::TouchFailedToCreateFile(x)
                ))
            }
            CommandError::RmFailedToDeleteFile(x) => {
                return InterpreterError::CommandError(format!(
                    "{}",
                    CommandError::RmFailedToDeleteFile(x)
                ))
            }
            CommandError::TruncateFailedToTruncateAFile(x) => {
                return InterpreterError::CommandError(format!(
                    "{}",
                    CommandError::TruncateFailedToTruncateAFile(x)
                ))
            }
            CommandError::WcBothOptionsListed() => {
                return InterpreterError::CommandError(format!(
                    "{}",
                    CommandError::WcBothOptionsListed()
                ))
            }
            CommandError::OptionsNotDefined() => {
                return InterpreterError::CommandError(format!(
                    "{}",
                    CommandError::OptionsNotDefined()
                ))
            },
            CommandError::TooManyArguments() => {
                return InterpreterError::CommandError(format!(
                    "{}",
                    CommandError::TooManyArguments()
                ))
            },
            CommandError::HeadCountNotGiven() => {
                return InterpreterError::CommandError(format!(
                    "{}",
                    CommandError::HeadCountNotGiven()
                ))
            },            
            CommandError::HeadCountTooLarge() => {
                return InterpreterError::CommandError(format!(
                    "{}",
                    CommandError::HeadCountTooLarge()
                ))
            }
        };
    }
}
