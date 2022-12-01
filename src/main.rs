use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (method, input_path) = parse_arguments(&args);
    let input = fs::read_to_string(input_path).expect("File could not be read");

    match input.as_str() {

    };
}

fn parse_arguments(args: &[String]) -> (String, String) {
    let method: String = args[1].clone();
    let input_path: String = args[2].clone();

    (method, input_path)
}