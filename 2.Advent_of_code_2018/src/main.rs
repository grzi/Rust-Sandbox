use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate regex;
extern crate chrono;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day4 = day_4::execute(read_input_file(&args[1]));
    println!("result : {}, {}", day4.0, day4.1);
}

fn read_input_file(file: &str) -> String {
    let mut file = File::open(file).expect("Error");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Error");
    buffer
}
