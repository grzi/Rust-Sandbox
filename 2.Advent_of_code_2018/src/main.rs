use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate regex;

mod day_1;
mod day_2;
mod day_3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day3 = day_3::execute(read_input_file(&args[1]));
    println!("result : {}, {}", day3.0, day3.1);
}

fn read_input_file(file: &str) -> String {
    let mut file = File::open(file).expect("Error");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Error");
    buffer
}
