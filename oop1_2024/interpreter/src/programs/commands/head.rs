
use std::collections::HashMap;

use super::super::i_intepretable::{Interpretable, StdInput, StdOutput};
use crate::{cli::Interpreter, programs::errors::CommandError};

pub struct Head {
    std_input: String,
}
/*

    head -ncount [arguments]

    options: -ncount

*/
impl Head {    
    fn get_options(&self) -> Result<HashMap<String, String>, CommandError> {
        let mut option_map: HashMap<String, String> = HashMap::new();
        let mut iterator = self.std_input.trim().split_whitespace().into_iter();

        while let Some(word) = iterator.next() {
            if word.len() >= 2 && word[0..2] == *"-n" {

                if word[2..].len() == 0 {
                    return Err(CommandError::HeadCountNotGiven());
                }

                option_map
                    .entry("-n".to_owned())
                    .or_insert(word[2..].to_owned());
            }
            // Todo: add error handling for other options
        }

        if option_map.len() != 1 {
            return Err(CommandError::OptionsNotDefined());
        }

        Ok(option_map)
    }

    fn get_input(&self) -> StdInput {
        /*
            Possible inputs are like this:

            > head -n5 file.txt

        */

        let has_options = self.get_options()?.len() > 0;
        if has_options == false {
            return Err(CommandError::OptionsNotDefined());
        }

        let mut words = self.std_input.split_whitespace();
        words.next(); // Skip the first word
        let remainder: String = words.collect::<Vec<&str>>().join(" ");

        if let Some(first_char) = remainder.chars().next() {

            if first_char == '"' {
                // Handle quoted input
                return Ok(remainder.trim_matches('"').to_owned());
            } else {
                // Handle file name
                let file_name = remainder.trim();
                return std::fs::read_to_string(file_name).map_err(|_| {
                    CommandError::FileNotFound(file_name.to_owned())
                });
            }
        }

       Err(CommandError::EmptyString())
    }
}

impl Interpretable for Head {
    fn execute(&self, _: &mut Interpreter) -> StdOutput {
        let input = self.get_input();
        match input {
            Ok(value) => {

                let options = self.get_options()?;
                let split = value.split('\n').collect::<Vec<&str>>();

                if let Some(num) = options.get("-n") {
                    // FIX: error handling
                    let num = num.parse::<usize>().unwrap();

                    if num > split.len() {
                        return Ok(split.join("\n"));
                    } else {
                        let mut ret = String::new();
                        for idx in 0..num {
                           ret += split[idx];
                           ret += "\n";
                        }
                        return Ok(ret);
                    }


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
        Head { std_input: input }
    }

}
