use std::fs::{metadata, OpenOptions};

use super::super::i_intepretable::{Interpretable, StdInput, StdOutput};
use crate::{cli::Interpreter, programs::errors::CommandError};

pub struct Truncate {
    std_input: String,
}
/*

    truncate filename.ext

    options: none

*/
impl Truncate {
    fn get_input(&self) -> StdInput {
        /*
            Possible inputs are like this:

            > touch filename.extension

        */

        // Check for empty string
        if self.std_input == "" {
            return Err(CommandError::EmptyString());
        }

        let has_quotes = self.std_input.chars().collect::<Vec<char>>()[0] == '"';

        if has_quotes {
            return Err(CommandError::NotAllowedArguments());
        } else {
            return Ok(self.std_input.trim().to_owned());
        }
    }
}

impl Interpretable for Truncate {
    fn execute(&self, _: &mut Interpreter) -> StdOutput {
        let input = self.get_input();
        match input {
            Ok(value) => {
                match metadata(value.clone()) {
                    Err(_) => {
                        return Err(CommandError::FileNotFound(value));
                    }
                    _ => {}
                }

                if let Err(e) = OpenOptions::new().write(true).truncate(true).open(value) {
                    return Err(CommandError::TruncateFailedToTruncateAFile(e.to_string()));
                };
                Ok(String::new())
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    fn new(input: String) -> Self {
        Truncate { std_input: input }
    }
}
