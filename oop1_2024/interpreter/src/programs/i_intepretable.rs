use crate::{cli::Interpreter, programs::errors::CommandError};

pub type StdInput = Result<String, CommandError>;
pub type StdOutput = Result<String, CommandError>;

pub trait Interpretable {
    fn execute(&self, _interpreter: &mut Interpreter) -> StdOutput;
    fn new(input: String) -> Self;
}
