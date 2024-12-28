use std::time::SystemTime;

use super::super::i_intepretable::{Interpretable, StdInput, StdOutput};
use crate::{cli::Interpreter, programs::errors::CommandError};

pub struct Time {
    std_input: String,
}
/*

    prompt [arguments]

    options: none

*/

impl Interpretable for Time {
    fn execute(&self, _: &mut Interpreter)-> StdOutput {
        match self.get_input() {
            Ok(_) => {
                let now = format!("{:?}", SystemTime::now());
                return Ok(now);
            },
            Err(e) => {return Err(e)}
        }
    }

    fn new(input: String) -> Self {
        Time { std_input: input }
    }

    fn get_input(&self) -> StdInput {
        /*
            Possible inputs are like this:

            " something something something "

            or

            something.txt
        */

        // Check for empty string
        if self.std_input != "" {
            return Err(CommandError::NotAllowedArguments());
        }
        
        Ok(String::from(""))
    }
}