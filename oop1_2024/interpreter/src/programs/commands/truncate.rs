use std::fs::{metadata, OpenOptions};

use super::super::i_intepretable::{Interpretable, StdOutput};
use crate::{
    cli::Interpreter,
    programs::{errors::CommandError, i_intepretable::StdInput},
};

pub struct Truncate {
    std_input: StdInput,
    std_output: StdOutput,
}
/*

    truncate filename.ext

    options: none

*/

struct TruncatePackage {
    filename: String,
}

impl Truncate {
    fn get_input(&self) -> Result<TruncatePackage, CommandError> {
        /*
            Possible inputs are like this:

            > touch filename.extension

        */

        if let Some(first_char) = self.std_input.chars().next() {
            if first_char == '"' {
                return Err(CommandError::NotAllowedArguments());
            } else {
                return Ok(TruncatePackage {
                    filename: self.std_input.trim().to_owned(),
                });
            }
        }

        return Err(CommandError::EmptyString());
    }
}

impl Interpretable for Truncate {
    fn get_output(&self) -> StdOutput {
        self.std_output.clone()
    }

    fn execute(&mut self, _: &mut Interpreter) {
        let input = self.get_input();
        match input {
            Ok(value) => {
                match metadata(value.filename.clone()) {
                    Err(_) => {
                        self.std_output = Err(CommandError::FileNotFound(value.filename.clone()));
                    }
                    _ => {}
                }

                if let Err(e) = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(value.filename)
                {
                    self.std_output =
                        Err(CommandError::TruncateFailedToTruncateAFile(e.to_string()));
                };
                self.std_output = Ok(String::new());
            }
            Err(error) => {
                self.std_output = Err(error);
            }
        }
    }

    fn new(input: String) -> Self {
        Truncate {
            std_input: input,
            std_output: Ok(String::new()),
        }
    }
}
