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
mod day_8;
mod day_9;
mod day_10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let response = day_10::execute(read_input_file(&args[1].to_string()));
    println!("result : {:?}", response);
}

fn read_input_file(file: &str) -> String {
    let mut file = File::open(file).expect("Error");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Error");
    buffer
}
