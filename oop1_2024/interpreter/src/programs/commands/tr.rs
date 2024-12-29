use super::{
    super::intepretable::{Interpretable, StdOutput},
    utility::get_quoted_strings,
};
use crate::{
    cli::Interpreter, programs::{errors::CommandError, intepretable::StdInput}
};

pub struct Tr {
    std_input: StdInput,
    std_output: StdOutput,
}

struct TrPackage {
    arguments: String,
    what: Option<String>,
    with: Option<String>,
}
/*

    tr [arguments] with [what]

    options: None

*/
impl Tr {
    fn get_input(&self) -> Result<TrPackage, CommandError> {
        /*
            Possible inputs are like this:

            > tr "test test test"/filename "bato" "sestro"

        */

        if let None = self.std_input.chars().next() {
            return Err(CommandError::EmptyString());
        }
        
        let first_char = self.std_input.chars().next().unwrap();
    
        if first_char == '"' {
            let quoted_strings = get_quoted_strings(self.std_input.as_str())?;
            //In case only argjuments were given
            if quoted_strings.len() == 1 || quoted_strings.len() == 2 {
                return Ok(TrPackage {
                    arguments: quoted_strings[0].clone(),
                    what: None,
                    with: None,
                });
            }

            //Check if only 3 were defined here
            // > if there are 2 ignore second
            // > if there are more then 3 error it

            if quoted_strings.len() == 3 {
                return Ok(TrPackage {
                    arguments: quoted_strings[0].clone(),
                    what: Some(quoted_strings[1].clone()),
                    with: Some(quoted_strings[2].clone()),
                });
            }
            return Err(CommandError::TrTooManyStrings());
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

                    let quoted_strings = get_quoted_strings(split_string.join(" ").as_str())?;

                    if quoted_strings.len() == 1 || quoted_strings.len() == 0 {
                        return Ok(TrPackage {
                            arguments: f,
                            what: None,
                            with: None,
                        });
                    }

                    //Check if only 3 were defined here
                    // > if there are 2 ignore second
                    // > if there are more then 3 error it

                    if quoted_strings.len() == 2 {
                        return Ok(TrPackage {
                            arguments: f,
                            what: Some(quoted_strings[0].clone()),
                            with: Some(quoted_strings[1].clone()),
                        });
                    }
                    return Err(CommandError::TrTooManyStrings());
                }
                Err(_) => {
                    return Err(CommandError::FileNotFound(file_name.to_owned()));
                }
            }
        }
    }
}

impl Interpretable for Tr {
    fn get_output(&self) -> StdOutput {
        self.std_output.clone()
    }

    fn execute(&mut self, _: &mut Interpreter) {
        let input = self.get_input();
        match input {
            Ok(value) => {
                let what = value.what.clone();
                let with = value.with.clone();

                if let Some(_) = value.with {
                    self.std_output = Ok(value
                        .arguments
                        .replace(what.unwrap().as_str(), with.unwrap().as_str()));
                } else {
                    self.std_output = Ok(value.arguments);
                }
            }
            Err(error) => {
                self.std_output = Err(error);
            }
        }
    }

    fn new(input: String) -> Self {
        Tr {
            std_input: input,
            std_output: Ok(String::new()),
        }
    }
}
