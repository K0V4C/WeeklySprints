use std::collections::HashMap;

use super::super::intepretable::{Interpretable, StdOutput};
use crate::{
    cli::Interpreter,
    programs::{errors::CommandError, intepretable::StdInput},
};

pub struct Wc {
    std_input: StdInput,
    std_output: StdOutput,
}
/*

    Wc -opt [arguments]

    options: -w / -c

*/

struct WcPackage {
    option: String,
    arguments: String,
}

impl Wc {
    fn get_input(&self) -> Result<WcPackage, CommandError> {
        /*
            Possible inputs are like this:

            > wc -opt [arguments]

            options: -w -> word count
                     -c -> char count

        */
        let mut option_map: HashMap<String, String> = HashMap::new();
        let mut iterator = self.std_input.trim().split_whitespace().into_iter();

        while let Some(word) = iterator.next() {
            if word == "-c" {
                option_map
                    .entry("-c".to_owned())
                    .or_insert("set".to_owned());
            }

            if word == "-w" {
                option_map
                    .entry("-w".to_owned())
                    .or_insert("set".to_owned());
            }
        }

        if option_map.len() >= 2 {
            return Err(CommandError::WcBothOptionsListed());
        }
        if option_map.len() == 0 {
            return Err(CommandError::OptionsNotDefined());
        }

        let mut _selected_option = String::new();
        if let Some(_) = option_map.get("-c") {
            _selected_option = "-c".to_owned();
        } else if let Some(_) = option_map.get("-w") {
            _selected_option = "-w".to_owned();
        } else {
            return Err(CommandError::OptionsNotDefined());
        }

        let remainder = self
            .std_input
            .clone()
            .replace("-c", "")
            .replace("-w", "")
            .trim()
            .to_owned();
        
        if let None = self.std_input.chars().next() {
            return Err(CommandError::EmptyString());
        }
            
        let first_character = self.std_input.chars().next().unwrap();

        if first_character == '"' {
            let ret = remainder.clone().trim_matches('"').to_owned();
            return Ok(WcPackage {
                option: _selected_option,
                arguments: ret,
            });
        } else {
            let file_name = remainder.trim();
            let file = std::fs::read_to_string(file_name);
            match file {
                Ok(f) => {
                    return Ok(WcPackage {
                        option: _selected_option,
                        arguments: f,
                    });
                }
                Err(_) => {
                    return Err(CommandError::FileNotFound(file_name.to_owned()));
                }
            }
        }
    }
}

impl Interpretable for Wc {
    fn get_output(&self) -> StdOutput {
        self.std_output.clone()
    }

    fn execute(&mut self, _: &mut Interpreter) {
        let input = self.get_input();
        match input {
            Ok(value) => {
                let option = value.option;

                match option.as_str() {
                    "-w" => {
                        self.std_output = Ok(value
                            .arguments
                            .trim()
                            .split_whitespace()
                            .collect::<Vec<&str>>()
                            .len()
                            .to_string());
                    }

                    "-c" => {
                        self.std_output = Ok(value
                            .arguments
                            .trim()
                            .chars()
                            .collect::<Vec<char>>()
                            .len()
                            .to_string());
                    }
                    _ => {
                        self.std_output = Err(CommandError::Undefined());
                    }
                };
            }
            Err(error) => {
                self.std_output = Err(error);
            }
        }
    }

    fn new(input: String) -> Self {
        Wc {
            std_input: input,
            std_output: Ok(String::new()),
        }
    }
}
