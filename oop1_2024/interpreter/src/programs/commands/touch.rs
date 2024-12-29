use std::fs::File;

use super::super::intepretable::{Interpretable, StdOutput};
use crate::{
    cli::Interpreter,
    programs::{errors::CommandError, intepretable::StdInput},
};

pub struct Touch {
    std_input: StdInput,
    std_output: StdOutput,
}
/*

    echo filename

    options: none

*/

struct TouchPackage {
    filename: String,
}

impl Touch {
    fn get_input(&self) -> Result<TouchPackage, CommandError> {
        /*
            Possible inputs are like this:

            > touch filename.extension

        */

        if let None = self.std_input.chars().next() {
            return Err(CommandError::EmptyString());
        }
        
       let first_char = self.std_input.chars().next().unwrap(); 
        
        if first_char == '"' {
            return Err(CommandError::NotAllowedArguments());
        } else {
            return Ok(TouchPackage {
                filename: self.std_input.trim().to_owned(),
            });
        }
    }
}

impl Interpretable for Touch {
    fn get_output(&self) -> StdOutput {
        self.std_output.clone()
    }
    fn execute(&mut self, _: &mut Interpreter) {
        let input = self.get_input();
        match input {
            Ok(value) => {
                match File::create(value.filename) {
                    Ok(_file) => {
                        self.std_output = Ok(String::new());
                    }
                    Err(error) => {
                        self.std_output =
                            Err(CommandError::TouchFailedToCreateFile(error.to_string()));
                    }
                };
            }
            Err(error) => {
                self.std_output = Err(error);
            }
        }
    }

    fn new(input: String) -> Self {
        Touch {
            std_input: input,
            std_output: Ok(String::new()),
        }
    }
}
