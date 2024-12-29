use super::super::i_intepretable::{Interpretable, StdOutput};
use crate::{
    cli::Interpreter,
    programs::{errors::CommandError, i_intepretable::StdInput},
};

pub struct Prompt {
    std_input: StdInput,
    std_output: StdOutput,
}
/*

    prompt [arguments]

    options: none

*/

struct PromptPackage {
    arguments: String,
}

impl Prompt {
    fn get_input(&self) -> Result<PromptPackage, CommandError> {
        /*
            Possible inputs are like this:

            > prompt "sign"

        */

        // Emprty string error
        if let None = self.std_input.chars().next() {
            return Err(CommandError::EmptyString());
        }

        // In case it is a string
        if self.std_input.chars().next().unwrap() == '"' {
            let ret = self.std_input.clone().trim_matches('"').to_owned();
            return Ok(PromptPackage { arguments: ret });
        }

        // In case it is a file
        let file_name = self.std_input.trim();
        let file = std::fs::read_to_string(file_name);
        match file {
            Ok(f) => {
                return Ok(PromptPackage { arguments: f });
            }
            Err(_) => {
                return Err(CommandError::FileNotFound(file_name.to_owned()));
            }
        }
    }
}

impl Interpretable for Prompt {
    fn get_output(&self) -> StdOutput {
        self.std_output.clone()
    }

    fn execute(&mut self, _interpreter: &mut Interpreter) {
        match self.get_input() {
            Ok(s) => {
                _interpreter.set_prompt(s.arguments.clone());
                self.std_output = Ok(s.arguments);
            }
            Err(e) => self.std_output = Err(e),
        }
    }

    fn new(input: String) -> Self {
        Prompt {
            std_input: input,
            std_output: Ok(String::new()),
        }
    }
}
