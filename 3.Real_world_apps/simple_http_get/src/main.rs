extern crate reqwest;
use std::io::Read;

fn main() {
    match reqwest::get("https://jsonplaceholder.typicode.com/todos/1"){
        Ok(mut thing) => {
            let mut body = String::new();
            let test = thing.read_to_string(&mut body);

            println!("Status: {}", thing.status());
            println!("Headers:\n{:#?}", thing.headers());
            println!("Body:\n{}", body);
        },
        Err(e) => {
            println!("Error : {}", e);
        }
    }
}
