use super::super::i_intepretable::{Interpretable, StdOutput};
use crate::{
    cli::Interpreter,
    programs::{errors::CommandError, i_intepretable::StdInput},
};

pub struct Batch {
    std_input: StdInput,
    std_output: StdOutput,
}
/*

    Wc -opt [arguments]

    options: -w / -c

*/

struct BatchPackage {
    arguments: String,
}

impl Batch {
    fn get_input(&self) -> Result<BatchPackage, CommandError> {
        /*
            Possible inputs are like this:

            > batch  [arguments]

            options: None
        */

        if let Some(first_character) = self.std_input.chars().next() {
            if first_character == '"' {
                let ret = self
                    .std_input
                    .clone()
                    .strip_prefix('"')
                    .and_then(|x| x.strip_suffix('"'))
                    .unwrap_or(self.std_input.as_str())
                    .to_owned();
                return Ok(BatchPackage { arguments: ret });
            } else {
                let file_name = self.std_input.trim();
                let file = std::fs::read_to_string(file_name);
                match file {
                    Ok(f) => {
                        return Ok(BatchPackage { arguments: f });
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

impl Interpretable for Batch {
    fn get_output(&self) -> StdOutput {
        self.std_output.clone()
    }

    fn execute(&mut self, _interpreter: &mut Interpreter) {
        let input = self.get_input();
        match input {
            Ok(value) => {
                let split_on_new_line = value.arguments.split('\n').collect::<Vec<&str>>();
                for line in split_on_new_line {
                    if line == "" {
                        continue;
                    }
                    let command_list = _interpreter.parse_thorugh_reader(line.trim().to_owned());
                    _interpreter.add_to_command_line_queue(command_list);
                }
            }
            Err(error) => {
                self.std_output = Err(error);
            }
        }
    }

    fn new(input: String) -> Self {
        Batch {
            std_input: input,
            std_output: Ok(String::new()),
        }
    }
}
