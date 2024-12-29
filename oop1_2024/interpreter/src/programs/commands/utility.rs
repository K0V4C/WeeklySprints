use crate::programs::errors::CommandError;

pub fn get_quoted_strings(string: &str) -> Result<Vec<String>, CommandError> {
    let mut quoted_strings: Vec<String> = vec![];
    let mut quotes_open = false;

    let mut start_idx = 0;
    for (running_idx, x) in string.chars().enumerate() {
        if quotes_open && x == '"' {
            quoted_strings.push(string[start_idx..=running_idx].trim_matches('"').to_owned());
            quotes_open = false;
        } else if !quotes_open && x == '"' {
            start_idx = running_idx;
            quotes_open = true;
        } else if !quotes_open && x.is_whitespace() == false {
            return Err(CommandError::TrArgumentsAreNotString());
        }
    }

    Ok(quoted_strings)
}
