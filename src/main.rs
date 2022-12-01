use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let code = &args[1];
    let input: &String = &args[2];

    println!("Running {code} with input {input}");
}
