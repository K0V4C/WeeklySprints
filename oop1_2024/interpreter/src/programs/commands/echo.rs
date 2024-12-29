use super::super::i_intepretable::{Interpretable, StdOutput};
use crate::{
    cli::Interpreter,
    programs::{errors::CommandError, i_intepretable::StdInput},
};

pub struct Echo {
    std_input: StdInput,
    std_output: StdOutput,
}
/*

    echo [arguments]

    options: none

*/

struct EchoPackage {
    arguments: String,
}

impl Echo {
    fn get_input(&self) -> Result<EchoPackage, CommandError> {
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
            return Ok(EchoPackage { arguments: ret });
        } else {
            let file_name = self.std_input.trim();
            let file = std::fs::read_to_string(file_name);
            match file {
                Ok(f) => {
                    return Ok(EchoPackage { arguments: f });
                }
                Err(_) => {
                    return Err(CommandError::FileNotFound(file_name.to_owned()));
                }
            }
        }
    }
}

impl Interpretable for Echo {
    fn get_output(&self) -> StdOutput {
        self.std_output.clone()
    }

    fn execute(&mut self, _interpreter: &mut Interpreter) {
        let input = self.get_input();
        match input {
            Ok(value) => {
                self.std_output = Ok(value.arguments);
            }
            Err(error) => {
                self.std_output = Err(error);
            }
        }
    }

    fn new(input: String) -> Self {
        Echo {
            std_input: input,
            std_output: Ok(String::new()),
        }
    }
}
