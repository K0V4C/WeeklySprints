use std::{env, io::{stdin, stdout, Write}, path::Path, process::{Child, Command, Stdio}};

fn main() {

    loop {
        // Deteailing so we recognise this as a shell
        print!("> ");
        stdout().flush().unwrap();

        // First thing to do is to get user input
        let mut user_input = String::new();

        // Read the input
        stdin()
            .read_line(&mut user_input)
            .expect(" User input could not have been read");

        // Split into command and arguments
        let mut commands = user_input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {

            // Trim the excess whitespaces
            let mut split_data = command.trim().split_whitespace();
            let command = split_data.next().unwrap();
            let args = split_data;

            match command {
               "cd" => {
                   // If nothing is provided resort to root i.e. '\'

                   let new_dir = args.peekable().peek().map_or("/", |x| *x);
                   let root = Path::new(new_dir);
                   if let Err(e)  = env::set_current_dir(&root) {
                       eprintln!("{}", e);
                   }
               },
               "exit" => return,
               command => {

                   // We either get our inptu from std or intherit it from child that died
                   let stdin = previous_command.map_or(Stdio::inherit(), |child: Child| Stdio::from(child.stdout.unwrap()));

                   let stdout = if commands.peek().is_some() {
                       // Prepare to send output to next command
                       Stdio::piped()
                   } else {
                       // We are last boys o7
                       Stdio::inherit()
                   };

                   // Start new thread with this command
                   let child = Command::new(command)
                       .args(args)
                    .stdin(stdin)
                    .stdout(stdout)
                       .spawn();

                   // In case of an error handle it
                   match child {
                       Ok(child) => {
                           previous_command = Some(child);
                       },
                       Err(e) => {
                           previous_command = None;
                           eprintln!("{}", e);
                       }
                   }

               }
            }
        }

        // Waiting for the last solider
        if let Some(mut final_command) = previous_command {
            final_command.wait().expect("Final solider exploaded");
        }
    }

}
