use crate::cli::CommandFormat;
use crate::input::errors::ReaderError;

#[derive(Debug)]
pub struct Reader;

impl Reader {
    pub fn new() -> Self {
        Reader
    }

    fn get_all_text(&self) -> Result<String, ReaderError> {
        let mut quote_count = 0;
        let mut total_output = String::new();

        loop {
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    for x in input.chars() {
                        if x == '"' {
                            quote_count += 1;
                        }
                    }
                }
                Err(_) => {
                    return Err(ReaderError::ReadLineError());
                }
            }

            total_output += input.as_str();

            if quote_count % 2 == 0 {
                break;
            }
        }

        Ok(total_output)
    }

    fn split_on_pipe(&self, text: &String) -> Result<Vec<String>, ReaderError> {
        let mut quotes_open = false;
        let mut split_input: Vec<String> = vec![];
        let mut start = 0;

        for (idx, x) in text.clone().chars().enumerate() {
            if x == '"' {
                quotes_open = !quotes_open;
            }

            if x == '|' && !quotes_open {
                split_input.push(text.as_str()[start..idx].to_owned());
                start = idx + 1;
            }
        }

        split_input.push(text.as_str()[start..].to_owned());

        // This should not be able to happen but still it is there
        if quotes_open == true {
            return Err(ReaderError::UnclosedQuotes());
        }

        Ok(split_input)
    }

    fn convert_to_commands(
        &self,
        split_text: &Vec<String>,
    ) -> Result<Vec<CommandFormat>, ReaderError> {
        // This is going to be the main point for error handling probbobly
        //
        //

        let mut vec_commands: Vec<CommandFormat> = vec![];

        for input_text in split_text.into_iter() {
            let split_input = input_text.split_whitespace().collect::<Vec<&str>>();
            let mut text_iterator = split_input.into_iter();
            let mut command_name = String::new();
            let mut in_file = String::new();
            let mut out_file = String::new();
            let mut args = String::new();

            while let Some(word) = text_iterator.next() {
                if word == "<" {
                    match text_iterator.next() {
                        Some(s) => in_file = s.to_owned(),
                        None => return Err(ReaderError::UndefinedInputRedirection()),
                    }
                } else if word == ">" {
                    match text_iterator.next() {
                        Some(s) => out_file = s.to_owned(),
                        None => return Err(ReaderError::UndefinedOutputRedirection()),
                    }
                } else {
                    if command_name == "" {
                        command_name = word.to_owned();
                    } else {
                        args += word;
                        args += " ";
                    }
                }
            }
            vec_commands.push(CommandFormat::new(
                command_name,
                in_file.trim().to_owned(),
                out_file.trim().to_owned(),
                args.trim().to_owned(),
            ));
        }
        Ok(vec_commands)
    }

    pub fn get_next_input(&self) -> Result<Vec<CommandFormat>, ReaderError> {
        let text = self.get_all_text()?;
        let split_text = self.split_on_pipe(&text)?;
        self.convert_to_commands(&split_text)
    }
}
