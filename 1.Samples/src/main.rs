extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::cmp::Ord;

fn main() {
    guessing_game();
}

fn guessing_game(){
    let secret_number = rand::thread_rng().gen_range(1, 51);

    loop {
        let mut guess = String::new(); // Important que ce mut soit dans la boucle

        println!("Choisissez un nombre");

        io::stdin().read_line(&mut guess)
            .expect("Impossible de lire la ligne");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num, // Le parse a bien fonctionné
            Err(e) => {
                println!("Erreur de parsing : {}", e);
                continue
            }
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("C'est gagné !");
                break
            }
        }
    }
}
