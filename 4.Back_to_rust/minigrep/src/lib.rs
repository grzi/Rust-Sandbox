use std::{fs, error::Error, env};

pub struct Request {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool
}

impl Request {
	fn new (q: &String, f: &String) -> Request {	
		Request {
			query: q.clone(),
			filename: f.clone(),
			case_sensitive: env::var("CASE_INSENSITIVE").is_err()
		}
	}
}

pub fn parse_args(args: &Vec<String>) -> Result<Request, &'static str>{
	if args.len() < 3 {
		return Err("You didn't enter enough argument");
	}

	Ok(Request::new(&args[1], &args[2]))
}

pub fn run (request: Request) -> Result<(), Box<dyn Error>>{
	let text = fs::read_to_string(request.filename)?;
	

	let matching_lines = if request.case_sensitive {
		search(&request.query, &text)
	} else {
		search_case_insensitive(&request.query, &text)
	};

	
	for line in matching_lines {
		println!("{}",line);
	}

	Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
	let mut result = Vec::new();
	for line in contents.lines() {
		if line.contains(query) {
			result.push(line);
		}
	}
	result
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
		let mut result = Vec::new();
      	for line in contents.lines() {
			let to_check = &line.to_ascii_lowercase();
        	if to_check.contains(&query.to_ascii_lowercase()) {
            	result.push(line);
          	}
      	}
      	result	
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let query = "duct";

		let contents = "\
Rust:
safe, fast, productive.
Pick three.";
		assert_eq!(
			vec!["safe, fast, productive."],
			search(query, contents)
			);
	}

	#[test]
	fn case_sensitive() {
		let query = "dUct";

		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(
			Vec::<String>::new(),
			search(query, contents)
		);
	}

	#[test]
	fn case_insensitive() {
		let query = "dUct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(
			vec!["safe, fast, productive."],
			search_case_insensitive(query, contents)			
		);		
	}
}
