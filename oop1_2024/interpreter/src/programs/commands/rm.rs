use super::super::i_intepretable::{Interpretable, StdOutput};
use crate::{
    cli::Interpreter,
    programs::{errors::CommandError, i_intepretable::StdInput},
};

pub struct Rm {
    std_input: StdInput,
    std_output: StdOutput,
}
/*

    echo filename

    options: none

*/

struct RmPackage {
    filename: String,
}

impl Rm {
    fn get_input(&self) -> Result<RmPackage, CommandError> {
        /*
            Possible inputs are like this:

            > rm filename.ext

        */

        if let Some(first_char) = self.std_input.chars().next() {
            if first_char == '"' {
                return Err(CommandError::NotAllowedArguments());
            } else {
                return Ok(RmPackage {
                    filename: self.std_input.trim().to_owned(),
                });
            }
        }

        return Err(CommandError::EmptyString());
    }
}

impl Interpretable for Rm {
    fn get_output(&self) -> StdOutput {
        self.std_output.clone()
    }

    fn execute(&mut self, _: &mut Interpreter) {
        let input = self.get_input();
        match input {
            Ok(value) => {
                match std::fs::remove_file(value.filename) {
                    Ok(_file) => {
                        self.std_output = Ok(String::new());
                    }
                    Err(error) => {
                        self.std_output =
                            Err(CommandError::RmFailedToDeleteFile(error.to_string()));
                    }
                };
            }
            Err(error) => {
                self.std_output = Err(error);
            }
        }
    }

    fn new(input: String) -> Self {
        Rm {
            std_input: input,
            std_output: Ok(String::new()),
        }
    }
}
