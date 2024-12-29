use super::super::i_intepretable::{Interpretable, StdInput, StdOutput};
use crate::{cli::Interpreter, programs::errors::CommandError};

pub struct Prompt {
    std_input: String,
}
/*

    prompt [arguments]

    options: none

*/
impl Prompt {    
    fn get_input(&self) -> StdInput {
        /*
            Possible inputs are like this:

            > prompt "sign"

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

impl Interpretable for Prompt {
    fn execute(&self, _interpreter: &mut Interpreter) -> StdOutput {
        match self.get_input() {
            Ok(s) => {
                _interpreter.set_prompt(s.clone());
                return Ok(s);
            }
            Err(e) => return Err(e),
        }
    }

    fn new(input: String) -> Self {
        Prompt { std_input: input }
    }

}
