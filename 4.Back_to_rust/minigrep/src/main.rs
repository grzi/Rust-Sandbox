use std::{env, process};

fn main() {
	let args: Vec<String> = env::args().collect();
	let request = minigrep::parse_args(&args).unwrap_or_else(|e| {
		eprintln!("Problem parsing arguments {:?} -> {}", args, e);
		process::exit(1);
	});

	minigrep::run(request).unwrap_or_else( |e| {
		eprintln!("Error during mini grep : {}", e);
		process::exit(1);
	});
}
