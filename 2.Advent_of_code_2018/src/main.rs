use std::env;
use std::fs::File;
use std::io::prelude::*;

mod day_1;
mod day_2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = day_2::execute(read_input_file(&args[1]));
    println!("result : {}, {}", result.0, result.1);
}

fn read_input_file(file: &str) -> String {
    let mut file = File::open(file).expect("Error");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Error");
    buffer
}
