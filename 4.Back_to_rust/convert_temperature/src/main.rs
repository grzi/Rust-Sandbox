use std::io;

fn main() {
    println!("Hello, enter a temperature!");
	let mut val= String::new();

	io::stdin().read_line(&mut val)
		.expect("Failed to read input");

	let val:u32 = val.trim().parse().expect("Number expected");

	println!("Calculated : {}", convert(val))
	
}

fn convert(val: u32) -> u32 {
	val - 32
}
