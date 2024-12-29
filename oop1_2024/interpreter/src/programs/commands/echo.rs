use super::super::i_intepretable::{Interpretable, StdInput, StdOutput};
use crate::{cli::Interpreter, programs::errors::CommandError};

pub struct Echo {
    std_input: String,
}
/*

    echo [arguments]

    options: none

*/
impl Echo {
    fn get_input(&self) -> StdInput {
        /*
            Possible inputs are like this:

            " something something something "

            or

            something.txt
        */

        // Check for empty string
        if self.std_input == "" {
            return Err(CommandError::EmptyString());
        }

        let has_quotes = self.std_input.chars().collect::<Vec<char>>()[0] == '"';

        if has_quotes {
            let ret = self.std_input.clone().trim_matches('"').to_owned();
            return Ok(ret);
        } else {
            let file_name = self.std_input.trim();
            let file = std::fs::read_to_string(file_name);
            match file {
                Ok(f) => {
                    return Ok(f);
                }
                Err(_) => {
                    return Err(CommandError::FileNotFound(file_name.to_owned()));
                }
            }
        }
    }
}

impl Interpretable for Echo {
    fn execute(&self, _interpreter: &mut Interpreter) -> StdOutput {
        let input = self.get_input();
        match input {
            Ok(value) => {
                return Ok(value);
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    fn new(input: String) -> Self {
        Echo { std_input: input }
    }
}
