use std::collections::HashMap;

use super::super::i_intepretable::{Interpretable, StdInput, StdOutput};
use crate::{cli::Interpreter, programs::errors::CommandError};

pub struct Wc {
    std_input: String,
}
/*

    Wc -opt [arguments]

    options: -w / -c

*/
impl Wc {
    fn get_options(&self) -> Result<HashMap<String, String>, CommandError> {
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

        Ok(option_map)
    }

    fn get_input(&self) -> StdInput {
        /*
            Possible inputs are like this:

            > wc -opt [arguments]

            options: -w -> word count
                     -c -> char count

        */

        // Check for empty string
        if self.std_input == "" {
            return Err(CommandError::EmptyString());
        }

        let has_options = self.get_options()?.len() > 0;
        if has_options == false {
            return Err(CommandError::OptionsNotDefined());
        }

        let remainder = self
            .std_input
            .clone()
            .replace("-c", "")
            .replace("-w", "")
            .trim()
            .to_owned();

        let has_quotes = remainder.chars().collect::<Vec<char>>()[0] == '"';
        println!("{}", remainder);
        if has_quotes {
            let ret = remainder.clone().trim_matches('"').to_owned();
            println!("{}", ret);
            return Ok(ret);
        } else {
            let file_name = remainder.trim();
            let file = std::fs::read_to_string(file_name);
            match file {
                Ok(f) => {
                    return Ok(f);
                }
                Err(_) => {
                    return Err(CommandError::FileNotFound(file_name.to_owned()));
                }
            }
        }
    }
}

impl Interpretable for Wc {
    fn execute(&self, _: &mut Interpreter) -> StdOutput {
        let input = self.get_input();
        match input {
            Ok(value) => {
                let options = self.get_options()?;
                if options.contains_key("-w") {
                    return Ok(value
                        .trim()
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .len()
                        .to_string());
                } else if options.contains_key("-c") {
                    return Ok(value
                        .trim()
                        .chars()
                        .collect::<Vec<char>>()
                        .len()
                        .to_string());
                } else {
                    return Err(CommandError::OptionsNotDefined());
                }
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    fn new(input: String) -> Self {
        Wc { std_input: input }
    }

}
