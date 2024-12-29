use std::io::Write;

use crate::input::errors::ReaderError;
use crate::input::reader::Reader;

use crate::programs::commands::date::Date;
use crate::programs::commands::echo::Echo;
use crate::programs::commands::head::Head;
use crate::programs::commands::prompt::Prompt;
use crate::programs::commands::rm::Rm;
use crate::programs::commands::time::Time;
use crate::programs::commands::touch::Touch;
use crate::programs::commands::tr::Tr;
use crate::programs::commands::truncate::Truncate;
use crate::programs::commands::wc::Wc;
use crate::programs::i_intepretable::Interpretable;
use crate::programs::i_intepretable::StdOutput;

use super::errors::InterpreterError;

#[derive(Debug)]
pub struct Interpreter {
    promt_sign: String,
    running: bool,
    input_reader: Reader,
    command_line_queue: Vec<Vec<CommandFormat>>
}

#[derive(Debug, Clone)]
pub struct CommandFormat {
    command: String,
    in_redirection: String,
    out_redirection: String,
    command_args: String,
}

impl CommandFormat {
    pub fn new(
        command: String,
        in_redirection: String,
        out_redirection: String,
        command_args: String,
    ) -> Self {
        CommandFormat {
            command,
            in_redirection,
            out_redirection,
            command_args,
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let reader = Reader::new();
        Interpreter {
            promt_sign: String::from("$"),
            running: true,
            input_reader: reader,
            command_line_queue: vec![]
        }
    }

    /*
        Operations for working with interpreter / direct interaction with interpreter
        > printing promprt sign
        > changing promt sign
        > printing result of command line
        > feeding the command queue
        > getting next command line
    */
    
    fn set_promt_ready(&self) {
        print!("{}", self.promt_sign);
        std::io::stdout().flush().unwrap();
    }

    pub fn set_prompt(&mut self, sign: String) {
        self.promt_sign = sign;
    }

    fn print(&self, pipe_to_next: StdOutput, output_file: String) {
        // Output of CLI LINE
        match pipe_to_next {
            Ok(final_value) => {
                if output_file != "" {
                    if let Err(x) = std::fs::write(output_file, final_value) {
                        println!("Writing to file at the end went wrong: {}", x);
                    }
                } else {
                    println!("{}", final_value)
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
    
    /*
   
        Used by batch cli command to fill it
    
    */
    pub fn add_to_command_line_queue(&self) {
        
    }
    
    fn get_next_cli_line(&mut self) -> Result<Vec<CommandFormat>, ReaderError> {
        if let Some(data) = self.command_line_queue.pop() {
            return Ok(data);
        } else {
            return self.input_reader.get_next_input();
        }
    }

    /*
    
        Working place of interpreter [all command functions and operations are under]
        
        > creates command and executes them
        > run body for the main logic of CLI interpreter
   
    */
    
    fn operate_over_commands(
        &mut self,
        command_data: &CommandFormat,
        cli_input: String,
        pipe_to_next: &mut StdOutput,
    ) {
        match command_data.command.as_str() {
            "echo" => {
                let command = Echo::new(cli_input);
                *pipe_to_next = command.execute(self);
            }

            "prompt" => {
                let command = Prompt::new(cli_input);
                *pipe_to_next = command.execute(self);
            }

            "time" => {
                let command = Time::new(cli_input);
                *pipe_to_next = command.execute(self);
            }

            "date" => {
                let command = Date::new(cli_input);
                *pipe_to_next = command.execute(self);
            }

            "touch" => {
                let command = Touch::new(cli_input);
                *pipe_to_next = command.execute(self);
            }

            "rm" => {
                let command = Rm::new(cli_input);
                *pipe_to_next = command.execute(self);
            }

            "truncate" => {
                let command = Truncate::new(cli_input);
                *pipe_to_next = command.execute(self);
            }
            "wc" => {
                let command = Wc::new(cli_input);
                *pipe_to_next = command.execute(self);
            }
            "tr" => {
                let command = Tr::new(cli_input);
                *pipe_to_next = command.execute(self);
            },
            
            "head" => {
                let command = Head::new(cli_input);
                *pipe_to_next = command.execute(self);
            },
            
            "batch" => {
                // let command = ::new(cli_input);
                // *pipe_to_next = command.execute(self);
            }
            _ => {
                println!("Command name is not correct, try again");
            }
        };
    }
    
    
    /*
   
        Used to figure out what should be next input for command, pipe/argument or < redirection
    
    */
    fn get_input_for_next_command(
        &self,
        command_data: &CommandFormat,
        pipe_to_next: StdOutput,
    ) -> Result<String, InterpreterError> {
        // If error occured in last command just error it again
        let pipe_to_next = pipe_to_next?;

        // If there is redirection in and pipe it is also an error
        if command_data.in_redirection != "" && pipe_to_next != "" {
            return Err(InterpreterError::ColideInRedirectionAndPipe());
        }

        //Reading from file or normal std_in
        let mut cli_input = if command_data.in_redirection != "" {
            if let Ok(x) = std::fs::read_to_string(command_data.in_redirection.as_str()) {
                Ok(x)
            } else {
                Err(InterpreterError::FileNotFound(
                    command_data.in_redirection.clone(),
                ))
            }
        } else {
            Ok(command_data.command_args.clone())
        }?;

        // If pipe was given and all errors were resolved before, now just concat
        if pipe_to_next != "" {
            cli_input += pipe_to_next.as_str();
            cli_input.insert(0, '"');
            cli_input += "\"";
        }

        // Return proper input
        Ok(cli_input)
    }

    
    /*
   
        Main body of the interpreter, rough logic for operations is located here
   
    */
    pub fn run(&mut self) {
        while self.running {
            // Ready up the prompt
            self.set_promt_ready();

            // Get input
            let data = match self.get_next_cli_line() {
                Ok(x) => x,
                Err(error) => {
                    println!("{}", error);
                    continue;
                }
            };

            // If someone is pressing enter
            if data.len() == 0 {
                continue;
            }

            // Used to pass to next command
            let mut pipe_to_next: StdOutput = Ok(String::new());
            // Used to rember output file if it exists
            let mut output_file = String::new();

            // Iterating through command that has multiple pipes
            for command_data in data {
                /*
                    There are few errors that can happen here:

                    1. input redirection given and also argument input? Does one just take priority?
                    2. Output redirection as not the last pipe
                    3. What if we have a pipe? -> args can exist but only options, input redirection cant exists
                */

                // Test if reader works correctly
                // println!("{:?}", command_data);

                let cli_input =
                    match self.get_input_for_next_command(&command_data, pipe_to_next.clone()) {
                        Ok(x) => x,
                        Err(error) => {
                            println!("{}", error);
                            break;
                        }
                    };

                // Test if we are reading cli correctly
                // println!("Cli input: {}", cli_input);

                output_file = command_data.out_redirection.clone();
                self.operate_over_commands(&command_data, cli_input, &mut pipe_to_next);
            }

            // Output of CLI LINE
            self.print(pipe_to_next, output_file);
        }
    }
}