use crate::{cli::Interpreter, programs::errors::CommandError};

pub type StdInput = String;
pub type StdOutput = Result<String, CommandError>;

pub trait Interpretable {
    fn execute(&mut self, _interpreter: &mut Interpreter);
    fn new(input: String) -> Self;
    fn get_output(&self) -> StdOutput;
}
