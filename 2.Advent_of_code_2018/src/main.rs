use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate regex;
extern crate chrono;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

fn main() {
    let args: Vec<String> = env::args().collect();
    let response = day_7::execute(read_input_file(&args[1]));
    println!("result : {}, {}", response.0, response.1);
}

fn read_input_file(file: &str) -> String {
    let mut file = File::open(file).expect("Error");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Error");
    buffer
}
