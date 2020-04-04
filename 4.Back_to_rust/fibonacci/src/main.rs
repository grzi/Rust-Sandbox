fn main() {
    println!("Hello, world!");
	println!("fibo {}", fibo(23));
}


fn fibo(val:u32) -> u32 {
	match val {
		0 => 0,
		1 => 1,
		_ => {
			fibo(val-1) + fibo(val-2)
		}
	}
}
