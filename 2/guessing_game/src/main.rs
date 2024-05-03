use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("ADIVINA EL NUMERO");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("INGRESA LA CONJETURA.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("MUY PEQUEÑO!"),
            Ordering::Greater => println!("MUY GRANDE!"),
            Ordering::Equal => {
                println!("HAZ GANADO!");
                break;
            }
        }
    }
}