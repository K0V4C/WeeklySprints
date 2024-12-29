use std::collections::HashMap;

use super::super::i_intepretable::{Interpretable, StdOutput};
use crate::{
    cli::Interpreter,
    programs::{errors::CommandError, i_intepretable::StdInput},
};

pub struct Head {
    std_input: StdInput,
    std_output: StdOutput,
}
/*

    head -ncount [arguments]

    options: -ncount

*/

const MAX_COUNT_NUMBER: u32 = 99999;

struct HeadPackage {
    option_n: u32,
    arguments: String,
}

impl Head {
    fn get_input(&self) -> Result<HeadPackage, CommandError> {
        /*
            Possible inputs are like this:

            > head -n5 file.txt
        */

        /*
            Check if -ncount option was set
        */

        let mut iterator = self.std_input.trim().split_whitespace().into_iter();
        let mut option_map = HashMap::new();

        while let Some(word) = iterator.next() {
            if word.len() >= 2 && word[0..2] == *"-n" {
                if word[2..].len() == 0 {
                    return Err(CommandError::HeadCountNotGiven());
                }

                let number = match word[2..].parse::<u32>() {
                    Ok(x) => {
                        if x > MAX_COUNT_NUMBER {
                            return Err(CommandError::HeadCountTooLarge());
                        }
                        x
                    }
                    Err(_) => return Err(CommandError::HeadCountNumberInvalid()),
                };

                option_map.entry("-n".to_owned()).or_insert(number);
            }
            // Todo: add error handling for other options
        }

        if option_map.len() != 1 {
            return Err(CommandError::OptionsNotDefined());
        }

        let mut words = self.std_input.split_whitespace();
        words.next(); // Skip the first word
        let remainder: String = words.collect::<Vec<&str>>().join(" ");

        if let Some(first_char) = remainder.chars().next() {
            if first_char == '"' {
                // Handle quoted input
                return Ok(HeadPackage {
                    option_n: *option_map.get("-n").unwrap(),
                    arguments: remainder.trim_matches('"').to_owned(),
                });
            } else {
                // Handle file name
                let file_name = remainder.trim();
                let file_content = match std::fs::read_to_string(file_name) {
                    Ok(x) => x,
                    Err(_) => return Err(CommandError::FileNotFound(file_name.to_owned())),
                };
                return Ok(HeadPackage {
                    option_n: *option_map.get("-n").unwrap(),
                    arguments: file_content,
                });
            }
        }

        Err(CommandError::EmptyString())
    }
}

impl Interpretable for Head {
    fn get_output(&self) -> StdOutput {
        self.std_output.clone()
    }

    fn execute(&mut self, _: &mut Interpreter) {
        let input = self.get_input();
        match input {
            Ok(value) => {
                let num = value.option_n as usize;
                let split = value.arguments.split('\n').collect::<Vec<&str>>();

                if num > split.len() {
                    self.std_output = Ok(split.join("\n"));
                } else {
                    let mut ret = String::new();
                    for idx in 0..num {
                        ret += split[idx];
                        ret += "\n";
                    }
                    self.std_output = Ok(ret);
                }
            }
            Err(error) => {
                self.std_output = Err(error);
            }
        }
    }

    fn new(input: String) -> Self {
        Head {
            std_input: input,
            std_output: Ok(String::new()),
        }
    }
}
