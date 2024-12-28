use std::collections::HashMap;

use crate::{cli::Interpreter, programs::errors::CommandError};

pub type StdInput = Result<String, CommandError>;
pub type StdOutput = Result<String, CommandError>;

pub trait Interpretable {
    fn get_input(&self) -> StdInput {
        Err(CommandError::Undefined())
    }

    fn get_options(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    fn execute(&self, _interpreter: &mut Interpreter) -> StdOutput;

    fn new(input: String) -> Self;
}
