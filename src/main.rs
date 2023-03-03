use std::{
    io,
    str::FromStr,
};

// ############# Presets for input handling #############
fn next_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let trimmed_len = input_line.as_mut_str().trim_end().len();
    input_line.truncate(trimmed_len);
    input_line
}

trait SplitParse<T: FromStr> {
    fn split_parse(&self) -> Vec<T>;
}

impl<T: FromStr> SplitParse<T> for String {
    fn split_parse(&self) -> Vec<T> {
        self.split_whitespace().flat_map(str::parse::<T>).collect::<Vec<T>>()
    }
}

// ############# Main program starts here #############

fn main() {
    println!("Hello, World!");
}