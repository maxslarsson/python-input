use std::io::{stdin, stdout, Write};

pub fn input(input_string: &str) -> String {
    print!("{}", input_string);
    let mut input = String::new();

    stdout().flush().expect("Failed to flush stdout!");
    stdin().read_line(&mut input).expect("Failed to read line");

    input.pop();

    return input;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
