use std::cell::RefCell;

mod cli;
mod input;
mod programs;

fn main() {
    let interpreter = RefCell::new(cli::Interpreter::new());
    interpreter.borrow_mut().run();
}
