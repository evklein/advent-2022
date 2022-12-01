use std::env;
use std::fs;

pub mod days;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (method, input_path) = parse_arguments(&args);
    let input = fs::read_to_string(input_path).expect("File could not be read");

    match method.as_str() {
        "d1" => days::day1::part1(input),
        _ => println!("Invalid input."),
    };
}

fn parse_arguments(args: &[String]) -> (String, String) {
    let method: String = args[1].clone();
    let input_path: String = args[2].clone();

    (method, input_path)
}