use std::io::{stdin, stdout, Write};

/// Returns a String with what the user typed in response to the prompt.
///
/// # Arguments
///
/// * `prompt` - A &str that is printed to the console as a prompt for the user.
///
/// # Remarks
///
/// This is a convenience function that just shortens the amount of code that is
/// necessary to recieve user input in response to a prompt, such as a question.
pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    let mut input = String::new();

    stdout().flush().expect("Failed to flush stdout!");
    stdin().read_line(&mut input).expect("Failed to read line");

    input.pop();

    return input;
}
