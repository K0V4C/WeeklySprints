use std::time::SystemTime;

use super::super::i_intepretable::{Interpretable, StdOutput};
use crate::{
    cli::Interpreter,
    programs::{errors::CommandError, i_intepretable::StdInput},
};

pub struct Time {
    std_input: StdInput,
    std_output: StdOutput,
}
/*

    prompt [arguments]

    options: none

*/

struct TimePackage;

impl Time {
    fn get_input(&self) -> Result<TimePackage, CommandError> {
        /*
            Possible inputs are like this:

           > time

        */

        // Check for empty string
        if self.std_input != "" {
            return Err(CommandError::NotAllowedArguments());
        }

        Ok(TimePackage)
    }
}

impl Interpretable for Time {
    fn get_output(&self) -> StdOutput {
        self.std_output.clone()
    }

    fn execute(&mut self, _: &mut Interpreter) {
        match self.get_input() {
            Ok(_) => {
                let now = format!("{:?}", SystemTime::now());
                self.std_output = Ok(now);
            }
            Err(e) => self.std_output = Err(e),
        }
    }

    fn new(input: String) -> Self {
        Time {
            std_input: input,
            std_output: Ok(String::new()),
        }
    }
}
