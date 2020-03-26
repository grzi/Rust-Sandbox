use std::io;
use rand::{thread_rng, Rng};
use std::cmp;

fn main() {
	let mut rng = thread_rng();
	let generated:u32 = rng.gen_range(0, 100);

	
	loop {	
    	
		let mut guess = String::new();
    	println!("Guess a number");

		io::stdin() .read_line(&mut guess)
         	        .expect("Failed to read line");
		let guess: u32 = match guess.trim().parse(){
			Ok(num) => num,
			Err(_) => continue
		};

		match guess.cmp(&generated) {
			cmp::Ordering::Less => println!("Too small!"),
			cmp::Ordering::Greater => println!("Too big!"),
			cmp::Ordering::Equal => {
				println!("Yeaaaaaah");
				break;
			}
		} 
	}
}
