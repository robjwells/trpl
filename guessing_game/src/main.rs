use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number, 1-100!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let Ok(guess) = guess.trim().parse::<u8>() else {
            println!("Couldn't parse your guess as a number 0-255.");
            continue;
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
