#[macro_use]
extern crate serde_derive;
extern crate reqwest;

fn main() {
    run(1);
}

#[derive(Deserialize, Debug)]
struct Task {
    userId: u32,
    id: u32,
    title : String,
    completed : bool
}

fn run(_id: u32) {
    let request_url = format!("https://jsonplaceholder.typicode.com/todos/{id}",
                              id = _id );
    println!("{}", request_url);
    match reqwest::get(&request_url) {
        Ok(mut something) => {
            let task: Task = something.json().unwrap();
            println!("{:?}", task);
        },
        Err(e) => {
            println!("error : {}", e);
        }
    }


}
