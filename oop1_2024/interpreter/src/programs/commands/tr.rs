use super::super::i_intepretable::{Interpretable, StdInput, StdOutput};
use crate::{cli::Interpreter, programs::errors::CommandError};

pub struct Tr {
    std_input: String,
}
/*

    tr [arguments] with [what]

    options: None

*/
impl Tr {
    fn get_input(&self) -> StdInput {
        /*
            Possible inputs are like this:

            > tr "test test test"/filename "bato" "sestro"

        */

        // Check for empty string
        if self.std_input == "" {
            return Err(CommandError::EmptyString());
        }

        let has_quotes = self.std_input.trim().chars().collect::<Vec<char>>()[0] == '"';

        if has_quotes {
            let ret = self.std_input.clone();

            //In case only argjuments were given
            if ret.len() == 1 {
                return Ok(ret);
            }

            //Check if only 3 were defined here
            // > if there are 2 ignore second
            // > if there are more then 3 error it

            // Temp
            return Ok(ret);
        } else {
            let mut split_string = self.std_input.split_whitespace().collect::<Vec<&str>>();
            let file_name = split_string.remove(0);
            let file = std::fs::read_to_string(file_name);
            match file {
                Ok(f) => {
                    // Check if there is 2 elements and if they are there check if they both what with "
                    // In future should check if they are bouth encapuslated in ""
                    // Output is either [arguments]
                    // or
                    //                  [arguments] "what" "with"

                    if split_string.len() == 0 {
                        return Ok(f);
                    }

                    //Check if only 3 were defined here
                    // > if there are 2 ignore second
                    // > if there are more then 3 error it

                    // Temp
                    Ok(f)
                }
                Err(_) => {
                    return Err(CommandError::FileNotFound(file_name.to_owned()));
                }
            }
        }
    }
}

impl Interpretable for Tr {
    fn execute(&self, _: &mut Interpreter) -> StdOutput {
        let input = self.get_input();
        match input {
            Ok(value) => {
                println!("{}", value);
                return Ok(value);
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    fn new(input: String) -> Self {
        Tr { std_input: input }
    }
}
