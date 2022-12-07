use std::env;
use std::fs;

pub mod days;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (method, input_path) = parse_arguments(&args);
    let input = fs::read_to_string(input_path).expect("File could not be read");

    match method.as_str() {
        "d1p1" => days::day1::part1(input),
        "d1p2" => days::day1::part2(input),
        "d2p1" => days::day2::part1(input),
        _ => println!("Invalid input."),
    };
}

fn parse_arguments(args: &[String]) -> (String, String) {
    let method: String = args[1].clone();
    let input_path: String = args[2].clone();

    (method, input_path)
}