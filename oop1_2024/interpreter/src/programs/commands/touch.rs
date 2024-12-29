use std::fs::File;

use super::super::i_intepretable::{Interpretable, StdInput, StdOutput};
use crate::{cli::Interpreter, programs::errors::CommandError};

pub struct Touch {
    std_input: String,
}
/*

    echo filename

    options: none

*/
impl Touch {
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

impl Interpretable for Touch {
    fn execute(&self, _: &mut Interpreter) -> StdOutput {
        let input = self.get_input();
        match input {
            Ok(value) => {
                match File::create(value) {
                    Ok(_file) => {
                        return Ok(String::new());
                    }
                    Err(error) => {
                        return Err(CommandError::TouchFailedToCreateFile(error.to_string()));
                    }
                };
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    fn new(input: String) -> Self {
        Touch { std_input: input }
    }

}
