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
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "1" => println!("result : {:?}", day_1::execute(read_input_file("src/day_1/input.txt"))),
        "2" => println!("result : {:?}", day_2::execute(read_input_file("src/day_2/input.txt"))),
        "3" => println!("result : {:?}", day_3::execute(read_input_file("src/day_3/input.txt"))),
        "4" => println!("result : {:?}", day_4::execute(read_input_file("src/day_4/input.txt"))),
        "5" => println!("result : {:?}", day_5::execute(read_input_file("src/day_5/input.txt"))),
        "6" => println!("result : {:?}", day_6::execute(read_input_file("src/day_6/input.txt"))),
        "7" => println!("result : {:?}", day_7::execute(read_input_file("src/day_7/input.txt"))),
        "8" => println!("result : {:?}", day_8::execute(read_input_file("src/day_8/input.txt"))),
        "9" => println!("result : {:?}", day_9::execute()),
        "10" => println!("result : {:?}", day_10::execute(read_input_file("src/day_10/input.txt"))),
        "11" => println!("result : {:?}", day_11::execute(2187)),
        "12" => println!("result : {:?}", day_12::execute(read_input_file("src/day_12/input.txt"))),
        "13" => println!("result : {:?}", day_13::execute(read_input_file("src/day_13/input.txt"))),
        "14" => println!("result : {:?}", day_14::execute()),
        "15" => println!("result : {:?}", day_15::execute(read_input_file("src/day_15/input.txt"))),
        "16" => println!("result : {:?}", day_16::execute(read_input_file("src/day_16/input.txt"))),
        "17" => println!("result : {:?}", day_17::execute(read_input_file("src/day_17/input.txt"))),
        _ => println!("Le numéro du jour est requis")
    }
}

fn read_input_file(file: &str) -> String {
    let mut file = File::open(file).expect("Error");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Error");
    buffer
}
