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

        if let Some(first_char) = self.std_input.chars().next() {
            if first_char == '"' {
                let ret = self.std_input.clone().trim_matches('"').to_owned();
                return Ok(PromptPackage { arguments: ret });
            } else {
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

        return Err(CommandError::EmptyString());
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
