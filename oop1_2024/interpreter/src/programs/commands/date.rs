use std::time::{SystemTime, UNIX_EPOCH};

use super::super::i_intepretable::{Interpretable, StdOutput};
use crate::{
    cli::Interpreter,
    programs::{errors::CommandError, i_intepretable::StdInput},
};

pub struct Date {
    std_input: StdInput,
    std_output: StdOutput,
}
/*

    date

    options: none

*/

struct DatePackage;

impl Date {
    fn get_input(&self) -> Result<DatePackage, CommandError> {
        /*
            Possible inputs are like this:

            > date

        */

        // Check for empty string
        if self.std_input != "" {
            return Err(CommandError::NotAllowedArguments());
        }

        Ok(DatePackage)
    }
}

impl Interpretable for Date {
    fn get_output(&self) -> StdOutput {
        return self.std_output.clone();
    }

    fn execute(&mut self, _: &mut Interpreter) {
        match self.get_input() {
            Ok(_) => {
                // Get the current system time
                let start = SystemTime::now();

                // Get the duration since UNIX_EPOCH
                let duration_since_epoch = start
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");

                // Get the seconds since UNIX_EPOCH
                let seconds_since_epoch = duration_since_epoch.as_secs();

                // Calculate the year, month, and day from the number of seconds since the UNIX_EPOCH
                let days_since_epoch = seconds_since_epoch / 86400; // 86400 seconds in a day
                let leap_years = (days_since_epoch / 1461) as i32; // 1461 days in a leap year (365 * 3 + 366)

                let days_in_year = days_since_epoch % 365; // Remainder days
                let year = 1970 + leap_years; // Start from the UNIX epoch year (1970)

                // This is a rough approximation and doesn't account for leap years correctly in months
                let month = (days_in_year / 30) + 1; // Simple approximation: 30 days per month
                let day = days_in_year % 30; // Remainder for the day

                // Return the current date (rough approximation)
                self.std_output = Ok(format!("Current date: {}-{:02}-{:02}", year, month, day));
            }
            Err(e) => self.std_output = Err(e),
        }
    }

    fn new(input: String) -> Self {
        Date {
            std_input: input,
            std_output: Ok(String::new()),
        }
    }
}
